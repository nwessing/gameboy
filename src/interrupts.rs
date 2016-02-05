use game_boy::GameBoy;
use util::push_word;

const INTERRUPT_ENABLE_REG: u16 = 0xFFFF;
const INTERRUPT_FLAG_REG: u16 = 0xFF0F;

const V_BLANK: u8 = 0x01;
const LCD_STAT: u8 = 0x02;
const TIMER: u8 = 0x04;
const SERIAL: u8 = 0x08;
const JOYPAD: u8 = 0x10;

pub fn check_interrupts(gb: &mut GameBoy) {
    if gb.cpu.interrupt_enable_master {
        let enabled = gb.memory.get_byte(INTERRUPT_ENABLE_REG);
        let flag = gb.memory.get_byte(INTERRUPT_FLAG_REG);

        let interrupts = enabled & flag;

        if interrupts & V_BLANK == V_BLANK {
            handle_interrupt(gb, flag, V_BLANK);
        } else if interrupts & LCD_STAT == LCD_STAT {
            handle_interrupt(gb, flag, LCD_STAT);
        } else if interrupts & TIMER == TIMER {
            handle_interrupt(gb, flag, TIMER);
        } else if interrupts & SERIAL == SERIAL {
            handle_interrupt(gb, flag, SERIAL);
        } else if interrupts & JOYPAD == JOYPAD {
            handle_interrupt(gb, flag, JOYPAD);
        }
    }
}

fn handle_interrupt(gb: &mut GameBoy, flags: u8, interrupt: u8) {
    // println!("Handling interrupt {:02X}", interrupt);
    gb.cpu.interrupt_enable_master = false;
    gb.memory.set_byte(INTERRUPT_FLAG_REG, flags & !interrupt);
    let pc = gb.cpu.pc;
    push_word(gb, pc);
    gb.cpu.pc = get_interrupt_handler_addr(interrupt);
}

fn get_interrupt_handler_addr(interrupt: u8) -> u16 {
    match interrupt {
        V_BLANK => 0x40,
        LCD_STAT => 0x48,
        TIMER => 0x50,
        SERIAL => 0x58,
        JOYPAD => 0x60,
        _ => panic!("Invalid interrupt")
    }
}