use game_boy::GameBoy;
use cpu::Cpu;
use util::to_signed_word;

pub struct Instruction {
    pub name: &'static str,
    pub opcode: u8,
    pub operand_length: u8,
    pub cycles: u8,
    pub exec: fn(&mut GameBoy, u8, u8)
}

impl Instruction {
    pub fn new(name: &'static str, opcode: u8, operand_length: u8, cycles: u8, exec: fn(&mut GameBoy, u8, u8)) -> Instruction {
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
        Instruction::new("NOP", 0x00, 0, 4, nop),
        
        Instruction::new("LD B,n", 0x06, 1, 8, load_b_n),
        Instruction::new("LD C,n", 0x0E, 1, 8, load_c_n),
        Instruction::new("LD D,n", 0x16, 1, 8, load_d_n),
        Instruction::new("LD E,n", 0x1E, 1, 8, load_e_n),
        Instruction::new("LD H,n", 0x26, 1, 8, load_h_n),
        Instruction::new("LD L,n", 0x2E, 1, 8, load_l_n),

        Instruction::new("LD A,A", 0x7F, 0, 4, load_a_a),
        Instruction::new("LD A,B", 0x78, 0, 4, load_a_b),
        Instruction::new("LD A,C", 0x79, 0, 4, load_a_c),
        Instruction::new("LD A,D", 0x7A, 0, 4, load_a_d),
        Instruction::new("LD A,E", 0x7B, 0, 4, load_a_e),
        Instruction::new("LD A,H", 0x7C, 0, 4, load_a_h),
        Instruction::new("LD A,L", 0x7D, 0, 4, load_a_l),

        Instruction::new("LD A,n", 0x3E, 1, 8, load_a_n),



        Instruction::new("LD B,A", 0x47, 0, 4, load_b_a),
        Instruction::new("LD C,A", 0x4F, 0, 4, load_c_a),
        Instruction::new("LD D,A", 0x57, 0, 4, load_d_a),
        Instruction::new("LD E,A", 0x5F, 0, 4, load_e_a),
        Instruction::new("LD H,A", 0x67, 0, 4, load_h_a),
        Instruction::new("LD L,A", 0x6f, 0, 4, load_l_a),
        Instruction::new("LD (BC),A", 0x02, 0, 8, load_bc_a),
        // Instruction::new("LD (DE),A", 0x12, 0, 8, load_bc_a),
        // Instruction::new("LD (HL),A", 0x77, 0, 8, load_bc_a),
        // Instruction::new("LD (nn),A", 0xEA, 0, 16, load_bc_a),


        Instruction::new("LD BC,nn", 0x01, 2, 12, load_bc_nn),
        Instruction::new("LD DE,nn", 0x11, 2, 12, load_de_nn),
        Instruction::new("LD HL,nn", 0x21, 2, 12, load_hl_nn),
        Instruction::new("LD SP,nn", 0x31, 2, 12, load_sp_nn),

        Instruction::new("LD (HL),A; DEC HL", 0x32, 0, 8, load_mem_hl_with_a_dec_hl),
        Instruction::new("LD (0xFF00 + n),A", 0xE0, 1, 12, load_ff00_plus_n_with_a),
        Instruction::new("LD A,(0xFF00 + n)", 0xF0, 1, 12, load_a_with_ff00_plus_n),

        Instruction::new("XOR A", 0xAF, 0, 4, xor_a),
        Instruction::new("XOR B", 0xA8, 0, 4, xor_b),
        Instruction::new("XOR C", 0xA9, 0, 4, xor_c),
        Instruction::new("XOR D", 0xAA, 0, 4, xor_d),
        Instruction::new("XOR E", 0xAB, 0, 4, xor_e),
        Instruction::new("XOR H", 0xAC, 0, 4, xor_h),
        Instruction::new("XOR L", 0xAD, 0, 4, xor_l),
        // Instruction::new("XOR (HL)", 0xAE, 0, 8, ?),
        // Instruction::new("XOR *", 0xEE, 0, 8, ?),



        Instruction::new("INC A", 0x3C, 0, 4, increment_a),
        Instruction::new("INC B", 0x04, 0, 4, increment_b),
        Instruction::new("INC C", 0x0C, 0, 4, increment_c),
        Instruction::new("INC D", 0x14, 0, 4, increment_d),
        Instruction::new("INC E", 0x1C, 0, 4, increment_e),
        Instruction::new("INC H", 0x24, 0, 4, increment_h),
        Instruction::new("INC L", 0x2C, 0, 4, increment_l),
        // Instruction::new("INC (HL)", 0x34, 0, 12, increment_hl),

        Instruction::new("DEC A", 0x3D, 0, 4, decrement_a),
        Instruction::new("DEC B", 0x05, 0, 4, decrement_b),
        Instruction::new("DEC C", 0x0D, 0, 4, decrement_c),
        Instruction::new("DEC D", 0x15, 0, 4, decrement_d),
        Instruction::new("DEC E", 0x1D, 0, 4, decrement_e),
        Instruction::new("DEC H", 0x25, 0, 4, decrement_h),
        Instruction::new("DEC L", 0x2D, 0, 4, decrement_l),
        // Instruction::new("DEC (HL)", 0x35, 0, 12, decrement_l),

        Instruction::new("JP nn", 0xC3, 2, 12, jump_immediate),
        Instruction::new("JP NZ,nn", 0x20, 1, 8, jump_not_z_flag),
        Instruction::new("JP Z,nn", 0x28, 1, 8, jump_z_flag),
        Instruction::new("JP NC,nn", 0x30, 1, 8, jump_not_c_flag),
        Instruction::new("JP C,nn", 0x38, 1, 8, jump_c_flag),

        Instruction::new("DI", 0xF3, 0, 4, disable_interrupts),
        Instruction::new("EI", 0xFB, 0, 4, enable_interrupts),


    ]
}

