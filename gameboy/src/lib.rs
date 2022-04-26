pub mod c_bindings;
pub mod cb_instructions;
pub mod clock;
pub mod controller;
pub mod cpu;
pub mod game_boy;
pub mod gpu;
pub mod instructions;
pub mod interrupts;
pub mod math;
pub mod mbc1;
pub mod memory;
pub mod sound;
pub mod tests;
pub mod util;

use sound::SoundController;

use crate::clock::Clock;
use crate::controller::Controller;
use crate::cpu::InstructionSet;
use crate::gpu::Gpu;

use crate::game_boy::GameBoy;
pub struct System {
    gameboy: GameBoy,
    gpu: Gpu,
    sound: SoundController,
    instruction_set: InstructionSet,
    clock: Clock,
    controller: Controller,
    debug_mode: bool,
    checkpoint: time::Instant,
    frame_count: u32,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct InputEvent {
    pub button: Button,
    pub state: ButtonState,
}

pub struct InitializationOptions<'a> {
    pub boot_rom: Option<&'a [u8]>,
    pub game_rom: &'a [u8],
    pub external_ram: Option<&'a [u8]>,
    pub debug_mode: bool,
    pub sound_frequency: u32,
}

impl System {
    pub fn new(options: InitializationOptions) -> Self {
        let mut gameboy = GameBoy::new();
        let instruction_set = InstructionSet::new();
        let clock = Clock::new();
        let gpu = Gpu::new();
        let sound = SoundController::new(options.sound_frequency);
        let controller = Controller::new();

        gameboy.power_on();

        if let Some(boot_rom) = options.boot_rom {
            gameboy.load_boot_rom(&boot_rom);
        } else {
            gameboy.memory.set_byte(0xFF50, 1);
            gameboy.cpu.pc = 0x100;
        }

        gameboy.load_rom(options.game_rom);
        if let Some(external_ram) = options.external_ram {
            if gameboy.memory.use_battery() {
                gameboy.load_save_data(external_ram);
            }
        }

        Self {
            gameboy,
            instruction_set,
            gpu,
            sound,
            clock,
            controller,
            debug_mode: options.debug_mode,
            checkpoint: time::Instant::now(),
            frame_count: 0,
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.gameboy.exit_requested()
    }

    pub fn request_exit(&mut self) {
        self.gameboy.request_exit();
    }

    /// Returns the external RAM bank data if the current loaded
    /// game supports that feature
    pub fn copy_external_ram_banks(&self) -> Option<Vec<u8>> {
        if self.gameboy.memory.use_battery() {
            Some(self.gameboy.memory.get_external_ram_banks())
        } else {
            None
        }
    }

    pub fn screen_width() -> u32 {
        160
    }

    pub fn screen_height() -> u32 {
        144
    }

    // pub fn num_samples(&self) -> usize {
    //     (self.sound.last_sample_output + 1) as usize
    // }

    /// Continue execution until a new frame is ready
    /// Returns whether the game is still running
    pub fn run_single_frame(
        &mut self,
        events: &[InputEvent],
        framebuffer: &mut [u8],
        sound_buffer: &mut Vec<u8>,
    ) -> bool {
        // self.sound.last_sample_output = -1;
        // self.sound.total_cycle_count = 0;

        for event in events {
            let is_pressed = event.state == ButtonState::Pressed;
            match event.button {
                Button::A => self.controller.a_changed(is_pressed),
                Button::B => self.controller.b_changed(is_pressed),
                Button::Start => self.controller.start_changed(is_pressed),
                Button::Select => self.controller.select_changed(is_pressed),
                Button::Up => self.controller.up_changed(is_pressed),
                Button::Down => self.controller.down_changed(is_pressed),
                Button::Left => self.controller.left_changed(is_pressed),
                Button::Right => self.controller.right_changed(is_pressed),
            }
        }

        loop {
            let cycles_elapsed = self.execute_next_instruction();

            self.clock.tick(&mut self.gameboy, cycles_elapsed);

            self.sound
                .update(&mut self.gameboy, sound_buffer, cycles_elapsed);
            let frame_end = self
                .gpu
                .update(&mut self.gameboy, framebuffer, cycles_elapsed);

            self.controller.update_joypad_register(&mut self.gameboy);
            crate::interrupts::check_interrupts(&mut self.gameboy);
            self.gameboy.memory.reset_triggers();

            if self.debug_mode {
                println!("{}", self.gameboy.cpu);
            }

            if self.gameboy.exit_requested() {
                return false;
            }

            if frame_end {
                self.frame_count += 1;
                // if self.frame_count > 10 {
                //     panic!("yeet");
                // }

                // if self.checkpoint.elapsed().whole_nanoseconds() >= 1_000_000_000 {
                //     let average = self.gpu.total_render_ns / self.gpu.scan_lines_rendered as i128;
                //     let frame_average =
                //         self.checkpoint.elapsed().whole_nanoseconds() / self.frame_count as i128;

                //     self.gpu.total_render_ns = 0;
                //     self.gpu.scan_lines_rendered = 0;
                //     self.frame_count = 0;
                //     self.checkpoint = time::Instant::now();
                //     println!(
                //         "Average per scan line = {}ns, per frame = {}us",
                //         average,
                //         frame_average / 1000
                //     );
                // }

                // println!("last samp {}", self.sound.last_sample_output);
                return true;
            }
        }
    }

    fn execute_next_instruction(&mut self) -> u8 {
        if self.gameboy.cpu.is_halted {
            return 4;
        }

        let mut opcode = self.gameboy.memory.get_byte(self.gameboy.cpu.pc);
        let use_cb = opcode == 0xCB;
        if use_cb {
            opcode = self.gameboy.memory.get_byte(self.gameboy.cpu.pc + 1);
        }
        let arg1 = self
            .gameboy
            .memory
            .get_byte(self.gameboy.cpu.pc + if use_cb { 2 } else { 1 });
        let arg2 = self
            .gameboy
            .memory
            .get_byte(self.gameboy.cpu.pc + if use_cb { 3 } else { 2 });

        let instruction = if use_cb {
            self.instruction_set.get_cb_instruction(opcode)
        } else {
            self.instruction_set.get_instruction(opcode)
        };

        let instruction = match instruction {
            Option::None => {
                // pause();
                if use_cb {
                    panic!(
                        "CB{:02X} instruction not implemented\n{}",
                        opcode, self.gameboy.cpu
                    )
                } else {
                    panic!(
                        "{:02X} instruction not implemented\n{}",
                        opcode, self.gameboy.cpu
                    )
                }
            }
            Option::Some(x) => x,
        };

        // println!("{} {:02X} {:02X}", instruction.name, arg1, arg2);
        self.gameboy.cpu.pc =
            self.gameboy.cpu.pc + (instruction.operand_length as u16) + if use_cb { 2 } else { 1 };
        (instruction.exec)(&mut self.gameboy, arg1, arg2);
        instruction.cycles
    }
}
