use volatile::Volatile;
use core::fmt;
use x86_64::instructions::interrupts;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt::Write;

use crate::renderer::text::CHARACTERS;

const BUFFER_WIDTH: usize = 320;
const BUFFER_HEIGHT: usize = 200;

#[repr(transparent)]
struct Buffer {
    pixels: [Volatile<u8>; BUFFER_WIDTH * BUFFER_HEIGHT],
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::window::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        SCREEN_WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn clear_screen() {
    for _ in 0..100 {
        println!("");
    }
}

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub struct ScreenWriter {
    buffer: &'static mut Buffer,
    frames: [(i32, i32, i32, i32); 4],
    terminal_column_position: usize,
    terminal_character_buffer: [[u8; 26]; 19]
}
impl ScreenWriter {
    #[allow(dead_code)]
    fn get_rgb(&self, r: u8, g: u8, b: u8) -> u8 {
        let mut closest_color: (i16, usize) = (-1, 999999);
    
        for color in COLOR_PALETTE.iter().enumerate() {
            let dr = r as isize - color.1.0 as isize;
            let dg = g as isize - color.1.1 as isize;
            let db = b as isize - color.1.2 as isize;
            let color_distance = (dr * dr + dg * dg + db * db) as usize;
    
            if color_distance < closest_color.1 {
                closest_color = (color.0 as i16, color_distance);
                if color_distance < 100 {
                    return closest_color.0 as u8
                }
            }
        }
    
        closest_color.0 as u8
    }
    
    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        x + y * BUFFER_WIDTH
    }
    
    fn draw_character(&mut self, character: u8, x: usize, y: usize) {
        let characters = CHARACTERS[character as usize];
    
        for char in characters.iter().enumerate() {
            if char.1 == &true {
                self.buffer.pixels[self.get_pixel_index(x + char.0 % 5, y + char.0 / 5)].write(15);
            } else {
                self.buffer.pixels[self.get_pixel_index(x + char.0 % 5, y + char.0 / 5)].write(0);
            }
        }
    }
    
    fn clear_characters(&mut self, line: usize) {
        for i in 0..25 {
            self.draw_character(0, 2 + i * 6, 183 - 10 * line);
            self.terminal_character_buffer[line][i] = 0;
        }
    }
    
    fn shift_characters(&mut self) {
        for line in 1..19 {
            for i in 0..25 {
                let character = self.terminal_character_buffer[18 - line][i];
                self.terminal_character_buffer[19 - line][i] = character;
                self.draw_character(character, 2 + i * 6, 183 - 10 * (19 - line));
                //draw_character(buffer, 100, 9 + i * 6, 183 - 10 * (line + 1));
            }
        }
        self.clear_characters(0);
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        self.buffer.pixels[self.get_pixel_index(x, y)].write(color);
    }

    pub fn write_string(&mut self, s: &str) {
        for char in s.bytes() {
            let mut char_writing = char;
            if char == b'\n' {
                self.shift_characters();
                self.terminal_column_position = 0;
                continue;
            }
            if self.terminal_column_position == 24 {
                self.shift_characters();
                self.terminal_column_position = 0;
            }
            if char_writing >= CHARACTERS.len() as u8 {
                char_writing = 0;
            }
            self.draw_character(char_writing,  2 + self.terminal_column_position * 6, 183);
            self.terminal_character_buffer[0][self.terminal_column_position] = char_writing;
            self.terminal_column_position += 1;
        }
    }
}

lazy_static! {
    pub static ref SCREEN_WRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
        buffer: unsafe { &mut *(0xa0000 as *mut Buffer) },
        frames: [(0, 0, 160, 100); 4],
        terminal_column_position: 0,
        terminal_character_buffer: [[0; 26]; 19]
    });
}

pub fn init() {
    let mut screen_writer = SCREEN_WRITER.lock();

    let background_color = screen_writer.get_rgb(0, 0, 0);
    for x in 0..BUFFER_WIDTH {
        for y in 0..BUFFER_HEIGHT {
            if x > 160 {
                screen_writer.set_pixel(x, y, 215);
            } else {
                screen_writer.set_pixel(x, y, background_color);
            }
        }
    }

    let terminal_line = "hello world\nthis is a line of text\ngood day yall\nblack jack is overrated\ni will give you a medal\npot dor dot\ni have a question\nzen browser";

    screen_writer.write_string(terminal_line);
}

