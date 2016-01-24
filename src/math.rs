use game_boy::GameBoy;

pub fn rotate_left(gb: &mut GameBoy, val: u8, through: bool) -> u8 {
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    gb.cpu.flag.carry = val & 0xA0 > 0;


    let result = if through {
        val.rotate_left(1)
    } else {
        val << 1
    };

    gb.cpu.flag.zero = result == 0;

    result
}