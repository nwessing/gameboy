use time;
use game_boy::GameBoy;

const CLOCK_SPEED: u64 = 4_194_304; 
const DIVIDER_TICK: u64 = CLOCK_SPEED / 16_384;

const CLOCK0_TICK: u64 = CLOCK_SPEED / 4096;
const CLOCK1_TICK: u64 = CLOCK_SPEED / 262_144;
const CLOCK2_TICK: u64 = CLOCK_SPEED / 65_536;
const CLOCK3_TICK: u64 = CLOCK_SPEED / 16_384;

// const NON_SEC_PER_CYCLE: u64 = 238;
const DIVIDER_REG: u16 = 0xFF04;
const TIMER_COUNTER_REG: u16 = 0xFF05;
const TIMER_MODULO_REG: u16 = 0xFF06;
const TIMER_CONTROL_REG: u16 = 0xFF07;

const INTERRUPT_FLAG_REG: u16 = 0xFF0F;

pub struct Clock {
    ticks: u64,
    last_time: u64
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            ticks: 0,
            last_time: 0
        }
    }

    pub fn start(&mut self) {
        self.last_time = time::precise_time_ns();
    }

    pub fn tick(&mut self, gb: &mut GameBoy, num_cycle: u8) {
        let new_ticks = self.ticks + (num_cycle as u64);

        if self.ticks / DIVIDER_TICK < new_ticks / DIVIDER_TICK {
            let divider_val = gb.memory.get_byte(DIVIDER_REG);
            gb.memory.set_owned_byte(DIVIDER_REG, divider_val.wrapping_add(1));
        }

        let timer_control = gb.memory.get_byte(TIMER_CONTROL_REG);
        let timer_enabled = timer_control & 0b100 == 0b100;
        if timer_enabled {
            let rate = match timer_control & 0b11 {
                0b00 => CLOCK0_TICK,
                0b01 => CLOCK1_TICK,
                0b10 => CLOCK2_TICK,
                0b11 => CLOCK3_TICK,
                _ => panic!("Timer control clock mode decoded incorrectly")
            };

            let counter = gb.memory.get_byte(TIMER_COUNTER_REG);
            if self.ticks / rate < new_ticks / rate {
                let result = if counter == 0xFF {
                    let int_flags = gb.memory.get_byte(INTERRUPT_FLAG_REG);
                    gb.memory.set_owned_byte(INTERRUPT_FLAG_REG, int_flags | 0b100);
                    gb.memory.get_byte(TIMER_MODULO_REG)
                } else {
                    counter + 1
                };
                gb.memory.set_owned_byte(TIMER_COUNTER_REG, result);
            }
        }

        self.ticks += num_cycle as u64;
    }
}