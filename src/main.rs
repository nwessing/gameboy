mod instructions;
mod cpu;
mod game_boy;
mod util;
mod memory;

use std::io::prelude::*;
use std::fs;
use game_boy::GameBoy;

fn main() {
    let mut file = fs::File::open("roms/Tetris (JUE) (V1.1) [!].gb").unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    let mut gb = GameBoy::new();
    gb.power_on();
    gb.load_rom(&file_buf);

    loop {
        let opcode = gb.memory.get_byte(gb.cpu.pc);
        let arg1 = gb.memory.get_byte(gb.cpu.pc + 1);
        let arg2 = gb.memory.get_byte(gb.cpu.pc + 2);
        let exec;
        let arg_len;
        {
            let instruction = gb.cpu.get_instruction(opcode);
            let instruction = match instruction {
                Option::None => panic!("{:02X} instruction not implemented", opcode),
                Option::Some(x) => x,
            };
            arg_len = instruction.operand_length as u16;
            exec = instruction.exec;
            print!("\nExecuting instruction {} ", instruction.name);
            if arg_len == 1 {
                print!("0x{:02X}", arg1);
            }
            if arg_len == 2 {
                print!(" 0x{:02X}{:02X}", arg1, arg2);
            }
            println!("");
        }
        
        gb.cpu.pc = gb.cpu.pc + 1 + arg_len
;        exec(&mut gb, arg1, arg2);
        println!("{}", gb.cpu);
    }

}
