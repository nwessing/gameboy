pub struct Memory {
    pub mem: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; 0xFFFF]
        }
    }

    pub fn load_rom(&mut self, rom_buf: &Vec<u8>) {
        for i in 0..rom_buf.len() {
            self.mem[i] = rom_buf[i];
        }
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn set_byte(&mut self, address: u16, b: u8) {
        self.mem[address as usize] = b;
    }
}