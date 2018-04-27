use game_boy::GameBoy;
use util::concat_bytes;
use util::push_word;
use util::Reg8;
use util::get_reg8;
use util::set_reg8;
use math::add_u16_and_i8;
use math::add_u16_and_i8_affect_flags;
use math::{rotate_left, rotate_right};

pub struct Instruction {
    pub name: &'static str,
    pub opcode: u8,
    pub operand_length: u8,
    pub cycles: u8,
    pub exec: Box<Fn(&mut GameBoy, u8, u8)>
}

impl Instruction {
    pub fn new(name: &'static str, opcode: u8, operand_length: u8, cycles: u8, exec: Box<Fn(&mut GameBoy, u8, u8)>) -> Instruction {
        Instruction {
            name: name,
            opcode: opcode,
            operand_length: operand_length,
            cycles: cycles,
            exec: exec,
        }
    }
}

pub fn get_instruction_set() -> Vec<Instruction> {
    vec![
        Instruction::new("NOP", 0x00, 0, 4, Box::new(nop)),
        Instruction::new("HALT", 0x76, 0, 4, Box::new(halt)),
        Instruction::new("STOP", 0x10, 1, 4, Box::new(stop)), // STOP has a length of 2 bytes, but doesn't use it as parameter.

        Instruction::new("LD A,n", 0x3E, 1, 8, load_x_imm(Reg8::A)),
        Instruction::new("LD B,n", 0x06, 1, 8, load_x_imm(Reg8::B)),
        Instruction::new("LD C,n", 0x0E, 1, 8, load_x_imm(Reg8::C)),
        Instruction::new("LD D,n", 0x16, 1, 8, load_x_imm(Reg8::D)),
        Instruction::new("LD E,n", 0x1E, 1, 8, load_x_imm(Reg8::E)),
        Instruction::new("LD H,n", 0x26, 1, 8, load_x_imm(Reg8::H)),
        Instruction::new("LD L,n", 0x2E, 1, 8, load_x_imm(Reg8::L)),
        Instruction::new("LD A,A", 0x7F, 0, 4, load_x_y(Reg8::A, Reg8::A)),
        Instruction::new("LD A,B", 0x78, 0, 4, load_x_y(Reg8::A, Reg8::B)),
        Instruction::new("LD A,C", 0x79, 0, 4, load_x_y(Reg8::A, Reg8::C)),
        Instruction::new("LD A,D", 0x7A, 0, 4, load_x_y(Reg8::A, Reg8::D)),
        Instruction::new("LD A,E", 0x7B, 0, 4, load_x_y(Reg8::A, Reg8::E)),
        Instruction::new("LD A,H", 0x7C, 0, 4, load_x_y(Reg8::A, Reg8::H)),
        Instruction::new("LD A,L", 0x7D, 0, 4, load_x_y(Reg8::A, Reg8::L)),
        Instruction::new("LD A,(HL)", 0x7E, 0, 8, load_x_y(Reg8::A, Reg8::MemHl)),
        Instruction::new("LD B,A", 0x47, 0, 4, load_x_y(Reg8::B, Reg8::A)),
        Instruction::new("LD B,B", 0x40, 0, 4, load_x_y(Reg8::B, Reg8::B)),
        Instruction::new("LD B,C", 0x41, 0, 4, load_x_y(Reg8::B, Reg8::C)),
        Instruction::new("LD B,D", 0x42, 0, 4, load_x_y(Reg8::B, Reg8::D)),
        Instruction::new("LD B,E", 0x43, 0, 4, load_x_y(Reg8::B, Reg8::E)),
        Instruction::new("LD B,H", 0x44, 0, 4, load_x_y(Reg8::B, Reg8::H)),
        Instruction::new("LD B,L", 0x45, 0, 4, load_x_y(Reg8::B, Reg8::L)),
        Instruction::new("LD B,(HL)", 0x46, 0, 8, load_x_y(Reg8::B, Reg8::MemHl)),
        Instruction::new("LD C,A", 0x4F, 0, 4, load_x_y(Reg8::C, Reg8::A)),
        Instruction::new("LD C,B", 0x48, 0, 4, load_x_y(Reg8::C, Reg8::B)),
        Instruction::new("LD C,C", 0x49, 0, 4, load_x_y(Reg8::C, Reg8::C)),
        Instruction::new("LD C,D", 0x4A, 0, 4, load_x_y(Reg8::C, Reg8::D)),
        Instruction::new("LD C,E", 0x4B, 0, 4, load_x_y(Reg8::C, Reg8::E)),
        Instruction::new("LD C,H", 0x4C, 0, 4, load_x_y(Reg8::C, Reg8::H)),
        Instruction::new("LD C,L", 0x4D, 0, 4, load_x_y(Reg8::C, Reg8::L)),
        Instruction::new("LD C,(HL)", 0x4E, 0, 8, load_x_y(Reg8::C, Reg8::MemHl)),
        Instruction::new("LD D,A", 0x57, 0, 4, load_x_y(Reg8::D, Reg8::A)),
        Instruction::new("LD D,B", 0x50, 0, 4, load_x_y(Reg8::D, Reg8::B)),
        Instruction::new("LD D,C", 0x51, 0, 4, load_x_y(Reg8::D, Reg8::C)),
        Instruction::new("LD D,D", 0x52, 0, 4, load_x_y(Reg8::D, Reg8::D)),
        Instruction::new("LD D,E", 0x53, 0, 4, load_x_y(Reg8::D, Reg8::E)),
        Instruction::new("LD D,H", 0x54, 0, 4, load_x_y(Reg8::D, Reg8::H)),
        Instruction::new("LD D,L", 0x55, 0, 4, load_x_y(Reg8::D, Reg8::L)),
        Instruction::new("LD D,(HL)", 0x56, 0, 8, load_x_y(Reg8::D, Reg8::MemHl)),
        Instruction::new("LD E,A", 0x5F, 0, 4, load_x_y(Reg8::E, Reg8::A)),
        Instruction::new("LD E,B", 0x58, 0, 4, load_x_y(Reg8::E, Reg8::B)),
        Instruction::new("LD E,C", 0x59, 0, 4, load_x_y(Reg8::E, Reg8::C)),
        Instruction::new("LD E,D", 0x5A, 0, 4, load_x_y(Reg8::E, Reg8::D)),
        Instruction::new("LD E,E", 0x5B, 0, 4, load_x_y(Reg8::E, Reg8::E)),
        Instruction::new("LD E,H", 0x5C, 0, 4, load_x_y(Reg8::E, Reg8::H)),
        Instruction::new("LD E,L", 0x5D, 0, 4, load_x_y(Reg8::E, Reg8::L)),
        Instruction::new("LD E,(HL)", 0x5E, 0, 8, load_x_y(Reg8::E, Reg8::MemHl)),
        Instruction::new("LD H,A", 0x67, 0, 4, load_x_y(Reg8::H, Reg8::A)),
        Instruction::new("LD H,B", 0x60, 0, 4, load_x_y(Reg8::H, Reg8::B)),
        Instruction::new("LD H,C", 0x61, 0, 4, load_x_y(Reg8::H, Reg8::C)),
        Instruction::new("LD H,D", 0x62, 0, 4, load_x_y(Reg8::H, Reg8::D)),
        Instruction::new("LD H,E", 0x63, 0, 4, load_x_y(Reg8::H, Reg8::E)),
        Instruction::new("LD H,H", 0x64, 0, 4, load_x_y(Reg8::H, Reg8::H)),
        Instruction::new("LD H,L", 0x65, 0, 4, load_x_y(Reg8::H, Reg8::L)),
        Instruction::new("LD H,(HL)", 0x66, 0, 8, load_x_y(Reg8::H, Reg8::MemHl)),
        Instruction::new("LD L,A", 0x6f, 0, 4, load_x_y(Reg8::L, Reg8::A)),
        Instruction::new("LD L,B", 0x68, 0, 4, load_x_y(Reg8::L, Reg8::B)),
        Instruction::new("LD L,C", 0x69, 0, 4, load_x_y(Reg8::L, Reg8::C)),
        Instruction::new("LD L,D", 0x6A, 0, 4, load_x_y(Reg8::L, Reg8::D)),
        Instruction::new("LD L,E", 0x6B, 0, 4, load_x_y(Reg8::L, Reg8::E)),
        Instruction::new("LD L,H", 0x6C, 0, 4, load_x_y(Reg8::L, Reg8::H)),
        Instruction::new("LD L,L", 0x6D, 0, 4, load_x_y(Reg8::L, Reg8::L)),
        Instruction::new("LD L,(HL)", 0x6E, 0, 8, load_x_y(Reg8::L, Reg8::MemHl)),
        Instruction::new("LD (HL),A", 0x77, 0, 8, load_x_y(Reg8::MemHl, Reg8::A)),
        Instruction::new("LD (HL),B", 0x70, 0, 8, load_x_y(Reg8::MemHl, Reg8::B)),
        Instruction::new("LD (HL),C", 0x71, 0, 8, load_x_y(Reg8::MemHl, Reg8::C)),
        Instruction::new("LD (HL),D", 0x72, 0, 8, load_x_y(Reg8::MemHl, Reg8::D)),
        Instruction::new("LD (HL),E", 0x73, 0, 8, load_x_y(Reg8::MemHl, Reg8::E)),
        Instruction::new("LD (HL),H", 0x74, 0, 8, load_x_y(Reg8::MemHl, Reg8::H)),
        Instruction::new("LD (HL),L", 0x75, 0, 8, load_x_y(Reg8::MemHl, Reg8::L)),
        Instruction::new("LD (HL),n", 0x36, 1, 12, load_x_imm(Reg8::MemHl)),

        Instruction::new("LD A,(BC)", 0x0A, 0, 8, Box::new(load_a_mem_bc)),
        Instruction::new("LD A,(DE)", 0x1A, 0, 8, Box::new(load_a_mem_de)),
        Instruction::new("LD A,(nn)", 0xFA, 2, 16, Box::new(load_a_mem_nn)),
        Instruction::new("LD (BC),A", 0x02, 0, 8, Box::new(load_mem_bc_a)),
        Instruction::new("LD (DE),A", 0x12, 0, 8, Box::new(load_mem_de_a)),
        Instruction::new("LD (nn),A", 0xEA, 2, 16, Box::new(load_mem_nn_a)),

        Instruction::new("LD BC,nn", 0x01, 2, 12, Box::new(load_bc_nn)),
        Instruction::new("LD DE,nn", 0x11, 2, 12, Box::new(load_de_nn)),
        Instruction::new("LD HL,nn", 0x21, 2, 12, Box::new(load_hl_nn)),
        Instruction::new("LD SP,nn", 0x31, 2, 12, Box::new(load_sp_nn)),

        Instruction::new("LD (HL),A; DEC HL", 0x32, 0, 8, Box::new(load_mem_hl_with_a_dec_hl)),
        Instruction::new("LD (HL),A; INC HL", 0x22, 0, 8, Box::new(load_mem_hl_with_a_inc_hl)),
        Instruction::new("LD A,(HL); INC HL", 0x2A, 0, 8, Box::new(load_a_with_mem_hl_inc_hl)),
        Instruction::new("LD A,(HL); DEC HL", 0x3A, 0, 8, Box::new(load_a_with_mem_hl_dec_hl)),

        Instruction::new("LD SP,HL", 0xF9, 0, 8, Box::new(load_sp_hl)),
        Instruction::new("LD HL,SP+n", 0xF8, 1, 12, Box::new(load_hl_sp_plus_signed_n)),

        Instruction::new("LD (0xFF00 + n),A", 0xE0, 1, 12, Box::new(load_ff00_plus_n_with_a)),
        Instruction::new("LD A,(0xFF00 + n)", 0xF0, 1, 12, Box::new(load_a_with_ff00_plus_n)),
        Instruction::new("LD (0xFF00 + C),A", 0xE2, 0, 8, Box::new(load_ff00_plus_c_with_a)),
        Instruction::new("LD A,(0xFF00 + C)", 0xF2, 0, 8, Box::new(load_a_with_ff00_plus_c)),
        Instruction::new("LD (nn),SP", 0x08, 2, 20, Box::new(load_mem_nn_sp)),

        Instruction::new("PUSH AF", 0xF5, 0, 16, Box::new(push_af)),
        Instruction::new("PUSH BC", 0xC5, 0, 16, Box::new(push_bc)),
        Instruction::new("PUSH DE", 0xD5, 0, 16, Box::new(push_de)),
        Instruction::new("PUSH HL", 0xE5, 0, 16, Box::new(push_hl)),

        Instruction::new("POP AF", 0xF1, 0, 12, Box::new(pop_af)),
        Instruction::new("POP BC", 0xC1, 0, 12, Box::new(pop_bc)),
        Instruction::new("POP DE", 0xD1, 0, 12, Box::new(pop_de)),
        Instruction::new("POP HL", 0xE1, 0, 12, Box::new(pop_hl)),

        Instruction::new("AND A", 0xA7, 0, 4, Box::new(and_a)),
        Instruction::new("AND B", 0xA0, 0, 4, Box::new(and_b)),
        Instruction::new("AND C", 0xA1, 0, 4, Box::new(and_c)),
        Instruction::new("AND D", 0xA2, 0, 4, Box::new(and_d)),
        Instruction::new("AND E", 0xA3, 0, 4, Box::new(and_e)),
        Instruction::new("AND H", 0xA4, 0, 4, Box::new(and_h)),
        Instruction::new("AND L", 0xA5, 0, 4, Box::new(and_l)),
        Instruction::new("AND (HL)", 0xA6, 0, 8, Box::new(and_mem_hl)),
        Instruction::new("AND n", 0xE6, 1, 8, Box::new(and_n)),

        Instruction::new("OR A", 0xB7, 0, 4, Box::new(or_a)),
        Instruction::new("OR B", 0xB0, 0, 4, Box::new(or_b)),
        Instruction::new("OR C", 0xB1, 0, 4, Box::new(or_c)),
        Instruction::new("OR D", 0xB2, 0, 4, Box::new(or_d)),
        Instruction::new("OR E", 0xB3, 0, 4, Box::new(or_e)),
        Instruction::new("OR H", 0xB4, 0, 4, Box::new(or_h)),
        Instruction::new("OR L", 0xB5, 0, 4, Box::new(or_l)),
        Instruction::new("OR (HL)", 0xB6, 0, 8, Box::new(or_mem_hl)),
        Instruction::new("OR n", 0xF6, 1, 8, Box::new(or_n)),

        Instruction::new("XOR A", 0xAF, 0, 4, Box::new(xor_a)),
        Instruction::new("XOR B", 0xA8, 0, 4, Box::new(xor_b)),
        Instruction::new("XOR C", 0xA9, 0, 4, Box::new(xor_c)),
        Instruction::new("XOR D", 0xAA, 0, 4, Box::new(xor_d)),
        Instruction::new("XOR E", 0xAB, 0, 4, Box::new(xor_e)),
        Instruction::new("XOR H", 0xAC, 0, 4, Box::new(xor_h)),
        Instruction::new("XOR L", 0xAD, 0, 4, Box::new(xor_l)),
        Instruction::new("XOR (HL)", 0xAE, 0, 8, Box::new(xor_mem_hl)),
        Instruction::new("XOR n", 0xEE, 1, 8, Box::new(xor_n)),

        Instruction::new("INC A", 0x3C, 0, 4, Box::new(increment_a)),
        Instruction::new("INC B", 0x04, 0, 4, Box::new(increment_b)),
        Instruction::new("INC C", 0x0C, 0, 4, Box::new(increment_c)),
        Instruction::new("INC D", 0x14, 0, 4, Box::new(increment_d)),
        Instruction::new("INC E", 0x1C, 0, 4, Box::new(increment_e)),
        Instruction::new("INC H", 0x24, 0, 4, Box::new(increment_h)),
        Instruction::new("INC L", 0x2C, 0, 4, Box::new(increment_l)),
        Instruction::new("INC BC", 0x03, 0, 8, Box::new(increment_bc)),
        Instruction::new("INC DE", 0x13, 0, 8, Box::new(increment_de)),
        Instruction::new("INC HL", 0x23, 0, 8, Box::new(increment_hl)),
        Instruction::new("INC SP", 0x33, 0, 8, Box::new(increment_sp)),
        Instruction::new("INC (HL)", 0x34, 0, 12, Box::new(increment_mem_hl)),

        Instruction::new("DEC A", 0x3D, 0, 4, Box::new(decrement_a)),
        Instruction::new("DEC B", 0x05, 0, 4, Box::new(decrement_b)),
        Instruction::new("DEC C", 0x0D, 0, 4, Box::new(decrement_c)),
        Instruction::new("DEC D", 0x15, 0, 4, Box::new(decrement_d)),
        Instruction::new("DEC E", 0x1D, 0, 4, Box::new(decrement_e)),
        Instruction::new("DEC H", 0x25, 0, 4, Box::new(decrement_h)),
        Instruction::new("DEC L", 0x2D, 0, 4, Box::new(decrement_l)),
        Instruction::new("DEC BC", 0x0B, 0, 8, Box::new(decrement_bc)),
        Instruction::new("DEC DE", 0x1B, 0, 8, Box::new(decrement_de)),
        Instruction::new("DEC HL", 0x2B, 0, 8, Box::new(decrement_hl)),
        Instruction::new("DEC SP", 0x3B, 0, 8, Box::new(decrement_sp)),
        Instruction::new("DEC (HL)", 0x35, 0, 12, Box::new(decrement_mem_hl)),

        Instruction::new("ADD A", 0x87, 0, 4, Box::new(add_a)),
        Instruction::new("ADD B", 0x80, 0, 4, Box::new(add_b)),
        Instruction::new("ADD C", 0x81, 0, 4, Box::new(add_c)),
        Instruction::new("ADD D", 0x82, 0, 4, Box::new(add_d)),
        Instruction::new("ADD E", 0x83, 0, 4, Box::new(add_e)),
        Instruction::new("ADD H", 0x84, 0, 4, Box::new(add_h)),
        Instruction::new("ADD L", 0x85, 0, 4, Box::new(add_l)),
        Instruction::new("ADD (HL)", 0x86, 0, 8, Box::new(add_mem_hl)),
        Instruction::new("ADD n", 0xC6, 1, 8, Box::new(add_n)),
        Instruction::new("ADD HL,BC", 0x09, 0, 8, Box::new(add_hl_bc)),
        Instruction::new("ADD HL,DE", 0x19, 0, 8, Box::new(add_hl_de)),
        Instruction::new("ADD HL,HL", 0x29, 0, 8, Box::new(add_hl_hl)),
        Instruction::new("ADD HL,SP", 0x39, 0, 8, Box::new(add_hl_sp)),
        Instruction::new("ADD SP,n", 0xE8, 1, 16, Box::new(add_sp_signed_n)),

        Instruction::new("ADC A,A", 0x8F, 0, 4, Box::new(add_a_with_carry)),
        Instruction::new("ADC A,B", 0x88, 0, 4, Box::new(add_b_with_carry)),
        Instruction::new("ADC A,C", 0x89, 0, 4, Box::new(add_c_with_carry)),
        Instruction::new("ADC A,D", 0x8A, 0, 4, Box::new(add_d_with_carry)),
        Instruction::new("ADC A,E", 0x8B, 0, 4, Box::new(add_e_with_carry)),
        Instruction::new("ADC A,H", 0x8C, 0, 4, Box::new(add_h_with_carry)),
        Instruction::new("ADC A,L", 0x8D, 0, 4, Box::new(add_l_with_carry)),
        Instruction::new("ADC A,(HL)", 0x8E, 0, 8, Box::new(add_mem_hl_with_carry)),
        Instruction::new("ADC A,n", 0xCE, 1, 8, Box::new(add_n_with_carry)),

        Instruction::new("SUB A", 0x97, 0, 4, Box::new(subtract_a)),
        Instruction::new("SUB B", 0x90, 0, 4, Box::new(subtract_b)),
        Instruction::new("SUB C", 0x91, 0, 4, Box::new(subtract_c)),
        Instruction::new("SUB D", 0x92, 0, 4, Box::new(subtract_d)),
        Instruction::new("SUB E", 0x93, 0, 4, Box::new(subtract_e)),
        Instruction::new("SUB H", 0x94, 0, 4, Box::new(subtract_h)),
        Instruction::new("SUB L", 0x95, 0, 4, Box::new(subtract_l)),
        Instruction::new("SUB (HL)", 0x96, 0, 8, Box::new(subtract_mem_hl)),
        Instruction::new("SUB n", 0xD6, 1, 8, Box::new(subtract_n)),

        Instruction::new("SBC A", 0x9F, 0, 4, Box::new(subtract_a_with_carry)),
        Instruction::new("SBC B", 0x98, 0, 4, Box::new(subtract_b_with_carry)),
        Instruction::new("SBC C", 0x99, 0, 4, Box::new(subtract_c_with_carry)),
        Instruction::new("SBC D", 0x9A, 0, 4, Box::new(subtract_d_with_carry)),
        Instruction::new("SBC E", 0x9B, 0, 4, Box::new(subtract_e_with_carry)),
        Instruction::new("SBC H", 0x9C, 0, 4, Box::new(subtract_h_with_carry)),
        Instruction::new("SBC L", 0x9D, 0, 4, Box::new(subtract_l_with_carry)),
        Instruction::new("SBC (HL)", 0x9E, 0, 8, Box::new(subtract_mem_hl_with_carry)),
        Instruction::new("SBC n", 0xDE, 1, 8, Box::new(subtract_n_with_carry)),

        Instruction::new("CP A", 0xBF, 0, 4, Box::new(compare_a)),
        Instruction::new("CP B", 0xB8, 0, 4, Box::new(compare_b)),
        Instruction::new("CP C", 0xB9, 0, 4, Box::new(compare_c)),
        Instruction::new("CP D", 0xBA, 0, 4, Box::new(compare_d)),
        Instruction::new("CP E", 0xBB, 0, 4, Box::new(compare_e)),
        Instruction::new("CP H", 0xBC, 0, 4, Box::new(compare_h)),
        Instruction::new("CP L", 0xBD, 0, 4, Box::new(compare_l)),
        Instruction::new("CP (HL)", 0xBE, 0, 8, Box::new(compare_mem_hl)),
        Instruction::new("CP n", 0xFE, 1, 8, Box::new(compare_n)),

        Instruction::new("JP nn", 0xC3, 2, 12, Box::new(jump_immediate)),
        Instruction::new("JP NZ,nn", 0xC2, 2, 12, Box::new(jump_not_z_flag)),
        Instruction::new("JP Z,nn", 0xCA, 2, 12, Box::new(jump_z_flag)),
        Instruction::new("JP NC,nn", 0xD2, 2, 12, Box::new(jump_not_c_flag)),
        Instruction::new("JP C,nn", 0xDA, 2, 12, Box::new(jump_c_flag)),
        Instruction::new("JR n", 0x18, 1, 8, Box::new(jump_pc_plus_byte)),
        Instruction::new("JR NZ,n", 0x20, 1, 8, Box::new(jump_not_z_flag_pc_plus)),
        Instruction::new("JR Z,n", 0x28, 1, 8, Box::new(jump_z_flag_pc_plus)),
        Instruction::new("JR NC,n", 0x30, 1, 8, Box::new(jump_not_c_flag_pc_plus)),
        Instruction::new("JR C,n", 0x38, 1, 8, Box::new(jump_c_flag_pc_plus)),
        Instruction::new("JP (HL)", 0xE9, 0, 4, Box::new(jump_hl)),

        Instruction::new("CALL nn", 0xCD, 2, 12, Box::new(call_nn)),
        Instruction::new("CALL NZ,nn", 0xC4, 2, 12, Box::new(call_if_not_zero)),
        Instruction::new("CALL Z,nn", 0xCC, 2, 12, Box::new(call_if_zero)),
        Instruction::new("CALL NC,nn", 0xD4, 2, 12, Box::new(call_if_not_carry)),
        Instruction::new("CALL C,nn", 0xDC, 2, 12, Box::new(call_if_carry)),

        Instruction::new("RET", 0xC9, 0, 8, Box::new(sub_return)),
        Instruction::new("RET NZ", 0xC0, 0, 8, Box::new(sub_return_if_not_z)),
        Instruction::new("RET Z", 0xC8, 0, 8, Box::new(sub_return_if_z)),
        Instruction::new("RET NC", 0xD0, 0, 8, Box::new(sub_return_if_not_c)),
        Instruction::new("RET C", 0xD8, 0, 8, Box::new(sub_return_if_c)),
        Instruction::new("RETI", 0xD9, 0, 8, Box::new(sub_return_enable_interrupts)),

        Instruction::new("DI", 0xF3, 0, 4, Box::new(disable_interrupts)),
        Instruction::new("EI", 0xFB, 0, 4, Box::new(enable_interrupts)),

        Instruction::new("RLCA", 0x07, 0, 4, Box::new(rotate_left_a)),
        Instruction::new("RLA", 0x17, 0, 4, Box::new(rotate_left_a_through)),
        Instruction::new("RRCA", 0x0F, 0, 4, Box::new(rotate_right_a)),
        Instruction::new("RRA", 0x1F, 0, 4, Box::new(rotate_right_a_through)),

        Instruction::new("DAA", 0x27, 0, 4, Box::new(decimal_adjust_a)),
        Instruction::new("CPL", 0x2F, 0, 4, Box::new(complement_a)),
        Instruction::new("CCF", 0x3F, 0, 4, Box::new(complement_carry)),
        Instruction::new("SCF", 0x37, 0, 4, Box::new(set_carry)),

        Instruction::new("RST 0x00", 0xC7, 0, 32, Box::new(restart_00)),
        Instruction::new("RST 0x08", 0xCF, 0, 32, Box::new(restart_08)),
        Instruction::new("RST 0x10", 0xD7, 0, 32, Box::new(restart_10)),
        Instruction::new("RST 0x18", 0xDF, 0, 32, Box::new(restart_18)),
        Instruction::new("RST 0x20", 0xE7, 0, 32, Box::new(restart_20)),
        Instruction::new("RST 0x28", 0xEF, 0, 32, Box::new(restart_28)),
        Instruction::new("RST 0x30", 0xF7, 0, 32, Box::new(restart_30)),
        Instruction::new("RST 0x38", 0xFF, 0, 32, Box::new(restart_38)),
    ]
}

