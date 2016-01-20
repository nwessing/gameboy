use std::io::prelude::*;
use std::fs;

struct Cpu {
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
    exec: fn(&mut GameBoy, u8, u8)
}

impl Instruction {
    fn new(name: &'static str, opcode: u8, operand_length: u8, exec: fn(&mut GameBoy, u8, u8)) -> Instruction {
        Instruction {
            name: name,
            opcode: opcode,
            operand_length: operand_length,
            exec: exec,
        }
    }
}

fn get_upper(b: u16) -> u8 {
    (b & 0xFF00 >> 8) as u8
}

fn get_lower(b: u16) -> u8 {
    (b & 0x00FF) as u8
}

fn set_upper(to_set: &mut u16, b: u8) {
    *to_set = (*to_set & 0x00FF) + ((b as u16) << 8);
}

fn set_lower(to_set: &mut u16, b: u8) {
    *to_set = (*to_set & 0xFF00) + (b as u16);
}
 
impl Cpu {
    fn new() -> Cpu {
        Cpu {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            instructions: vec![
                Instruction::new("NOP", 0x00, 0, nop),
                Instruction::new("LD (BC),A", 0x02, 0, nop),
                Instruction::new("INC C", 0x0C, 0, increment_c),
                Instruction::new("JMP nn", 0xC3, 1, jump_immediate),
            ]
        }
    }

    fn get_a(&self) -> u8 {
        get_upper(self.af)
    }
    
    fn get_b(&self) -> u8 {
        get_upper(self.bc)
    }

    fn get_d(&self) -> u8 {
        get_upper(self.de)
    }

    fn get_h(&self) -> u8 {
        get_upper(self.hl)
    }

    fn get_f(&self) -> u8 {
        get_lower(self.af)
    }

    fn get_c(&self) -> u8 {
        get_lower(self.bc)
    }

    fn get_e(&self) -> u8 {
        get_lower(self.de)
    }

    fn get_l(&self) -> u8 {
        get_lower(self.hl)
    }

    fn set_a(&mut self, b: u8) {
        set_upper(&mut self.af, b);
    }

    fn set_b(&mut self, b: u8) {
        set_upper(&mut self.bc, b);
    }

    fn set_d(&mut self, b: u8) {
        set_upper(&mut self.de, b);
    }

    fn set_h(&mut self, b: u8) {
        set_upper(&mut self.hl, b);
    }

    fn set_f(&mut self, b: u8) {
        set_lower(&mut self.af, b);
    }

    fn set_c(&mut self, b: u8) {
        set_lower(&mut self.bc, b);
    }

    fn set_e(&mut self, b: u8) {
        set_lower(&mut self.de, b);
    }

    fn set_l(&mut self, b: u8) {
        set_lower(&mut self.hl, b);
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

fn nop(_: &mut GameBoy, _: u8, _: u8) {

}

fn jump_immediate(gb: &mut GameBoy, a1: u8, a2: u8) {
    let new_val = ((a2 as u16) << 8) + (a1 as u16);
    gb.cpu.pc = new_val;
}

fn increment_c(gb: &mut GameBoy, _: u8, _: u8) {
    let c = gb.cpu.get_c();
    gb.cpu.set_c(c + 1);
}

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

    fn get_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
}

struct GameBoy {
    cpu: Cpu,
    memory: Memory,
}

impl GameBoy {
    fn new() -> GameBoy{
        GameBoy {
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }

    fn power_on(&mut self) {
        self.cpu.power_on();
    }

    fn load_rom(&mut self, rom_buf: &Vec<u8>) {
        self.memory.load_rom(rom_buf);
    }
}

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
        {
            let instruction = gb.cpu.get_instruction(opcode);
            let instruction = match instruction {
                Option::None => panic!("{:X} instruction not implemented", opcode),
                Option::Some(x) => x,
            };
            exec = instruction.exec;
        }
        
        exec(&mut gb, arg1, arg2);

        gb.cpu.pc = gb.cpu.pc + 1;
    }

}
