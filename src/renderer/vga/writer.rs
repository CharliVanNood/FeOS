use crate::renderer::{colors, text::CHARACTERS};
use volatile::Volatile;

pub const BUFFER_WIDTH: usize = 320;
pub const BUFFER_HEIGHT: usize = 200;

#[repr(transparent)]
struct Buffer {
    pixels: [Volatile<u8>; BUFFER_WIDTH * BUFFER_HEIGHT],
}

pub struct ScreenWriter {
    buffer: &'static mut Buffer,
    screen_buffer: [u8; BUFFER_WIDTH * BUFFER_HEIGHT],
    frame: u8, // the current frame being written to: terminal / application / status bar
    clock_column_position: usize,
    ram_column_position: usize,
    terminal_column_position: usize,
    terminal_character_buffer: [[(u8, u8, u8); 27]; 19],
    terminal_background_color: u8,
    terminal_foreground_color: u8,
    clock_background_color: u8,
    clock_foreground_color: u8,
    ram_background_color: u8,
    ram_foreground_color: u8,
    status_bar_background_color: u8,
    status_bar_foreground_color: u8,
    frame_time: usize // this keeps track of what frame we're on

}
impl ScreenWriter {
    pub fn init() -> Self {
        Self {
            buffer: unsafe { &mut *(0xa0000 as *mut Buffer) },
            frame: 0,
            screen_buffer: [0; BUFFER_WIDTH * BUFFER_HEIGHT],
            clock_column_position: 0,
            ram_column_position: 0,
            terminal_column_position: 0,
            terminal_character_buffer: [[(0, 15, 0); 27]; 19],
            terminal_background_color: 0,
            terminal_foreground_color: 15,
            clock_background_color: 17,
            clock_foreground_color: 15,
            ram_background_color: 17,
            ram_foreground_color: 15,
            status_bar_background_color: 17,
            status_bar_foreground_color: 15,
            frame_time: 0
        }
    }

    pub fn increment_frame(&mut self) {
        self.frame_time += 1;
    }
    pub fn get_frame(&self) -> usize {
        self.frame_time
    }

    pub fn get_terminal_colors(&self) -> (u8, u8) {
        (self.terminal_foreground_color, self.terminal_background_color)
    }
    pub fn get_status_bar_colors(&self) -> (u8, u8) {
        (self.status_bar_foreground_color, self.status_bar_background_color)
    }

    pub fn get_screen_size(&self) -> (usize, usize) {
        (BUFFER_WIDTH, BUFFER_HEIGHT)
    }

    pub fn get_rgb(&self, r: u8, g: u8, b: u8) -> u8 {
        let mut closest_color: (i16, usize) = (-1, 999999);
    
        for color in colors::COLOR_PALETTE.iter().enumerate() {
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

    pub fn set_frame(&mut self, frame: u8) {
        self.frame = frame;
    }

    pub fn set_color(&mut self, foreground: u8, background: u8, frame: u8) {
        match frame {
            0 => {
                self.terminal_background_color = background;
                self.terminal_foreground_color = foreground;
            },
            1 => {
                self.clock_background_color = background;
                self.clock_foreground_color = foreground;
            },
            2 => {
                self.ram_background_color = background;
                self.ram_foreground_color = foreground;
            },
            3 => {
                self.clock_background_color = background;
                self.ram_background_color = background;
                self.status_bar_background_color = background;
            },
            _ => {
                self.terminal_background_color = background;
                self.terminal_foreground_color = foreground;
            }
        }
    }
    
    pub fn draw_character(&mut self, character: u8, x: usize, y: usize, foreground: u8, background: u8) {
        let characters = CHARACTERS[character as usize];
    
        for char in characters.iter().enumerate() {
            if char.1 == &true {
                self.set_pixel(x + char.0 % 5, y + char.0 / 5, foreground);
            } else {
                self.set_pixel(x + char.0 % 5, y + char.0 / 5, background);
            }
        }
    }
    
    fn clear_characters(&mut self, line: usize) {
        for i in 0..26 {
            self.draw_character(0, 2 + i * 6, 183 - 10 * line, 15, 0);
            self.terminal_character_buffer[line][i] = (0, 15, 0);
        }
    }
    
    fn shift_characters(&mut self) {
        for line in 1..19 {
            for i in 0..26 {
                let character = self.terminal_character_buffer[18 - line][i];
                self.terminal_character_buffer[19 - line][i] = character;
                self.draw_character(character.0, 2 + i * 6, 183 - 10 * (19 - line), character.1, character.2);
            }
        }
        self.clear_characters(0);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        let pixel_index = self.get_pixel_index(x, y);
        self.buffer.pixels[pixel_index].write(color);
        self.screen_buffer[pixel_index] = color;
    }

    pub fn set_rect(&mut self, x: usize, y: usize, size_x: usize, size_y: usize, color: u8) {
        for offset_x in 0..size_x {
            for offset_y in 0..size_y {
                if self.get_pixel(x + offset_x, y + offset_y) == color { continue; }
                self.set_pixel(x + offset_x, y + offset_y, color);
            }
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        let pixel_index = self.get_pixel_index(x, y);
        self.screen_buffer[pixel_index]
    }

    fn draw_terminal_character(&mut self, char: u8) {
        let mut char_writing = char;
        if char == b'\n' {
            self.shift_characters();
            self.terminal_column_position = 0;
            return;
        }
        if self.terminal_column_position == 25 {
            self.shift_characters();
            self.terminal_column_position = 0;
        }
        if char_writing >= CHARACTERS.len() as u8 {
            char_writing = 0;
        }
        self.draw_character(char_writing,  2 + self.terminal_column_position * 6, 183, 15, 0);
        self.terminal_character_buffer[0][self.terminal_column_position] = (
            char_writing, self.terminal_foreground_color, self.terminal_background_color
        );
        self.terminal_column_position += 1;
    }

    pub fn remove_terminal_character(&mut self) {
        if self.terminal_column_position == 0 { return; }
        self.terminal_column_position -= 1;

        self.draw_character(0,  2 + self.terminal_column_position * 6, 183, 15, 0);
        self.terminal_character_buffer[0][self.terminal_column_position] = (
            0, self.terminal_foreground_color, self.terminal_background_color
        );
    }

    fn draw_clock_character(&mut self, char: u8) {
        if char == b'\n' {
            self.clock_column_position = 0;
            return;
        }

        self.draw_character(char,  164 + self.clock_column_position * 6, 191, 
            self.clock_foreground_color, self.clock_background_color);
        self.clock_column_position += 1;
    }
    fn draw_ram_character(&mut self, char: u8) {
        if char == b'\n' {
            self.ram_column_position = 0;
            return;
        }

        self.draw_character(char,  252 + self.ram_column_position * 6, 191, 
            self.ram_foreground_color, self.ram_background_color);
        self.ram_column_position += 1;
    }

    pub fn write_string(&mut self, s: &str) {
        for char in s.bytes() {
            match self.frame {
                0 => self.draw_terminal_character(char),
                1 => self.draw_clock_character(char),
                2 => self.draw_ram_character(char),
                _ => self.draw_terminal_character(char)
            }
        }
    }
}