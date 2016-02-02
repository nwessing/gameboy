use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use game_boy::GameBoy;
use glium::texture::texture2d::Texture2d;
use time;

const LCDC_STATUS_REG: u16 = 0xFF41;
const SCROLL_Y_REG: u16 = 0xFF42;
const SCROLL_X_REG: u16 = 0xFF43;
const WINDOW_Y_REG: u16 = 0xFF4A;
const WINDOW_X_REG: u16 = 0xFF4B;
const LCDC_Y_COORD: u16 = 0xFF44;
const MODE_FLAG_MASK: u8 = 0b1111_1100;


pub struct Gpu {
    window: GlutinFacade,
    screen_buf: Vec<Vec<(u8, u8, u8)>>,
    window_buf: Vec<Vec<(u8, u8, u8)>>,
    last_frame_time: u64    
}

impl Gpu {
    pub fn new() -> Gpu {
        let window = create_window();
        let mut screen_buf: Vec<Vec<(u8, u8, u8)>> = Vec::new();
        for x in 0..256 {
            screen_buf.push(Vec::<(u8, u8, u8)>::new());
            for y in 0..256 {
                screen_buf[x].push((255u8, 0u8, 0u8));
            } 
        }
        let mut window_buf: Vec<Vec<(u8, u8, u8)>> = Vec::new();
        for iy in 0..144 {
            window_buf.push(Vec::<(u8, u8, u8)>::new());
            for ix in 0..160 {
                window_buf[iy as usize].push((255u8, 0u8, 0u8));
            } 
        }
        Gpu {
            window: window,
            screen_buf: screen_buf,
            window_buf: window_buf,
            last_frame_time: 0
        }
    }

    pub fn update(&mut self, gb: &mut GameBoy) {
        let frame = 70224;
        let v_blank = 4560;
        let mode0 = 203;
        let mode2 = 80;
        let mode3 = 173;

        let mut status = gb.memory.get_byte(LCDC_STATUS_REG);
        let frame_step = gb.clock.current_tick() % frame;
        if frame_step > 65664 {
            //VBLANK
            status = (status & MODE_FLAG_MASK) | 0b01;
        } else {
            let scan_line = (frame_step / 144) as u8;
            let scan_line_clk = frame_step % 456;
            if scan_line_clk <= mode0 {
                //HBLANK
                status = status & MODE_FLAG_MASK;        
            } else if scan_line_clk <= mode0 + mode2 {
                //OAM
                status = (status & MODE_FLAG_MASK) | 0b10; 
            } else {
                //OAM + VRAM
                status = (status & MODE_FLAG_MASK) | 0b11; 
            }

            let lcdc_y_coord = gb.memory.get_byte(LCDC_Y_COORD);
            if scan_line == 0 && lcdc_y_coord != 0 {
                self.draw_screen(gb);
                // let frame_time = time::precise_time_ns() - self.last_frame_time;
                // println!("Frame time was {}ms", frame_time / 1_000_000);
                // self.last_frame_time = time::precise_time_ns();
            }
            gb.memory.set_byte(LCDC_Y_COORD, scan_line);
        }

        gb.memory.set_byte(LCDC_STATUS_REG, status);
    }

    pub fn draw_screen(&mut self, gb: &GameBoy) {
        let mut target = self.window.draw();

        let num_tiles = 1024;
        let tile_map_addr = if window_tile_map(gb) == 1 { 0x9C00 } else { 0x9800 };
        let tile_data_addr = if tile_data(gb) == 1 { 0x8000 } else { 0x8800 };
        let bg_palette = bg_palette(gb);

        let scroll_y = 255 - gb.memory.get_byte(SCROLL_Y_REG);
        let scroll_x = gb.memory.get_byte(SCROLL_X_REG);

        let start = time::precise_time_ns();

        for i in 0..num_tiles {
            let addr = tile_map_addr + i;
            let tile_index = gb.memory.get_byte(addr); 
            let x_pos = i % 32;
            let y_pos = 31 - (i / 32);

            let sprite_offset = (tile_index as u16) * 16;

            for y in 0..8 {
                let sprite = gb.memory.get_word(tile_data_addr + sprite_offset + (14 - (y*2)));
                for x in 0..8 {
                    let xi = 7 - x;
                    let palette_index = ((sprite >> xi) &0b1) | ((sprite >> (xi+7)) &0b10);
                    let color_id = get_palette_color(bg_palette, palette_index as u8);
                    let color = get_color(color_id);
                    self.screen_buf[((y_pos * 8) + y) as usize][((x_pos * 8) + x) as usize] = color;
                }
            }
        }

        for iy in 0..144 {
            let mut y = ((iy as u16) + (111u16 + scroll_y as u16)) as usize;
            if y > 255 {
                y = y - 255;
            }
            for ix in 0..160 {
                let mut x = (ix + scroll_x) as usize;
                if x > 255 {
                    x = 255 - x;
                }
                self.window_buf[iy as usize][ix as usize] = self.screen_buf[y][x];
            } 
        }

        let texture = glium::Texture2d::new(&self.window, self.window_buf.clone()).unwrap();
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