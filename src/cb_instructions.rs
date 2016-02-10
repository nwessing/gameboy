use instructions::Instruction;
use game_boy::GameBoy;
use math::rotate_left;
use math::rotate_right;
use util::Reg8;
use util::get_reg8;

pub fn get_cb_instruction_set() -> Vec<Instruction> {
    vec![
        Instruction::new("RL A", 0x17, 0, 8, Box::new(rotate_left_a_through)),
        Instruction::new("RL B", 0x10, 0, 8, Box::new(rotate_left_b_through)),
        Instruction::new("RL C", 0x11, 0, 8, Box::new(rotate_left_c_through)),
        Instruction::new("RL D", 0x12, 0, 8, Box::new(rotate_left_d_through)),
        Instruction::new("RL E", 0x13, 0, 8, Box::new(rotate_left_e_through)),
        Instruction::new("RL H", 0x14, 0, 8, Box::new(rotate_left_h_through)),
        Instruction::new("RL L", 0x15, 0, 8, Box::new(rotate_left_l_through)),
        // Instruction::new("RL (HL)", 0x16, 0, 16, rotate_left_mem_hl),

        Instruction::new("RR A", 0x1F, 0, 8, Box::new(rotate_right_a_through)),
        Instruction::new("RR B", 0x18, 0, 8, Box::new(rotate_right_b_through)),
        Instruction::new("RR C", 0x19, 0, 8, Box::new(rotate_right_c_through)),
        Instruction::new("RR D", 0x1A, 0, 8, Box::new(rotate_right_d_through)),
        Instruction::new("RR E", 0x1B, 0, 8, Box::new(rotate_right_e_through)),
        Instruction::new("RR H", 0x1C, 0, 8, Box::new(rotate_right_h_through)),
        Instruction::new("RR L", 0x1D, 0, 8, Box::new(rotate_right_l_through)),

        Instruction::new("SWAP A", 0x37, 0, 8, Box::new(swap_a)),
        Instruction::new("SWAP B", 0x30, 0, 8, Box::new(swap_b)),
        Instruction::new("SWAP C", 0x31, 0, 8, Box::new(swap_c)),
        Instruction::new("SWAP D", 0x32, 0, 8, Box::new(swap_d)),
        Instruction::new("SWAP E", 0x33, 0, 8, Box::new(swap_e)),
        Instruction::new("SWAP H", 0x34, 0, 8, Box::new(swap_h)),
        Instruction::new("SWAP L", 0x35, 0, 8, Box::new(swap_l)),
        Instruction::new("SWAP (HL)", 0x36, 0, 16, Box::new(swap_mem_hl)),

        Instruction::new("SLA A", 0x27, 0, 8, Box::new(shift_left_low_carry_a)),
        Instruction::new("SLA B", 0x20, 0, 8, Box::new(shift_left_low_carry_b)),
        Instruction::new("SLA C", 0x21, 0, 8, Box::new(shift_left_low_carry_c)),
        Instruction::new("SLA D", 0x22, 0, 8, Box::new(shift_left_low_carry_d)),
        Instruction::new("SLA E", 0x23, 0, 8, Box::new(shift_left_low_carry_e)),
        Instruction::new("SLA H", 0x24, 0, 8, Box::new(shift_left_low_carry_h)),
        Instruction::new("SLA L", 0x24, 0, 8, Box::new(shift_left_low_carry_l)),

        Instruction::new("SRL A", 0x3F, 0, 8, Box::new(shift_right_low_carry_a)),
        Instruction::new("SRL B", 0x38, 0, 8, Box::new(shift_right_low_carry_b)),
        Instruction::new("SRL C", 0x39, 0, 8, Box::new(shift_right_low_carry_c)),
        Instruction::new("SRL D", 0x3A, 0, 8, Box::new(shift_right_low_carry_d)),
        Instruction::new("SRL E", 0x3B, 0, 8, Box::new(shift_right_low_carry_e)),
        Instruction::new("SRL H", 0x3C, 0, 8, Box::new(shift_right_low_carry_h)),
        Instruction::new("SRL L", 0x3D, 0, 8, Box::new(shift_right_low_carry_l)),

        Instruction::new("BIT 0,A", 0x47, 0, 8, test_bit_n(0, Reg8::A)),
        Instruction::new("BIT 1,A", 0x4F, 0, 8, test_bit_n(1, Reg8::A)),
        Instruction::new("BIT 2,A", 0x57, 0, 8, test_bit_n(2, Reg8::A)),
        Instruction::new("BIT 3,A", 0x5F, 0, 8, test_bit_n(3, Reg8::A)),
        Instruction::new("BIT 4,A", 0x67, 0, 8, test_bit_n(4, Reg8::A)),
        Instruction::new("BIT 5,A", 0x6F, 0, 8, test_bit_n(5, Reg8::A)),
        Instruction::new("BIT 6,A", 0x77, 0, 8, test_bit_n(6, Reg8::A)),
        Instruction::new("BIT 7,A", 0x7F, 0, 8, test_bit_n(7, Reg8::A)),

        Instruction::new("BIT 0,B", 0x40, 0, 8, test_bit_n(0, Reg8::B)),
        Instruction::new("BIT 1,B", 0x48, 0, 8, test_bit_n(1, Reg8::B)),
        Instruction::new("BIT 2,B", 0x50, 0, 8, test_bit_n(2, Reg8::B)),
        Instruction::new("BIT 3,B", 0x58, 0, 8, test_bit_n(3, Reg8::B)),
        Instruction::new("BIT 4,B", 0x60, 0, 8, test_bit_n(4, Reg8::B)),
        Instruction::new("BIT 5,B", 0x68, 0, 8, test_bit_n(5, Reg8::B)),
        Instruction::new("BIT 6,B", 0x70, 0, 8, test_bit_n(6, Reg8::B)),
        Instruction::new("BIT 7,B", 0x78, 0, 8, test_bit_n(7, Reg8::B)),

        Instruction::new("BIT 0,H", 0x44, 0, 8, test_bit_n(0, Reg8::H)),
        Instruction::new("BIT 1,H", 0x4C, 0, 8, test_bit_n(1, Reg8::H)),
        Instruction::new("BIT 2,H", 0x54, 0, 8, test_bit_n(2, Reg8::H)),
        Instruction::new("BIT 3,H", 0x5C, 0, 8, test_bit_n(3, Reg8::H)),
        Instruction::new("BIT 4,H", 0x64, 0, 8, test_bit_n(4, Reg8::H)),
        Instruction::new("BIT 5,H", 0x6C, 0, 8, test_bit_n(5, Reg8::H)),
        Instruction::new("BIT 6,H", 0x74, 0, 8, test_bit_n(6, Reg8::H)),
        Instruction::new("BIT 7,H", 0x7C, 0, 8, test_bit_n(7, Reg8::H)),

        Instruction::new("BIT 0,(HL)", 0x46, 0, 16, test_bit_n(0, Reg8::MemHl)),
        Instruction::new("BIT 1,(HL)", 0x4E, 0, 16, test_bit_n(1, Reg8::MemHl)),
        Instruction::new("BIT 2,(HL)", 0x56, 0, 16, test_bit_n(2, Reg8::MemHl)),
        Instruction::new("BIT 3,(HL)", 0x5E, 0, 16, test_bit_n(3, Reg8::MemHl)),
        Instruction::new("BIT 4,(HL)", 0x66, 0, 16, test_bit_n(4, Reg8::MemHl)),
        Instruction::new("BIT 5,(HL)", 0x6E, 0, 16, test_bit_n(5, Reg8::MemHl)),
        Instruction::new("BIT 6,(HL)", 0x76, 0, 16, test_bit_n(6, Reg8::MemHl)),
        Instruction::new("BIT 7,(HL)", 0x7E, 0, 16, test_bit_n(7, Reg8::MemHl)),

        Instruction::new("RES 0,A", 0x87, 0, 8, Box::new(reset_bit_0_a)),
        Instruction::new("RES 0,(HL)", 0x86, 0, 16, Box::new(reset_bit_0_mem_hl)),
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
        _ => panic!("Test bit instruction passing a number not 0-7")
    };

    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = true;
    gb.cpu.flag.zero = val & mask == 0;
}

