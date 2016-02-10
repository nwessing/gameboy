use instructions::Instruction;
use instructions::get_instruction_set;
use cb_instructions::get_cb_instruction_set;
use std::fmt;
use util::get_upper;
use util::get_lower;
use util::set_upper;
use util::set_lower;
use util::concat_bytes;

pub struct FlagRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        FlagRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }

    pub fn value(&self) -> u8 {
        let mut val: u8 = 0;
        if self.zero {
            val |= 0x80;
        }
        if self.subtract {
            val |= 0x40;
        }
        if self.half_carry {
            val |= 0x20;
        }
        if self.carry {
            val |= 0x10;
        }
        val
    }

    pub fn set(&mut self, val: u8) {
        self.zero = val & 0x80 == 0x80;
        self.subtract = val & 0x40 == 0x40;
        self.half_carry = val & 0x20 == 0x20;
        self.carry = val & 0x10 == 0x10;
    }
}

impl fmt::Display for FlagRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.value();
        write!(f, "{:02X}", val)
    }
}

pub struct Cpu {
    a: u8,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
    pub interrupt_enable_master: bool,
    pub flag: FlagRegister
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "af: 0x{:02X}{}\nbc: 0x{:04X}\nde: 0x{:04X}\nhl: 0x{:04X}\nsp: 0x{:04X}\npc: 0x{:04X}\n", 
            self.get_a(), self.flag, self.bc, self.de, self.hl, self.sp, self.pc)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            interrupt_enable_master: false,
            flag: FlagRegister::new()
        }
    }

    pub fn get_af(&self) -> u16 {
        concat_bytes(self.a, self.flag.value())
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }
    
    pub fn get_b(&self) -> u8 {
        get_upper(self.bc)
    }

    pub fn get_d(&self) -> u8 {
        get_upper(self.de)
    }

    pub fn get_h(&self) -> u8 {
        get_upper(self.hl)
    }

    pub fn get_c(&self) -> u8 {
        get_lower(self.bc)
    }

    pub fn get_e(&self) -> u8 {
        get_lower(self.de)
    }

    pub fn get_l(&self) -> u8 {
        get_lower(self.hl)
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = get_upper(val);
        self.flag.set(get_lower(val));
    }

    pub fn set_a(&mut self, b: u8) {
        self.a = b;
    }

    pub fn set_b(&mut self, b: u8) {
        set_upper(&mut self.bc, b);
    }

    pub fn set_d(&mut self, b: u8) {
        set_upper(&mut self.de, b);
    }

    pub fn set_h(&mut self, b: u8) {
        set_upper(&mut self.hl, b);
    }

    pub fn set_c(&mut self, b: u8) {
        set_lower(&mut self.bc, b);
    }

    pub fn set_e(&mut self, b: u8) {
        set_lower(&mut self.de, b);
    }

    pub fn set_l(&mut self, b: u8) {
        set_lower(&mut self.hl, b);
    }

    pub fn power_on(&mut self) {
        self.set_af(0x01B0);
        self.bc = 0x0013;
        self.de = 0x00D8;
        self.hl = 0x014D;
        self.sp = 0xFFFE;
        self.pc = 0x0000;
    }
}

pub struct InstructionSet {
    instructions: Vec<Instruction>,
    cb_instructions: Vec<Instruction>
}

impl InstructionSet {
    pub fn new() -> InstructionSet {
        InstructionSet {
            instructions: get_instruction_set(),
            cb_instructions: get_cb_instruction_set()
        }
    }
    pub fn get_instruction(&self, opcode: u8) -> Option<&Instruction> {
        for ins in self.instructions.iter() {
            if ins.opcode == opcode {
                return Some(&ins);
            }
        }
        None
    }

    pub fn get_cb_instruction(&self, opcode: u8) -> Option<&Instruction> {
        for ins in self.cb_instructions.iter() {
            if ins.opcode == opcode {
                return Some(&ins);
            }
        }
        None
    }
}