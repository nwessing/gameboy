use crate::mbc1::MemoryBankController1;
use crate::util::concat_bytes;
use crate::util::get_lower;
use crate::util::get_upper;

#[repr(u16)]
pub enum Register {
    SpriteData = 0xFE00,
    Joypad = 0xFF00,
    Divider = 0xFF04,
    TimerCounter = 0xFF05,
    TimerModulo = 0xFF06,
    TimerControl = 0xFF07,
    InterruptFlag = 0xFF0F,

    LcdControl = 0xFF40,
    LcdcStatus = 0xFF41,
    ScrollY = 0xFF42,
    ScrollX = 0xFF43,
    LcdcYCoord = 0xFF44,
    LyCompare = 0xFF45,

    BackgroundPaletteData = 0xFF47,
    ObjectPalette0Data = 0xFF48,
    ObjectPalette1Data = 0xFF49,
    WindowY = 0xFF4A,
    WindowX = 0xFF4B,

    InterruptEnable = 0xFFFF,
}

pub struct Memory {
    mem: Vec<u8>,
    boot_rom: Vec<u8>,
    mbc1: Option<MemoryBankController1>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpriteData {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_number: u8,
    pub attributes: u8,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; 0x10000],
            boot_rom: vec![0; 0x100],
            mbc1: None,
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

    pub fn load_boot_rom(&mut self, boot_buf: &[u8]) {
        for i in 0..boot_buf.len() {
            self.boot_rom[i] = boot_buf[i];
        }
    }

    pub fn load_rom(&mut self, rom_buf: &[u8]) {
        let mbc_type = rom_buf[0x147];
        println!("mbc type = {:02X}", mbc_type);

        // TODO: 19 is actually MBC3 but we will use MBC1 for now
        let use_mbc1 = mbc_type >= 1 && mbc_type <= 3 || mbc_type == 19;
        if use_mbc1 {
            let mut mbc1 = MemoryBankController1::new();
            mbc1.initialize(rom_buf);
            self.mbc1 = Some(mbc1);
        }

        let rom_size = if use_mbc1 { 0x4000 } else { 0x8000 };
        for i in 0..rom_size {
            self.mem[i] = rom_buf[i];
        }
    }

    pub fn load_external_ram(&mut self, save_buf: &[u8]) {
        match self.mbc1 {
            Some(ref mut mbc1) => mbc1.load_external_ram(save_buf),
            None => panic!("No external RAM banks"),
        };
    }

    pub fn use_battery(&self) -> bool {
        match self.mbc1 {
            Some(ref mbc1) => mbc1.use_battery,
            None => false,
        }
    }

    pub fn get_external_ram_banks(&self) -> Vec<u8> {
        match self.mbc1 {
            Some(ref mbc1) => mbc1.get_external_ram_banks(),
            None => panic!("No external RAM banks"),
        }
    }

    pub fn get_register(&self, reg: Register) -> u8 {
        self.get_unchecked(reg as u16)
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        if address < 0x100 && self.mem[0xFF50] == 0 {
            return self.boot_rom[address as usize];
        }

        match self.mbc1 {
            Some(ref mbc1) => match mbc1.get_byte(address) {
                Some(x) => {
                    return x;
                }
                None => (),
            },
            None => (),
        }

        if address >= 0xE000 && address < 0xFE00 {
            return self.get_unchecked(address - 0x2000);
        }

        self.get_unchecked(address)
    }

    pub fn read_sprite(&self, sprite_index: u8) -> SpriteData {
        unsafe {
            let sprite_ref = self.mem.get_unchecked(Register::SpriteData as usize);
            let sprites = sprite_ref as *const u8 as *const SpriteData;
            return *sprites.offset(sprite_index as isize);
        }
    }

    pub fn set_byte(&mut self, address: u16, b: u8) {
        // if address == 0xFF40 {
        //     println!("LCD Control {:08b}", self.mem[address as usize]);
        // }

        // if address == 0xFFFF {
        //     println!("IE {:08b}", b);
        // }

        match self.mbc1 {
            Some(ref mut mbc1) => {
                if mbc1.set_byte(address, b) {
                    return;
                }
            }
            None => (),
        }

        // blarrg's test roms store whether the machine is color or not at D800
        // for some reason the cpu instr test roms detect our emulator as color
        // if address == 0xD800 {
        //     self.mem[address as usize] = 0;
        //     println!("D800 = {:02X}", b);
        //     return;
        // }

        if address == 0xFF07 {
            println!("TAC set to {}", b);
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

        //TODO: investigate if this behavior is correct, it seems to break the boot ROM
        // if address >= 0x8000 && address < 0xA000 {
        //     // Cannot write VRAM during LCD mode 3 accessing VRAM
        //     let lcd_mode = self.mem[0xFF41] & 0b11;
        //     if lcd_mode == 3 {
        //         return;
        //     }
        // }

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
                self.mem[(0xFE00 + (trans_addr as u16)) as usize] =
                    self.mem[concat_bytes(b, trans_addr) as usize];
            }
            // return;
        }

        if address < 0x8000 {
            //Read only
            return;
        }

        self.mem[address as usize] = b;
    }

    pub fn set_owned_byte(&mut self, address: u16, value: u8) {
        self.set_unchecked(address, value);
    }

    pub fn set_register(&mut self, register: Register, value: u8) {
        self.set_unchecked(register as u16, value);
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

    pub fn get_unchecked(&self, address: u16) -> u8 {
        return unsafe { *self.mem.get_unchecked(address as usize) };
    }

    pub fn set_unchecked(&mut self, address: u16, value: u8) {
        unsafe { *self.mem.get_unchecked_mut(address as usize) = value };
    }
}
