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