fn nop(_: &mut GameBoy, _: u8, _: u8) {

}

fn jump_immediate(gb: &mut GameBoy, a1: u8, a2: u8) {
    let new_val = ((a2 as u16) << 8) + (a1 as u16);
    gb.cpu.pc = new_val;
}

fn jump_not_z_flag(gb: &mut GameBoy, a1: u8, _: u8) {
    if !gb.cpu.flag.zero {
        gb.cpu.pc = ((gb.cpu.pc as i16) + to_signed_word(a1)) as u16;
    }
}

fn jump_z_flag(gb: &mut GameBoy, a1: u8, _: u8) {
    if gb.cpu.flag.zero {
        gb.cpu.pc = ((gb.cpu.pc as i16) + to_signed_word(a1)) as u16;
    }
}

fn jump_not_c_flag(gb: &mut GameBoy, a1: u8, _: u8) {
    if !gb.cpu.flag.carry {
        gb.cpu.pc = ((gb.cpu.pc as i16) + to_signed_word(a1)) as u16;
    }
}

fn jump_c_flag(gb: &mut GameBoy, a1: u8, _: u8) {
    if gb.cpu.flag.carry {
        gb.cpu.pc = ((gb.cpu.pc as i16) + to_signed_word(a1)) as u16;
    }
}

// fn call(gb: &mut GameBoy, a1: u8, a2: u8) {

//     jump_immediate(gb, a1, a2);
// }

fn increment(getter: fn(&Cpu) -> u8, setter: fn(&mut Cpu, u8), gb: &mut GameBoy) {
    let mut reg_val = getter(&gb.cpu);

    if reg_val == 0xFF {
        gb.cpu.flag.zero = true;
        gb.cpu.flag.half_carry = true;
        reg_val = 0;
    } else {
        gb.cpu.flag.zero = false;
        gb.cpu.flag.half_carry = false;
        reg_val += 1;
    }

    gb.cpu.flag.subtract = false;
    setter(&mut gb.cpu, reg_val);
}

fn decrement(getter: fn(&Cpu) -> u8, setter: fn(&mut Cpu, u8), gb: &mut GameBoy) {
    let mut reg_val = getter(&gb.cpu);

    if reg_val == 0x00 {
        gb.cpu.flag.half_carry = true;
        reg_val = 0xFF;
    } else {
        gb.cpu.flag.half_carry = false;
        reg_val -= 1;
    }

    gb.cpu.flag.zero = reg_val == 0;

    gb.cpu.flag.subtract = true;
    setter(&mut gb.cpu, reg_val);
}

fn increment_a(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_a, Cpu::set_a, gb);
}

fn increment_b(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_b, Cpu::set_b, gb);
}

fn increment_c(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_c, Cpu::set_c, gb);
}

fn increment_d(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_d, Cpu::set_d, gb);
}

fn increment_e(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_e, Cpu::set_e, gb);
}

fn increment_h(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_h, Cpu::set_h, gb);
}

fn increment_l(gb: &mut GameBoy, _: u8, _: u8) {
    increment(Cpu::get_l, Cpu::set_l, gb);
}

