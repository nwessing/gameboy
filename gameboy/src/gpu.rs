use crate::game_boy::GameBoy;
use crate::memory::Register;

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

#[derive(Copy, Clone)]
enum TileAddressingMode {
    Unsigned,
    Signed,
}

pub struct Gpu {
    pub window_buf: Box<[u8; BUFFER_SIZE]>,
    frame_step: u32,
    pub total_render_ns: i128,
    pub scan_lines_rendered: u64,
    sprites: [Sprite; 40],
    sprite_order: [usize; 10],
}

#[derive(Debug, Copy, Clone)]
struct Sprite {
    y_pos: i16,
    x_pos: i16,
    tile_pattern_index: u8,
    attributes: u8,
    index: u8,
    height: i16,
    // on_scan_line: bool,
    pattern: u16,
}

impl Sprite {
    fn new(sprite_index: u8) -> Sprite {
        Sprite {
            index: sprite_index,
            y_pos: 0,
            x_pos: 0,
            tile_pattern_index: 0,
            attributes: 0,
            height: 0,
            // on_scan_line: false,
            pattern: 0,
        }
    }

    pub fn update(&mut self, gb: &GameBoy, height: u8) {
        let data = gb.memory.read_sprite(self.index);
        self.y_pos = data.y_pos as i16 - 16;
        self.x_pos = data.x_pos as i16 - 8;
        self.tile_pattern_index = data.tile_number;
        self.attributes = data.attributes;
        self.height = height as i16;
        // self.on_scan_line = on_scan_line;
    }

    pub fn retrieve_tile_pattern(&mut self, gb: &GameBoy, scan_line: u8) {
        let sprite_y = if self.is_mirrored_vertically() {
            ((self.height - 1) - ((scan_line as i16) - self.top()) % self.height) as u16
        } else {
            (((scan_line as i16) - self.top()) % self.height) as u16
        };

        let tile_pattern_addr = get_sprite_tile_addr(self.tile_pattern_index);
        self.pattern = gb.memory.get_word(tile_pattern_addr + (sprite_y * 2));
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

    pub fn get_tile_pattern(&self) -> u16 {
        self.pattern
    }

    fn get_palette(&self) -> bool {
        self.attributes & 0x10 == 0x10
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
        let sprites = {
            let mut data: [std::mem::MaybeUninit<Sprite>; 40] =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };

            for sprite_index in 0u8..40u8 {
                data[sprite_index as usize] = std::mem::MaybeUninit::new(Sprite::new(sprite_index));
            }

            unsafe { std::mem::transmute::<_, [Sprite; 40]>(data) }
        };

        let sprite_order = {
            let mut data: [std::mem::MaybeUninit<usize>; 10] =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };

            for i in 0..10 {
                data[i] = std::mem::MaybeUninit::new(0usize);
            }

            unsafe { std::mem::transmute::<_, [usize; 10]>(data) }
        };

