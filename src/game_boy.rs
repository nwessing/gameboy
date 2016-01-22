use cpu::Cpu;
use memory::Memory;

pub struct GameBoy {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl GameBoy {
    pub fn new() -> GameBoy{
        GameBoy {
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }

    pub fn power_on(&mut self) {
        self.cpu.power_on();
    }

    pub fn load_rom(&mut self, rom_buf: &Vec<u8>) {
        self.memory.load_rom(rom_buf);
    }
}