use core::fmt;
use x86_64::instructions::interrupts;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt::Write;

use crate::renderer::vga::writer::ScreenWriter;
use crate::renderer::colors::get_rgb;
use crate::alloc;

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::window::_warn(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! warnln {
    () => ($crate::warn!("\n"));
    ($($arg:tt)*) => ($crate::warn!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::window::_info(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! infoln {
    () => ($crate::info!("\n"));
    ($($arg:tt)*) => ($crate::info!("{}\n", format_args!($($arg)*)));
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
#[doc(hidden)]
pub fn _warn(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        let color = {
            SCREEN_WRITER.lock().get_terminal_colors().1
        };
        SCREEN_WRITER.lock().set_color(4, color, 0);
        SCREEN_WRITER.lock().write_fmt(args).unwrap();
        SCREEN_WRITER.lock().set_color(15, color, 0);
    });
}
#[doc(hidden)]
pub fn _info(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        let color = {
            SCREEN_WRITER.lock().get_terminal_colors().1
        };
        SCREEN_WRITER.lock().set_color(5, color, 0);
        SCREEN_WRITER.lock().write_fmt(args).unwrap();
        SCREEN_WRITER.lock().set_color(15, color, 0);
    });
}

pub fn clear_screen() {
    for _ in 0..19 {
        println!("");
    }
}

pub fn increment_frame() {
    SCREEN_WRITER.lock().increment_frame();
}

pub fn set_terminal_color(foreground: u8, background: u8) {
    SCREEN_WRITER.lock().set_color(foreground, background, 0);
}
pub fn get_terminal_color() -> u8 {
    SCREEN_WRITER.lock().get_terminal_colors().1
}

pub fn remove_terminal_character() {
    SCREEN_WRITER.lock().remove_terminal_character();
}

pub fn draw_menu_bar(time: (u8, u8, u8)) {
    let current_frame = SCREEN_WRITER.lock().get_frame();

    if current_frame % 30 == 0 {
        let status_background_color = SCREEN_WRITER.lock().get_status_bar_colors().1;
        let screen_size = SCREEN_WRITER.lock().get_screen_size();
        set_rect(0, screen_size.1 - 10, screen_size.0 / 2, 10, status_background_color);
    }

    SCREEN_WRITER.lock().set_frame(1);

    // just making sure the clock wont have 1:1:1 cause that'd be weird
    match time {
        (h, m, s) if h < 10 && m < 10 && s < 10 => {
            println!("0{}:0{}:0{}\n", time.0, time.1, time.2);
        },
        (h, m, s) if h > 10 && m < 10 && s < 10 => {
            println!("{}:0{}:0{}\n", time.0, time.1, time.2);
        },
        (h, m, s) if h < 10 && m < 10 && s > 10 => {
            println!("0{}:0{}:{}\n", time.0, time.1, time.2);
        },
        (h, m, s) if h < 10 && m > 10 && s < 10 => {
            println!("0{}:{}:0{}\n", time.0, time.1, time.2);
        },
        (h, m, s) if h > 10 && m > 10 && s < 10 => {
            println!("{}:{}:0{}\n", time.0, time.1, time.2);
        },
        (h, m, s) if h > 10 && m < 10 && s > 10 => {
            println!("{}:0{}:{}\n", time.0, time.1, time.2);
        },
        (h, m, s) if h < 10 && m > 10 && s > 10 => {
            println!("0{}:{}:{}\n", time.0, time.1, time.2);
        },
        (h, m, s) if h > 10 && m > 10 && s > 10 => {
            println!("{}:{}:{}\n", time.0, time.1, time.2);
        },
        _ => {
            println!("{}:{}:{}\n", time.0, time.1, time.2);
        }
    }
    
    SCREEN_WRITER.lock().set_frame(2);
    let ram_usage = alloc::get_usage();
    println!("Ram: {:.2}%\n", (ram_usage.0 as f32 / ram_usage.1 as f32) * 100.0);

    SCREEN_WRITER.lock().set_frame(0);
    increment_frame();
}

pub fn set_pixel(x: usize, y: usize, color: u8) {
    SCREEN_WRITER.lock().set_pixel(x + 160, y, color);
}
pub fn set_rect(x: usize, y: usize, size_x: usize, size_y: usize, color: u8) {
    SCREEN_WRITER.lock().set_rect(x + 160, y, size_x, size_y, color);
}

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref SCREEN_WRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter::init());
}

pub fn init() {
    let mut screen_writer = SCREEN_WRITER.lock();
    let screen_size = screen_writer.get_screen_size();
    let background_color = get_rgb(0, 0, 0);
    for x in 0..screen_size.0 {
        for y in 0..screen_size.1 {
            if x >= 160 {
                screen_writer.set_pixel(x, y, 215);
            } else {
                screen_writer.set_pixel(x, y, background_color);
            }
        }
    }

    let status_background_color = get_rgb(0, 0, 0);
    screen_writer.set_color(0, status_background_color, 3);
}

pub fn get_int(numbers: [usize; 3]) -> u8 {
    let mut int_val = 0;

    for i in 0..3 {
        let byte_number = numbers[i] as i32 - 48;
        int_val += byte_number * 10_i32.pow((3 - i) as u32 - 1);
    }
    int_val as u8
}

pub fn get_screen_size() -> (usize, usize) {
    SCREEN_WRITER.lock().get_screen_size()
}