fn nop(_: &mut GameBoy, _: u8, _: u8) {

}

fn halt(gb: &mut GameBoy, _: u8, _: u8) {
    gb.cpu.is_halted = true;
    println!("HALTED");
}

fn stop(gb: &mut GameBoy, a1: u8, _: u8) {
    // gb.cpu.is_halted = true;
    println!("STOPPED {}", a1);
}

fn pop_word(gb: &mut GameBoy) -> u16 {
    let result = gb.memory.get_word(gb.cpu.sp);
    gb.cpu.sp += 2;
    result
}

fn jump_immediate(gb: &mut GameBoy, a1: u8, a2: u8) {
    let new_val = concat_bytes(a2, a1);
    gb.cpu.pc = new_val;
}

fn jump_pc_plus_byte(gb: &mut GameBoy, a1: u8, _: u8) {
    let new_pc = add_u16_and_i8(gb.cpu.pc, a1);
    gb.cpu.pc = new_pc;
}

fn jump_not_z_flag_pc_plus(gb: &mut GameBoy, a1: u8, _: u8) {
    if !gb.cpu.flag.zero {
        jump_pc_plus_byte(gb, a1, 0);
    }
}

fn jump_z_flag_pc_plus(gb: &mut GameBoy, a1: u8, _: u8) {
    if gb.cpu.flag.zero {
        jump_pc_plus_byte(gb, a1, 0);
    }
}

