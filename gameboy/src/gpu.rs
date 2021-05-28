use crate::game_boy::GameBoy;

const LCD_CONTROL_REG: u16 = 0xFF40;
const LCDC_STATUS_REG: u16 = 0xFF41;
const SCROLL_Y_REG: u16 = 0xFF42;
const SCROLL_X_REG: u16 = 0xFF43;

const SPRITE_DATA_REG: u16 = 0xFE00;
const OBJECT_PALETTE0_DATA_REG: u16 = 0xFF48;
const OBJECT_PALETTE1_DATA_REG: u16 = 0xFF49;

const WINDOW_Y_REG: u16 = 0xFF4A;
const WINDOW_X_REG: u16 = 0xFF4B;

const LCDC_Y_COORD: u16 = 0xFF44;
const LY_COMPARE: u16 = 0xFF45;
const LCD_STATUS_FLAG_MASK: u8 = 0b1111_1000;
const LCD_STATUS_COINCIDENCE_INT: u8 = 0b0100_0000;
const LCD_STATUS_MODE2_INT: u8 = 0b0010_0000;
const LCD_STATUS_MODE1_INT: u8 = 0b0001_0000;
const LCD_STATUS_MODE0_INT: u8 = 0b0000_1000;
const LCD_STATUS_COINCIDENCE: u8 = 0b0000_0100;

pub const VERTICAL_RES: u8 = 144;
pub const HORIZONTAL_RES: u8 = 160;
pub const PIXELS_PER_BYTE: u8 = 4;
pub const BUFFER_SIZE: usize =
    (VERTICAL_RES as usize * HORIZONTAL_RES as usize) / PIXELS_PER_BYTE as usize;

const MODE0_HBLANK: u8 = 0;
const MODE1_VBLANK: u8 = 1;
const MODE2_ACCESSING_OAM: u8 = 2;
const MODE3_ACCESSING_VRAM: u8 = 3;

pub struct Gpu {
    pub window_buf: Box<[u8; BUFFER_SIZE]>,
    frame_step: u32,
}

#[derive(Debug)]
struct Sprite {
    y_pos: i16,
    x_pos: i16,
    tile_pattern_addr: u16,
    attributes: u8,
    index: u8,
    height: i16,
}

impl Sprite {
    fn new(gb: &GameBoy, sprite_index: u8, height: u8) -> Sprite {
        let sprite_addr = SPRITE_DATA_REG + ((sprite_index as u16) * 4);
        let top = (gb.memory.get_byte(sprite_addr) as i16) - 16;
        let left = (gb.memory.get_byte(sprite_addr + 1) as i16) - 8;
        let tile_pattern_index = gb.memory.get_byte(sprite_addr + 2);
        let tile_pattern_addr = get_sprite_tile_addr(tile_pattern_index);
        let attributes = gb.memory.get_byte(sprite_addr + 3);
        Sprite {
            y_pos: top,
            x_pos: left,
            tile_pattern_addr,
            attributes,
            index: sprite_index,
            height: height as i16,
        }
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn left(&self) -> i16 {
        self.x_pos
    }

    pub fn top(&self) -> i16 {
        self.y_pos
    }

    pub fn right(&self) -> i16 {
        self.x_pos + 8
    }

    pub fn bottom(&self) -> i16 {
        self.y_pos + self.height
    }

    pub fn get_tile_pattern(&self, gb: &GameBoy, scan_line: u8) -> u16 {
        let sprite_y = if self.is_mirrored_vertically() {
            ((self.height - 1) - ((scan_line as i16) - self.top()) % self.height) as u16
        } else {
            (((scan_line as i16) - self.top()) % self.height) as u16
        };

        let pattern = gb.memory.get_word(self.tile_pattern_addr + (sprite_y * 2));
        pattern
    }

    fn get_palette(&self, gb: &GameBoy) -> u8 {
        if self.attributes & 0x10 == 0x10 {
            gb.memory.get_byte(OBJECT_PALETTE1_DATA_REG)
        } else {
            gb.memory.get_byte(OBJECT_PALETTE0_DATA_REG)
        }
    }

    pub fn above_bg(&self) -> bool {
        self.attributes & 0x80 == 0x00
    }

    pub fn is_mirrored_horizontally(&self) -> bool {
        self.attributes & 0x20 == 0x20
    }

    pub fn is_mirrored_vertically(&self) -> bool {
        self.attributes & 0x40 == 0x40
    }
}

impl Gpu {
    pub fn new() -> Gpu {
        let window_buf = Box::new([0; BUFFER_SIZE]);
        Gpu {
            window_buf,
            frame_step: 0,
        }
    }

