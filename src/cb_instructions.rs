use instructions::Instruction;
use game_boy::GameBoy;
use cpu::Cpu;
use math::rotate_left;

pub fn get_cb_instruction_set() -> Vec<Instruction> {
    vec![
        Instruction::new("RL A", 0x17, 0, 8, rotate_left_a),
        Instruction::new("RL B", 0x10, 0, 8, rotate_left_b),
        Instruction::new("RL C", 0x11, 0, 8, rotate_left_c),
        Instruction::new("RL D", 0x12, 0, 8, rotate_left_d),
        Instruction::new("RL E", 0x13, 0, 8, rotate_left_e),
        Instruction::new("RL H", 0x14, 0, 8, rotate_left_h),
        Instruction::new("RL L", 0x15, 0, 8, rotate_left_l),
        // Instruction::new("RL (HL)", 0x16, 0, 16, rotate_left_mem_hl),

        Instruction::new("SWAP A", 0x37, 0, 8, swap_a),
        Instruction::new("SWAP B", 0x30, 0, 8, swap_b),
        Instruction::new("SWAP C", 0x31, 0, 8, swap_c),
        Instruction::new("SWAP D", 0x32, 0, 8, swap_d),
        Instruction::new("SWAP E", 0x33, 0, 8, swap_e),
        Instruction::new("SWAP H", 0x34, 0, 8, swap_h),
        Instruction::new("SWAP L", 0x35, 0, 8, swap_l),
        Instruction::new("SWAP (HL)", 0x36, 0, 16, swap_mem_hl),

        Instruction::new("BIT 0,H", 0x44, 0, 8, test_bit_0_h),
        Instruction::new("BIT 1,H", 0x4C, 0, 8, test_bit_1_h),
        Instruction::new("BIT 2,H", 0x54, 0, 8, test_bit_2_h),
        Instruction::new("BIT 3,H", 0x5C, 0, 8, test_bit_3_h),
        Instruction::new("BIT 4,H", 0x64, 0, 8, test_bit_4_h),
        Instruction::new("BIT 5,H", 0x6C, 0, 8, test_bit_5_h),
        Instruction::new("BIT 6,H", 0x74, 0, 8, test_bit_6_h),
        Instruction::new("BIT 7,H", 0x7C, 0, 8, test_bit_7_h),

        Instruction::new("RES 0,A", 0x87, 0, 8, reset_bit_0_a),
    ]
}

fn test_bit(gb: &mut GameBoy, val: u8, bit: u8) {
    let mask = match bit {
        0 => 0b00000001,
        1 => 0b00000010,
        2 => 0b00000100,
        3 => 0b00001000,
        4 => 0b00010000,
        5 => 0b00100000,
        6 => 0b01000000,
        7 => 0b10000000,
        _ => panic!("fdsadf")
    };

    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = true;
    gb.cpu.flag.zero = val & mask == 0;
}

fn reset_bit(value: u8, bit: u8) -> u8 {
    let mask = match bit {
        0 => 0b11111110,
        1 => 0b11111101,
        2 => 0b11111011,
        3 => 0b11110111,
        4 => 0b11101111,
        5 => 0b11011111,
        6 => 0b10111111,
        7 => 0b01111111,
        _ => panic!("sdfaadsf")
    };
    value & mask
}

fn swap(gb: &mut GameBoy, value: u8) -> u8 {
    gb.cpu.flag.zero = value == 0;
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    gb.cpu.flag.carry = false;

    (value << 4) | (value >> 4)
}

// fn potato(getter: Fn(&Cpu) -> u8, bit: u8) -> Box<Fn(& mut GameBoy, u8, u8)> {
//     Box::new(move |gb, a1, a2| test_bit(gb, getter(), bit))
// }

// fn rotate_left(gb: &mut GameBoy, val: u8) -> u8 {
//     gb.cpu.flag.subtract = false;
//     gb.cpu.flag.half_carry = false;
//     gb.cpu.flag.carry = val & 0xA0 > 0;

//     let result = val.rotate_left(1);
//     gb.cpu.flag.zero = result == 0;

//     result
// }

pub fn rotate_left_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_a(result);
}

fn rotate_left_b(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_b(result);
}

fn rotate_left_c(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_c(result);
}

fn rotate_left_d(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_d(result);
}

fn rotate_left_e(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_e(result);
}

fn rotate_left_h(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_h(result);
}

fn rotate_left_l(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_l(result);
}

fn swap_a(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.cpu.get_a();
    let result = swap(gb, value);
    gb.cpu.set_a(result);
}

fn swap_b(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.cpu.get_b();
    let result = swap(gb, value);
    gb.cpu.set_b(result);
}

fn swap_c(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.cpu.get_c();
    let result = swap(gb, value);
    gb.cpu.set_c(result);
}

fn swap_d(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.cpu.get_d();
    let result = swap(gb, value);
    gb.cpu.set_d(result);
}

fn swap_e(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.cpu.get_e();
    let result = swap(gb, value);
    gb.cpu.set_e(result);
}

fn swap_h(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.cpu.get_h();
    let result = swap(gb, value);
    gb.cpu.set_h(result);
}

fn swap_l(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.cpu.get_l();
    let result = swap(gb, value);
    gb.cpu.set_l(result);
}

fn swap_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let value = gb.memory.get_byte(gb.cpu.hl);
    let result = swap(gb, value);
    gb.memory.set_byte(gb.cpu.hl, result);
}

fn reset_bit_0_a(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    gb.cpu.set_a(reset_bit(a, 0));
}

fn test_bit_0_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 0);
}

fn test_bit_1_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 1);
}

fn test_bit_2_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 2);
}

fn test_bit_3_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 3);
}

fn test_bit_4_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 4);
}

fn test_bit_5_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 5);
}

fn test_bit_6_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 6);
}

fn test_bit_7_h(gb: &mut GameBoy, _: u8, _: u8) {
    let h = gb.cpu.get_h();
    test_bit(gb, h, 7);
}