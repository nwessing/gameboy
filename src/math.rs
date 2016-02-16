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
    let signed_arg = to_signed_word(sign);
    if signed_arg < 0 {
        unsign.wrapping_sub(to_unsigned_word(signed_arg))
    } else {
        unsign.wrapping_add(to_unsigned_word(signed_arg))
    }
}

pub fn add_u16_and_i8_affect_flags(gb: &mut GameBoy, unsign: u16, sign: u8) -> u16 {
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.zero = false;
    let signed_arg = to_signed_word(sign);
    let unsigned_arg = sign as u16;
     
    if signed_arg < 0 {
        let result = (unsign as i32) + (signed_arg as i32);
        gb.cpu.flag.carry = result & 0xFF <= (unsign as i32) & 0xFF;
        gb.cpu.flag.half_carry = result & 0xF <= (unsign as i32) & 0xF;
        (result & 0xFFFF) as u16
    } else {
        let result = (unsign as u32) + (unsigned_arg as u32);
        gb.cpu.flag.carry = ((unsign as u32) & 0xFF) + (unsigned_arg as u32) > 0xFF;
        gb.cpu.flag.half_carry = (unsign & 0xF) + (unsigned_arg & 0xF) > 0xF;
        (result & 0xFFFF) as u16
    }
}