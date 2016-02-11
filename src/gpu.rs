use std::cmp;
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use game_boy::GameBoy;

const LCD_CONTROL_REG: u16 = 0xFF40;
const LCDC_STATUS_REG: u16 = 0xFF41;
const SCROLL_Y_REG: u16 = 0xFF42;
const SCROLL_X_REG: u16 = 0xFF43;

const SPRITE_DATA_REG: u16 = 0xFE00;
// const WINDOW_Y_REG: u16 = 0xFF4A;
// const WINDOW_X_REG: u16 = 0xFF4B;
const LCDC_Y_COORD: u16 = 0xFF44;
const MODE_FLAG_MASK: u8 = 0b1111_1100;

const VERTICAL_RES: u8 = 144;
const HORIZONTAL_RES: u8 = 160;

const MODE0_HBLANK: u8 = 0;
const MODE1_VBLANK: u8 = 1;
const MODE2_ACCESSING_OAM: u8 = 2;
const MODE3_ACCESSING_VRAM: u8 = 3;


pub struct Gpu {
    window: GlutinFacade,
    window_buf: Vec<Vec<(u8, u8, u8)>>,
    ticks: u64
}

impl Gpu {
    pub fn new() -> Gpu {
        let window = create_window();   
        let window_buf = new_window_buf();
        Gpu {
            window: window,
            window_buf: window_buf,
            ticks: 0
        }
    }

    pub fn update(&mut self, gb: &mut GameBoy, ticks: u8) {
        let status = gb.memory.get_byte(LCDC_STATUS_REG);
        let prev_mode = status & 0b0000_0011; 
        let mode;
        
        if !display_enabled(gb) {
            if prev_mode != MODE1_VBLANK {
                panic!("LCD turned off outside of VBLANK, this should not happen.");
            }
            gb.memory.set_scan_line(0);
            return;
        }
        
        self.ticks += ticks as u64;

        let frame = 70224;
        let mode2 = 80;
        let mode3 = 173;


        let frame_step = self.ticks % frame;
        let scan_line_clk = frame_step % 456;
        let scan_line = (frame_step / 456) as u8;
        
        if frame_step >= 65664 {
            mode = MODE1_VBLANK;
        } else {
            if scan_line_clk < mode2 {
                mode = MODE2_ACCESSING_OAM;
            } else if scan_line_clk < mode2 + mode3 {
                mode = MODE3_ACCESSING_VRAM;
            } else {
                mode = MODE0_HBLANK;
            }
        }

        if prev_mode == MODE3_ACCESSING_VRAM && mode == MODE0_HBLANK {
            self.draw_scan_line(gb, scan_line);    
        }
        
        if prev_mode == MODE0_HBLANK && mode == MODE1_VBLANK {
            self.render_screen(gb);
            // if 0xFFFF & 0x01 == 0x01 {
                let int_flags = gb.memory.get_byte(0xFF0F);
                gb.memory.set_byte(0xFF0F, int_flags | 0x01);
            // }
        }

        // println!("total ticks: {}, frame: {}, scan line: {}, mode: {}, delta ticks: {}, sl ticks: {}", self.ticks, self.ticks / frame, scan_line,  mode, ticks, scan_line_clk);

        gb.memory.set_scan_line(scan_line);
        gb.memory.set_byte(LCDC_STATUS_REG, (status & MODE_FLAG_MASK) | mode);
    }