    /// Updates GPU state and returns whether the frame buffer has a completed
    /// frame
    pub fn update(&mut self, gb: &mut GameBoy, ticks: u8) -> bool {
        let status = gb.memory.get_byte(LCDC_STATUS_REG);
        let prev_mode = status & 0b0000_0011;

        if !display_enabled(gb) {
            if prev_mode != MODE1_VBLANK {
                // println!("LCD turned off outside of VBLANK, this should not happen.");
            }
            gb.memory.set_owned_byte(LCDC_Y_COORD, 0);
            return false;
        }

        let frame = 70224;
        let mode0 = 203;
        let mode2 = 80;
        let mode3 = 173;

        self.frame_step += ticks as u32;
        if self.frame_step >= frame {
            self.frame_step -= frame;
        }

        let scan_line_clk = self.frame_step % 456;

        let scan_line = gb.memory.get_byte(LCDC_Y_COORD);
        let mut next_scan_line = ((self.frame_step + mode0) / 456) as u8;
        if next_scan_line > 153 {
            next_scan_line = 0;
        }

        let mut interrupt_flags = gb.memory.get_byte(0xFF0F);

        let mode = if self.frame_step >= 65664 {
            MODE1_VBLANK
        } else if scan_line_clk < mode2 {
            MODE2_ACCESSING_OAM
        } else if scan_line_clk < mode2 + mode3 {
            MODE3_ACCESSING_VRAM
        } else {
            MODE0_HBLANK
        };

        if prev_mode == MODE3_ACCESSING_VRAM && mode == MODE0_HBLANK {
            if status & LCD_STATUS_MODE0_INT == LCD_STATUS_MODE0_INT {
                interrupt_flags |= 0b10;
            }
            self.draw_scan_line(gb, scan_line);
        }

        let mut coincidence_flag = status & LCD_STATUS_COINCIDENCE;
        if scan_line != next_scan_line {
            gb.memory.set_owned_byte(LCDC_Y_COORD, next_scan_line);
            let ly_compare = gb.memory.get_byte(LY_COMPARE);
            if ly_compare == next_scan_line {
                coincidence_flag = LCD_STATUS_COINCIDENCE;
                if status & LCD_STATUS_COINCIDENCE_INT == LCD_STATUS_COINCIDENCE_INT {
                    interrupt_flags |= 0b10;
                }
            } else {
                coincidence_flag = 0;
            }
        }

        let ready_for_render = if prev_mode == MODE0_HBLANK && mode == MODE1_VBLANK {
            interrupt_flags |= 0b1;
            if status & LCD_STATUS_MODE1_INT == LCD_STATUS_MODE1_INT {
                interrupt_flags |= 0b10;
            }
            true
        } else {
            false
        };

        if prev_mode == MODE1_VBLANK
            && mode == MODE2_ACCESSING_OAM
            && status & LCD_STATUS_MODE2_INT == LCD_STATUS_MODE2_INT
        {
            interrupt_flags |= 0b10;
        }

        gb.memory.set_owned_byte(0xFF0F, interrupt_flags);
        gb.memory.set_owned_byte(
            LCDC_STATUS_REG,
            (status & LCD_STATUS_FLAG_MASK) | mode | coincidence_flag,
        );

        ready_for_render
    }

    pub fn draw_scan_line(&mut self, gb: &GameBoy, scan_line: u8) {
        let window_y = (scan_line as i16) - (window_y_offset(gb) as i16);

        let bg_palette = bg_palette(gb);

        let scroll_y = gb.memory.get_byte(SCROLL_Y_REG) as u16;
        let scroll_x = gb.memory.get_byte(SCROLL_X_REG) as u16;

        let mut y_bg = scroll_y + (scan_line as u16);
        if y_bg > 255 {
            y_bg -= 256;
        }

        let sprites = get_sprites_in_scan_line(gb, scan_line);
        for x in 0..HORIZONTAL_RES {
            let mut draw_bg = true;
            let window_x = (x as i16) - (window_x_offset(gb) as i16) + 7;
            if window_enabled(gb) && window_y >= 0 && window_x >= 0 {
                let window_palette_index = get_tile_map_palette_index(
                    gb,
                    window_tile_map(gb),
                    window_x as u16,
                    window_y as u16,
                );
                let window_color_id = get_palette_color(bg_palette, window_palette_index);
                self.set_pixel(x, scan_line, window_color_id);
                draw_bg = false;
            }

            let bg_palette_index = if bg_enabled(gb) {
                let mut x_bg = (x as u16) + scroll_x;
                if x_bg > 255 {
                    x_bg -= 256;
                }
                get_tile_map_palette_index(gb, bg_tile_map(gb), x_bg, y_bg)
            } else {
                0 //white
            };

            for i in 0..sprites.len() {
                let sprite = &sprites[i];
                if sprite.left() <= (x as i16)
                    && sprite.right() > (x as i16)
                    && (sprite.above_bg() || bg_palette_index == 0)
                {
                    let sprite_pattern = sprite.get_tile_pattern(gb, scan_line);
                    let sprite_x = if sprite.is_mirrored_horizontally() {
                        (((x as i16) - sprite.left()) % 8) as u8
                    } else {
                        (7 - (((x as i16) - sprite.left()) % 8)) as u8
                    };
                    let sprite_palette_index = get_palette_index(sprite_pattern, sprite_x);
                    if sprite_palette_index != 0 {
                        let sprite_palette = sprite.get_palette(gb);
                        let sprite_color_id =
                            get_palette_color(sprite_palette, sprite_palette_index);
                        self.set_pixel(x, scan_line, sprite_color_id);
                        draw_bg = false;
                        break;
                    }
                }
            }

            if draw_bg {
                let bg_color_id = get_palette_color(bg_palette, bg_palette_index);
                self.set_pixel(x, scan_line, bg_color_id);
            }
        }
    }

