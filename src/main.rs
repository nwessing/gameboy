use std::io::prelude::*;
use std::fs;
use std::fmt;

struct FlagRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

impl FlagRegister {
    fn new() -> FlagRegister {
        FlagRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

struct Cpu {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    flag: FlagRegister,
    instructions: Vec<Instruction>,
}

struct Instruction {
    name: &'static str,
    opcode: u8,
    operand_length: u8,
    cycles: u8,
    exec: fn(&mut GameBoy, u8, u8)
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "af: 0x{:04X}\nbc: 0x{:04X}\nde: 0x{:04X}\nhl: 0x{:04X}\nsp: 0x{:04X}\npc: 0x{:04X}\n", 
            self.af, self.bc, self.de, self.hl, self.sp, self.pc)
    }
}

impl Instruction {
    fn new(name: &'static str, opcode: u8, operand_length: u8, cycles: u8, exec: fn(&mut GameBoy, u8, u8)) -> Instruction {
        Instruction {
            name: name,
            opcode: opcode,
            operand_length: operand_length,
            cycles: cycles,
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
            flag: FlagRegister::new(),
            instructions: vec![
                Instruction::new("NOP", 0x00, 0, 4, nop),
                
                Instruction::new("LD A,A", 0x7F, 0, 4, load_a_a),
                Instruction::new("LD B,A", 0x47, 0, 4, load_b_a),
                Instruction::new("LD C,A", 0x4F, 0, 4, load_c_a),
                Instruction::new("LD D,A", 0x57, 0, 4, load_d_a),
                Instruction::new("LD E,A", 0x5F, 0, 4, load_e_a),
                Instruction::new("LD H,A", 0x67, 0, 4, load_h_a),
                Instruction::new("LD L,A", 0x6f, 0, 4, load_l_a),
                Instruction::new("LD (BC),A", 0x02, 0, 8, load_bc_a),
                // Instruction::new("LD (DE),A", 0x12, 0, 8, load_bc_a),
                // Instruction::new("LD (HL),A", 0x77, 0, 8, load_bc_a),
                // Instruction::new("LD (nn),A", 0xEA, 0, 16, load_bc_a),


                Instruction::new("INC A", 0x3C, 0, 4, increment_a),
                Instruction::new("INC B", 0x04, 0, 4, increment_b),
                Instruction::new("INC C", 0x0C, 0, 4, increment_c),
                Instruction::new("INC D", 0x14, 0, 4, increment_d),
                Instruction::new("INC E", 0x1C, 0, 4, increment_e),
                Instruction::new("INC H", 0x24, 0, 4, increment_h),
                Instruction::new("INC L", 0x2C, 0, 4, increment_l),
                // Instruction::new("INC (HL)", 0x34, 0, 12, increment_hl),
                Instruction::new("JP nn", 0xC3, 2, 12, jump_immediate),
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

fn call(gb: &mut GameBoy, a1: u8, a2: u8) {

    jump_immediate(gb, a1, a2);
}

fn increment(getter: fn(&Cpu) -> u8, setter: fn(&mut Cpu, u8), gb: &mut GameBoy) {
    let mut reg_val = getter(&gb.cpu);

    if reg_val == 0xFF {
        gb.cpu.flag.zero = true;
        gb.cpu.flag.half_carry = true;
        reg_val = 0;
    } else {
        reg_val += 1;
    }

    gb.cpu.flag.subtract = false;
    setter(&mut gb.cpu, reg_val);
}

fn increment_a(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_a, Cpu::set_a, gb);
}

fn increment_b(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_b, Cpu::set_b, gb);
}

fn increment_c(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_c, Cpu::set_c, gb);
}

fn increment_d(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_d, Cpu::set_d, gb);
}

fn increment_e(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_e, Cpu::set_e, gb);
}

fn increment_h(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_h, Cpu::set_h, gb);
}

fn increment_l(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_l, Cpu::set_l, gb);
}

fn load_bc_a(gb: &mut GameBoy, _: u8, _: u8) {
    gb.memory.set_byte(gb.cpu.bc, gb.cpu.get_a());
}

fn load_a_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_a(val);
}

fn load_b_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_b(val);
}

fn load_c_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_c(val);
}

fn load_d_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_d(val);
}

fn load_e_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_e(val);
}

fn load_h_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_h(val);
}

fn load_l_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_l(val);
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

    fn set_byte(&mut self, address: u16, b: u8) {
        self.mem[address as usize] = b;
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
        let arg_len;
        {
            let instruction = gb.cpu.get_instruction(opcode);
            let instruction = match instruction {
                Option::None => panic!("{:X} instruction not implemented", opcode),
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
        
        gb.cpu.pc = gb.cpu.pc + 1 + arg_len;
        exec(&mut gb, arg1, arg2);
        println!("cpu: {}", gb.cpu);
    }

}