fn jump_not_c_flag_pc_plus(gb: &mut GameBoy, a1: u8, _: u8) {
    if !gb.cpu.flag.carry {
        jump_pc_plus_byte(gb, a1, 0);
    }
}

fn jump_c_flag_pc_plus(gb: &mut GameBoy, a1: u8, _: u8) {
    if gb.cpu.flag.carry {
        jump_pc_plus_byte(gb, a1, 0);
    }
}

fn jump_not_z_flag(gb: &mut GameBoy, a1: u8, a2: u8) {
    if !gb.cpu.flag.zero {
        gb.cpu.pc = concat_bytes(a2, a1);
    }
}

fn jump_z_flag(gb: &mut GameBoy, a1: u8, a2: u8) {
    if gb.cpu.flag.zero {
        gb.cpu.pc = concat_bytes(a2, a1);
    }
}

fn jump_not_c_flag(gb: &mut GameBoy, a1: u8, a2: u8) {
    if !gb.cpu.flag.carry {
        gb.cpu.pc = concat_bytes(a2, a1);
    }
}

fn jump_c_flag(gb: &mut GameBoy, a1: u8, a2: u8) {
    if gb.cpu.flag.carry {
        gb.cpu.pc = concat_bytes(a2, a1);
    }
}

fn jump_hl(gb: &mut GameBoy, _: u8, _: u8) {
    gb.cpu.pc = gb.cpu.hl;
}

