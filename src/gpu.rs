use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use game_boy::GameBoy;
use glium::texture::texture2d::Texture2d;
use std::mem;

pub struct Gpu {
    window: GlutinFacade,
    program: glium::program::Program
}

#[derive(Copy, Clone)]
struct pixel(u8, u8, u8);

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    sprite: f32
}


impl Gpu {
    pub fn new() -> Gpu {
        let v_shader_src = r#"
            #version 140

            in float sprite; 
            in vec2 position;

            out float v_sprite;

            void main() {
                v_sprite = sprite;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in float sprite; 
            out vec4 color;

            void main() {
                if (sprite > 0) 
                {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
                else 
                {
                    color = vec4(1.0, 1.0, 1.0, 1.0);
                }
            }
        "#;
        let window = create_window();
        let program = glium::Program::from_source(&window, v_shader_src, fragment_shader_src, None).unwrap(); 
        Gpu {
            window: window,
            program: program
        }
    }
    pub fn draw_screen(&mut self, gb: &GameBoy) {
        let mut target = self.window.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let num_tiles = 1024;
        let tile_map_addr = if window_tile_map(gb) == 1 { 0x9C00 } else { 0x9800 };
        let tile_data_addr = if tile_data(gb) == 1 { 0x8000 } else { 0x8800 };
        // println!("Using tile map at {:X}", tile_data_addr);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        if gb.cpu.pc >= 0x64 && gb.cpu.pc < 0x69 {
            // println!("Tiles: {:04X}{:04X}{:04X}{:04X}", gb.memory.get_word(tile_data_addr + 10)
            //     , gb.memory.get_word(tile_data_addr + 12)
            //     , gb.memory.get_word(tile_data_addr + 14)
            //     , gb.memory.get_word(tile_data_addr + 16));

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
                // let sprite = gb.memory.get_word(tile_data_addr + ((tile_index as u16) * 2));

                let x_pos = i % 32;
                let y_pos = i / 32;

                let sprite_offset = (tile_index as u16) * 16;
                if tile_index > 0 {
                    let base_addr = sprite_offset + tile_data_addr;
                    println!("Drawing sprite {:04X} at {},{}", base_addr, x_pos, y_pos);
                    println!("Sprint data: {:04X}{:04X}{:04X}{:04X}{:04X}{:04X}{:04X}{:04X}", gb.memory.get_word(base_addr), gb.memory.get_word(base_addr+2), gb.memory.get_word(base_addr+4), gb.memory.get_word(base_addr+6), gb.memory.get_word(base_addr+8), gb.memory.get_word(base_addr+10), gb.memory.get_word(base_addr+12), gb.memory.get_word(base_addr+14));
                }

                for x in 0..8 {
                    
                    let sprite = gb.memory.get_word(tile_data_addr + sprite_offset + (x*2));
                    for y in 0..8 {
                        let color_id = ((sprite >> y) | (sprite >> (y+7))) & 0b11;
                        let color = match color_id {
                            3 => (0u8, 0u8, 0u8),
                            2 => (90u8, 90u8, 90u8),
                            1 => (180u8, 180u8, 180u8),
                            0 => (255u8, 255u8, 255u8),
                            _ => (0u8, 255u8, 0u8)                            
                        };

                        screen_buf[((x_pos * 8) + x) as usize][((y_pos * 8) + y) as usize] = color;
                    }
                }




                // let width = 1.0 / 64.0;
                // let left = (x as f32) * width * 2.0;
                // let top = 1.0 - ((y as f32) * width * 2.0);
                // let right = left + width + width;
                // let bottom = top + width + width;
                // let v1 = Vertex {position: [left - 0.5, top - 0.5], sprite: sprite as f32};
                // let v2 = Vertex {position: [left- 0.5, bottom- 0.5], sprite: sprite as f32};
                // let v3 = Vertex {position: [right- 0.5, top- 0.5], sprite: sprite as f32};
                // let v4 = Vertex {position: [right- 0.5, bottom- 0.5], sprite: sprite as f32};
                // let shape = vec![v1, v2, v3, v4];
                // let v_buffer = glium::VertexBuffer::new(&self.window, &shape).unwrap();
                // target.draw(&v_buffer, &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }

            let texture = glium::Texture2d::new(&self.window, screen_buf).unwrap();
            texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        }



        target.finish().unwrap();
    }



    // gb_tile_map
} 

fn window_tile_map(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 6 &0b1
}

fn bg_tile_map(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 3 &0b1
}

fn tile_data(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 4 &0b1
}

fn create_window() -> GlutinFacade {
    use glium::DisplayBuild;

    implement_vertex!(Vertex, position, sprite);

    glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_title("Gameboy Emulator".to_string())
        .build_glium()
        .unwrap()
}