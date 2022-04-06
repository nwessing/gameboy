use fs::File;
use gameboy::{Button, ButtonState, Framebuffer, InitializationOptions, InputEvent, System};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::WindowCanvas;

// #[link(name = "vcruntime")]
// extern "C" {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a path to a GameBoy ROM file");
    }

    let boot_rom = match File::open("roms/boot_rom.gb") {
        Ok(mut boot_file) => {
            let mut boot_buf = Vec::new();
            boot_file.read_to_end(&mut boot_buf).unwrap();
            Some(boot_buf)
        }
        Err(_) => None,
    };

    let game_file_path = Path::new(&args[1]);
    let game_rom = match File::open(game_file_path) {
        Ok(mut game_file) => {
            let mut game_buf = Vec::new();
            game_file.read_to_end(&mut game_buf).unwrap();
            game_buf
        }
        Err(x) => panic!("{}", x),
    };

    let game_save_path = game_file_path.with_extension("gbsave");
    let external_ram = match File::open(game_save_path) {
        Ok(mut game_save_file) => {
            let mut save_buf = Vec::new();
            game_save_file.read_to_end(&mut save_buf).unwrap();
            Some(save_buf)
        }
        Err(_) => None,
    };

    let options = InitializationOptions {
        game_rom: &game_rom,
        external_ram: external_ram.as_deref(),
        boot_rom: boot_rom.as_deref(),
        debug_mode: false,
    };
    let mut system = System::new(options);

    let sdl_context = sdl2::init().unwrap(); //.ok_or("Could not create SDL Context.");
    let video_subsystem = sdl_context.video().unwrap();
    let mut timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("Gameboy Emulator", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut events = Vec::with_capacity(8);
    loop {
        events.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => system.request_exit(),
                Event::KeyDown { keycode, .. } => {
                    if let Some(button) = keycode_to_button(keycode) {
                        events.push(InputEvent {
                            button,
                            state: ButtonState::Pressed,
                        });
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(button) = keycode_to_button(keycode) {
                        events.push(InputEvent {
                            button,
                            state: ButtonState::Released,
                        });
                    }
                }
                _ => (),
            }
        }

        match system.run_single_frame(&events) {
            Some(framebuffer) => {
                render_frame(&mut canvas, &framebuffer);
                // timer_subsystem.delay(15);
            }
            None => {}
        }

        if system.exit_requested() {
            break;
        }
    }
    save_external_ram(&system, &game_file_path);
}

fn save_external_ram(system: &System, game_file_path: &Path) {
    if let Some(external_ram) = system.copy_external_ram_banks() {
        let game_save_path = game_file_path.with_extension("gbsave");
        let mut save_file = match File::create(game_save_path) {
            Ok(x) => x,
            Err(x) => panic!("{}", x),
        };

        match save_file.write_all(external_ram.as_slice()) {
            Ok(x) => x,
            Err(x) => panic!("{}", x),
        };
    }
}

fn render_frame(canvas: &mut WindowCanvas, frame_buffer: &Framebuffer) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            frame_buffer.width,
            frame_buffer.height,
        )
        .unwrap();
    texture
        .with_lock(None, |buffer: &mut [u8], _: usize| {
            let mut out_index: usize = 0;
            for value in frame_buffer.buffer {
                let color0 = get_color(0b00000011 & (value >> 6));
                buffer[out_index + 0] = color0.0;
                buffer[out_index + 1] = color0.1;
                buffer[out_index + 2] = color0.2;
                out_index += 3;

                let color1 = get_color(0b00000011 & (value >> 4));
                buffer[out_index + 0] = color1.0;
                buffer[out_index + 1] = color1.1;
                buffer[out_index + 2] = color1.2;
                out_index += 3;

                let color2 = get_color(0b00000011 & (value >> 2));
                buffer[out_index + 0] = color2.0;
                buffer[out_index + 1] = color2.1;
                buffer[out_index + 2] = color2.2;
                out_index += 3;

                let color3 = get_color(0b00000011 & (value >> 0));
                buffer[out_index + 0] = color3.0;
                buffer[out_index + 1] = color3.1;
                buffer[out_index + 2] = color3.2;
                out_index += 3;
            }
        })
        .unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

fn get_color(color_id: u8) -> (u8, u8, u8) {
    match color_id {
        3 => (0u8, 0u8, 0u8),
        2 => (96u8, 96u8, 96u8),
        1 => (192u8, 192u8, 192u8),
        0 => (255u8, 255u8, 255u8),
        _ => (255u8, 0u8, 0u8), //Having Red on the screen should indicate something went wrong.
    }
}

fn keycode_to_button(key: Option<Keycode>) -> Option<Button> {
    let keycode = match key {
        Some(keycode) => keycode,
        None => return None,
    };

    match keycode {
        Keycode::W => Some(Button::Up),
        Keycode::A => Some(Button::Left),
        Keycode::S => Some(Button::Down),
        Keycode::D => Some(Button::Right),
        Keycode::M => Some(Button::B),
        Keycode::K => Some(Button::A),
        Keycode::J => Some(Button::Start),
        Keycode::H => Some(Button::Select),
        _ => None,
    }
}