fn call_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    let pc = gb.cpu.pc;
    push_word(gb, pc);
    jump_immediate(gb, a1, a2);
}

fn call_if_not_zero(gb: &mut GameBoy, a1: u8, a2: u8) {
    if !gb.cpu.flag.zero {
        call_nn(gb, a1, a2);
    }
}

fn call_if_zero(gb: &mut GameBoy, a1: u8, a2: u8) {
    if gb.cpu.flag.zero {
        call_nn(gb, a1, a2);
    }
}

fn call_if_not_carry(gb: &mut GameBoy, a1: u8, a2: u8) {
    if !gb.cpu.flag.carry {
        call_nn(gb, a1, a2);
    }
}

fn call_if_carry(gb: &mut GameBoy, a1: u8, a2: u8) {
    if gb.cpu.flag.carry {
        call_nn(gb, a1, a2);
    }
}

fn sub_return(gb: &mut GameBoy, _: u8, _: u8) {
    let addr = pop_word(gb);
    gb.cpu.pc = addr;
}

fn sub_return_enable_interrupts(gb: &mut GameBoy, _: u8, _: u8) {
    sub_return(gb, 0, 0);
    enable_interrupts(gb, 0, 0);
}

fn sub_return_if_not_z(gb: &mut GameBoy, _: u8, _: u8) {
    if !gb.cpu.flag.zero {
        sub_return(gb, 0, 0);
    }
}

