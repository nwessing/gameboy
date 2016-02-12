use instructions::Instruction;
use game_boy::GameBoy;
use math::rotate_left;
use math::rotate_right;
use util::Reg8;
use util::get_reg8;
use util::set_reg8;

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
        Instruction::new("SLA L", 0x25, 0, 8, Box::new(shift_left_low_carry_l)),

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

        Instruction::new("BIT 0,C", 0x41, 0, 8, test_bit_n(0, Reg8::C)),
        Instruction::new("BIT 1,C", 0x49, 0, 8, test_bit_n(1, Reg8::C)),
        Instruction::new("BIT 2,C", 0x51, 0, 8, test_bit_n(2, Reg8::C)),
        Instruction::new("BIT 3,C", 0x59, 0, 8, test_bit_n(3, Reg8::C)),
        Instruction::new("BIT 4,C", 0x61, 0, 8, test_bit_n(4, Reg8::C)),
        Instruction::new("BIT 5,C", 0x69, 0, 8, test_bit_n(5, Reg8::C)),
        Instruction::new("BIT 6,C", 0x71, 0, 8, test_bit_n(6, Reg8::C)),
        Instruction::new("BIT 7,C", 0x79, 0, 8, test_bit_n(7, Reg8::C)),

        Instruction::new("BIT 0,D", 0x42, 0, 8, test_bit_n(0, Reg8::D)),
        Instruction::new("BIT 1,D", 0x4A, 0, 8, test_bit_n(1, Reg8::D)),
        Instruction::new("BIT 2,D", 0x52, 0, 8, test_bit_n(2, Reg8::D)),
        Instruction::new("BIT 3,D", 0x5A, 0, 8, test_bit_n(3, Reg8::D)),
        Instruction::new("BIT 4,D", 0x62, 0, 8, test_bit_n(4, Reg8::D)),
        Instruction::new("BIT 5,D", 0x6A, 0, 8, test_bit_n(5, Reg8::D)),
        Instruction::new("BIT 6,D", 0x72, 0, 8, test_bit_n(6, Reg8::D)),
        Instruction::new("BIT 7,D", 0x7A, 0, 8, test_bit_n(7, Reg8::D)),

        Instruction::new("BIT 0,E", 0x43, 0, 8, test_bit_n(0, Reg8::E)),
        Instruction::new("BIT 1,E", 0x4B, 0, 8, test_bit_n(1, Reg8::E)),
        Instruction::new("BIT 2,E", 0x53, 0, 8, test_bit_n(2, Reg8::E)),
        Instruction::new("BIT 3,E", 0x5B, 0, 8, test_bit_n(3, Reg8::E)),
        Instruction::new("BIT 4,E", 0x63, 0, 8, test_bit_n(4, Reg8::E)),
        Instruction::new("BIT 5,E", 0x6B, 0, 8, test_bit_n(5, Reg8::E)),
        Instruction::new("BIT 6,E", 0x73, 0, 8, test_bit_n(6, Reg8::E)),
        Instruction::new("BIT 7,E", 0x7B, 0, 8, test_bit_n(7, Reg8::E)),

        Instruction::new("BIT 0,H", 0x44, 0, 8, test_bit_n(0, Reg8::H)),
        Instruction::new("BIT 1,H", 0x4C, 0, 8, test_bit_n(1, Reg8::H)),
        Instruction::new("BIT 2,H", 0x54, 0, 8, test_bit_n(2, Reg8::H)),
        Instruction::new("BIT 3,H", 0x5C, 0, 8, test_bit_n(3, Reg8::H)),
        Instruction::new("BIT 4,H", 0x64, 0, 8, test_bit_n(4, Reg8::H)),
        Instruction::new("BIT 5,H", 0x6C, 0, 8, test_bit_n(5, Reg8::H)),
        Instruction::new("BIT 6,H", 0x74, 0, 8, test_bit_n(6, Reg8::H)),
        Instruction::new("BIT 7,H", 0x7C, 0, 8, test_bit_n(7, Reg8::H)),

        Instruction::new("BIT 0,L", 0x45, 0, 8, test_bit_n(0, Reg8::L)),
        Instruction::new("BIT 1,L", 0x4D, 0, 8, test_bit_n(1, Reg8::L)),
        Instruction::new("BIT 2,L", 0x55, 0, 8, test_bit_n(2, Reg8::L)),
        Instruction::new("BIT 3,L", 0x5D, 0, 8, test_bit_n(3, Reg8::L)),
        Instruction::new("BIT 4,L", 0x65, 0, 8, test_bit_n(4, Reg8::L)),
        Instruction::new("BIT 5,L", 0x6D, 0, 8, test_bit_n(5, Reg8::L)),
        Instruction::new("BIT 6,L", 0x75, 0, 8, test_bit_n(6, Reg8::L)),
        Instruction::new("BIT 7,L", 0x7D, 0, 8, test_bit_n(7, Reg8::L)),

        Instruction::new("BIT 0,(HL)", 0x46, 0, 16, test_bit_n(0, Reg8::MemHl)),
        Instruction::new("BIT 1,(HL)", 0x4E, 0, 16, test_bit_n(1, Reg8::MemHl)),
        Instruction::new("BIT 2,(HL)", 0x56, 0, 16, test_bit_n(2, Reg8::MemHl)),
        Instruction::new("BIT 3,(HL)", 0x5E, 0, 16, test_bit_n(3, Reg8::MemHl)),
        Instruction::new("BIT 4,(HL)", 0x66, 0, 16, test_bit_n(4, Reg8::MemHl)),
        Instruction::new("BIT 5,(HL)", 0x6E, 0, 16, test_bit_n(5, Reg8::MemHl)),
        Instruction::new("BIT 6,(HL)", 0x76, 0, 16, test_bit_n(6, Reg8::MemHl)),
        Instruction::new("BIT 7,(HL)", 0x7E, 0, 16, test_bit_n(7, Reg8::MemHl)),

        Instruction::new("RES 0,A", 0x87, 0, 8, reset_bit_n(0, Reg8::A)),
        Instruction::new("RES 1,A", 0x8F, 0, 8, reset_bit_n(1, Reg8::A)),
        Instruction::new("RES 2,A", 0x97, 0, 8, reset_bit_n(2, Reg8::A)),
        Instruction::new("RES 3,A", 0x9F, 0, 8, reset_bit_n(3, Reg8::A)),
        Instruction::new("RES 4,A", 0xA7, 0, 8, reset_bit_n(4, Reg8::A)),
        Instruction::new("RES 5,A", 0xAF, 0, 8, reset_bit_n(5, Reg8::A)),
        Instruction::new("RES 6,A", 0xB7, 0, 8, reset_bit_n(6, Reg8::A)),
        Instruction::new("RES 7,A", 0xBF, 0, 8, reset_bit_n(7, Reg8::A)),

        Instruction::new("RES 0,B", 0x80, 0, 8, reset_bit_n(0, Reg8::B)),
        Instruction::new("RES 1,B", 0x88, 0, 8, reset_bit_n(1, Reg8::B)),
        Instruction::new("RES 2,B", 0x90, 0, 8, reset_bit_n(2, Reg8::B)),
        Instruction::new("RES 3,B", 0x98, 0, 8, reset_bit_n(3, Reg8::B)),
        Instruction::new("RES 4,B", 0xA0, 0, 8, reset_bit_n(4, Reg8::B)),
        Instruction::new("RES 5,B", 0xA8, 0, 8, reset_bit_n(5, Reg8::B)),
        Instruction::new("RES 6,B", 0xB0, 0, 8, reset_bit_n(6, Reg8::B)),
        Instruction::new("RES 7,B", 0xB8, 0, 8, reset_bit_n(7, Reg8::B)),

        Instruction::new("RES 0,C", 0x81, 0, 8, reset_bit_n(0, Reg8::C)),
        Instruction::new("RES 1,C", 0x89, 0, 8, reset_bit_n(1, Reg8::C)),
        Instruction::new("RES 2,C", 0x91, 0, 8, reset_bit_n(2, Reg8::C)),
        Instruction::new("RES 3,C", 0x99, 0, 8, reset_bit_n(3, Reg8::C)),
        Instruction::new("RES 4,C", 0xA1, 0, 8, reset_bit_n(4, Reg8::C)),
        Instruction::new("RES 5,C", 0xA9, 0, 8, reset_bit_n(5, Reg8::C)),
        Instruction::new("RES 6,C", 0xB1, 0, 8, reset_bit_n(6, Reg8::C)),
        Instruction::new("RES 7,C", 0xB9, 0, 8, reset_bit_n(7, Reg8::C)),

        Instruction::new("RES 0,D", 0x82, 0, 8, reset_bit_n(0, Reg8::D)),
        Instruction::new("RES 1,D", 0x8A, 0, 8, reset_bit_n(1, Reg8::D)),
        Instruction::new("RES 2,D", 0x92, 0, 8, reset_bit_n(2, Reg8::D)),
        Instruction::new("RES 3,D", 0x9A, 0, 8, reset_bit_n(3, Reg8::D)),
        Instruction::new("RES 4,D", 0xA2, 0, 8, reset_bit_n(4, Reg8::D)),
        Instruction::new("RES 5,D", 0xAA, 0, 8, reset_bit_n(5, Reg8::D)),
        Instruction::new("RES 6,D", 0xB2, 0, 8, reset_bit_n(6, Reg8::D)),
        Instruction::new("RES 7,D", 0xBA, 0, 8, reset_bit_n(7, Reg8::D)),

        Instruction::new("RES 0,E", 0x83, 0, 8, reset_bit_n(0, Reg8::E)),
        Instruction::new("RES 1,E", 0x8B, 0, 8, reset_bit_n(1, Reg8::E)),
        Instruction::new("RES 2,E", 0x93, 0, 8, reset_bit_n(2, Reg8::E)),
        Instruction::new("RES 3,E", 0x9B, 0, 8, reset_bit_n(3, Reg8::E)),
        Instruction::new("RES 4,E", 0xA3, 0, 8, reset_bit_n(4, Reg8::E)),
        Instruction::new("RES 5,E", 0xAB, 0, 8, reset_bit_n(5, Reg8::E)),
        Instruction::new("RES 6,E", 0xB3, 0, 8, reset_bit_n(6, Reg8::E)),
        Instruction::new("RES 7,E", 0xBB, 0, 8, reset_bit_n(7, Reg8::E)),

        Instruction::new("RES 0,H", 0x84, 0, 8, reset_bit_n(0, Reg8::H)),
        Instruction::new("RES 1,H", 0x8C, 0, 8, reset_bit_n(1, Reg8::H)),
        Instruction::new("RES 2,H", 0x94, 0, 8, reset_bit_n(2, Reg8::H)),
        Instruction::new("RES 3,H", 0x9C, 0, 8, reset_bit_n(3, Reg8::H)),
        Instruction::new("RES 4,H", 0xA4, 0, 8, reset_bit_n(4, Reg8::H)),
        Instruction::new("RES 5,H", 0xAC, 0, 8, reset_bit_n(5, Reg8::H)),
        Instruction::new("RES 6,H", 0xB4, 0, 8, reset_bit_n(6, Reg8::H)),
        Instruction::new("RES 7,H", 0xBC, 0, 8, reset_bit_n(7, Reg8::H)),

        Instruction::new("RES 0,L", 0x85, 0, 8, reset_bit_n(0, Reg8::L)),
        Instruction::new("RES 1,L", 0x8D, 0, 8, reset_bit_n(1, Reg8::L)),
        Instruction::new("RES 2,L", 0x95, 0, 8, reset_bit_n(2, Reg8::L)),
        Instruction::new("RES 3,L", 0x9D, 0, 8, reset_bit_n(3, Reg8::L)),
        Instruction::new("RES 4,L", 0xA5, 0, 8, reset_bit_n(4, Reg8::L)),
        Instruction::new("RES 5,L", 0xAD, 0, 8, reset_bit_n(5, Reg8::L)),
        Instruction::new("RES 6,L", 0xB5, 0, 8, reset_bit_n(6, Reg8::L)),
        Instruction::new("RES 7,L", 0xBD, 0, 8, reset_bit_n(7, Reg8::L)),

        Instruction::new("RES 0,(HL)", 0x86, 0, 8, reset_bit_n(0, Reg8::MemHl)),
        Instruction::new("RES 1,(HL)", 0x8E, 0, 8, reset_bit_n(1, Reg8::MemHl)),
        Instruction::new("RES 2,(HL)", 0x96, 0, 8, reset_bit_n(2, Reg8::MemHl)),
        Instruction::new("RES 3,(HL)", 0x9E, 0, 8, reset_bit_n(3, Reg8::MemHl)),
        Instruction::new("RES 4,(HL)", 0xA6, 0, 8, reset_bit_n(4, Reg8::MemHl)),
        Instruction::new("RES 5,(HL)", 0xAE, 0, 8, reset_bit_n(5, Reg8::MemHl)),
        Instruction::new("RES 6,(HL)", 0xB6, 0, 8, reset_bit_n(6, Reg8::MemHl)),
        Instruction::new("RES 7,(HL)", 0xBE, 0, 8, reset_bit_n(7, Reg8::MemHl)),
        
        Instruction::new("SET 0,A", 0xC7, 0, 8, set_bit_n(0, Reg8::A)),
        Instruction::new("SET 1,A", 0xCF, 0, 8, set_bit_n(1, Reg8::A)),
        Instruction::new("SET 2,A", 0xD7, 0, 8, set_bit_n(2, Reg8::A)),
        Instruction::new("SET 3,A", 0xDF, 0, 8, set_bit_n(3, Reg8::A)),
        Instruction::new("SET 4,A", 0xE7, 0, 8, set_bit_n(4, Reg8::A)),
        Instruction::new("SET 5,A", 0xEF, 0, 8, set_bit_n(5, Reg8::A)),
        Instruction::new("SET 6,A", 0xF7, 0, 8, set_bit_n(6, Reg8::A)),
        Instruction::new("SET 7,A", 0xFF, 0, 8, set_bit_n(7, Reg8::A)),

        Instruction::new("SET 0,B", 0xC0, 0, 8, set_bit_n(0, Reg8::B)),
        Instruction::new("SET 1,B", 0xC8, 0, 8, set_bit_n(1, Reg8::B)),
        Instruction::new("SET 2,B", 0xD0, 0, 8, set_bit_n(2, Reg8::B)),
        Instruction::new("SET 3,B", 0xD8, 0, 8, set_bit_n(3, Reg8::B)),
        Instruction::new("SET 4,B", 0xE0, 0, 8, set_bit_n(4, Reg8::B)),
        Instruction::new("SET 5,B", 0xE8, 0, 8, set_bit_n(5, Reg8::B)),
        Instruction::new("SET 6,B", 0xF0, 0, 8, set_bit_n(6, Reg8::B)),
        Instruction::new("SET 7,B", 0xF8, 0, 8, set_bit_n(7, Reg8::B)),

        Instruction::new("SET 0,C", 0xC1, 0, 8, set_bit_n(0, Reg8::C)),
        Instruction::new("SET 1,C", 0xC9, 0, 8, set_bit_n(1, Reg8::C)),
        Instruction::new("SET 2,C", 0xD1, 0, 8, set_bit_n(2, Reg8::C)),
        Instruction::new("SET 3,C", 0xD9, 0, 8, set_bit_n(3, Reg8::C)),
        Instruction::new("SET 4,C", 0xE1, 0, 8, set_bit_n(4, Reg8::C)),
        Instruction::new("SET 5,C", 0xE9, 0, 8, set_bit_n(5, Reg8::C)),
        Instruction::new("SET 6,C", 0xF1, 0, 8, set_bit_n(6, Reg8::C)),
        Instruction::new("SET 7,C", 0xF9, 0, 8, set_bit_n(7, Reg8::C)),

        Instruction::new("SET 0,D", 0xC2, 0, 8, set_bit_n(0, Reg8::D)),
        Instruction::new("SET 1,D", 0xCA, 0, 8, set_bit_n(1, Reg8::D)),
        Instruction::new("SET 2,D", 0xD2, 0, 8, set_bit_n(2, Reg8::D)),
        Instruction::new("SET 3,D", 0xDA, 0, 8, set_bit_n(3, Reg8::D)),
        Instruction::new("SET 4,D", 0xE2, 0, 8, set_bit_n(4, Reg8::D)),
        Instruction::new("SET 5,D", 0xEA, 0, 8, set_bit_n(5, Reg8::D)),
        Instruction::new("SET 6,D", 0xF2, 0, 8, set_bit_n(6, Reg8::D)),
        Instruction::new("SET 7,D", 0xFA, 0, 8, set_bit_n(7, Reg8::D)),

        Instruction::new("SET 0,E", 0xC3, 0, 8, set_bit_n(0, Reg8::E)),
        Instruction::new("SET 1,E", 0xCB, 0, 8, set_bit_n(1, Reg8::E)),
        Instruction::new("SET 2,E", 0xD3, 0, 8, set_bit_n(2, Reg8::E)),
        Instruction::new("SET 3,E", 0xDB, 0, 8, set_bit_n(3, Reg8::E)),
        Instruction::new("SET 4,E", 0xE3, 0, 8, set_bit_n(4, Reg8::E)),
        Instruction::new("SET 5,E", 0xEB, 0, 8, set_bit_n(5, Reg8::E)),
        Instruction::new("SET 6,E", 0xF3, 0, 8, set_bit_n(6, Reg8::E)),
        Instruction::new("SET 7,E", 0xFB, 0, 8, set_bit_n(7, Reg8::E)),

        Instruction::new("SET 0,H", 0xC4, 0, 8, set_bit_n(0, Reg8::H)),
        Instruction::new("SET 1,H", 0xCC, 0, 8, set_bit_n(1, Reg8::H)),
        Instruction::new("SET 2,H", 0xD4, 0, 8, set_bit_n(2, Reg8::H)),
        Instruction::new("SET 3,H", 0xDC, 0, 8, set_bit_n(3, Reg8::H)),
        Instruction::new("SET 4,H", 0xE4, 0, 8, set_bit_n(4, Reg8::H)),
        Instruction::new("SET 5,H", 0xEC, 0, 8, set_bit_n(5, Reg8::H)),
        Instruction::new("SET 6,H", 0xF4, 0, 8, set_bit_n(6, Reg8::H)),
        Instruction::new("SET 7,H", 0xFC, 0, 8, set_bit_n(7, Reg8::H)),
        
        Instruction::new("SET 0,L", 0xC5, 0, 8, set_bit_n(0, Reg8::L)),
        Instruction::new("SET 1,L", 0xCD, 0, 8, set_bit_n(1, Reg8::L)),
        Instruction::new("SET 2,L", 0xD5, 0, 8, set_bit_n(2, Reg8::L)),
        Instruction::new("SET 3,L", 0xDD, 0, 8, set_bit_n(3, Reg8::L)),
        Instruction::new("SET 4,L", 0xE5, 0, 8, set_bit_n(4, Reg8::L)),
        Instruction::new("SET 5,L", 0xED, 0, 8, set_bit_n(5, Reg8::L)),
        Instruction::new("SET 6,L", 0xF5, 0, 8, set_bit_n(6, Reg8::L)),
        Instruction::new("SET 7,L", 0xFD, 0, 8, set_bit_n(7, Reg8::L)),

        Instruction::new("SET 0,(HL)", 0xC6, 0, 8, set_bit_n(0, Reg8::MemHl)),
        Instruction::new("SET 1,(HL)", 0xCE, 0, 8, set_bit_n(1, Reg8::MemHl)),
        Instruction::new("SET 2,(HL)", 0xD6, 0, 8, set_bit_n(2, Reg8::MemHl)),
        Instruction::new("SET 3,(HL)", 0xDE, 0, 8, set_bit_n(3, Reg8::MemHl)),
        Instruction::new("SET 4,(HL)", 0xE6, 0, 8, set_bit_n(4, Reg8::MemHl)),
        Instruction::new("SET 5,(HL)", 0xEE, 0, 8, set_bit_n(5, Reg8::MemHl)),
        Instruction::new("SET 6,(HL)", 0xF6, 0, 8, set_bit_n(6, Reg8::MemHl)),
        Instruction::new("SET 7,(HL)", 0xFE, 0, 8, set_bit_n(7, Reg8::MemHl)), 
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
        _ => panic!("Reset bit instruction configured incorrectly")
    };
    value & mask
}

fn reset_bit_n(bit: u8, reg: Reg8) -> Box<Fn(&mut GameBoy, u8, u8)> {
    Box::new(move |gb, _, _| {
        let reg_val = get_reg8(gb, reg);
        let result = reset_bit(reg_val, bit);
        set_reg8(gb, reg, result);
    })
}

fn set_bit(value: u8, bit: u8) -> u8 {
    let mask = match  bit {
        0 => 0b00000001,
        1 => 0b00000010,
        2 => 0b00000100,
        3 => 0b00001000,
        4 => 0b00010000,
        5 => 0b00100000,
        6 => 0b01000000,
        7 => 0b10000000,
        _ => panic!("Set bit instruction configured incorrectly")
    };
    value | mask
}

fn set_bit_n(bit: u8, reg: Reg8) -> Box<Fn(&mut GameBoy, u8, u8)> {
    Box::new(move |gb, _, _| {
        let reg_val = get_reg8(gb, reg);
        let result = set_bit(reg_val, bit);
        set_reg8(gb, reg, result);
    })
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