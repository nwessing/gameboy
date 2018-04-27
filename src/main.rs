mod instructions;
mod cb_instructions;
mod cpu;
mod game_boy;
mod util;
mod memory;
mod math;
mod gpu;
mod clock;
mod interrupts;
mod controller;
mod tests;

extern crate sdl2;
extern crate time;

use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;
use game_boy::GameBoy;
use controller::Controller;
use cpu::InstructionSet;

use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a path to a GameBoy ROM file");
    }

    let game_file_path = args[1].to_string();
    let game_buf = load_rom(&game_file_path);

    let mut boot_buf = Vec::new();
    match fs::File::open("roms/boot_rom.gb") {
        Ok(mut boot_file) => { boot_file.read_to_end(&mut boot_buf).unwrap(); },
        Err(_) => {}
    }

    let mut gb = GameBoy::new();
    gb.power_on();
    let skip_boot = if boot_buf.len() > 0 {
        gb.load_boot_rom(&boot_buf);
        false
    } else {
        true
    };

    gb.load_rom(&game_buf);

    let instruction_set = InstructionSet::new();
    let mut clock = clock::Clock::new();
    let mut gpu = gpu::Gpu::new();
    let mut controller = Controller::new();
    let debug_mode = false;

    if skip_boot {
        gb.memory.set_byte(0xFF50, 1);
        gb.cpu.pc = 0x100;
    }

    let sdl_context = sdl2::init().unwrap();//.ok_or("Could not create SDL Context.");
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Gameboy Emulator", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();


    clock.start();
    loop {
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
            if use_cb { panic!("CB{:02X} instruction not implemented\n{}", opcode, gb.cpu) } else { panic!("{:02X} instruction not implemented\n{}", opcode, gb.cpu) }
        },
        Option::Some(x) => x,
    };

    gb.cpu.pc = gb.cpu.pc + (instruction.operand_length as u16) + if use_cb { 2 } else { 1 };
    (instruction.exec)(&mut gb, arg1, arg2);
    instruction.cycles
}

fn pause() {
    let mut guess = String::new();
    println!("Paused");
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
}

fn load_rom(game_file_path: &String) -> Vec<u8> {
    let mut game_file = match fs::File::open(game_file_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x)
    };

    let mut game_buf = Vec::new();
    game_file.read_to_end(&mut game_buf).unwrap();
    game_buf
}

fn render_frame(canvas: &mut WindowCanvas, frame_buffer: &[u8; gpu::BUFFER_SIZE]) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 160, 144).unwrap();
    texture.with_lock(None, |buffer: &mut [u8], _: usize| {
        for i in 0..frame_buffer.len() {
            buffer[i] = frame_buffer[i];
        }
    }).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

pub fn check_input(event_pump: &mut EventPump, gb: &mut GameBoy, controller: &mut Controller) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => gb.request_exit(),
            Event::KeyDown { keycode, .. } => handle_input(controller, true, keycode),
            Event::KeyUp { keycode, .. } => handle_input(controller, false, keycode),
            _ => ()
        }
    }
}

fn handle_input(controller: &mut Controller, pressed: bool, key: Option<Keycode>) {
    let keycode = match key {
        Some(keycode) => keycode,
        None => return
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
        _ => ()
    }
}