fn sub_return_if_z(gb: &mut GameBoy, _: u8, _: u8) {
    if gb.cpu.flag.zero {
        sub_return(gb, 0, 0);
    }
}

fn sub_return_if_not_c(gb: &mut GameBoy, _: u8, _: u8) {
    if !gb.cpu.flag.carry {
        sub_return(gb, 0, 0);
    }
}

fn sub_return_if_c(gb: &mut GameBoy, _: u8, _: u8) {
    if gb.cpu.flag.carry {
        sub_return(gb, 0, 0);
    }
}

fn load_mem_nn_sp(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.memory.set_word(concat_bytes(a2, a1), gb.cpu.sp);
}

fn push_af(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_af();
    push_word(gb, val);
}

fn push_bc(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.bc;
    push_word(gb, val);
}

fn push_de(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.de;
    push_word(gb, val);
}

fn push_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.hl;
    push_word(gb, val);
}

fn pop_af(gb: &mut GameBoy, _: u8, _: u8) {
    let val = pop_word(gb);
    gb.cpu.set_af(val);
}

fn pop_bc(gb: &mut GameBoy, _: u8, _: u8) {
    let val = pop_word(gb);
    gb.cpu.bc = val;
}

fn pop_de(gb: &mut GameBoy, _: u8, _: u8) {
    let val = pop_word(gb);
    gb.cpu.de = val;
}

fn pop_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let val = pop_word(gb);
    gb.cpu.hl = val;
}

