use time;

// let clock_speed = 4_194_304; 
const NON_SEC_PER_CYCLE: u64 = 238;

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

    pub fn tick(&mut self, num_cycle: u8) {
        self.ticks += num_cycle as u64;
        let nano_sec_to_wait = (num_cycle as u64) * NON_SEC_PER_CYCLE;

        // if (self.last_time + nano_sec_to_wait < time::precise_time_ns()) {
        //     println!("Emulation fell behind by {}ns", time::precise_time_ns() - (self.last_time + nano_sec_to_wait));
        // }
        // while self.last_time + nano_sec_to_wait > time::precise_time_ns() {
        
        // }
        self.last_time = time::precise_time_ns();
    }

    pub fn current_tick(&self) -> u64 {
        self.ticks
    }
}