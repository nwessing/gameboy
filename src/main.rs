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

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate time;

use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;
use game_boy::GameBoy;
use cpu::InstructionSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a path to a GameBoy ROM file");
    }
    let game_file_path = args[1].to_string();
    let mut game_file = match fs::File::open(game_file_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x)
    };

    let mut game_buf = Vec::new();
    game_file.read_to_end(&mut game_buf).unwrap();

    let mut boot_file = fs::File::open("roms/boot_rom.gb").unwrap();
    let mut boot_buf = Vec::new();
    boot_file.read_to_end(&mut boot_buf).unwrap();

    let mut gb = GameBoy::new();
    let instruction_set = InstructionSet::new();

    gb.power_on();
    gb.load_boot_rom(&boot_buf);
    gb.load_rom(&game_buf);

    let mut clock = clock::Clock::new();
    let mut gpu = gpu::Gpu::new();
    let mut controller = controller::Controller::new();
    let mut debug_mode = false;

    let skip_boot = false;
    if skip_boot {
        gb.memory.set_byte(0xFF50, 1);
        gb.cpu.pc = 0x100;
    }

    clock.start();
    loop {
        let cycles_elapsed = execute_next_instruction(&mut gb, &instruction_set);

        clock.tick(&mut gb, cycles_elapsed);
        gpu.update(&mut gb, cycles_elapsed);
        gpu.check_input(&mut gb, &mut controller);
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
