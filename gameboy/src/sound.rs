use crate::{game_boy::GameBoy, memory::Register};

const CLOCKS_PER_FRAME: u32 = 70_224;
const FRAME_SEQUENCER_TICK: u64 = 8_192;
const WAVE_MEMORY_START: u16 = 0xFF30;
const WAVE_MEMORY_END: u16 = 0xFF40;

pub struct SoundController {
    total_cycle_count: u32,
    last_sample_output: i32,
    frame_sequencer: FrameSequencer,
    channel_one: SoundChannel<SquareWave>,
    channel_two: SoundChannel<SquareWave>,
    channel_three: SoundChannel<CustomWave>,
    frequency: u32,
}

fn channel_1_triggered(gb: &GameBoy) -> bool {
    gb.memory.channel_1_triggered()
}
fn channel_2_triggered(gb: &GameBoy) -> bool {
    gb.memory.channel_2_triggered()
}
fn channel_3_triggered(gb: &GameBoy) -> bool {
    gb.memory.channel_3_triggered()
}

impl SoundController {
    pub fn new(frequency: u32) -> Self {
        Self {
            total_cycle_count: 0,
            last_sample_output: -1,
            frame_sequencer: FrameSequencer::new(),
            channel_one: SoundChannel::new(
                SquareWave::new(Register::Channel1LengthDuty),
                Some(Register::Channel1Sweep),
                Some(Register::Channel1VolumeEnvelope),
                Register::Channel1FrequencyLo,
                Register::Channel1FrequencyHi,
                channel_1_triggered,
                0b0000_0001,
                0b0001_0000,
                0b0000_0001,
            ),
            channel_two: SoundChannel::new(
                SquareWave::new(Register::Channel2LengthDuty),
                None,
                Some(Register::Channel2VolumeEnvelope),
                Register::Channel2FrequencyLo,
                Register::Channel2FrequencyHi,
                channel_2_triggered,
                0b0000_0010,
                0b0010_0000,
                0b0000_0010,
            ),

            channel_three: SoundChannel::new(
                CustomWave::new(Register::Channel3Length, Register::Channel3VolumeCode),
                None,
                None,
                Register::Channel3FrequencyLo,
                Register::Channel3FrequencyHi,
                channel_3_triggered,
                0b0000_0100,
                0b0100_0000,
                0b0000_0100,
            ),
            frequency,
        }
    }
    pub fn update(&mut self, gb: &mut GameBoy, sound_buffer: &mut Vec<u8>, cycles_elapsed: u8) {
        for _ in 0..cycles_elapsed {
            let clocks = self.frame_sequencer.update(1);

            self.channel_one.update(gb, clocks, 1);
            self.channel_two.update(gb, clocks, 1);
            self.channel_three.update(gb, clocks, 1);

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
            let sample3 = self.channel_three.sample();
            let sample_left =
                ((sample1.0 as i16 + sample2.0 as i16 + sample3.0 as i16) / 3i16) as i8;
            let sample_right =
                ((sample1.1 as i16 + sample2.1 as i16 + sample3.1 as i16) / 3i16) as i8;
            // let sample_left = sample2.0;
            // let sample_right = sample2.0;

            let left_sample = ((sample_left as i32 * left_volume as i32 / 8) + 127) as u8;
            let right_sample = ((sample_right as i32 * right_volume as i32 / 8) + 127) as u8;

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

struct SoundChannel<T: SoundChannelType> {
    frequency_timer: u32,
    period_timer: u8,
    length_timer: u32,
    prev_length_counter: u32,
    envelope_volume: u8,
    disabled: bool,
    sweep_enabled: bool,
    shadow_freqency: u32,
    sweep_timer: u8,
    channel_type: T,
    sweep_register: Option<Register>,
    volume_envelope_register: Option<Register>,
    frequency_lo_register: Register,
    frequency_hi_register: Register,
    check_trigger_event: fn(&GameBoy) -> bool,
    left_accumulator: i32,
    right_accumulator: i32,
    samples_accumulated: u32,
    channel_enable_mask: u8,
    left_output_mask: u8,
    right_output_mask: u8,
}

impl<T: SoundChannelType> SoundChannel<T> {
    fn new(
        channel_type: T,
        sweep_register: Option<Register>,
        volume_envelope_register: Option<Register>,
        frequency_lo_register: Register,
        frequency_hi_register: Register,
        check_trigger_event: fn(&GameBoy) -> bool,
        channel_enable_mask: u8,
        left_output_mask: u8,
        right_output_mask: u8,
    ) -> Self {
        Self {
            frequency_timer: 0,
            period_timer: 0,
            length_timer: 0,
            prev_length_counter: 0,
            envelope_volume: 0,
            disabled: false,
            sweep_enabled: false,
            shadow_freqency: 0,
            sweep_timer: 0,
            channel_type,
            sweep_register,
            volume_envelope_register,
            frequency_lo_register,
            frequency_hi_register,
            check_trigger_event,
            left_accumulator: 0,
            right_accumulator: 0,
            samples_accumulated: 0,
            channel_enable_mask,
            left_output_mask,
            right_output_mask,
        }
    }

    fn update(&mut self, gb: &mut GameBoy, clocks: FrameSequencerClocks, cycles_elapsed: u8) {
        for _ in 0..cycles_elapsed {
            if self.frequency_timer == 0 {
                let timer = self.get_frequency(gb);

                self.frequency_timer = self.channel_type.reload_frequency_counter(timer);
                self.channel_type.cycle(gb);
            }
            self.frequency_timer -= 1;
        }

        let new_length_counter = self.channel_type.length_counter(gb);
        if new_length_counter != self.prev_length_counter {
            self.prev_length_counter = new_length_counter;
            self.length_timer = self.channel_type.new_length_timer(new_length_counter);
        }

        let envelope_values = self
            .volume_envelope_register
            .map(|register| EnvelopeValues::from_register(gb.memory.get_register(register)));

        if (self.check_trigger_event)(gb) {
            self.channel_type.trigger_event();
            if let Some(envelope_values) = envelope_values.as_ref() {
                self.envelope_volume = envelope_values.initial_volume;
                self.period_timer = envelope_values.initial_period;
            }

            self.disabled = false;
            if self.length_timer == 0 {
                self.length_timer = self.channel_type.new_length_timer(0);
            }

            if let Some(sweep_register) = self.sweep_register {
                self.shadow_freqency = self.get_frequency(gb);
                let sweep_values = self.reload_sweep_timer(gb, sweep_register);
                self.sweep_enabled = sweep_values.period > 0 || sweep_values.shift > 0;

                if sweep_values.shift != 0 {
                    self.calculate_sweep_frequency(sweep_values);
                }
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
        if clocks.envelope {
            if let Some(envelope_values) = envelope_values.as_ref() {
                if envelope_values.initial_period > 0 {
                    if self.period_timer > 0 {
                        self.period_timer -= 1;
                    }

                    if self.period_timer == 0 {
                        self.period_timer = envelope_values.initial_period;
                        if envelope_values.add_mode {
                            if self.envelope_volume < 0xF {
                                self.envelope_volume += 1;
                            }
                        } else {
                            if self.envelope_volume > 0 {
                                self.envelope_volume -= 1;
                            }
                        }
                    }
                }
            }
        }

        if clocks.sweep {
            if let Some(sweep_register) = self.sweep_register {
                if self.sweep_timer > 0 {
                    self.sweep_timer -= 1;

                    if self.sweep_timer == 0 {
                        let sweep_values = self.reload_sweep_timer(gb, sweep_register);
                        if self.sweep_enabled && sweep_values.period != 0 {
                            let new_frequency = self.calculate_sweep_frequency(sweep_values);

                            if new_frequency < 2048 && sweep_values.shift > 0 {
                                self.shadow_freqency = new_frequency;
                                self.set_frequency(gb, new_frequency);
                                self.calculate_sweep_frequency(sweep_values);
                            }
                        }
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
            self.left_accumulator = 0;
            self.right_accumulator = 0;
            self.samples_accumulated = 1;
            return;
        } else {
            let channel_control = gb.memory.get_register(Register::ChannelControl);
            let channel_volume = (channel_control & 0b0111_0000) >> 4;

            self.samples_accumulated = 1;
            let sample = self
                .channel_type
                .sample(gb, channel_volume, self.envelope_volume);
            let terminals = gb.memory.get_register(Register::SoundOutputTerminal);

            if terminals & self.left_output_mask > 0 {
                self.left_accumulator = sample as i32;
            }

            if terminals & self.right_output_mask > 0 {
                self.right_accumulator = sample as i32;
            }
        }
    }

    fn calculate_sweep_frequency(&mut self, sweep_values: SweepValues) -> u32 {
        let mut new_frequency = self.shadow_freqency >> sweep_values.shift;

        // if sweep_values.direction {
        //     new_frequency = !new_frequency;
        // }
        // new_frequency += self.shadow_freqency;
        if sweep_values.direction {
            new_frequency = self.shadow_freqency - new_frequency;
        } else {
            new_frequency = self.shadow_freqency + new_frequency;
        }

        if new_frequency > 2047 {
            self.disabled = true;
        }

        return new_frequency;
    }

    fn sample(&mut self) -> (i8, i8) {
        let left_sample = self.left_accumulator / self.samples_accumulated as i32;
        let right_sample = self.right_accumulator / self.samples_accumulated as i32;
        self.left_accumulator = 0;
        self.right_accumulator = 0;
        self.samples_accumulated = 0;

        check_sample_bounds(left_sample, "Left");
        check_sample_bounds(right_sample, "Right");

        return (left_sample as i8, right_sample as i8);
    }

    fn get_frequency(&self, gb: &mut GameBoy) -> u32 {
        let timer_hi = gb.memory.get_register(self.frequency_hi_register);
        let timer_lo = gb.memory.get_register(self.frequency_lo_register);

        let timer = (((timer_hi & 0b0000_0111) as u32) << 8) | timer_lo as u32;
        timer
    }

    fn set_frequency(&self, gb: &mut GameBoy, new_frequency: u32) {
        let existing_hi = gb.memory.get_register(self.frequency_hi_register);
        gb.memory.set_register(
            self.frequency_hi_register,
            ((new_frequency >> 8) & 0b0111) as u8 | (existing_hi & 0b1111_1000),
        );
        gb.memory
            .set_register(self.frequency_lo_register, (new_frequency & 0xFF) as u8);
    }

    fn reload_sweep_timer(&mut self, gb: &GameBoy, sweep_register: Register) -> SweepValues {
        let sweep_values = SweepValues::from_register(gb.memory.get_register(sweep_register));

        self.sweep_timer = if sweep_values.period != 0 {
            sweep_values.period
        } else {
            8
        };

        sweep_values
    }
}

#[derive(Copy, Clone, Debug)]
struct SweepValues {
    period: u8,
    shift: u8,
    direction: bool,
}

impl SweepValues {
    pub fn from_register(sweep_values: u8) -> Self {
        let sweep_period = (sweep_values & 0b0111_0000) >> 4;
        let sweep_shift = sweep_values & 0b0000_0111;
        let sweep_direction = (sweep_values & 0b0000_1000) > 0;

        Self {
            period: sweep_period,
            shift: sweep_shift,
            direction: sweep_direction,
        }
    }
}

struct EnvelopeValues {
    initial_volume: u8,
    add_mode: bool,
    initial_period: u8,
}

impl EnvelopeValues {
    pub fn from_register(envelope: u8) -> Self {
        Self {
            initial_volume: (envelope & 0b1111_0000) >> 4,
            add_mode: (envelope & 0b0000_1000) > 0,
            initial_period: (envelope & 0b0000_0111),
        }
    }
}

fn check_sample_bounds(sample: i32, channel: &'static str) {
    if !(sample <= i8::MAX as i32 && sample >= i8::MIN as i32) {
        println!("oops ({}) {}", channel, sample);
    }

    assert!(sample <= i8::MAX as i32 && sample >= i8::MIN as i32);
}

trait SoundChannelType {
    fn cycle(&mut self, gb: &GameBoy);
    fn reload_frequency_counter(&self, timer: u32) -> u32;
    // Channel volume will be between 0-7 while envelope volume will be between 0-15
    fn sample(&self, gb: &GameBoy, channel_volume: u8, envelope_volume: u8) -> i8;
    fn length_counter(&self, gb: &GameBoy) -> u32;
    fn new_length_timer(&self, length: u32) -> u32;
    fn trigger_event(&mut self);
}

const DUTY_PATTERNS: [u8; 4] = [0b0000_0001, 0b0000_0011, 0b0000_1111, 0b1111_1100];
struct SquareWave {
    length_duty_register: Register,
    duty_position: u32,
}

impl SquareWave {
    fn new(length_duty_register: Register) -> Self {
        Self {
            duty_position: 0,
            length_duty_register,
        }
    }
}

impl SoundChannelType for SquareWave {
    fn cycle(&mut self, _: &GameBoy) {
        self.duty_position += 1;
        if self.duty_position > 7 {
            self.duty_position = 0;
        }
    }
    fn reload_frequency_counter(&self, timer: u32) -> u32 {
        (2048 - timer) * 4
    }
    fn sample(&self, gb: &GameBoy, channel_volume: u8, envelope_volume: u8) -> i8 {
        let duty_length = gb.memory.get_register(self.length_duty_register);
        let duty = (0b1100_0000 & duty_length) >> 6;

        let pattern = DUTY_PATTERNS[duty as usize];
        let amplitude = (pattern >> self.duty_position) & 0b0000_0001;

        let volume = (channel_volume * envelope_volume) as i8;
        if amplitude > 0 {
            volume
        } else {
            -volume
        }
    }
    fn length_counter(&self, gb: &GameBoy) -> u32 {
        let length_register = gb.memory.get_register(self.length_duty_register);
        let new_length_counter = length_register & 0b0011_1111;
        new_length_counter as u32
    }

    fn new_length_timer(&self, length: u32) -> u32 {
        64 - length
    }
    fn trigger_event(&mut self) {}
}

struct CustomWave {
    length_register: Register,
    volume_shift_register: Register,
    position_counter: u8,
    sample_buffer: u8,
}

impl CustomWave {
    fn new(length_register: Register, volume_shift_register: Register) -> Self {
        Self {
            length_register,
            volume_shift_register,
            position_counter: 0,
            sample_buffer: 0,
        }
    }
}

impl SoundChannelType for CustomWave {
    fn cycle(&mut self, gb: &GameBoy) {
        self.position_counter += 1;
        if self.position_counter > 31 {
            self.position_counter = 0;
        }
        let offset = self.position_counter as u16 / 2;
        let data = gb.memory.get_byte(WAVE_MEMORY_START + offset);
        self.sample_buffer = if self.position_counter % 2 == 0 {
            (data & 0xF0) >> 4
        } else {
            data & 0x0F
        };
    }
    fn reload_frequency_counter(&self, timer: u32) -> u32 {
        (2048 - timer) * 2
    }
    fn sample(&self, gb: &GameBoy, channel_volume: u8, _: u8) -> i8 {
        let value = gb.memory.get_register(self.volume_shift_register);
        let volume_code = (value & 0b0110_0000) >> 5;

        let volume_shift: u8 = match volume_code {
            1 => 0, // 100%
            2 => 1, // 50%
            3 => 2, // 25%
            _ => 4, // mute (0)
        };

        let (sample_magnitude, sample_modifier) = if self.sample_buffer >= 7 {
            (self.sample_buffer - 7, 1)
        } else {
            (7 - self.sample_buffer, -1)
        };

        // Sample magnitude can be up to 8
        let volume = (sample_magnitude >> volume_shift) * channel_volume * 2;
        sample_modifier * (volume as i8)
    }
    fn length_counter(&self, gb: &GameBoy) -> u32 {
        let length_register = gb.memory.get_register(self.length_register);
        length_register as u32
    }
    fn new_length_timer(&self, length_counter: u32) -> u32 {
        256 - length_counter
    }
    fn trigger_event(&mut self) {
        self.position_counter = 0;
    }
}
