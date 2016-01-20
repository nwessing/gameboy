use std::io::prelude::*;
use std::fs;

struct CpuRegs {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    instructions: Vec<Instruction>,
}

struct Instruction {
    name: &'static str,
    opcode: u8,
    operand_length: u8,
    exec: fn(u8, u8)
}

impl Instruction {
    fn new(name: &'static str, opcode: u8, operand_length: u8, exec: fn(u8, u8)) -> Instruction {
        Instruction {
            name: name,
            opcode: opcode,
            operand_length: operand_length,
            exec: exec,
        }
    }
}

impl CpuRegs {
    fn new() -> CpuRegs {
        CpuRegs {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            instructions: vec![
                Instruction::new("NOP", 0x00, 0, nop),
                Instruction::new("JMP nn", 0xC3, 1, nop),
            ]
        }
    }

    fn power_on(&mut self) {
        self.af = 0x01B0;
        self.bc = 0x0013;
        self.de = 0x00D8;
        self.hl = 0x014D;
        self.sp = 0xFFFE;
        self.pc = 0x0100;
    }

    fn get_instruction(&self, opcode: u8) -> Option<&Instruction> {
        for ins in &self.instructions {
            if ins.opcode == opcode {
                return Option::Some(&ins);
            }
        }
        Option::None
    }
}

fn nop(_: u8, _: u8) {

}

// fn jump_immediate(&mut self, address: usize) {
//     self.pc = address;
// }

struct Memory {
    mem: Vec<u8>
}

impl Memory {
    fn new() -> Memory {
        Memory {
            mem: vec![0; 0xFFFF]
        }
    }

    fn load_rom(&mut self, rom_buf: &Vec<u8>) {
        for i in 0..rom_buf.len() {
            self.mem[i] = rom_buf[i];
        }
    }
}

fn main() {
    let mut file = fs::File::open("roms/Tetris (JUE) (V1.1) [!].gb").unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    let mut cpu = CpuRegs::new();
    cpu.power_on();
    let mut mem = Memory::new();
    mem.load_rom(&file_buf);

    loop {
        let opcode = mem.mem[cpu.pc as usize];
        let arg1 = mem.mem[(cpu.pc + 1) as usize];
        let arg2 = mem.mem[(cpu.pc + 2) as usize];
        {
            let instruction = cpu.get_instruction(opcode);
            let instruction = match instruction {
                Option::None => panic!("{:X} instruction not implemented", opcode),
                Option::Some(x) => x,
            };
            let exec = instruction.exec;
            exec(arg1, arg2);
        }

        cpu.pc = cpu.pc + 1;
    }

}
