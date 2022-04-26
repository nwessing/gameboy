use crate::{game_boy::GameBoy, memory::Register};

const CLOCKS_PER_FRAME: u32 = 70_224;
const FRAME_SEQUENCER_TICK: u64 = 8_192;

pub struct SoundController {
    total_cycle_count: u32,
    last_sample_output: i32,
    frame_sequencer: FrameSequencer,
    channel_one: QuadrangularChannel,
    channel_two: QuadrangularChannel,
    frequency: u32,
}

impl SoundController {
    pub fn new(frequency: u32) -> Self {
        Self {
            total_cycle_count: 0,
            last_sample_output: -1,
            frame_sequencer: FrameSequencer::new(),
            channel_one: QuadrangularChannel::new(
                Register::Channel1LengthDuty,
                Register::Channel1VolumeEnvelope,
                Register::Channel1FrequencyLo,
                Register::Channel1FrequencyHi,
                0b0000_0001,
                0b0001_0000,
                0b0000_0001,
            ),

            channel_two: QuadrangularChannel::new(
                Register::Channel2LengthDuty,
                Register::Channel2VolumeEnvelope,
                Register::Channel2FrequencyLo,
                Register::Channel2FrequencyHi,
                0b0000_0010,
                0b0010_0000,
                0b0000_0010,
            ),
            frequency,
        }
    }
    pub fn update(&mut self, gb: &mut GameBoy, sound_buffer: &mut Vec<u8>, cycles_elapsed: u8) {
        for _ in 0..cycles_elapsed {
            let clocks = self.frame_sequencer.update(1);

            self.channel_one.update(gb, clocks, 1);
            self.channel_two.update(gb, clocks, 1);

            self.total_cycle_count += 1;
            self.output_sample(gb, sound_buffer);
        }
    }

