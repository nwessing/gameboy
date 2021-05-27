use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct GameBoy {
    pub cpu: Cpu,
    pub memory: Memory,
    exit_requested: bool,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            exit_requested: false,
        }
    }

    pub fn power_on(&mut self) {
        self.cpu.power_on();
        self.memory.power_on();
    }

    pub fn load_boot_rom(&mut self, boot_buf: &[u8]) {
        self.memory.load_boot_rom(boot_buf);
    }

    pub fn load_rom(&mut self, rom_buf: &[u8]) {
        self.memory.load_rom(rom_buf);
    }

    pub fn load_save_data(&mut self, save_buf: &[u8]) {
        self.memory.load_external_ram(save_buf);
    }

    pub fn request_exit(&mut self) {
        self.exit_requested = true;
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }
}
