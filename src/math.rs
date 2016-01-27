use game_boy::GameBoy;

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