        Gpu {
            window_buf,
            frame_step: 0,
            scan_lines_rendered: 0,
            total_render_ns: 0,
            sprites,
            sprite_order,
        }
    }

    /// Updates GPU state and returns whether the frame buffer has a completed
    /// frame
    pub fn update(&mut self, gb: &mut GameBoy, framebuffer: &mut [u8], ticks: u8) -> bool {
        let status = gb.memory.get_register(Register::LcdcStatus);
        let prev_mode = status & 0b0000_0011;

        if !display_enabled(gb) {
            if prev_mode != MODE1_VBLANK {
                // println!("LCD turned off outside of VBLANK, this should not happen.");
            }
            gb.memory.set_register(Register::LcdcYCoord, 0);
            return false;
        }

        let frame = 70224;
        let mode0 = 203;
        let mode2 = 80;
        let mode3 = 173;

        let mut ready_for_render = false;
        self.frame_step += ticks as u32;
        if self.frame_step >= frame {
            self.frame_step -= frame;
            ready_for_render = true;
        }

        let scan_line_clk = self.frame_step % 456;

        let scan_line = gb.memory.get_register(Register::LcdcYCoord);
        let mut next_scan_line = ((self.frame_step + mode0) / 456) as u8;
        if next_scan_line > 153 {
            next_scan_line = 0;
        }

        let mut interrupt_flags = gb.memory.get_register(Register::InterruptFlag);

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
            self.draw_scan_line(gb, framebuffer, scan_line);
        }

        let mut coincidence_flag = status & LCD_STATUS_COINCIDENCE;
        if scan_line != next_scan_line {
            gb.memory.set_register(Register::LcdcYCoord, next_scan_line);
            let ly_compare = gb.memory.get_register(Register::LyCompare);
            if ly_compare == next_scan_line {
                coincidence_flag = LCD_STATUS_COINCIDENCE;
                if status & LCD_STATUS_COINCIDENCE_INT == LCD_STATUS_COINCIDENCE_INT {
                    interrupt_flags |= 0b10;
                }
            } else {
                coincidence_flag = 0;
            }
        }

        if prev_mode == MODE0_HBLANK && mode == MODE1_VBLANK {
            interrupt_flags |= 0b1;
            if status & LCD_STATUS_MODE1_INT == LCD_STATUS_MODE1_INT {
                interrupt_flags |= 0b10;
            }
        }

        if prev_mode == MODE1_VBLANK
            && mode == MODE2_ACCESSING_OAM
            && status & LCD_STATUS_MODE2_INT == LCD_STATUS_MODE2_INT
        {
            interrupt_flags |= 0b10;
        }

        gb.memory
            .set_register(Register::InterruptFlag, interrupt_flags);
        gb.memory.set_register(
            Register::LcdcStatus,
            (status & LCD_STATUS_FLAG_MASK) | mode | coincidence_flag,
        );

        ready_for_render
    }

    pub fn draw_scan_line(&mut self, gb: &GameBoy, framebuffer: &mut [u8], scan_line: u8) {
        let start = time::Instant::now();
        let window_y = (scan_line as i16) - (window_y_offset(gb) as i16);

        let bg_palette = bg_palette(gb);

        let scroll_y = gb.memory.get_register(Register::ScrollY) as u16;
        let scroll_x = gb.memory.get_register(Register::ScrollX) as u16;

        let mut y_bg = scroll_y + (scan_line as u16);
        if y_bg > 255 {
            y_bg -= 256;
        }

        let sprite_count =
            get_sprites_in_scan_line(gb, &mut self.sprites, &mut self.sprite_order, scan_line);
        let is_window_enabled = window_enabled(gb);
        let window_map_id = window_tile_map(gb);
        let window_x_offset = window_x_offset(gb) as i16;
        let is_bg_enabled = bg_enabled(gb);
        let bg_map_id = bg_tile_map(gb);
        let sprite_palette1 = gb.memory.get_register(Register::ObjectPalette1Data);
        let sprite_palette0 = gb.memory.get_register(Register::ObjectPalette0Data);
        let tile_addressing_mode = if tile_data(gb) == 1 {
            TileAddressingMode::Unsigned
        } else {
            TileAddressingMode::Signed
        };
        let mut current_bg_tile_index = 0u16;
        let mut current_bg_tile_pattern = 0u16;
        let mut current_window_tile_index = 0u16;
        let mut current_window_tile_pattern = 0u16;
        let mut start_sprite_index = 0usize;

        for x in 0..HORIZONTAL_RES {
            let mut draw_bg = true;
            let window_x = (x as i16) - window_x_offset + 7;
            if is_window_enabled && window_y >= 0 && window_x >= 0 {
                let window_x = window_x as u16;
                let window_y = window_y as u16;
                let next_window_tile_index = get_tile_index(window_x, window_y);
                if x == 0 || current_window_tile_index != next_window_tile_index {
                    current_window_tile_index = next_window_tile_index;
                    current_window_tile_pattern = get_tile_pattern(
                        gb,
                        window_map_id,
                        tile_addressing_mode,
                        current_window_tile_index,
                        window_y,
                    );
                }

                let window_palette_index =
                    get_palette_index(current_window_tile_pattern, (7 - (window_x % 8)) as u8);
                let window_color_id = get_palette_color(bg_palette, window_palette_index);
                set_pixel(framebuffer, x, scan_line, window_color_id);
                draw_bg = false;
            }

            let bg_palette_index = if is_bg_enabled {
                let mut x_bg = (x as u16) + scroll_x;
                if x_bg > 255 {
                    x_bg -= 256;
                }

                let next_bg_tile_index = get_tile_index(x_bg, y_bg);
                if x == 0 || current_bg_tile_index != get_tile_index(x_bg, y_bg) {
                    current_bg_tile_index = next_bg_tile_index;
                    current_bg_tile_pattern = get_tile_pattern(
                        gb,
                        bg_map_id,
                        tile_addressing_mode,
                        current_bg_tile_index,
                        y_bg,
                    );
                }
                get_palette_index(current_bg_tile_pattern, (7 - (x_bg % 8)) as u8)
            } else {
                0 //white
            };

            // Only first 10 sprites are rendered
            for i in start_sprite_index..sprite_count {
                let sprite = &self.sprites[self.sprite_order[i]];

                if sprite.left() > x as i16 {
                    break;
                }

                if sprite.right() <= x as i16 {
                    start_sprite_index = i + 1;
                    continue;
                }

                if sprite.left() <= (x as i16)
                    && sprite.right() > (x as i16)
                    && (sprite.above_bg() || bg_palette_index == 0)
                {
                    let sprite_pattern = sprite.get_tile_pattern();
                    let sprite_x = if sprite.is_mirrored_horizontally() {
                        (((x as i16) - sprite.left()) % 8) as u8
                    } else {
                        (7 - (((x as i16) - sprite.left()) % 8)) as u8
                    };
                    let sprite_palette_index = get_palette_index(sprite_pattern, sprite_x);
                    if sprite_palette_index != 0 {
                        let sprite_palette = if sprite.get_palette() {
                            sprite_palette1
                        } else {
                            sprite_palette0
                        };
                        let sprite_color_id =
                            get_palette_color(sprite_palette, sprite_palette_index);
                        set_pixel(framebuffer, x, scan_line, sprite_color_id);
                        draw_bg = false;
                        break;
                    }
                }
            }

            if draw_bg {
                let bg_color_id = get_palette_color(bg_palette, bg_palette_index);
                set_pixel(framebuffer, x, scan_line, bg_color_id);
            }
        }

        self.total_render_ns += start.elapsed().whole_nanoseconds();
        self.scan_lines_rendered += 1;
    }
}

