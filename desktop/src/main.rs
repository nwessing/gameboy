use fs::File;
use gameboy::{Button, ButtonState, InitializationOptions, InputEvent, System};
use sdl2::audio::{AudioQueue, AudioSpecDesired, AudioStatus};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

const FREQUENCY: u32 = 48000;

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
        sound_frequency: FREQUENCY,
    };
    let mut system = System::new(options);

    let sdl_context = sdl2::init().unwrap(); //.ok_or("Could not create SDL Context.");
    let video_subsystem = sdl_context.video().unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
    let audio_spec = AudioSpecDesired {
        freq: Some(FREQUENCY as i32),
        channels: Some(1),
        samples: None,
    };

    let mut audio_framebuffer = Vec::with_capacity(FREQUENCY as usize);

    let queue: AudioQueue<u8> = audio_subsystem.open_queue(None, &audio_spec).unwrap();

    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("Gameboy Emulator", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            System::screen_width(),
            System::screen_height(),
        )
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut events = Vec::with_capacity(8);
    let mut paused = false;
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
                    if keycode == Some(Keycode::Space) {
                        paused = !paused;
                        if paused {
                            queue.pause();
                        } else {
                            queue.resume();
                        }
                    }

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

        if !paused {
            texture
                .with_lock(None, |buffer: &mut [u8], _: usize| {
                    system.run_single_frame(&events, buffer, &mut audio_framebuffer);

                    queue.queue_audio(&audio_framebuffer).unwrap();
                    if queue.status() != AudioStatus::Playing {
                        queue.resume();
                    }

                    audio_framebuffer.clear();
                })
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
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