    pub fn draw_scan_line(&mut self, gb: &GameBoy, scan_line: u8) {
        let display_on = display_enabled(gb);
        let tile_map_addr = if bg_tile_map(gb) == 1 { 0x9C00 } else { 0x9800 };
        let bg_palette = bg_palette(gb);

        let scroll_y = gb.memory.get_byte(SCROLL_Y_REG) as u16;
        let scroll_x = gb.memory.get_byte(SCROLL_X_REG) as u16;

        let mut y = scroll_y + (scan_line as u16);
        if y > 255 {
            y -= 256;
        }
        let base_tile_map_index = (y as u16) / 8 * 32;

        for x in scroll_x..(scroll_x + (HORIZONTAL_RES as u16)) {
            let mut x = x;
            if x > 255 {
                x -= 256;
            }

            let tile_index = base_tile_map_index + ((x / 8) as u16);
            let tile_data_index = gb.memory.get_byte(tile_map_addr + tile_index);
            let sprite_addr = get_bg_tile_addr(gb, tile_data_index);

            let sprite_scan_line = (y % 8) as u16;
            let sprite = gb.memory.get_word(sprite_addr + (sprite_scan_line*2));

            let sprite_x_index =  7 - (x % 8);
            let palette_index = ((sprite >> sprite_x_index) &0b1) | ((sprite >> (sprite_x_index+7)) &0b10);
            let color_id = get_palette_color(bg_palette, palette_index as u8);
            let color = get_color(color_id);
            self.window_buf[scan_line as usize][(x - scroll_x) as usize] = color;
        }

        for i_sprite in 0..40 {
            let sprite_addr = SPRITE_DATA_REG + (i_sprite * 4);
            let top = (gb.memory.get_byte(sprite_addr) as i16) - 16;
            let bottom = top + 8;
            if top <= (scan_line as i16) && bottom > (scan_line as i16) {
                let right = gb.memory.get_byte(sprite_addr + 1) as i16;
                let left = right - 8;

                let sprite_pattern_index = gb.memory.get_byte(sprite_addr + 2);
                let sprite_tile_addr = get_sprite_tile_addr(gb, sprite_pattern_index);
                let sprite_scan_line = (((scan_line as i16) - top) % 8) as u16;
                let sprite = gb.memory.get_word(sprite_tile_addr + (sprite_scan_line * 2));
                let sprite_attrs = gb.memory.get_byte(sprite_tile_addr + 3);
                let palette = sprite_pallete(gb, sprite_attrs);

                for x_pix in left..right {
                    if x_pix < 0 || x_pix >= (HORIZONTAL_RES as i16) {
                        continue;
                    }
                    let sprite_x_index = 7 - (x_pix % 8);
                    let palette_index = ((sprite >> sprite_x_index) &0b1) | ((sprite >> (sprite_x_index+7)) &0b10);
                    let color_id = get_palette_color(palette, palette_index as u8);
                    let color = get_color(color_id);
                    self.window_buf[scan_line as usize][x_pix as usize] = color;
                }
            } 
        }
        // for x in 0..HORIZONTAL_RES {

        // }
    }

    // fn draw_tile_scan_line(&mut self, ) {

    // }

    pub fn render_screen(&mut self, gb: &GameBoy) {
        let target = self.window.draw();
        let mut reversed_buf = self.window_buf.clone();
        reversed_buf.reverse();
        let texture = glium::Texture2d::new(&self.window, reversed_buf).unwrap();
        texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        target.finish().unwrap();
        // print_tile_map(gb);
    }
}

fn print_tile_data(gb: &GameBoy) {
    println!("FRAME FINISHED; Tile Data:");
    for addr in 0x8000..0x9000 {
        print!("{:02x}", gb.memory.get_byte(addr));
    }
}

fn print_tile_map(gb: &GameBoy) {
    println!("FRAME FINISHED; Tile Map:");
    for addr in 0x9800..0xA000 {
        print!("{:02x}", gb.memory.get_byte(addr));
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

fn sprite_pallete(gb: &GameBoy, sprite_attr: u8) -> u8 {
    if sprite_attr & 0x10 == 0x10 {
       gb.memory.get_byte(0xFF49)
    } else {
       gb.memory.get_byte(0xFF48)
    }
}

fn tile_data(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 4 &0b1
}

fn get_sprite_tile_addr(gb: &GameBoy, tile_index: u8) -> u16 {
    0x8000 + ((tile_index as u16) * 16)
}

fn get_bg_tile_addr(gb: &GameBoy, tile_index: u8) -> u16 {
    if tile_data(gb) == 1 {
        get_sprite_tile_addr(gb, tile_index)
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