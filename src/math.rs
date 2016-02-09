use game_boy::GameBoy;
use util::to_signed_word;
use util::to_unsigned_word;

pub fn rotate_left(gb: &mut GameBoy, val: u8, through: bool) -> u8 {
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    let carry = if gb.cpu.flag.carry { 1 } else { 0 };

    gb.cpu.flag.carry = val & 0x80 == 0x80;

    let result = if through {
        (val << 1) | carry
    } else {
        val << 1
    };

    gb.cpu.flag.zero = result == 0;

    result
}

pub fn rotate_right(gb: &mut GameBoy, val: u8, through: bool) -> u8 {
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    let carry = if gb.cpu.flag.carry { 0x80 } else { 0 };

    gb.cpu.flag.carry = val & 0x01 == 0x001;

    let result = if through {
        (val >> 1) | carry
    } else {
        val >> 1
    };

    gb.cpu.flag.zero = result == 0;

    result
}

pub fn add_u16_and_i8(unsign: u16, sign: u8) -> u16 {
    let to_sub = to_signed_word(sign);
    if to_sub < 0 {
        unsign - to_unsigned_word(to_sub)
    } else {
        unsign + to_unsigned_word(to_sub)
    }
}