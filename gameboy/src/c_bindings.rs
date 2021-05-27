use crate::{Button, ButtonState, InitializationOptions, InputEvent, System};
use std::{ptr, slice};

#[repr(C)]
pub struct SystemInitializationOptions {
    pub boot_rom_length: u32,
    pub boot_rom: *const u8,
    pub game_rom_length: u32,
    pub game_rom: *const u8,
    pub external_ram_length: u32,
    pub external_ram: *const u8,
    pub debug_mode: bool,
}

#[repr(C)]
pub struct SystemHandle {
    system: System,
    events_buffer: Vec<InputEvent>,
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_create_system(
    options: SystemInitializationOptions,
) -> *mut SystemHandle {
    if options.game_rom == ptr::null_mut() || options.game_rom_length == 0 {
        return std::ptr::null_mut();
    }

    let game_rom = slice::from_raw_parts(options.game_rom, options.game_rom_length as usize);
    let boot_rom = to_optional_array_slice(options.boot_rom, options.boot_rom_length as usize);
    let external_ram =
        to_optional_array_slice(options.external_ram, options.external_ram_length as usize);

    println!(
        "[gameboy] boot rom pointer = {:p}, length = {}",
        options.game_rom, options.game_rom_length
    );
    let internals = Box::new(SystemHandle {
        system: System::new(InitializationOptions {
            game_rom,
            boot_rom,
            external_ram,
            debug_mode: options.debug_mode,
        }),
        events_buffer: Vec::with_capacity(8),
    });

    println!("[gameboy] system initialized");

    Box::into_raw(internals)
}

unsafe fn to_optional_array_slice<'a, T>(data: *const T, length: usize) -> Option<&'a [T]> {
    if data != ptr::null_mut() && length > 0 {
        Some(slice::from_raw_parts(data, length))
    } else {
        None
    }
}

#[repr(u32)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

impl Key {
    fn to_button(&self) -> Button {
        match self {
            Key::Up => Button::Up,
            Key::Down => Button::Down,
            Key::Left => Button::Left,
            Key::Right => Button::Right,
            Key::A => Button::A,
            Key::B => Button::B,
            Key::Start => Button::Start,
            Key::Select => Button::Select,
        }
    }
}

#[repr(C)]
pub struct Event {
    pub key: Key,
    pub is_pressed: bool,
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_add_event(handle: *mut SystemHandle, event: Event) {
    if handle.is_null() {
        return;
    }

    let internals = &mut *handle;
    internals.events_buffer.push(InputEvent {
        state: if event.is_pressed {
            ButtonState::Pressed
        } else {
            ButtonState::Released
        },
        button: event.key.to_button(),
    });
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_run_single_frame(
    handle: *mut SystemHandle,
    output: *mut u8,
    width: *mut u32,
    height: *mut u32,
) {
    if handle.is_null() {
        *width = 0;
        *height = 0;
        return;
    }

    let internals = &mut *handle;

    if let Some(framebuffer) = internals.system.run_single_frame(&internals.events_buffer) {
        let mut offset = 0;
        if framebuffer.width != *width || framebuffer.height != *height {
            *width = framebuffer.width;
            *height = framebuffer.height;
            return;
        }

        for value in framebuffer.buffer {
            output.offset(offset).write(*value);
            offset += 1;
        }
    } else {
        *width = 0;
        *height = 0;
    };
    internals.events_buffer.clear();
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_is_exit_requested(handle: *const SystemHandle) -> bool {
    if handle.is_null() {
        return true;
    }

    (&*handle).system.exit_requested()
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_request_exit(handle: *mut SystemHandle) {
    if handle.is_null() {
        return;
    }

    (&mut *handle).system.request_exit()
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_destroy_system(handle: *mut SystemHandle) {
    std::mem::drop(handle);
}
