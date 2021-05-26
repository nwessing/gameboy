mod cb_instructions;
mod clock;
mod controller;
mod cpu;
mod game_boy;
mod gpu;
mod instructions;
mod interrupts;
mod math;
mod mbc1;
mod memory;
mod tests;
mod util;

extern crate sdl2;
extern crate time;

use controller::Controller;
use cpu::InstructionSet;
use fs::File;
use game_boy::GameBoy;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a path to a GameBoy ROM file");
    }

    let game_file_path = Path::new(&args[1]);

    let mut gb = GameBoy::new();
    let skip_boot = true;
    gb.power_on();

    if !skip_boot {
        load_boot_rom(&mut gb);
    }
    load_rom(&mut gb, game_file_path);
    load_external_ram(&mut gb, &game_file_path);

    let instruction_set = InstructionSet::new();
    let mut clock = clock::Clock::new();
    let mut gpu = gpu::Gpu::new();
    let mut controller = Controller::new();
    let debug_mode = false;

    if skip_boot {
        gb.memory.set_byte(0xFF50, 1);
        gb.cpu.pc = 0x100;
    }

    let sdl_context = sdl2::init().unwrap(); //.ok_or("Could not create SDL Context.");
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Gameboy Emulator", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    clock.start();
    loop {
        // pause();
        let cycles_elapsed = execute_next_instruction(&mut gb, &instruction_set);

        clock.tick(&mut gb, cycles_elapsed);
        if gpu.update(&mut gb, cycles_elapsed) {
            render_frame(&mut canvas, &gpu.window_buf);
        }

        check_input(&mut event_pump, &mut gb, &mut controller);
        controller.update_joypad_register(&mut gb);
        interrupts::check_interrupts(&mut gb);

        if debug_mode {
            println!("{}", gb.cpu);
        }

        if gb.exit_requested() {
            break;
        }
    }
    save_external_ram(&gb, &game_file_path);
}

fn execute_next_instruction(mut gb: &mut GameBoy, instruction_set: &InstructionSet) -> u8 {
    if gb.cpu.is_halted {
        return 4;
    }

    let mut opcode = gb.memory.get_byte(gb.cpu.pc);
    let use_cb = opcode == 0xCB;
    if use_cb {
        opcode = gb.memory.get_byte(gb.cpu.pc + 1);
    }
    let arg1 = gb.memory.get_byte(gb.cpu.pc + if use_cb { 2 } else { 1 });
    let arg2 = gb.memory.get_byte(gb.cpu.pc + if use_cb { 3 } else { 2 });

    let instruction = if use_cb {
        instruction_set.get_cb_instruction(opcode)
    } else {
        instruction_set.get_instruction(opcode)
    };

    let instruction = match instruction {
        Option::None => {
            pause();
            if use_cb {
                panic!("CB{:02X} instruction not implemented\n{}", opcode, gb.cpu)
            } else {
                panic!("{:02X} instruction not implemented\n{}", opcode, gb.cpu)
            }
        }
        Option::Some(x) => x,
    };

    // println!("{} {:02X} {:02X}", instruction.name, arg1, arg2);
    gb.cpu.pc = gb.cpu.pc + (instruction.operand_length as u16) + if use_cb { 2 } else { 1 };
    (instruction.exec)(&mut gb, arg1, arg2);
    instruction.cycles
}

fn load_rom(gb: &mut GameBoy, game_file_path: &Path) {
    let mut game_file = match File::open(game_file_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    };

    let mut game_buf = Vec::new();
    game_file.read_to_end(&mut game_buf).unwrap();
    gb.load_rom(&game_buf);
}

fn load_boot_rom(gb: &mut GameBoy) {
    let mut boot_file = fs::File::open("roms/boot_rom.gb").unwrap();
    let mut boot_buf = Vec::new();
    boot_file.read_to_end(&mut boot_buf).unwrap();
    gb.load_boot_rom(&boot_buf);
}

fn load_external_ram(gb: &mut GameBoy, game_file_path: &Path) {
    if gb.memory.use_battery() {
        let game_save_path = game_file_path.with_extension("gbsave");
        match File::open(game_save_path) {
            Ok(mut game_save_file) => {
                let mut save_buf = Vec::new();
                game_save_file.read_to_end(&mut save_buf).unwrap();
                gb.load_save_data(&save_buf);
            }
            Err(_) => {}
        };
    }
}

fn save_external_ram(gb: &GameBoy, game_file_path: &Path) {
    if gb.memory.use_battery() {
        let game_save_path = game_file_path.with_extension("gbsave");
        let mut save_file = match File::create(game_save_path) {
            Ok(x) => x,
            Err(x) => panic!("{}", x),
        };

        match save_file.write_all(gb.memory.get_external_ram_banks().as_slice()) {
            Ok(x) => x,
            Err(x) => panic!("{}", x),
        };
    }
}

fn pause() {
    let mut guess = String::new();
    println!("Paused");
    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
}

fn render_frame(canvas: &mut WindowCanvas, frame_buffer: &[u8; gpu::BUFFER_SIZE]) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
        .unwrap();
    texture
        .with_lock(None, |buffer: &mut [u8], _: usize| {
            for i in 0..frame_buffer.len() {
                buffer[i] = frame_buffer[i];
            }
        })
        .unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

pub fn check_input(event_pump: &mut EventPump, gb: &mut GameBoy, controller: &mut Controller) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => gb.request_exit(),
            Event::KeyDown { keycode, .. } => handle_input(controller, true, keycode),
            Event::KeyUp { keycode, .. } => handle_input(controller, false, keycode),
            _ => (),
        }
    }
}

fn handle_input(controller: &mut Controller, pressed: bool, key: Option<Keycode>) {
    let keycode = match key {
        Some(keycode) => keycode,
        None => return,
    };

    match keycode {
        Keycode::W => controller.up_changed(pressed),
        Keycode::A => controller.left_changed(pressed),
        Keycode::S => controller.down_changed(pressed),
        Keycode::D => controller.right_changed(pressed),
        Keycode::M => controller.b_changed(pressed),
        Keycode::K => controller.a_changed(pressed),
        Keycode::J => controller.start_changed(pressed),
        Keycode::H => controller.select_changed(pressed),
        _ => (),
    }
}

