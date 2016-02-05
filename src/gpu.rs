use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use game_boy::GameBoy;
use glium::texture::texture2d::Texture2d;
use time;

const LCD_CONTROL_REG: u16 = 0xFF40;
const LCDC_STATUS_REG: u16 = 0xFF41;
const SCROLL_Y_REG: u16 = 0xFF42;
const SCROLL_X_REG: u16 = 0xFF43;
const WINDOW_Y_REG: u16 = 0xFF4A;
const WINDOW_X_REG: u16 = 0xFF4B;
const LCDC_Y_COORD: u16 = 0xFF44;
const MODE_FLAG_MASK: u8 = 0b1111_1100;

const VERTICAL_RES: u8 = 144;
const HORIZONTAL_RES: u8 = 160;

pub struct Gpu {
    window: GlutinFacade,
    window_buf: Vec<Vec<(u8, u8, u8)>>,
    last_frame_time: u64    
}

impl Gpu {
    pub fn new() -> Gpu {
        let window = create_window();   
        let mut window_buf = new_window_buf();
        Gpu {
            window: window,
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

        let status = gb.memory.get_byte(LCDC_STATUS_REG);
        let prev_mode = status & 0b0000_0011; 
        let mode;
        let frame_step = gb.clock.current_tick() % frame;
        let scan_line = (frame_step / 456) as u8;
        
        if frame_step > 65664 {
            //VBLANK
            mode = 0b01;
        } else {
            let scan_line_clk = frame_step % 456;
            if scan_line_clk < mode2 {
                mode = 0b10;
            } else if scan_line_clk < mode2 + mode3 {
                mode = 0b11;
            } else {
                mode = 0b00;
            }

            // if scan_line_clk <= mode0 {
            //     //HBLANK
            //     mode = 0b00;
            // } else if scan_line_clk <= mode0 + mode2 {
            //     //OAM
            //     mode = 0b10;
            // } else {
            //     //OAM + VRAM
            //     mode = 0b11;
            // }
        }

        // print!("{}", mode);

        if prev_mode != mode { //&& display_enabled(gb) {
            if prev_mode == 0b10 && mode == 0b11 {
                self.draw_scan_line(gb, scan_line);    
            }
            if prev_mode == 0b00 && mode == 0b01 {
                self.render_screen();
                let int_flags = gb.memory.get_byte(0xFF0F);
                gb.memory.set_byte(0xFF0F, int_flags | 0x01);
                // println!("VBLANK INTERR line {}", scan_line);
            }
        }

        gb.memory.set_byte(LCDC_Y_COORD, scan_line);
        gb.memory.set_byte(LCDC_STATUS_REG, (status & MODE_FLAG_MASK) | mode);
    }

    pub fn draw_scan_line(&mut self, gb: &GameBoy, scan_line: u8) {
        let display_on = display_enabled(gb);
        let tile_map_addr = if bg_tile_map(gb) == 1 { 0x9C00 } else { 0x9800 };
        let bg_palette = bg_palette(gb);

        let scroll_y = gb.memory.get_byte(SCROLL_Y_REG);
        let scroll_x = gb.memory.get_byte(SCROLL_X_REG);

        let y = scroll_y + scan_line;
        let base_tile_map_index = (y as u16) / 8 * 32;

        for x in scroll_x..(scroll_x + HORIZONTAL_RES) {
            // if !display_on {
            //     self.window_buf[scan_line as usize][(x - scroll_x) as usize] = (255, 255, 255);
            //     continue;
            // }

            let tile_index = base_tile_map_index + ((x / 8) as u16);
            let tile_data_index = gb.memory.get_byte(tile_map_addr + tile_index);
            let sprite_addr = get_sprite_addr(gb, tile_data_index);

            let sprite_scan_line = (y % 8) as u16;
            let sprite = gb.memory.get_word(sprite_addr + (sprite_scan_line*2));

            let sprite_x_index =  7 - (x % 8);
            let palette_index = ((sprite >> sprite_x_index) &0b1) | ((sprite >> (sprite_x_index+7)) &0b10);
            let color_id = get_palette_color(bg_palette, palette_index as u8);
            let color = get_color(color_id);
            self.window_buf[scan_line as usize][(x - scroll_x) as usize] = color;
        }
    }

    pub fn render_screen(&mut self) {
        let mut target = self.window.draw();
        let mut reversed_buf = self.window_buf.clone();
        reversed_buf.reverse();
        let texture = glium::Texture2d::new(&self.window, reversed_buf).unwrap();
        texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        target.finish().unwrap();
        // self.window_buf = new_window_buf();
    }
}

fn new_window_buf() -> Vec<Vec<(u8, u8, u8)>> {
    let mut window_buf: Vec<Vec<(u8, u8, u8)>> = Vec::new();
    for iy in 0..144 {
        window_buf.push(Vec::<(u8, u8, u8)>::new());
        for ix in 0..160 {
            window_buf[iy as usize].push((255u8, 0u8, 0u8));
        } 
    }
    window_buf
}

fn display_enabled(gb: &GameBoy) -> bool {
    gb.memory.get_byte(LCD_CONTROL_REG) & 0x80 == 0x80
}

fn window_tile_map(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(LCD_CONTROL_REG) >> 6 &0b1
}

fn bg_tile_map(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(LCD_CONTROL_REG) >> 3 &0b1
}

fn bg_palette(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF47)
}

fn tile_data(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 4 &0b1
}

fn get_sprite_addr(gb: &GameBoy, tile_index: u8) -> u16{
    if tile_data(gb) == 1 {
        0x8000 + ((tile_index as u16) * 16)
    } else {
        let signed_index = tile_index as i8;

        ((0x9000i32 + (signed_index as i32)) as u32) as u16
    }
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