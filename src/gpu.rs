use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use game_boy::GameBoy;
use glium::texture::texture2d::Texture2d;

pub struct Gpu {
    window: GlutinFacade,
    program: glium::program::Program
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    is_colored: f32
}


impl Gpu {
    pub fn new() -> Gpu {
        let v_shader_src = r#"
            #version 140

            in float is_colored; 
            in vec2 position;

            out float v_is_colored;

            void main() {
                v_is_colored = is_colored;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in float v_is_colored; 
            out vec4 color;

            void main() {
                if (v_is_colored > 0) 
                {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
                else 
                {
                    color = vec4(0.0, 0.0, 0.0, 1.0);
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

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);


        if gb.cpu.pc >= 0x64 && gb.cpu.pc < 0x69 {
            for i in 0..num_tiles {
                let addr = tile_map_addr + i;
                let tile_ref = gb.memory.get_byte(addr);

                let x = i % 32;
                let y = i / 32;

                let width = 1.0 / 64.0;
                let left = (x as f32) * width * 2.0;
                let top = (y as f32) * width * 2.0;
                let right = left + width + width;
                let bottom = top + width + width;
                // 13
                // 24
                let v1 = Vertex {position: [left - 0.5, top - 0.5], is_colored: tile_ref as f32};
                let v2 = Vertex {position: [left- 0.5, bottom- 0.5], is_colored: tile_ref as f32};
                let v3 = Vertex {position: [right- 0.5, top- 0.5], is_colored: tile_ref as f32};
                let v4 = Vertex {position: [right- 0.5, bottom- 0.5], is_colored: tile_ref as f32};
                let shape = vec![v1, v2, v3, v4];
                let v_buffer = glium::VertexBuffer::new(&self.window, &shape).unwrap();
                target.draw(&v_buffer, &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
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

    implement_vertex!(Vertex, position, is_colored);

    glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Gameboy Emulator".to_string())
        .build_glium()
        .unwrap()
}