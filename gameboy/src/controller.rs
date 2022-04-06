use crate::game_boy::GameBoy;
use crate::memory::Register;

const JOYPAD_REG_ADDR: u16 = 0xFF00;

pub struct Controller {
    direction_states: u8,
    button_states: u8,
}

fn set_bit(val: u8, mask: u8, set: bool) -> u8 {
    if set {
        val | mask
    } else {
        val & !mask
    }
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            direction_states: 0x0F,
            button_states: 0x0F,
        }
    }

    pub fn down_changed(&mut self, pressed: bool) {
        self.direction_states = set_bit(self.direction_states, 0b1000, !pressed);
    }

    pub fn up_changed(&mut self, pressed: bool) {
        self.direction_states = set_bit(self.direction_states, 0b0100, !pressed);
    }

    pub fn left_changed(&mut self, pressed: bool) {
        self.direction_states = set_bit(self.direction_states, 0b0010, !pressed);
    }

    pub fn right_changed(&mut self, pressed: bool) {
        self.direction_states = set_bit(self.direction_states, 0b0001, !pressed);
    }

    pub fn start_changed(&mut self, pressed: bool) {
        self.button_states = set_bit(self.button_states, 0b1000, !pressed);
    }

    pub fn select_changed(&mut self, pressed: bool) {
        self.button_states = set_bit(self.button_states, 0b0100, !pressed);
    }

    pub fn b_changed(&mut self, pressed: bool) {
        self.button_states = set_bit(self.button_states, 0b0010, !pressed);
    }

    pub fn a_changed(&mut self, pressed: bool) {
        self.button_states = set_bit(self.button_states, 0b0001, !pressed);
    }

    pub fn update_joypad_register(&self, gb: &mut GameBoy) {
        let joypad_select = gb.memory.get_register(Register::Joypad) & 0xF0;
        if joypad_select & 0x20 == 0x00 {
            gb.memory
                .set_register(Register::Joypad, joypad_select | self.button_states);
        }

        if joypad_select & 0x10 == 0x00 {
            gb.memory
                .set_register(Register::Joypad, joypad_select | self.direction_states);
        }
    }
}