fn set_pixel(framebuffer: &mut [u8], x: u8, y: u8, color_id: u8) {
    let color = get_color(color_id);
    let pixel_index = 4 * ((y as usize * HORIZONTAL_RES as usize) + x as usize);
    framebuffer[pixel_index + 0] = color;
    framebuffer[pixel_index + 1] = color;
    framebuffer[pixel_index + 2] = color;
    framebuffer[pixel_index + 3] = 255u8;
}

fn get_color(color_id: u8) -> u8 {
    match color_id {
        3 => 0u8,
        2 => 96u8,
        1 => 192u8,
        _ => 255u8,
    }
}

fn get_sprites_in_scan_line(
    gb: &GameBoy,
    sprites: &mut [Sprite; 40],
    order: &mut [usize; 10],
    scan_line: u8,
) -> usize {
    let sprite_size = sprite_size(gb);
    let mut sprite_count = 0usize;

    for i_sprite in 0..sprites.len() {
        sprites[i_sprite].update(gb, sprite_size);

        let sprite = &sprites[i_sprite];
        let on_scan_line =
            sprite.top() <= (scan_line as i16) && sprite.bottom() > (scan_line as i16);

        if !on_scan_line {
            continue;
        }

        let mut inserted = false;
        for i_sort in 0..sprite_count {
            let i_other_sprite = order[i_sort];
            let other_sprite = &sprites[i_other_sprite];
            if sprite.x_pos < other_sprite.x_pos {
                inserted = true;
                sprite_count += 1;

                for j in ((i_sort + 1)..sprite_count).rev() {
                    order[j] = order[j - 1];
                }
                order[i_sort] = i_sprite;
                break;
            }
        }

        if !inserted {
            order[sprite_count] = i_sprite;
            sprite_count += 1;
        }

        if sprite_count == 10 {
            break;
        }
    }

    for i in 0..sprite_count {
        sprites[order[i]].retrieve_tile_pattern(gb, scan_line);
    }

    return sprite_count;
}