fn test_bit_n(bit: u8, reg: Reg8) -> Box<Fn(&mut GameBoy, u8, u8)> {
    Box::new(move |gb, _, _| {
        let b = get_reg8(gb, reg);
        test_bit(gb, b, bit)
    })
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
    let result = rotate_left(gb, val, false);
    gb.cpu.set_a(result);
}

pub fn rotate_left_a_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_a(result);
}

fn rotate_left_b_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_b(result);
}

fn rotate_left_c_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_c(result);
}

fn rotate_left_d_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_d(result);
}

fn rotate_left_e_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_e(result);
}

fn rotate_left_h_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_h(result);
}

fn rotate_left_l_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    let result = rotate_left(gb, val, true);
    gb.cpu.set_l(result);
}

pub fn rotate_right_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = rotate_right(gb, val, false);
    gb.cpu.set_a(result);
}

pub fn rotate_right_a_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = rotate_right(gb, val, true);
    gb.cpu.set_a(result);
}

fn rotate_right_b_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    let result = rotate_right(gb, val, true);
    gb.cpu.set_b(result);
}

fn rotate_right_c_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    let result = rotate_right(gb, val, true);
    gb.cpu.set_c(result);
}

fn rotate_right_d_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    let result = rotate_right(gb, val, true);
    gb.cpu.set_d(result);
}