// yes I did do this myself, I know there might be a lookup table out there, but I decided to take the tedious route, why else would I make an OS
const COLOR_PALETTE: [(u8, u8, u8); 216] = [
    (0, 0, 0),
    (0, 0, 170),
    (0, 170, 0),
    (0, 170, 170),
    (170, 0, 0),
    (170, 0, 170),
    (170, 87, 0),
    (170, 170, 170),
    (87, 87, 87),
    (87, 87, 255),
    (87, 255, 87),
    (255, 87, 87),
    (255, 87, 255),
    (255, 255, 87),
    (255, 255, 255),
    (0, 0, 0),
    (23, 23, 23),
    (32, 32, 32),
    (47, 47, 47),
    (56, 56, 56),
    (71, 71, 71),
    (80, 80, 80),
    (96, 96, 96),
    (112, 112, 112),
    (128, 128, 128),
    (144, 144, 144),
    (160, 160, 160),
    (183, 183, 183),
    (200, 200, 200),
    (224, 224, 224),
    (255, 255, 255),
    (0, 0, 255),
    (64, 0, 255),
    (127, 0, 255),
    (191, 0, 255),
    (255, 0, 255),
    (255, 0, 191),
    (255, 0, 127),
    (255, 0, 64),
    (255, 0, 0),
    (255, 64, 0),
    (255, 127, 0),
    (255, 191, 0),
    (255, 255, 0),
    (191, 255, 0),
    (127, 255, 0),
    (64, 255, 0),
    (0, 255, 0),
    (0, 255, 64),
    (0, 255, 127),
    (0, 255, 191),
    (0, 255, 255),
    (0, 191, 255),
    (0, 127, 255),
    (0, 64, 255),
    (127, 127, 255),
    (159, 127, 255),
    (191, 127, 255),
    (223, 127, 255),
    (255, 127, 255),
    (255, 127, 223),
    (255, 127, 191),
    (255, 127, 159),
    (255, 127, 127),
    (255, 159, 127),
    (255, 191, 127),
    (255, 223, 127),
    (255, 255, 127),
    (223, 255, 127),
    (191, 255, 127),
    (159, 255, 127),
    (127, 255, 127),
    (127, 255, 159),
    (127, 255, 191),
    (127, 255, 223),
    (127, 255, 255),
    (127, 223, 255),
    (127, 191, 255),
    (127, 159, 255),
    (183, 183, 255),
    (199, 183, 255),
    (216, 183, 255),
    (232, 183, 255),
    (255, 183, 255),
    (255, 183, 232),
    (255, 183, 216),
    (255, 183, 199),
    (255, 183, 183),
    (255, 199, 183),
    (255, 216, 183),
    (255, 232, 183),
    (255, 255, 183),
    (232, 255, 183),
    (216, 255, 183),
    (199, 255, 183),
    (183, 255, 183),
    (183, 255, 199),
    (183, 255, 216),
    (183, 255, 232),
    (183, 255, 255),
    (183, 232, 255),
    (183, 216, 255),
    (183, 199, 255),
    (0, 0, 112),
    (31, 0, 112),
    (56, 0, 112),
    (87, 0, 112),
    (112, 0, 112),
    (112, 0, 87),
    (112, 0, 56),
    (112, 0, 31),
    (112, 0, 0),
    (112, 31, 0),
    (112, 56, 0),
    (112, 87, 0),
    (112, 112, 0),
    (87, 112, 0),
    (56, 112, 0),
    (31, 112, 0),
    (0, 112, 0),
    (0, 112, 31),
    (0, 112, 56),
    (0, 112, 87),
    (0, 112, 112),
    (0, 87, 112),
    (0, 56, 112),
    (0, 31, 112),
    (56, 56, 112),
    (71, 56, 112),
    (87, 56, 112),
    (96, 56, 112),
    (112, 56, 112),
    (112, 56, 96),
    (112, 56, 87),
    (112, 56, 71),
    (112, 56, 56),
    (112, 71, 56),
    (112, 87, 56),
    (112, 96, 56),
    (112, 112, 56),
    (96, 112, 56),
    (87, 112, 56),
    (71, 112, 56),
    (56, 112, 56),
    (56, 112, 71),
    (56, 112, 87),
    (56, 112, 96),
    (56, 112, 112),
    (56, 96, 112),
    (56, 87, 112),
    (56, 71, 112),
    (80, 80, 112),
    (88, 80, 112),
    (96, 80, 112),
    (104, 80, 112),
    (112, 80, 112),
    (112, 80, 104),
    (112, 80, 96),
    (112, 80, 88),
    (112, 80, 80),
    (112, 88, 80),
    (112, 96, 80),
    (112, 104, 80),
    (112, 112, 80),
    (104, 112, 80),
    (96, 112, 80),
    (88, 112, 80),
    (80, 112, 80),
    (80, 112, 88),
    (80, 112, 96),
    (80, 112, 104),
    (80, 112, 112),
    (80, 104, 112),
    (80, 96, 112),
    (80, 88, 112),
    (0, 0, 64),
    (16, 0, 64),
    (32, 0, 64),
    (48, 0, 64),
    (64, 0, 64),
    (64, 0, 48),
    (64, 0, 32),
    (64, 0, 16),
    (64, 0, 0),
    (64, 16, 0),
    (64, 32, 0),
    (64, 48, 0),
    (64, 64, 0),
    (48, 64, 0),
    (32, 64, 0),
    (16, 64, 0),
    (0, 64, 0),
    (0, 64, 16),
    (0, 64, 32),
    (0, 64, 48),
    (0, 64, 64),
    (0, 48, 64),
    (0, 32, 64),
    (0, 16, 64),
    (32, 32, 64),
    (40, 32, 64),
    (48, 32, 64),
    (56, 32, 64),
    (64, 32, 64),
    (64, 32, 56),
    (64, 32, 48),
    (64, 32, 40),
    (64, 32, 32),
    (64, 40, 32),
    (64, 48, 32),
    (64, 56, 32),
    (64, 64, 32),
    (56, 64, 32),
    (48, 64, 32),
    (40, 64, 32),
    (32, 64, 32),
];