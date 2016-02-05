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

#[macro_use]
extern crate glium;
extern crate time;

use std::io;
use std::io::prelude::*;
use std::fs;
use game_boy::GameBoy;

fn main() {
    let mut game_file = fs::File::open("roms/Tetris (JUE) (V1.1) [!].gb").unwrap();
    let mut game_buf = Vec::new();
    game_file.read_to_end(&mut game_buf).unwrap();

    let mut boot_file = fs::File::open("roms/boot_rom.gb").unwrap();
    let mut boot_buf = Vec::new();
    boot_file.read_to_end(&mut boot_buf).unwrap();

    let mut gb = GameBoy::new();
    gb.power_on();
    gb.load_boot_rom(&boot_buf);
    gb.load_rom(&game_buf);

    let mut gpu = gpu::Gpu::new();
    let mut debug_mode = false;

    gb.clock.start();
    loop {        
        let mut opcode = gb.memory.get_byte(gb.cpu.pc);
        let use_cb = opcode == 0xCB;
        if use_cb {
            opcode = gb.memory.get_byte(gb.cpu.pc + 1);
        }
        let arg1 = gb.memory.get_byte(gb.cpu.pc + if use_cb { 2 } else { 1 });
        let arg2 = gb.memory.get_byte(gb.cpu.pc + if use_cb { 3 } else { 2 });
        let exec; 
        let num_cycles;
        let arg_len;
        {
            let instruction = if use_cb {
                gb.cpu.get_cb_instruction(opcode)
            } else {
                gb.cpu.get_instruction(opcode)
            };

            let instruction = match instruction {
                Option::None => if use_cb { panic!("CB{:02X} instruction not implemented\n{}", opcode, gb.cpu) } else { panic!("{:02X} instruction not implemented\n{}", opcode, gb.cpu) },
                Option::Some(x) => x,
            };
            arg_len = instruction.operand_length as u16;
            exec = instruction.exec;
            num_cycles = instruction.cycles;

            if debug_mode {
                print!("\nExecuting instruction {} ", instruction.name);
                if arg_len == 1 {
                    print!("0x{:02X}", arg1);
                }
                if arg_len == 2 {
                    print!(" 0x{:02X}{:02X}", arg1, arg2);
                }
                println!("");
                pause();
            }
        }
        
        let pc = gb.cpu.pc;
        gb.cpu.pc = gb.cpu.pc + arg_len + if use_cb { 2 } else { 1 };
        let prev = gb.cpu.get_a();
        exec(&mut gb, arg1, arg2);

        // if gb.cpu.pc == 0x6841 {
        //     pause();
        // }
        
        // if gb.cpu.get_a() == 0x20 && prev != 0x20 {
        //     println!("Just executed {:02X} arg:{:02x}, pc = {:04X}\n{}, prev was{:02X}", opcode, arg1, pc, gb.cpu, prev);
        //     // debug_mode = true;
        //     pause();
        // }

        // if pc == 0x29D4 {
        //     println!("29D4 reached");
        // }
        
        // if gb.cpu.pc == 0x0231 {
        //     debug_mode = true;
        // }

        gb.clock.tick(num_cycles);
        gpu.update(&mut gb);
        interrupts::check_interrupts(&mut gb);

        if debug_mode {
            println!("{}", gb.cpu);
        }
    }

}

fn pause() {
    let mut guess = String::new();
    println!("Paused");
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
}
