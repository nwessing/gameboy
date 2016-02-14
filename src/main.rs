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
use game_boy::GameBoy;

fn main() {
    let mut game_file = fs::File::open("roms/tetris.gb").unwrap();
    // let mut game_file = fs::File::open("roms/opus5.gb").unwrap();
    // let mut game_file = fs::File::open("roms/cpu_instrs/cpu_instrs.gb").unwrap();
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/01-special.gb").unwrap();
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/02-interrupts.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/03-op sp,hl.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/04-op r,imm.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/05-op rp.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/06-ld r,r.gb").unwrap();
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/08-misc instrs.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/09-op r,r.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/10-bit ops.gb").unwrap();    
    // let mut game_file = fs::File::open("roms/cpu_instrs/individual/11-op a,(hl).gb").unwrap();    

    let mut game_buf = Vec::new();
    game_file.read_to_end(&mut game_buf).unwrap();

    let mut boot_file = fs::File::open("roms/boot_rom.gb").unwrap();
    let mut boot_buf = Vec::new();
    boot_file.read_to_end(&mut boot_buf).unwrap();

    let mut gb = GameBoy::new();
    let instruction_set = cpu::InstructionSet::new();

    gb.power_on();
    gb.load_boot_rom(&boot_buf);
    gb.load_rom(&game_buf);

    let mut clock = clock::Clock::new();
    let mut gpu = gpu::Gpu::new();
    let mut controller = controller::Controller::new();
    let mut debug_mode = false;

    let skip_boot = true;
    if skip_boot {
        gb.memory.set_byte(0xFF50, 1);
        gb.cpu.pc = 0x100;
    }

    clock.start();
    loop {        

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

        if debug_mode {
            print!("\nExecuting instruction {} ", instruction.name);
            if instruction.operand_length == 1 {
                print!("0x{:02X}", arg1);
            }
            if instruction.operand_length == 2 {
                print!(" 0x{:02X}{:02X}", arg1, arg2);
            }
            println!("");
            pause();
        }
        
        gb.cpu.pc = gb.cpu.pc + (instruction.operand_length as u16) + if use_cb { 2 } else { 1 };
        
        (instruction.exec)(&mut gb, arg1, arg2);

        clock.tick(&mut gb, instruction.cycles);
        gpu.update(&mut gb, instruction.cycles);
        gpu.check_input(&mut gb, &mut controller);
        controller.update_joypad_register(&mut gb);
        interrupts::check_interrupts(&mut gb);

        if debug_mode {
            println!("{}", gb.cpu);
        }

        if gb.exit_requested() {
            break;
        }
        // if gb.clock.current_tick() >= (4_194_304) * 2 {
        //     let path = path::Path::new("out.txt");
        //     let mut file = fs::File::create(&path).unwrap();
        //     for i in 0..log.len() {
        //         file.write((log[i]).as_bytes());
        //     }
        //     return;
        // }
    }

}

fn pause() {
    let mut guess = String::new();
    println!("Paused");
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
}