fn decrement_a(gb: &mut GameBoy, _: u8, _: u8) {
    decrement(Cpu::get_a, Cpu::set_a, gb);
}

fn decrement_b(gb: &mut GameBoy, _: u8, _: u8) {
    decrement(Cpu::get_b, Cpu::set_b, gb);
}

fn decrement_c(gb: &mut GameBoy, _: u8, _: u8) {
    decrement(Cpu::get_c, Cpu::set_c, gb);
}

fn decrement_d(gb: &mut GameBoy, _: u8, _: u8) {
    decrement(Cpu::get_d, Cpu::set_d, gb);
}

fn decrement_e(gb: &mut GameBoy, _: u8, _: u8) {
    decrement(Cpu::get_e, Cpu::set_e, gb);
}

fn decrement_h(gb: &mut GameBoy, _: u8, _: u8) {
    decrement(Cpu::get_h, Cpu::set_h, gb);
}

fn decrement_l(gb: &mut GameBoy, _: u8, _: u8) {
    decrement(Cpu::get_l, Cpu::set_l, gb);
}

fn load_bc_a(gb: &mut GameBoy, _: u8, _: u8) {
    gb.memory.set_byte(gb.cpu.bc, gb.cpu.get_a());
}

fn load_a_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_a(val);
}

fn load_a_b(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_b();
    gb.cpu.set_a(val);
}

fn load_a_c(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_c();
    gb.cpu.set_a(val);
}

fn load_a_d(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_d();
    gb.cpu.set_a(val);
}

fn load_a_e(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_e();
    gb.cpu.set_a(val);
}

fn load_a_h(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_h();
    gb.cpu.set_a(val);
}

fn load_a_l(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_l();
    gb.cpu.set_a(val);
}

fn load_b_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_b(val);
}

fn load_c_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_c(val);
}

fn load_d_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_d(val);
}

fn load_e_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_e(val);
}

fn load_h_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_h(val);
}

fn load_l_a(gb: &mut GameBoy, _: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.cpu.set_l(val);
}

fn load_a_n(gb: &mut GameBoy, a1: u8, _: u8) {
    gb.cpu.set_a(a1);
}

fn load_b_n(gb: &mut GameBoy, a1: u8, _: u8) {
    gb.cpu.set_b(a1);
}

fn load_c_n(gb: &mut GameBoy, a1: u8, _: u8) {
    gb.cpu.set_c(a1);
}

fn load_d_n(gb: &mut GameBoy, a1: u8, _: u8) {
    gb.cpu.set_d(a1);
}

fn load_e_n(gb: &mut GameBoy, a1: u8, _: u8) {
    gb.cpu.set_e(a1);
}

fn load_h_n(gb: &mut GameBoy, a1: u8, _: u8) {
    gb.cpu.set_h(a1);
}

fn load_l_n(gb: &mut GameBoy, a1: u8, _: u8) {
    gb.cpu.set_l(a1);
}

fn load_bc_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.bc = ((a2 as u16) << 8) + (a1 as u16);
}

fn load_de_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.de = ((a2 as u16) << 8) + (a1 as u16);
}

fn load_hl_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.hl = ((a2 as u16) << 8) + (a1 as u16);
}

fn load_sp_nn(gb: &mut GameBoy, a1: u8, a2: u8) {
    gb.cpu.sp = ((a2 as u16) << 8) + (a1 as u16);
}

fn load_ff00_plus_n_with_a(gb: &mut GameBoy, a1: u8, _: u8) {
    let val = gb.cpu.get_a();
    gb.memory.set_byte(0xFF00 + (a1 as u16), val);
}

fn load_a_with_ff00_plus_n(gb: &mut GameBoy, a1: u8, _: u8) {
    let val = gb.memory.get_byte(0xFF00 + (a1 as u16));
    gb.cpu.set_a(val);
}

fn load_mem_hl_with_a_dec_hl(gb: &mut GameBoy, _: u8, _: u8) {
    let mut hl = gb.cpu.hl;
    gb.memory.set_byte(hl, gb.cpu.get_a());
    hl -= 1;
    gb.cpu.hl = hl;
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

fn disable_interrupts(gb: &mut GameBoy, _: u8, _: u8) {
    //TODO some sources say this doesn't happen until after the next Instruction
    gb.cpu.interrupt_enable_master = false;
}

fn enable_interrupts(gb: &mut GameBoy, _: u8, _: u8) {
    //TODO some sources say this doesn't happen until after the next Instruction
    gb.cpu.interrupt_enable_master = true;
}