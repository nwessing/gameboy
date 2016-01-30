use time;

// let clock_speed = 4_194_304; 
const nano_sec_per_cycle: u64 = 238;

pub struct Clock {
    ticks: u64,
    last_time: u64,
    last_ticks: u64
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            ticks: 0,
            last_time: 0,
            last_ticks: 0
        }
    }

    pub fn start(&mut self) {
        self.last_time = time::precise_time_ns();
    }

    pub fn tick(&mut self, num_cycle: u8) {
        self.ticks += num_cycle as u64;
        let ticks_to_wait = self.ticks - self.last_ticks;
        let nano_sec_to_wait = ticks_to_wait * nano_sec_per_cycle;

        while self.last_time + nano_sec_to_wait > time::precise_time_ns() {
        
        }
        self.last_ticks = self.ticks;
        self.last_time = time::precise_time_ns();
    }

    pub fn current_tick(&self) -> u64 {
        self.ticks
    }
}