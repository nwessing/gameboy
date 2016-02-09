use instructions;
use game_boy::GameBoy;
use util;

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
}

#[test]
fn jump_pc_plus_bytes() {
    let exec;
    let mut gb = GameBoy::new();
    {
        let ins = match gb.cpu.get_instruction(0x18) {
            Some(x) => x,
            _ => panic!("instruction not found")
        };
        exec = ins.exec;
    }
    gb.cpu.pc = 0xFF00;
    exec(&mut gb, 0xFD, 0);
    assert_eq!(gb.cpu.pc, 0xFEFD);

    gb.cpu.pc = 0xFF00;
    exec(&mut gb, 0x05, 0);
    assert_eq!(gb.cpu.pc, 0xFF05);
}

#[test]
fn to_signed_word() {
    assert_eq!(util::to_signed_word(0xFD), -3);
    assert_eq!(util::to_signed_word(0x03), 3);
}

#[test]
fn stack_tests() {
    let mut gb = GameBoy::new();
    gb.cpu.sp = 0xFFFE;

    let push_hl = get_instruction(&gb, 0xE5).exec;
    let pop_hl = get_instruction(&gb, 0xE1).exec;

    gb.cpu.hl = 0x1234;
    push_hl(&mut gb, 0, 0);
    gb.cpu.hl = 0x5678;
    push_hl(&mut gb, 0, 0);

    gb.cpu.hl = 0xABCD;

    pop_hl(&mut gb, 0, 0);
    assert_eq!(0x5678, gb.cpu.hl);
    pop_hl(&mut gb, 0, 0);    
    assert_eq!(0x1234, gb.cpu.hl);
}

#[test]
fn adding_usign_and_sign() {
    let mut gb = GameBoy::new();

    let jump_plus_signed = get_instruction(&gb, 0x18).exec;
    gb.cpu.pc = 0xCBB0;
    jump_plus_signed(&mut gb, 0xFE, 0xC9);

    assert_eq!(0xCBAE, gb.cpu.pc);
}

fn get_instruction(gb: &GameBoy, opcode: u8) -> &instructions::Instruction {
    match gb.cpu.get_instruction(opcode) {
        Some(x) => x,
        _ => panic!("instruction not found")
    }
}