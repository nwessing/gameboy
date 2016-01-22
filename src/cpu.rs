use instructions::Instruction;
use instructions::get_instruction_set;
use std::fmt;
use util::get_upper;
use util::get_lower;
use util::set_upper;
use util::set_lower;

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
}

impl fmt::Display for FlagRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        write!(f, "{:02X}", val)
    }
}

pub struct Cpu {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
    pub flag: FlagRegister,
    pub instructions: Vec<Instruction>,
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
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            flag: FlagRegister::new(),
            instructions: get_instruction_set()
        }
    }

    pub fn get_a(&self) -> u8 {
        get_upper(self.af)
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

    // pub fn get_f(&self) -> u8 {
    //     get_lower(self.af)
    // }

    pub fn get_c(&self) -> u8 {
        get_lower(self.bc)
    }

    pub fn get_e(&self) -> u8 {
        get_lower(self.de)
    }

    pub fn get_l(&self) -> u8 {
        get_lower(self.hl)
    }

    pub fn set_a(&mut self, b: u8) {
        set_upper(&mut self.af, b);
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

    // pub fn set_f(&mut self, b: u8) {
    //     set_lower(&mut self.af, b);
    // }

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
        self.af = 0x01B0;
        self.bc = 0x0013;
        self.de = 0x00D8;
        self.hl = 0x014D;
        self.sp = 0xFFFE;
        self.pc = 0x0100;
    }

    pub fn get_instruction(&self, opcode: u8) -> Option<&Instruction> {
        for ins in &self.instructions {
            if ins.opcode == opcode {
                return Option::Some(&ins);
            }
        }
        Option::None
    }
}
