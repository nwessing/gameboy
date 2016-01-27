mod instructions;
mod cb_instructions;
mod cpu;
mod game_boy;
mod util;
mod memory;
mod math;
mod gpu;

#[macro_use]
extern crate glium;

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

    loop {        
        let mut opcode = gb.memory.get_byte(gb.cpu.pc);
        let use_cb = opcode == 0xCB;
        if use_cb {
            opcode = gb.memory.get_byte(gb.cpu.pc + 1);
        }
        let arg1 = gb.memory.get_byte(gb.cpu.pc + if use_cb { 2 } else { 1 });
        let arg2 = gb.memory.get_byte(gb.cpu.pc + if use_cb { 3 } else { 2 });
        let exec;
        let arg_len;
        {
            let instruction = if use_cb {
                gb.cpu.get_cb_instruction(opcode)
            } else {
                gb.cpu.get_instruction(opcode)
            };

            let instruction = match instruction {
                Option::None => if use_cb { panic!("CB{:02X} instruction not implemented", opcode) } else { panic!("{:02X} instruction not implemented", opcode) },
                Option::Some(x) => x,
            };
            arg_len = instruction.operand_length as u16;
            exec = instruction.exec;

            // print!("\nExecuting instruction {} ", instruction.name);
            // if arg_len == 1 {
            //     print!("0x{:02X}", arg1);
            // }
            // if arg_len == 2 {
            //     print!(" 0x{:02X}{:02X}", arg1, arg2);
            // }
            // println!("");

            // if gb.cpu.pc >= 0x95 && gb.cpu.pc < 0xA8 {
            //     pause();
            // }
        }
        
        gb.cpu.pc = gb.cpu.pc + arg_len + if use_cb { 2 } else { 1 };
        exec(&mut gb, arg1, arg2);

        gpu.draw_screen(&mut gb);

        // println!("{}", gb.cpu);
    }

}

fn pause() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
}