    fn output_sample(&mut self, gb: &GameBoy, sound_buffer: &mut Vec<u8>) {
        let sampling_rate = CLOCKS_PER_FRAME / (self.frequency / 60);
        let current_sample = self.total_cycle_count / sampling_rate;

        let do_output_new_sample = current_sample as i32 > self.last_sample_output;

        if do_output_new_sample {
            let channel_control = gb.memory.get_register(Register::ChannelControl);

            let left_volume = ((channel_control & 0b0111_0000) >> 4) as i8;
            let right_volume = (channel_control & 0b0000_0111) as i8;

            let sample1 = self.channel_one.sample();
            let sample2 = self.channel_two.sample();
            let sample = sample1 / 2i8 + sample2 / 2i8;

            let left_sample = ((sample as i32 * left_volume as i32 / 8) + 127) as u8;
            let right_sample = ((sample as i32 * right_volume as i32 / 8) + 127) as u8;

            sound_buffer.push(left_sample);
            sound_buffer.push(right_sample);
            self.last_sample_output = current_sample as i32;
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct FrameSequencerClocks {
    length: bool,
    envelope: bool,
    sweep: bool,
}

struct FrameSequencer {
    total_cycle_count: u64,
    frame_sequencer_clock: u32,
}

impl FrameSequencer {
    fn new() -> Self {
        Self {
            total_cycle_count: 0,
            frame_sequencer_clock: 0,
        }
    }
    fn update(&mut self, cycles_elapsed: u8) -> FrameSequencerClocks {
        let is_first_tick = self.total_cycle_count == 0;
        self.total_cycle_count += cycles_elapsed as u64;
        if (self.total_cycle_count / FRAME_SEQUENCER_TICK) as u32 > self.frame_sequencer_clock {
            self.frame_sequencer_clock += 1;

            let length = self.frame_sequencer_clock % 2 == 0;
            let envelope = (self.frame_sequencer_clock + 1) % 8 == 0;
            let sweep = (self.frame_sequencer_clock + 2) % 4 == 0;

            return FrameSequencerClocks {
                length,
                envelope,
                sweep,
            };
        }

        return FrameSequencerClocks {
            length: is_first_tick,
            envelope: false,
            sweep: false,
        };
    }
}

const DUTY_PATTERNS: [u8; 4] = [0b0000_0001, 0b0000_0011, 0b0000_1111, 0b1111_1100];

struct QuadrangularChannel {
    frequency_timer: u32,
    duty_position: u32,
    period_timer: u8,
    length_timer: u8,
    prev_length_counter: u8,
    volume: u8,
    disabled: bool,
    length_duty_register: Register,
    volume_envelope_register: Register,
    frequency_lo_register: Register,
    frequency_hi_register: Register,
    accumulator: i32,
    samples_accumulated: u32,
    channel_enable_mask: u8,
    left_output_mask: u8,
    right_output_mask: u8,
}

impl QuadrangularChannel {
    fn new(
        length_duty_register: Register,
        volume_envelope_register: Register,
        frequency_lo_register: Register,
        frequency_hi_register: Register,
        channel_enable_mask: u8,
        left_output_mask: u8,
        right_output_mask: u8,
    ) -> Self {
        Self {
            frequency_timer: 0,
            duty_position: 0,
            period_timer: 0,
            length_timer: 0,
            prev_length_counter: 0,
            volume: 0,
            disabled: false,
            length_duty_register,
            volume_envelope_register,
            frequency_lo_register,
            frequency_hi_register,
            accumulator: 0,
            samples_accumulated: 0,
            channel_enable_mask,
            left_output_mask,
            right_output_mask,
        }
    }

    fn update(&mut self, gb: &mut GameBoy, clocks: FrameSequencerClocks, cycles_elapsed: u8) {
        for _ in 0..cycles_elapsed {
            if self.frequency_timer == 0 {
                let timer_hi = gb.memory.get_register(self.frequency_hi_register);
                let timer_lo = gb.memory.get_register(self.frequency_lo_register);

                let timer = (((timer_hi & 0b0000_0111) as u32) << 8) | timer_lo as u32;

                self.frequency_timer = (2048 - timer) * 4;
                self.duty_position += 1;
                if self.duty_position > 7 {
                    self.duty_position = 0;
                }
            }
            self.frequency_timer -= 1;
        }

        let length_register = gb.memory.get_register(self.length_duty_register);
        let new_length_counter = length_register & 0b0011_1111;
        if new_length_counter != self.prev_length_counter {
            self.prev_length_counter = new_length_counter;
            // TODO 256 for channel 3
            self.length_timer = 64 - new_length_counter;
        }

        let envelope = gb.memory.get_register(self.volume_envelope_register);
        let initial_period_timer = envelope & 0b0000_0111;
        if gb.memory.channel_2_triggered() {
            self.volume = (envelope & 0b1111_0000) >> 4;
            self.period_timer = initial_period_timer;
            self.disabled = false;
            if self.length_timer == 0 {
                // TODO 256 for channel 3
                self.length_timer = 64;
            }
        }

        // Tick length function
        if clocks.length
            && gb.memory.get_register(self.frequency_hi_register) & 0b0100_0000 != 0
            && self.length_timer > 0
        {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.disabled = true;
            }
        }

        // Tick envelope function
        if clocks.envelope && initial_period_timer > 0 {
            if self.period_timer > 0 {
                self.period_timer -= 1;
            }

            if self.period_timer == 0 {
                self.period_timer = initial_period_timer;
                let increase_volume = (envelope & 0b0000_1000) != 0;
                if increase_volume {
                    if self.volume < 0xF {
                        self.volume += 1;
                    }
                } else {
                    if self.volume > 0 {
                        self.volume -= 1;
                    }
                }
            }
        }

        {
            let mut value = gb.memory.get_register(Register::SoundEnable);
            value = if self.disabled {
                value & !self.channel_enable_mask
            } else {
                value | self.channel_enable_mask
            };
            gb.memory.set_register(Register::SoundEnable, value);
        }

        self.samples_accumulated += cycles_elapsed as u32;
        let master_disable = gb.memory.get_register(Register::SoundEnable) & 0b1000_0000 == 0;
        if self.disabled || master_disable {
            self.accumulator = 0;
            self.samples_accumulated = 1;
            return;
        } else {
            let duty_length = gb.memory.get_register(self.length_duty_register);
            let duty = (0b1100_0000 & duty_length) >> 6;

            let pattern = DUTY_PATTERNS[duty as usize];
            let amplitude = (pattern >> self.duty_position) & 0b0000_0001;

            let terminals = gb.memory.get_register(Register::SoundOutputTerminal);
            let channel_control = gb.memory.get_register(Register::ChannelControl);

            let channel_volume = (channel_control & 0b0111_0000) >> 4;
            let volume = (channel_volume * self.volume) as i32;

            if terminals & self.left_output_mask > 0 {
                self.accumulator = if amplitude > 0 { volume } else { -volume };
                self.samples_accumulated = 1;
            } else {
                self.accumulator = 0;
                self.samples_accumulated = 1;
            }
        }
    }

    fn sample(&mut self) -> i8 {
        let sample = self.accumulator / self.samples_accumulated as i32;
        self.accumulator = 0;
        self.samples_accumulated = 0;

        if !(sample <= i8::MAX as i32 && sample >= i8::MIN as i32) {
            println!("oops {}", sample);
        }
        assert!(sample <= i8::MAX as i32 && sample >= i8::MIN as i32);

        return sample as i8;
    }
}