fn rotate_right_e_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    let result = rotate_right(gb, val, true);
    gb.cpu.set_e(result);
}

fn rotate_right_h_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    let result = rotate_right(gb, val, true);
    gb.cpu.set_h(result);
}

fn rotate_right_l_through(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    let result = rotate_right(gb, val, true);
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

fn reset_bit_0_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.memory.get_byte(gb.cpu.hl);
    gb.memory.set_byte(gb.cpu.hl, reset_bit(val, 0));
}

fn shift_right_low_carry(gb: &mut GameBoy, val: u8) -> u8 {
    gb.cpu.flag.carry = val & 0x01 == 0x01;
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    let result = val >> 1;
    gb.cpu.flag.zero = result == 0;
    result
}

fn shift_left_low_carry(gb: &mut GameBoy, val: u8) -> u8 {
    gb.cpu.flag.carry = val & 0x80 == 0x80;
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    let result = val << 1;
    gb.cpu.flag.zero = result == 0;
    result
}

fn shift_right_low_carry_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_right_low_carry(gb, val);
    gb.cpu.set_a(result);
}

fn shift_right_low_carry_b(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    let result = shift_right_low_carry(gb, val);
    gb.cpu.set_b(result);
}

fn shift_right_low_carry_c(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    let result = shift_right_low_carry(gb, val);
    gb.cpu.set_c(result);
}

fn shift_right_low_carry_d(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    let result = shift_right_low_carry(gb, val);
    gb.cpu.set_d(result);
}

fn shift_right_low_carry_e(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    let result = shift_right_low_carry(gb, val);
    gb.cpu.set_e(result);
}

fn shift_right_low_carry_h(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    let result = shift_right_low_carry(gb, val);
    gb.cpu.set_h(result);
}

fn shift_right_low_carry_l(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    let result = shift_right_low_carry(gb, val);
    gb.cpu.set_l(result);
}

fn shift_left_low_carry_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_left_low_carry(gb, val);
    gb.cpu.set_a(result);
}

fn shift_left_low_carry_b(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_left_low_carry(gb, val);
    gb.cpu.set_a(result);
}

fn shift_left_low_carry_c(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_left_low_carry(gb, val);
    gb.cpu.set_a(result);
}

fn shift_left_low_carry_d(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_left_low_carry(gb, val);
    gb.cpu.set_a(result);
}

fn shift_left_low_carry_e(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_left_low_carry(gb, val);
    gb.cpu.set_a(result);
}

fn shift_left_low_carry_h(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_left_low_carry(gb, val);
    gb.cpu.set_a(result);
}

fn shift_left_low_carry_l(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    let result = shift_left_low_carry(gb, val);
    gb.cpu.set_a(result);
}