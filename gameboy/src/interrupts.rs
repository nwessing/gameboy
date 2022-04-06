use crate::game_boy::GameBoy;
use crate::memory::Register;
use crate::util::push_word;

const V_BLANK: u8 = 0x01;
const LCD_STAT: u8 = 0x02;
const TIMER: u8 = 0x04;
const SERIAL: u8 = 0x08;
const JOYPAD: u8 = 0x10;

pub fn check_interrupts(gb: &mut GameBoy) {
    let enabled = gb.memory.get_register(Register::InterruptEnable);
    let flag = gb.memory.get_register(Register::InterruptFlag);
    let interrupts = enabled & flag;
    if gb.cpu.interrupt_enable_master {
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

    if interrupts != 0 {
        gb.cpu.is_halted = false;
        if gb.cpu.is_halted {
            println!("GO");
        }
    }
}

fn handle_interrupt(gb: &mut GameBoy, flags: u8, interrupt: u8) {
    gb.cpu.interrupt_enable_master = false;
    gb.memory
        .set_register(Register::InterruptFlag, flags & !interrupt);
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
        _ => panic!("Invalid interrupt"),
    }
}
