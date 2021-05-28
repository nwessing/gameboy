enum BankingMode {
    Rom,
    Ram,
}

const ROM_BANK_SELECT_LOWER_BIT_MASK: u8 = 0b0001_1111;
const ROM_BANK_SELECT_UPPER_BIT_MASK: u8 = 0b0110_0000;

pub struct MemoryBankController1 {
    pub use_battery: bool,
    ram_banks_enabled: bool,
    selected_rom_bank: u8,
    selected_ram_bank: u8,
    banking_mode: BankingMode,
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Vec<Vec<u8>>,
}

impl MemoryBankController1 {
    pub fn new() -> MemoryBankController1 {
        MemoryBankController1 {
            use_battery: false,
            ram_banks_enabled: false,
            selected_rom_bank: 1,
            selected_ram_bank: 0,
            banking_mode: BankingMode::Rom,
            rom_banks: vec![],
            ram_banks: vec![],
        }
    }

    pub fn initialize(&mut self, rom_buf: &[u8]) {
        let mbc_type = rom_buf[0x147];
        let num_rom_banks: u32 = match rom_buf[0x148] {
            0 => 2,
            1 => 4,
            2 => 8,
            3 => 16,
            4 => 32,
            5 => 64,
            6 => 128,
            x => panic!("ROM specified incorrect size at 0x148: {}", x),
        };

        let ram_size = rom_buf[0x149];
        let num_ram_banks = match ram_size {
            0 => 0,
            1 => 1, //2KB instead of 8KB
            2 => 1,
            3 => 4,
            4 => 16,
            x => panic!("ROM specified incorrect size for external RAM: {}", x),
        };

        self.use_battery = mbc_type == 0x03;

        println!("Game uses memory banking {}", mbc_type);
        println!("ROM banks = {}", num_rom_banks);
        println!("RAM banks = {}", num_ram_banks);

        for i_bank in 1..num_rom_banks {
            let start = i_bank * 0x4000;
            let mut bank = vec![0; 0x4000];
            for i in start..(start + 0x4000) {
                bank[(i - start) as usize] = rom_buf[i as usize];
            }
            self.rom_banks.push(bank);
        }

        for _i_bank in 0..num_ram_banks {
            self.ram_banks
                .push(vec![0; if ram_size == 1 { 0x800 } else { 0x2000 }]);
        }
    }

    pub fn get_byte(&self, address: u16) -> Option<u8> {
        if address >= 0x4000 && address < 0x8000 {
            return Some(
                self.rom_banks[(self.selected_rom_bank - 1) as usize][(address - 0x4000) as usize],
            );
        }

        if address >= 0xA000 && address < 0xC000 {
            return Some(
                self.ram_banks[self.selected_ram_bank as usize][(address - 0xA000) as usize],
            );
        }

        None
    }

    pub fn set_byte(&mut self, address: u16, b: u8) -> bool {
        if address >= 0x2000 && address < 0x4000 {
            let requested_bank = (b & ROM_BANK_SELECT_LOWER_BIT_MASK)
                | (self.selected_rom_bank & ROM_BANK_SELECT_UPPER_BIT_MASK);
            self.selected_rom_bank = map_to_rom_bank(requested_bank);
            // println!("Selected ROM bank {}", self.selected_rom_bank);
            return true;
        }

        if address >= 0xA000 && address < 0xC000 {
            //RAM banks
            if (self.selected_ram_bank as usize) < self.ram_banks.len() {
                self.ram_banks[self.selected_ram_bank as usize][(address - 0xA000) as usize] = b;
            }
            return true;
        }

        if address >= 0x4000 && address < 0x6000 {
            match self.banking_mode {
                BankingMode::Ram => {
                    self.selected_ram_bank = b & 0b11;
                }
                BankingMode::Rom => {
                    println!("Selecting upper bits of ROM: {:02X}", b);
                    let requested_bank = (self.selected_rom_bank & ROM_BANK_SELECT_LOWER_BIT_MASK)
                        | ((b << 5) & ROM_BANK_SELECT_UPPER_BIT_MASK);
                    self.selected_rom_bank = map_to_rom_bank(requested_bank);
                }
            }
            return true;
        }

        if address >= 0x6000 && address < 0x8000 {
            self.banking_mode = if b & 1 > 0 {
                BankingMode::Ram
            } else {
                BankingMode::Rom
            };

            return true;
        }

        false
    }

    pub fn load_external_ram(&mut self, save_buf: &[u8]) {
        println!("Loading external RAM {0}", save_buf.len());
        let num_ram_banks = self.ram_banks.len();
        for i_bank in 0..num_ram_banks {
            let bank = &mut self.ram_banks[i_bank];
            for i in 0..bank.len() {
                bank[i] = save_buf[(i_bank * bank.len()) + i];
            }
        }
    }

    pub fn get_external_ram_banks(&self) -> Vec<u8> {
        let num_ram_banks = self.ram_banks.len();
        let mut result = vec![0; num_ram_banks * self.ram_banks[0].len()];
        for i_bank in 0..num_ram_banks {
            let bank = &self.ram_banks[i_bank];
            for i in 0..bank.len() {
                result[(i_bank * bank.len()) + i] = bank[i];
            }
        }
        result
    }
}

fn map_to_rom_bank(requested_bank: u8) -> u8 {
    match requested_bank {
        0x00 => 0x01,
        0x20 => 0x21,
        0x40 => 0x41,
        0x60 => 0x61,
        x => x,
    }
}