fn add(gb: &mut GameBoy, reg_val: u8, value: u8, with_carry: bool) -> u8 {
    let reg_val = reg_val as u16;
    let extra = if with_carry && gb.cpu.flag.carry { 1 } else { 0 };
    let mut result = reg_val + (value as u16) + extra;
    if result > 255 {
        result -= 256;
        gb.cpu.flag.carry = true;
    } else {
        gb.cpu.flag.carry = false;
    }

    gb.cpu.flag.half_carry = (((reg_val as u8) & 0x0F) + (value & 0x0F) + (extra as u8)) & 0x10 == 0x10;
    gb.cpu.flag.zero = result == 0;
    gb.cpu.flag.subtract = false;
    result as u8
}

fn add_word(gb: &mut GameBoy, value: u16, arg: u16) -> u16 {
    let mut result = (value as u32) + (arg as u32);
    if result > 65535 {
        result -= 65536;
        gb.cpu.flag.carry = true;
    } else {
        gb.cpu.flag.carry = false;
    }

    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = ((value & 0xFFF) + (arg & 0xFFF)) & 0x1000 == 0x1000;

    result as u16
}

fn subtract(gb: &mut GameBoy, reg_val: u8, value: u8, with_carry: bool) -> u8 {
    let reg_val = reg_val as i16;
    let extra: i16 = if with_carry && gb.cpu.flag.carry { 1 } else { 0 };
    let value = value as i16;
    let mut result = reg_val - value - extra;
    if result < 0 {
        result += 256;
        gb.cpu.flag.carry = true;
    } else {
        gb.cpu.flag.carry = false;
    }

    gb.cpu.flag.half_carry = (value & 0x0F) + extra > (reg_val & 0x0F);
    gb.cpu.flag.zero = result == 0;
    gb.cpu.flag.subtract = true;
    result as u8
}

fn decrement(gb: &mut GameBoy, reg_val: u8) -> u8{
    //Decrement does not affect carry flag
    let carry = gb.cpu.flag.carry;
    let result = subtract(gb, reg_val, 1, false);
    gb.cpu.flag.carry = carry;
    result
}

fn increment(gb: &mut GameBoy, reg_val: u8) -> u8{
    //Increment does not affect carry flag
    let carry = gb.cpu.flag.carry;
    let result = add(gb, reg_val, 1, false);
    gb.cpu.flag.carry = carry;
    result
}

fn subtract_word(value: u16, arg: u16) -> u16 {
    let mut result = (value as i32) - (arg as i32);
    if result < 0 {
        result += 65536;
    }
    result as u16
}

fn compare(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.flag.subtract = true;
    gb.cpu.flag.zero = a1 == a2;
    gb.cpu.flag.half_carry = (a2 & 0x0F) > (a1 & 0x0F);
    gb.cpu.flag.carry = a1 < a2;
}

fn increment_a(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let result = increment(gb, reg_val);
    gb.cpu.set_a(result);
}

fn increment_b(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_b();
    let result = increment(gb, reg_val);
    gb.cpu.set_b(result);
}

fn increment_c(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_c();
    let result = increment(gb, reg_val);
    gb.cpu.set_c(result);
}

fn increment_d(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_d();
    let result = increment(gb, reg_val);
    gb.cpu.set_d(result);
}

fn increment_e(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_e();
    let result = increment(gb, reg_val);
    gb.cpu.set_e(result);
}

fn increment_h(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_h();
    let result = increment(gb, reg_val);
    gb.cpu.set_h(result);
}

fn increment_l(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_l();
    let result = increment(gb, reg_val);
    gb.cpu.set_l(result);
}

fn increment_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.memory.get_byte(gb.cpu.hl);
    let result = increment(gb, reg_val);
    gb.memory.set_byte(gb.cpu.hl, result);
}

fn increment_bc(gb: &mut GameBoy, _: u8, _: u8) {
    if gb.cpu.bc == 0xFFFF {
        gb.cpu.bc = 0;
    } else {
        gb.cpu.bc += 1;
    }
}

fn increment_de(gb: &mut GameBoy, _: u8, _: u8) {
    if gb.cpu.de == 0xFFFF {
        gb.cpu.de = 0;
    } else {
        gb.cpu.de += 1;
    }}

fn increment_hl(gb: &mut GameBoy, _: u8, _: u8) {
    if gb.cpu.hl == 0xFFFF {
        gb.cpu.hl = 0;
    } else {
        gb.cpu.hl += 1;
    }
}

fn increment_sp(gb: &mut GameBoy, _: u8, _: u8) {
    if gb.cpu.sp == 0xFFFF {
        gb.cpu.sp = 0;
    } else {
        gb.cpu.sp += 1;
    }
}

