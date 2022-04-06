use crate::game_boy::GameBoy;
use crate::memory::Register;

const CLOCK_SPEED: u64 = 4_194_304;
const DIVIDER_TICK: u64 = CLOCK_SPEED / 16_384;

const CLOCK0_TICK: u64 = CLOCK_SPEED / 4096;
const CLOCK1_TICK: u64 = CLOCK_SPEED / 262_144;
const CLOCK2_TICK: u64 = CLOCK_SPEED / 65_536;
const CLOCK3_TICK: u64 = CLOCK_SPEED / 16_384;

pub struct Clock {
    ticks: u64,
}

impl Clock {
    pub fn new() -> Clock {
        Clock { ticks: 0 }
    }

    pub fn tick(&mut self, gb: &mut GameBoy, num_cycle: u8) {
        let new_ticks = self.ticks + (num_cycle as u64);

        if self.ticks / DIVIDER_TICK < new_ticks / DIVIDER_TICK {
            let divider_val = gb.memory.get_register(Register::Divider);
            gb.memory
                .set_register(Register::Divider, divider_val.wrapping_add(1));
        }

        let timer_control = gb.memory.get_register(Register::TimerControl);
        let timer_enabled = timer_control & 0b100 == 0b100;
        if timer_enabled {
            let rate = match timer_control & 0b11 {
                0b00 => CLOCK0_TICK,
                0b01 => CLOCK1_TICK,
                0b10 => CLOCK2_TICK,
                0b11 => CLOCK3_TICK,
                _ => panic!("Timer control clock mode decoded incorrectly"),
            };

            let counter = gb.memory.get_register(Register::TimerCounter);
            if self.ticks / rate < new_ticks / rate {
                let result = if counter == 0xFF {
                    let int_flags = gb.memory.get_register(Register::InterruptFlag);
                    gb.memory
                        .set_register(Register::InterruptFlag, int_flags | 0b100);
                    gb.memory.get_register(Register::TimerModulo)
                } else {
                    counter + 1
                };
                gb.memory.set_register(Register::TimerCounter, result);
            }
        }

        self.ticks += num_cycle as u64;
    }
}
