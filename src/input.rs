use crate::{println, print, warnln};
use crate::vga;
use crate::python;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref CURRENT_TEXT: Mutex<[usize; 255]> = Mutex::new([0; 255]);
    static ref CURRENT_TEXT_END: Mutex<usize> = Mutex::new(0);
}

#[allow(dead_code)]
pub fn set_text(characters: [usize; 255]) {
    let mut text = CURRENT_TEXT.lock();
    *text = characters;
}

#[allow(dead_code)]
pub fn get_text() -> [usize; 255] {
    let text = CURRENT_TEXT.lock();
    *text
}

#[allow(dead_code)]
pub fn add_key(character: usize) -> bool {
    match character {
        10 => {
            match_commands();
            return false;
        },
        8 => {
            remove_byte();
            return false;
        }
        _ => {}
    }

    let mut text = CURRENT_TEXT.lock();
    let mut text_end = CURRENT_TEXT_END.lock();
    
    if *text_end < 255 {
        text[*text_end] = character;
        *text_end += 1;
        true
    } else {
        println!("You're at the typing limit :c");
        false
    }
}

fn remove_byte() {
    let mut text = CURRENT_TEXT.lock();
    let mut text_end = CURRENT_TEXT_END.lock();
    
    if *text_end > 0 {
        *text_end -= 1;
        text[*text_end] = 0;
        vga::remove_byte();
    }
}

#[allow(dead_code)]
pub fn match_commands() {
    let commands = ["info", "ping", "color", "clear", "help", "python"];

    print!("\n");

    let mut command_processed = false;
    for command in commands {
        let command_bytes = command.bytes();
        let command_length = command_bytes.len();
        let command_written = get_text();
        let mut is_command = true;

        let mut i = 0;
        for byte in command_bytes {
            if i + 1 == command_length && command_written[i + 1] != 0 && command_written[i + 1] != 32 {
                is_command = false;
            }
            if byte != command_written[i] as u8 {
                is_command = false;
            }
            i += 1;
        }

        if is_command {
            command_processed = true;
            match command {
                "info" => {
                    println!("\nWe have these general commands");
                    println!("   [ping]           - Just a simple test command");
                    println!("   [python] [code]  - Run python commands");
                    println!("   [color]          - Toggle the background color");
                    println!("   [clear]          - Clear the screen\n");
                },
                "help" => {
                    println!("\nWe have these general commands");
                    println!("   [ping]           - Just a simple test command");
                    println!("   [python] [code]  - Run python commands");
                    println!("   [color]          - Toggle the background color");
                    println!("   [clear]          - Clear the screen\n");
                },
                "ping" => println!("Pong"),
                "color" => {
                    print!("Changed the color to black");
                    let color = vga::get_color();
                    if color == 15 {
                        vga::set_color(13, 0);
                    } else {
                        vga::set_color(13, 15);
                    }
                    print!("\n");
                },
                "clear" => {
                    vga::clear_screen();
                    print!("The screen has been cleared");
                    print!("\n");
                },
                "python" => python::exec(command_written),
                _ => warnln!("This command is unimplemented :C")
            }
        }
    }
    if !command_processed {
        warnln!("This command does not seem to exist :C");
    }

    print!("-> ");

    {
        let mut text = CURRENT_TEXT.lock();
        let mut text_end = CURRENT_TEXT_END.lock();
        *text = [0; 255];
        *text_end = 0;
    }
}