fn add_a(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.cpu.get_a();
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_b(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.cpu.get_b();
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_c(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.cpu.get_c();
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_d(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.cpu.get_d();
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_e(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.cpu.get_e();
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_h(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.cpu.get_h();
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_l(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.cpu.get_l();
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_n(gb: &mut GameBoy, to_add: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let to_add = gb.memory.get_byte(gb.cpu.hl);
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, false);
    gb.cpu.set_a(result);
}

fn add_a_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_b_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.cpu.get_b();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_c_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.cpu.get_c();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_d_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.cpu.get_d();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_e_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.cpu.get_e();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_h_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.cpu.get_h();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_l_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.cpu.get_l();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_mem_hl_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let to_add = gb.memory.get_byte(gb.cpu.hl);
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_n_with_carry(gb: &mut GameBoy, to_add: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let result = add(gb, reg_val, to_add, true);
    gb.cpu.set_a(result);
}

fn add_hl_bc(gb: &mut GameBoy, _: u8, _: u8) {
    let hl = gb.cpu.hl;
    let arg = gb.cpu.bc;
    gb.cpu.hl = add_word(gb, hl, arg);
}

fn add_hl_de(gb: &mut GameBoy, _: u8, _: u8) {
    let hl = gb.cpu.hl;
    let arg = gb.cpu.de;
    gb.cpu.hl = add_word(gb, hl, arg);
}

fn add_hl_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let hl = gb.cpu.hl;
    gb.cpu.hl = add_word(gb, hl, hl);
}

fn add_hl_sp(gb: &mut GameBoy, _: u8, _: u8) {
    let hl = gb.cpu.hl;
    let arg = gb.cpu.sp;
    gb.cpu.hl = add_word(gb, hl, arg);
}

fn add_sp_signed_n(gb: &mut GameBoy, arg: u8, _: u8) {
    let sp = gb.cpu.sp;
    let result = add_u16_and_i8_affect_flags(gb, sp, arg);
    gb.cpu.sp = result;
}

fn decrement_a(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_a();
    let result = decrement(gb, reg_val);
    gb.cpu.set_a(result);
}

fn decrement_b(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_b();
    let result = decrement(gb, reg_val);
    gb.cpu.set_b(result);
}

fn decrement_c(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_c();
    let result = decrement(gb, reg_val);
    gb.cpu.set_c(result);
}

fn decrement_d(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_d();
    let result = decrement(gb, reg_val);
    gb.cpu.set_d(result);
}

fn decrement_e(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_e();
    let result = decrement(gb, reg_val);
    gb.cpu.set_e(result);
}

fn decrement_h(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_h();
    let result = decrement(gb, reg_val);
    gb.cpu.set_h(result);
}

fn decrement_l(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.cpu.get_l();
    let result = decrement(gb, reg_val);
    gb.cpu.set_l(result);
}

fn decrement_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let reg_val = gb.memory.get_byte(gb.cpu.hl);
    let result = decrement(gb, reg_val);
    gb.memory.set_byte(gb.cpu.hl, result);
}

fn decrement_bc(gb: &mut GameBoy, _: u8, _: u8) {
    let result = subtract_word(gb.cpu.bc, 1);
    gb.cpu.bc = result;
}

fn decrement_de(gb: &mut GameBoy, _: u8, _: u8) {
    let result = subtract_word(gb.cpu.de, 1);
    gb.cpu.de = result;
}

fn decrement_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let result = subtract_word(gb.cpu.hl, 1);
    gb.cpu.hl = result;
}

fn decrement_sp(gb: &mut GameBoy, _: u8, _: u8) {
    let result = subtract_word(gb.cpu.sp, 1);
    gb.cpu.sp = result;
}

fn subtract_a(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_a();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_b(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_b();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_c(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_c();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_d(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_d();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_e(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_e();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_h(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_h();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_l(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_l();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_n(gb: &mut GameBoy, to_sub: u8, _: u8) {
    let a = gb.cpu.get_a();
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.memory.get_byte(gb.cpu.hl);
    let result = subtract(gb, a, to_sub, false);
    gb.cpu.set_a(result);
}

fn subtract_a_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_a();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_b_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_b();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_c_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_c();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_d_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_d();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_e_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_e();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_h_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_h();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_l_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.cpu.get_l();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_n_with_carry(gb: &mut GameBoy, to_sub: u8, _: u8) {
    let a = gb.cpu.get_a();
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn subtract_mem_hl_with_carry(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let to_sub = gb.memory.get_byte(gb.cpu.hl);
    let result = subtract(gb, a, to_sub, true);
    gb.cpu.set_a(result);
}

fn compare_a(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    compare(gb, a, a);
}

fn compare_b(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let val = gb.cpu.get_b();
    compare(gb, a, val);
}

fn compare_c(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let val = gb.cpu.get_c();
    compare(gb, a, val);
}

fn compare_d(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let val = gb.cpu.get_d();
    compare(gb, a, val);
}

fn compare_e(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let val = gb.cpu.get_e();
    compare(gb, a, val);
}

fn compare_h(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let val = gb.cpu.get_h();
    compare(gb, a, val);
}

fn compare_l(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let val = gb.cpu.get_l();
    compare(gb, a, val);
}

fn compare_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let addr = gb.cpu.hl;
    let val = gb.memory.get_byte(addr);
    compare(gb, a, val);
}

fn compare_n(gb: &mut GameBoy, val: u8, _: u8) {
    let a = gb.cpu.get_a();
    compare(gb, a, val);
}

fn load_8(gb: &mut GameBoy, dest: Reg8, val: u8) {
    set_reg8(gb, dest, val);
}

fn load_x_y(dest: Reg8, val_reg: Reg8) -> Box<Fn(&mut GameBoy, u8, u8)> {
    Box::new(move |gb, _, _| {
        let val = get_reg8(gb, val_reg);
        load_8(gb, dest, val)
    })
}

fn load_x_imm(dest: Reg8) -> Box<Fn(&mut GameBoy, u8, u8)> {
    Box::new(move |gb, val, _| {
        load_8(gb, dest, val)
    })
}

fn load_mem_bc_a(gb: &mut GameBoy, _: u8, _: u8) {
    gb.memory.set_byte(gb.cpu.bc, gb.cpu.get_a());
}

fn load_mem_de_a(gb: &mut GameBoy, _: u8, _: u8) {
    gb.memory.set_byte(gb.cpu.de, gb.cpu.get_a());
}

fn load_mem_nn_a(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.memory.set_byte(concat_bytes(a2, a1), gb.cpu.get_a());
}

fn load_a_mem_bc(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.memory.get_byte(gb.cpu.bc);
    gb.cpu.set_a(val);
}

fn load_a_mem_de(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.memory.get_byte(gb.cpu.de);
    gb.cpu.set_a(val);
}

fn load_a_mem_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    let val = gb.memory.get_byte(concat_bytes(a2, a1));
    gb.cpu.set_a(val);
}

fn load_bc_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.bc = concat_bytes(a2, a1);
}

fn load_de_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.de = concat_bytes(a2, a1);
}

fn load_hl_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.hl = concat_bytes(a2, a1);
}

fn load_sp_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.sp = concat_bytes(a2, a1);
}

fn load_ff00_plus_n_with_a(gb: &mut GameBoy, a1: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.memory.set_byte(0xFF00 + (a1 as u16), val);
}

fn load_a_with_ff00_plus_n(gb: &mut GameBoy, a1: u8, _: u8) {
    let val = gb.memory.get_byte(0xFF00 + (a1 as u16));
    gb.cpu.set_a(val);
}

fn load_ff00_plus_c_with_a(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let c = gb.cpu.get_c();
    gb.memory.set_byte(0xFF00 + (c as u16), a);
}

fn load_a_with_ff00_plus_c(gb: &mut GameBoy, _: u8, _: u8) {
    let c = gb.cpu.get_c();
    let result = gb.memory.get_byte(0xFF00 + (c as u16));
    gb.cpu.set_a(result);
}

fn load_mem_hl_with_a_dec_hl(gb: &mut GameBoy, _: u8, _: u8) {
    gb.memory.set_byte(gb.cpu.hl, gb.cpu.get_a());
    decrement_hl(gb, 0, 0);
}

fn load_mem_hl_with_a_inc_hl(gb: &mut GameBoy, _: u8, _: u8) {
    gb.memory.set_byte(gb.cpu.hl, gb.cpu.get_a());
    increment_hl(gb, 0, 0);
}

fn load_a_with_mem_hl_inc_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.memory.get_byte(gb.cpu.hl);
    gb.cpu.set_a(a);
    increment_hl(gb, 0, 0);
}

fn load_a_with_mem_hl_dec_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.memory.get_byte(gb.cpu.hl);
    gb.cpu.set_a(a);
    decrement_hl(gb, 0, 0);
}

fn load_sp_hl(gb: &mut GameBoy, _: u8, _: u8) {
    gb.cpu.sp = gb.cpu.hl;
}

fn load_hl_sp_plus_signed_n(gb: &mut GameBoy, a1: u8, _: u8) {
    let sp = gb.cpu.sp;
    let result = add_u16_and_i8_affect_flags(gb, sp, a1);
    gb.cpu.hl = result;
}

fn or_a_with(gb: &mut GameBoy, val: u8) {
    let a = gb.cpu.get_a();
    let result = a | val;

    gb.cpu.flag.zero = result == 0;
    gb.cpu.flag.half_carry = false;
    gb.cpu.flag.carry = false;
    gb.cpu.flag.subtract = false;

    gb.cpu.set_a(result);
}

fn and_a_with(gb: &mut GameBoy, val: u8) {
    let a = gb.cpu.get_a();
    let result = a & val;

    gb.cpu.flag.zero = result == 0;
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = true;
    gb.cpu.flag.carry = false;

    gb.cpu.set_a(result);
}

fn xor_a_with(gb: &mut GameBoy, val: u8) {
    let a = gb.cpu.get_a();
    let result = a ^ val;

    gb.cpu.flag.zero = result == 0;
    gb.cpu.flag.half_carry = false;
    gb.cpu.flag.carry = false;
    gb.cpu.flag.subtract = false;

    gb.cpu.set_a(result);
}

fn or_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    or_a_with(gb, val);
}

fn or_b(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    or_a_with(gb, val);
}

fn or_c(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    or_a_with(gb, val);
}

fn or_d(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    or_a_with(gb, val);
}

fn or_e(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    or_a_with(gb, val);
}

fn or_h(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    or_a_with(gb, val);
}

fn or_l(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    or_a_with(gb, val);
}

fn or_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.memory.get_byte(gb.cpu.hl);
    or_a_with(gb, val);
}

fn or_n(gb: &mut GameBoy, a1: u8, _: u8) {
    or_a_with(gb, a1);
}

fn and_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    and_a_with(gb, val);
}

