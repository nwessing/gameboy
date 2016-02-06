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

fn get_instruction(gb: &GameBoy, opcode: u8) -> &instructions::Instruction {
    match gb.cpu.get_instruction(opcode) {
        Some(x) => x,
        _ => panic!("instruction not found")
    }
}