    fn set_pixel(&mut self, x: u8, y: u8, color: u8) {
        let pixel_index = (y as usize * HORIZONTAL_RES as usize) + x as usize;
        let buffer_index = pixel_index / 4;
        let (mask, color) = match pixel_index % 4 {
            0 => (0b11000000u8, color << 6),
            1 => (0b00110000u8, color << 4),
            2 => (0b00001100u8, color << 2),
            3 => (0b00000011u8, color),
            _ => panic!("NUM % 4 was not in range 0..=3"),
        };

        let value = &mut self.window_buf[buffer_index];
        *value = (*value & !mask) | color;
    }
}

fn get_sprites_in_scan_line(gb: &GameBoy, scan_line: u8) -> Vec<Sprite> {
    let sprite_size = sprite_size(gb);
    let mut sprites: Vec<Sprite> = vec![];
    for i_sprite in 0..40 {
        let sprite = Sprite::new(gb, i_sprite, sprite_size);
        if sprite.top() <= (scan_line as i16) && sprite.bottom() > (scan_line as i16) {
            let mut insertion_index = sprites.len();
            for (i, existing_sprite) in sprites.iter().enumerate() {
                if existing_sprite.left() > sprite.left()
                    || (existing_sprite.left() == sprite.left()
                        && sprite.index() > existing_sprite.index())
                {
                    insertion_index = i;
                    break;
                }
            }
            sprites.insert(insertion_index, sprite);
        }
    }
    sprites.truncate(10);
    sprites
}

fn get_palette_index(pattern: u16, x: u8) -> u8 {
    (((pattern >> x) & 0b1) | ((pattern >> (x + 7)) & 0b10)) as u8
}

fn get_tile_map_palette_index(gb: &GameBoy, map_id: bool, x: u16, y: u16) -> u8 {
    let tile_map_addr = if map_id { 0x9C00 } else { 0x9800 };

    let tile_index = ((y as u16) / 8 * 32) + ((x / 8) as u16);
    let tile_pattern_index = gb.memory.get_byte(tile_map_addr + tile_index);
    let base_tile_pattern_addr = get_bg_tile_addr(gb, tile_pattern_index);

    let pattern_y = (y % 8) as u16;
    let tile_pattern = gb.memory.get_word(base_tile_pattern_addr + (pattern_y * 2));

    get_palette_index(tile_pattern, (7 - (x % 8)) as u8)
}

fn display_enabled(gb: &GameBoy) -> bool {
    gb.memory.get_byte(LCD_CONTROL_REG) & 0x80 == 0x80
}

// fn window_tile_map(gb: &GameBoy) -> u8 {
//     gb.memory.get_byte(LCD_CONTROL_REG) >> 6 &0b1
// }
fn bg_enabled(gb: &GameBoy) -> bool {
    gb.memory.get_byte(LCD_CONTROL_REG) & 0b1 == 0b1
}

fn bg_tile_map(gb: &GameBoy) -> bool {
    gb.memory.get_byte(LCD_CONTROL_REG) >> 3 == 0b1
}

fn window_enabled(gb: &GameBoy) -> bool {
    gb.memory.get_byte(LCD_CONTROL_REG) & 0x20 == 0x20
}

fn window_x_offset(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(WINDOW_X_REG)
}

fn window_y_offset(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(WINDOW_Y_REG)
}

fn window_tile_map(gb: &GameBoy) -> bool {
    gb.memory.get_byte(LCD_CONTROL_REG) & 0x40 == 0x40
}

fn bg_palette(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF47)
}

fn tile_data(gb: &GameBoy) -> u8 {
    gb.memory.get_byte(0xFF40) >> 4 & 0b1
}

fn sprite_size(gb: &GameBoy) -> u8 {
    if gb.memory.get_byte(0xFF40) & 0b100 == 0b100 {
        16
    } else {
        8
    }
}

fn get_sprite_tile_addr(tile_index: u8) -> u16 {
    0x8000 + ((tile_index as u16) * 16)
}

fn get_bg_tile_addr(gb: &GameBoy, tile_index: u8) -> u16 {
    if tile_data(gb) == 1 {
        get_sprite_tile_addr(tile_index)
    } else {
        let signed_index = tile_index as i8;
        (0x9000i32 + ((signed_index as i32) * 16)) as u16
    }
}

fn get_palette_color(palette: u8, index: u8) -> u8 {
    if index > 3 {
        panic!("Invalid palette id");
    }

    palette >> (index << 1) & 0b11
}
