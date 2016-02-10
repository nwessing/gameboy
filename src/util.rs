use game_boy::GameBoy;

#[derive(Copy, Clone)]
pub enum Reg8{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MemHl
}

pub fn get_reg8(gb: &GameBoy, reg: Reg8) -> u8 {
    match reg {
        Reg8::A => gb.cpu.get_a(),
        Reg8::B => gb.cpu.get_b(),
        Reg8::C => gb.cpu.get_c(),
        Reg8::D => gb.cpu.get_d(),
        Reg8::E => gb.cpu.get_e(),
        Reg8::H => gb.cpu.get_h(),
        Reg8::L => gb.cpu.get_l(),
        Reg8::MemHl => gb.memory.get_byte(gb.cpu.hl),
    }
}

pub fn set_reg8(gb: &mut GameBoy, reg: Reg8, val: u8) {
    match reg {
        Reg8::A => gb.cpu.set_a(val),
        Reg8::B => gb.cpu.set_b(val),
        Reg8::C => gb.cpu.set_c(val),
        Reg8::D => gb.cpu.set_d(val),
        Reg8::E => gb.cpu.set_e(val),
        Reg8::H => gb.cpu.set_h(val),
        Reg8::L => gb.cpu.set_l(val),
        Reg8::MemHl => gb.memory.set_byte(gb.cpu.hl, val),
    };
}

pub fn get_upper(b: u16) -> u8 {
    ((b & 0xFF00) >> 8) as u8
}

pub fn get_lower(b: u16) -> u8 {
    (b & 0x00FF) as u8
}

pub fn set_upper(to_set: &mut u16, b: u8) {
    *to_set = (*to_set & 0x00FF) | ((b as u16) << 8);
}

pub fn set_lower(to_set: &mut u16, b: u8) {
    *to_set = (*to_set & 0xFF00) | (b as u16);
}

pub fn to_signed_word(arg: u8) -> i16 {
    (arg as i8) as i16
}

pub fn to_unsigned_word(arg: i16) -> u16 {
    if arg < 0 {
        (!arg + 1) as u16
    } else {
        arg as u16
    }
}

pub fn concat_bytes(a1: u8, a2: u8) -> u16 {
    ((a1 as u16) << 8) + (a2 as u16)
}

pub fn push_word(gb: &mut GameBoy, value: u16) {
    gb.cpu.sp -= 2;
    gb.memory.set_word(gb.cpu.sp, value);
}

