use util::get_lower;
use util::get_upper;
use util::concat_bytes;

pub struct Memory {
    mem: Vec<u8>,
    boot_rom: Vec<u8>

}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; 0x10000],
            boot_rom: vec![0; 0x100]
        }
    }

    pub fn power_on(&mut self) {
        self.mem[0xFF05] = 0x00;
        self.mem[0xFF05] = 0x00;
        self.mem[0xFF06] = 0x00;
        self.mem[0xFF07] = 0x00;
        self.mem[0xFF10] = 0x80;
        self.mem[0xFF11] = 0xBF;
        self.mem[0xFF12] = 0xF3;
        self.mem[0xFF14] = 0xBF;
        self.mem[0xFF16] = 0x3F;
        self.mem[0xFF17] = 0x00;
        self.mem[0xFF19] = 0xBF;
        self.mem[0xFF1A] = 0x7F;
        self.mem[0xFF1B] = 0xFF;
        self.mem[0xFF1C] = 0x9F;
        self.mem[0xFF1E] = 0xBF;
        self.mem[0xFF20] = 0xFF;
        self.mem[0xFF21] = 0x00;
        self.mem[0xFF22] = 0x00;
        self.mem[0xFF23] = 0xBF;
        self.mem[0xFF24] = 0x77;
        self.mem[0xFF25] = 0xF3;
        self.mem[0xFF26] = 0xF1;
        self.mem[0xFF40] = 0x91;
        self.mem[0xFF42] = 0x00;
        self.mem[0xFF43] = 0x00;
        self.mem[0xFF45] = 0x00;
        self.mem[0xFF47] = 0xFC;
        self.mem[0xFF48] = 0xFF;
        self.mem[0xFF49] = 0xFF;
        self.mem[0xFF4A] = 0x00;
        self.mem[0xFF4B] = 0x00;
        self.mem[0xFFFF] = 0x00;
    }

    pub fn load_boot_rom(&mut self, boot_buf: &Vec<u8>) {
        for i in 0..boot_buf.len() {
            self.boot_rom[i] = boot_buf[i];
        }
    }

    pub fn load_rom(&mut self, rom_buf: &Vec<u8>) {
        for i in 0..rom_buf.len() {
            self.mem[i] = rom_buf[i];
        }
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        if address < 0x100 && self.mem[0xFF50] == 0 {
            return self.boot_rom[address as usize];
        }
        self.mem[address as usize]
    }

    pub fn set_byte(&mut self, address: u16, b: u8) {
        self.mem[address as usize] = b;
    }

    pub fn get_word(&self, address: u16) -> u16 {
        let lower = self.get_byte(address);
        let upper = self.get_byte(address + 1);
        concat_bytes(upper, lower)
    }

    pub fn set_word(&mut self, address: u16, word: u16) {
        self.mem[address as usize] = get_lower(word);
        self.mem[(address + 1) as usize] = get_upper(word);
    }
}