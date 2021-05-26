#[allow(unused_imports)]
use crate::cb_instructions;
#[allow(unused_imports)]
use crate::cpu::InstructionSet;
#[allow(unused_imports)]
use crate::game_boy::GameBoy;
#[allow(unused_imports)]
use crate::instructions;
#[allow(unused_imports)]
use crate::util;

#[test]
fn test() {
    let instructions = instructions::get_instruction_set();
    for i in 0..instructions.len() {
        for j in 0..instructions.len() {
            if instructions[i].opcode == instructions[j].opcode && i != j {
                panic!("Duplicate opcode found {:02X}", instructions[i].opcode);
            }
        }
    }

    let cb_instructions = cb_instructions::get_cb_instruction_set();
    for i in 0..cb_instructions.len() {
        for j in 0..cb_instructions.len() {
            if cb_instructions[i].opcode == cb_instructions[j].opcode && i != j {
                panic!("Duplicate opcode found CB{:02X}", cb_instructions[i].opcode);
            }
        }
    }
}

#[test]
fn jump_pc_plus_bytes() {
    let is = InstructionSet::new();
    let ins = match is.get_instruction(0x18) {
        Some(x) => x,
        None => panic!("No instruction found"),
    };
    let mut gb = GameBoy::new();

    gb.cpu.pc = 0xFF00;
    (ins.exec)(&mut gb, 0xFD, 0);
    assert_eq!(gb.cpu.pc, 0xFEFD);

    gb.cpu.pc = 0xFF00;
    (ins.exec)(&mut gb, 0x05, 0);
    assert_eq!(gb.cpu.pc, 0xFF05);
}

#[test]
fn to_signed_word() {
    assert_eq!(util::to_signed_word(0xFD), -3);
    assert_eq!(util::to_signed_word(0x03), 3);
}

#[test]
fn stack_tests() {
    let is = InstructionSet::new();
    let mut gb = GameBoy::new();
    gb.cpu.sp = 0xFFFE;

    let push_hl = match is.get_instruction(0xE5) {
        Some(x) => x,
        None => panic!("No instruction found"),
    };
    let pop_hl = match is.get_instruction(0xE1) {
        Some(x) => x,
        None => panic!("No instruction found"),
    };

    gb.cpu.hl = 0x1234;
    (push_hl.exec)(&mut gb, 0, 0);
    gb.cpu.hl = 0x5678;
    (push_hl.exec)(&mut gb, 0, 0);

    gb.cpu.hl = 0xABCD;

    (pop_hl.exec)(&mut gb, 0, 0);
    assert_eq!(0x5678, gb.cpu.hl);
    (pop_hl.exec)(&mut gb, 0, 0);
    assert_eq!(0x1234, gb.cpu.hl);
}

#[test]
fn adding_usign_and_sign() {
    let is = InstructionSet::new();
    let mut gb = GameBoy::new();

    let jump_plus_signed = match is.get_instruction(0x18) {
        Some(x) => x,
        None => panic!("No instruction found"),
    };
    gb.cpu.pc = 0xCBB0;
    (jump_plus_signed.exec)(&mut gb, 0xFE, 0xC9);

    assert_eq!(0xCBAE, gb.cpu.pc);
}