fn get_palette_index(pattern: u16, x: u8) -> u8 {
    (((pattern >> x) & 0b1) | ((pattern >> (x + 7)) & 0b10)) as u8
}

fn get_tile_index(x: u16, y: u16) -> u16 {
    ((y as u16) / 8 * 32) + ((x / 8) as u16)
}

fn get_tile_pattern(
    gb: &GameBoy,
    map_id: bool,
    mode: TileAddressingMode,
    tile_index: u16,
    y: u16,
) -> u16 {
    let tile_map_addr = if map_id { 0x9C00 } else { 0x9800 };
    let tile_pattern_index = gb.memory.get_byte(tile_map_addr + tile_index);
    let base_tile_pattern_addr = get_bg_tile_addr(mode, tile_pattern_index);
    let pattern_y = (y % 8) as u16;
    let tile_pattern = gb.memory.get_word(base_tile_pattern_addr + (pattern_y * 2));
    tile_pattern
}

fn display_enabled(gb: &GameBoy) -> bool {
    gb.memory.get_register(Register::LcdControl) & 0x80 == 0x80
}

fn bg_enabled(gb: &GameBoy) -> bool {
    gb.memory.get_register(Register::LcdControl) & 0b1 == 0b1
}

fn bg_tile_map(gb: &GameBoy) -> bool {
    gb.memory.get_register(Register::LcdControl) >> 3 == 0b1
}

fn window_enabled(gb: &GameBoy) -> bool {
    gb.memory.get_register(Register::LcdControl) & 0x20 == 0x20
}

fn window_x_offset(gb: &GameBoy) -> u8 {
    gb.memory.get_register(Register::WindowX)
}

fn window_y_offset(gb: &GameBoy) -> u8 {
    gb.memory.get_register(Register::WindowY)
}

fn window_tile_map(gb: &GameBoy) -> bool {
    gb.memory.get_register(Register::LcdControl) & 0x40 == 0x40
}

fn bg_palette(gb: &GameBoy) -> u8 {
    gb.memory.get_register(Register::BackgroundPaletteData)
}

fn tile_data(gb: &GameBoy) -> u8 {
    gb.memory.get_register(Register::LcdControl) >> 4 & 0b1
}

fn sprite_size(gb: &GameBoy) -> u8 {
    if gb.memory.get_register(Register::LcdControl) & 0b100 == 0b100 {
        16
    } else {
        8
    }
}

fn get_sprite_tile_addr(tile_index: u8) -> u16 {
    0x8000 + ((tile_index as u16) * 16)
}

fn get_bg_tile_addr(mode: TileAddressingMode, tile_index: u8) -> u16 {
    match mode {
        TileAddressingMode::Unsigned => get_sprite_tile_addr(tile_index),
        TileAddressingMode::Signed => {
            let signed_index = tile_index as i8;
            (0x9000i32 + ((signed_index as i32) * 16)) as u16
        }
    }
}

fn get_palette_color(palette: u8, index: u8) -> u8 {
    palette >> (index << 1) & 0b11
}