fn and_b(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    and_a_with(gb, val);
}

fn and_c(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    and_a_with(gb, val);
}

fn and_d(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    and_a_with(gb, val);
}

fn and_e(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    and_a_with(gb, val);
}

fn and_h(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    and_a_with(gb, val);
}

fn and_l(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    and_a_with(gb, val);
}

fn and_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.memory.get_byte(gb.cpu.hl);
    and_a_with(gb, val);
}

fn and_n(gb: &mut GameBoy, a1: u8, _: u8) {
    and_a_with(gb, a1);
}

fn xor_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    xor_a_with(gb, val);
}

fn xor_b(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    xor_a_with(gb, val);
}

fn xor_c(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    xor_a_with(gb, val);
}

fn xor_d(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    xor_a_with(gb, val);
}

fn xor_e(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    xor_a_with(gb, val);
}

fn xor_h(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    xor_a_with(gb, val);
}

fn xor_l(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    xor_a_with(gb, val);
}

fn xor_n(gb: &mut GameBoy, val: u8, _: u8) {
    xor_a_with(gb, val);
}

fn xor_mem_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.memory.get_byte(gb.cpu.hl);
    xor_a_with(gb, val);
}

fn disable_interrupts(gb: &mut GameBoy, _: u8, _: u8) {
    //TODO some sources say this doesn't happen until after the next Instruction
    gb.cpu.interrupt_enable_master = false;
}

fn enable_interrupts(gb: &mut GameBoy, _: u8, _: u8) {
    //TODO some sources say this doesn't happen until after the next Instruction
    gb.cpu.interrupt_enable_master = true;
}

fn complement_a(gb: &mut GameBoy, _: u8, _: u8) {
    gb.cpu.flag.subtract = true;
    gb.cpu.flag.half_carry = true;
    let a = gb.cpu.get_a();
    gb.cpu.set_a(!a);
}

fn complement_carry(gb: &mut GameBoy, _: u8, _: u8) {
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    gb.cpu.flag.carry = !gb.cpu.flag.carry;
}

fn set_carry(gb: &mut GameBoy, _: u8, _: u8) {
    gb.cpu.flag.subtract = false;
    gb.cpu.flag.half_carry = false;
    gb.cpu.flag.carry = true;
}

fn restart(gb: &mut GameBoy, a1: u8) {
    let pc = gb.cpu.pc;
    push_word(gb, pc);
    jump_immediate(gb, a1, 0x00);
}

fn restart_00(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x00);
}

fn restart_08(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x08);
}

fn restart_10(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x10);
}

fn restart_18(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x18);
}

fn restart_20(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x20);
}

fn restart_28(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x28);
}

fn restart_30(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x30);
}

fn restart_38(gb: &mut GameBoy, _: u8, _: u8) {
    restart(gb, 0x38);
}

fn rotate_left_a_through(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let result = rotate_left(gb, a, true);
    gb.cpu.set_a(result);
    gb.cpu.flag.zero = false;
}

fn rotate_left_a(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let result = rotate_left(gb, a, false);
    gb.cpu.set_a(result);
    gb.cpu.flag.zero = false;
}

fn rotate_right_a_through(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let result = rotate_right(gb, a, true);
    gb.cpu.set_a(result);
    gb.cpu.flag.zero = false;
}

fn rotate_right_a(gb: &mut GameBoy, _: u8, _: u8) {
    let a = gb.cpu.get_a();
    let result = rotate_right(gb, a, false);
    gb.cpu.set_a(result);
    gb.cpu.flag.zero = false;
}

fn decimal_adjust_a(gb: &mut GameBoy, _: u8, _: u8) {
    let mut a = gb.cpu.get_a() as u16;

    if gb.cpu.flag.subtract {
        if gb.cpu.flag.half_carry {
            a = a.wrapping_sub(0x06);
            a &= 0xFF;
        }
        if gb.cpu.flag.carry {
            a = a.wrapping_sub(0x60);
        }
    } else {
        if a & 0x0F > 0x09 || gb.cpu.flag.half_carry {
            a = a.wrapping_add(0x06);
        }

        if a > 0x9F || gb.cpu.flag.carry {
            a = a.wrapping_add(0x60);
        }
    }

    gb.cpu.flag.half_carry = false;
    if a > 0xFF {
        gb.cpu.flag.carry = true;
    }
    a &= 0xFF;
    gb.cpu.set_a(a as u8);
    gb.cpu.flag.zero = a == 0;
}