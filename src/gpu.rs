use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use game_boy::GameBoy;
use glium::texture::texture2d::Texture2d;
use std::mem;

pub struct Gpu {
    window: GlutinFacade
}

impl Gpu {
    pub fn new() -> Gpu {
        let window = create_window();
        Gpu {
            window: window,
        }
    }
    pub fn draw_screen(&mut self, gb: &GameBoy) {
        if gb.cpu.pc >= 0x64 && gb.cpu.pc < 0x69 {

        } else {
            return;
        }

        let mut target = self.window.draw();

        let num_tiles = 1024;
        let tile_map_addr = if window_tile_map(gb) == 1 { 0x9C00 } else { 0x9800 };
        let tile_data_addr = if tile_data(gb) == 1 { 0x8000 } else { 0x8800 };
        let bg_palette = bg_palette(gb);

        let mut screen_buf: Vec<Vec<(u8, u8, u8)>> = Vec::new();
        for x in 0..256 {
            screen_buf.push(Vec::<(u8, u8, u8)>::new());
            for y in 0..256 {
                screen_buf[x].push((255u8, 0u8, 0u8));
            } 
        }

        for i in 0..num_tiles {
            let addr = tile_map_addr + i;
            let tile_index = gb.memory.get_byte(addr); 
            let x_pos = i % 32;
            let y_pos = 31 - (i / 32);

            let sprite_offset = (tile_index as u16) * 16;
            // if tile_index > 0 {
            //     let base_addr = sprite_offset + tile_data_addr;
            //     println!("Drawing sprite {:04X} at {},{}", base_addr, x_pos, y_pos);
            //     println!("Sprint data: {:04X}{:04X}{:04X}{:04X}{:04X}{:04X}{:04X}{:04X}", gb.memory.get_word(base_addr), gb.memory.get_word(base_addr+2), gb.memory.get_word(base_addr+4), gb.memory.get_word(base_addr+6), gb.memory.get_word(base_addr+8), gb.memory.get_word(base_addr+10), gb.memory.get_word(base_addr+12), gb.memory.get_word(base_addr+14));
            // }

            for y in 0..8 {
                let sprite = gb.memory.get_word(tile_data_addr + sprite_offset + (14 - (y*2)));
                for x in 0..8 {
                    let xi = 7 - x;
                    let palette_index = ((sprite >> xi) &0b1) | ((sprite >> (xi+7)) &0b10);
                    let color_id = get_palette_color(bg_palette, palette_index as u8);
                    let color = get_color(color_id);
                    screen_buf[((y_pos * 8) + y) as usize][((x_pos * 8) + x) as usize] = color;
                }
            }
        }

        let texture = glium::Texture2d::new(&self.window, screen_buf).unwrap();
        texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        
        target.finish().unwrap();
    }
}

fn window_tile_map(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 6 &0b1
}

fn bg_tile_map(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 3 &0b1
}

fn bg_palette(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF47)
}

fn tile_data(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 4 &0b1
}

fn get_color(color_id: u8) -> (u8, u8, u8) {
    match color_id {
        3 => (0u8, 0u8, 0u8),
        2 => (96u8, 96u8, 96u8),
        1 => (192u8, 192u8, 192u8),
        0 => (255u8, 255u8, 255u8),
        _ => (255u8, 0u8, 0u8) //Having Red on the screen should indicate something went wrong.                            
    }
}

fn get_palette_color(palette: u8, index: u8) -> u8 {
    if index > 3 {
        panic!("Invalid palette id");
    }

    palette >> (index << 1) & 0b11    
}

fn create_window() -> GlutinFacade {
    use glium::DisplayBuild;
    glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_title("Gameboy Emulator".to_string())
        .build_glium()
        .unwrap()
}