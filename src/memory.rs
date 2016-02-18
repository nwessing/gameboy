use util::get_lower;
use util::get_upper;
use util::concat_bytes;

pub struct Memory {
    mem: Vec<u8>,
    use_mbc1: bool,
    boot_rom: Vec<u8>,
    selected_rom_bank: u8,
    rom_banks: Vec<Vec<u8>>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; 0x10000],
            boot_rom: vec![0; 0x100],
            use_mbc1: false,
            selected_rom_bank: 1,
            rom_banks: vec![]
        }
    }

    pub fn power_on(&mut self) {
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
        let mbc_type = rom_buf[0x147];
        let num_rom_banks: u32 = match rom_buf[0x148] {
            0 => 2,
            1 => 4,
            2 => 8,
            3 => 16,
            4 => 32,
            5 => 64,
            6 => 128,
            x => panic!("ROM specified incorrect size at 0x148: {}", x)
        };

        let ram_size = rom_buf[0x149];
        match ram_size {
            0 => println!("No External RAM"),
            1 => println!("RAM: 1 bank of 2kB"),
            2 => println!("RAM: 1 bank of 8kB"),
            3 => println!("RAM: 4 banks of 8kB (32Kb)"),
            4 => println!("RAM: 16 banks of 8kB (128Kb)"),
            x => panic!("ROM specified incorrect size for external RAM: {}", x)
        };
        
        self.use_mbc1 = mbc_type >= 1 && mbc_type < 4; 
        let rom_size = if self.use_mbc1 { 0x4000 } else { 0x8000 };
        for i in 0..rom_size {
            self.mem[i] = rom_buf[i];
        }

        println!("Game uses memory banking {}", mbc_type);
        println!("ROM banks = {}", num_rom_banks);
        if self.use_mbc1 {
            for i_bank in 1..num_rom_banks {
                let start = i_bank * 0x4000;
                let mut bank = vec![0; 0x4000];
                for i in start..(start + 0x4000) {
                    bank[(i - start) as usize] = rom_buf[i as usize];
                }
                self.rom_banks.push(bank);
            }
        }
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        if address < 0x100 && self.mem[0xFF50] == 0 {
            return self.boot_rom[address as usize];
        }

        if self.use_mbc1 && address >= 0x4000 && address < 0x8000 {
            //ROM banks
            return self.rom_banks[(self.selected_rom_bank - 1) as usize][(address - 0x4000) as usize];
        }

        if address >= 0xE000 && address < 0xFE00 {
            return self.mem[(address - 0x2000) as usize];
        }

        self.mem[address as usize]
    }

    pub fn set_byte(&mut self, address: u16, b: u8) {

        if address == 0xFF40 {
            println!("LCD Control {:08b}", self.mem[address as usize]);
        }

        if address == 0xFFFF {
            println!("IE {:08b}", b);
        }

        if address < 0x2000 {
            println!("RAM Enable: {}", b);
        }

        if address >= 0x2000 && address < 0x4000 {
            let b = b & 0x1F;
            self.selected_rom_bank = match b {
                0x00 => 0x01,
                0x20 => 0x21,
                0x40 => 0x41,
                0x60 => 0x61,
                x => x
            };
            println!("Selected ROM bank {}", self.selected_rom_bank);
            return;
        }

        if address >= 0xA000 && address < 0xC000 {
            println!("Selected RAM bank {}", b);
        }

        if address < 0x8000 {
            //Read only
            return;
        }

        if address == 0xFF04 {
            //Timer divider register
            self.mem[address as usize] = 0;
            return;
        }

        if address >= 0xFEA0 && address < 0xFF00 {
            //Unusable
            return;
        }

        if address == 0xFF44 {
            //LCDC Y-Coordinate is read only
            return;
        }

        if address == 0xFF41 {
            // lower three bits are read only
            let read_only_part = self.mem[0xFF41] & 0b111;
            self.mem[0xFF41 as usize] = (b & 0b1111_1000) | read_only_part;
            return;
        }

        if address >= 0xFE00 && address < 0xFEA0 {
            //Can only write OAM during HBLANK and BLANK
            let lcd_mode = self.mem[0xFF41] & 0b11;
            if lcd_mode == 2 || lcd_mode == 3 {
                return;
            }
        }

        if address >= 0x8000 && address < 0xA000 {
            // Cannot write VRAM during LCD mode 3 accessing VRAM
            let lcd_mode = self.mem[0xFF41] & 0b11;
            if lcd_mode == 3 {
                return;
            }
        }
        
        if address >= 0xE000 && address < 0xFE00 {
            self.mem[(address - 0x2000) as usize] = b;
            return;
        }

        if address == 0xFF44 {
            self.mem[address as usize] = 0;
            return;
        }

        if address == 0xFF46 {
            // OAM DMA transfer
            for trans_addr in 0x00..0xA0 {
                self.mem[(0xFE00 + (trans_addr as u16)) as usize] = self.mem[concat_bytes(b, trans_addr) as usize];
            }
            // return;
        }

        self.mem[address as usize] = b;
    }

    pub fn set_owned_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn get_word(&self, address: u16) -> u16 {
        let lower = self.get_byte(address);
        let upper = self.get_byte(address + 1);
        concat_bytes(upper, lower)
    }

    pub fn set_word(&mut self, address: u16, word: u16) {
        self.set_byte(address, get_lower(word));
        self.set_byte(address + 1, get_upper(word));
    }
}