use crate::alloc;
use crate::applications::blip;
use crate::clock;
use crate::disk::{convert_fs_to_bytes, write_fs_to_disk};
use crate::window;
use crate::{print, println, warnln};
use crate::applications;
use crate::filesystem;
use spin::Mutex;
use crate::window::render_image;

lazy_static::lazy_static! {
    static ref CURRENT_TEXT: Mutex<[u8; 256]> = Mutex::new([0; 256]);
    static ref CURRENT_TEXT_END: Mutex<usize> = Mutex::new(0);
    pub static ref KEYPRESSES: Mutex<([u16; 8], u8)> = Mutex::new(([0; 8], 0));
}

#[allow(dead_code)]
pub fn check_events() {
    // Get the current time and draw the menu bar
    let time = clock::get_time();
    window::draw_menu_bar(time);

    // Get all keys pressed
    let keypresses = {
        let lock = KEYPRESSES.lock();
        lock.clone()
    };

    // Add pressed keys
    for keypress in keypresses.0 {
        if keypress == 0 { break; }
        if keypress > 255 { continue; }
        add_key(keypress as u8);
    }

    // Reset the buffer
    KEYPRESSES.lock().0 = [0; 8];
    KEYPRESSES.lock().1 = 0;
}

#[allow(dead_code)]
pub fn set_text(characters: [u8; 256]) {
    let mut text = CURRENT_TEXT.lock();
    *text = characters;
}

#[allow(dead_code)]
pub fn get_text() -> [u8; 256] {
    let text = CURRENT_TEXT.lock();
    *text
}

#[allow(dead_code)]
pub fn add_key(character: u8) {
    match character {
        10 => {
            match_commands(get_text(), true);
            return;
        },
        8 => {
            remove_byte();
            return;
        }
        _ => {}
    }

    let mut text = CURRENT_TEXT.lock();
    let mut text_end = CURRENT_TEXT_END.lock();
    
    if *text_end < 255 {
        text[*text_end] = character;
        *text_end += 1;
        print!("{}", character as char);
    }
}

fn remove_byte() {
    let mut text = CURRENT_TEXT.lock();
    let mut text_end = CURRENT_TEXT_END.lock();
    
    if *text_end > 0 {
        *text_end -= 1;
        text[*text_end] = 0;
        window::remove_terminal_character();
    }
}

fn print_help_command() {
    println!("\nWe have these general commands");
    println!("[femc] [code] - FemC");
    println!("[basic] [code] - BASIC");
    println!("[color] - Toggle color");
    println!("[clear] - Clear screen");
    println!("[fl] - Show flow files");
    println!("[go] [flow] - Change flow");
    println!("[pong] - The game pong");
    println!("[cat] - Read a file");
    println!("[timeset] [hour] - Set the current hour");
    println!("[per] - Performance");
    println!("[run] [file] - Run code");
    println!("[nyo] [message] - NyoBot");
    println!("[imagine] [image] - Displays an image");
    println!("[blip] [file] - Edit file");
    println!("[clram] - Empty the ram");
}

#[allow(dead_code)]
pub fn match_commands(command_written:[u8; 256], user_ran:bool) {
    // To register a command add it here
    let commands = [
        "info", "ping", "color", "clear", "help", "femc", "fl", "go", 
        "install", "pong", "cat", "run", "per", "time", "input", "timeset",
        "basic", "nyo", "screen", "char", "imagine", "imgtest", "blip",
        "fsconvtest", "fswritetest", "clram", "shram", "meram"
    ];

    print!("\n");

    // Here commands will be matched and executed
    let mut command_processed = false;
    for command in commands {
        let command_bytes = command.bytes();
        let command_length = command_bytes.len();
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
            // Add your function as a match here
            match command {
                "info" => print_help_command(),
                "help" => print_help_command(),
                "ping" => println!("Pong"),
                "color" => {
                    print!("Changed the color to black");
                    let color = window::get_terminal_color();
                    if color == 15 {
                        window::set_terminal_color(13, 0);
                    } else {
                        window::set_terminal_color(13, 15);
                    }
                    print!("\n");
                },
                "clear" => {
                    window::clear_screen();
                    print!("The screen has been cleared");
                    print!("\n");
                },
                "femc" => applications::femc::exec(command_written),
                "basic" => {
                    let mut command_written_512 = [0u8; 512];
                    command_written_512[..256].copy_from_slice(&command_written);
                    applications::basic::exec(command_written_512)
                },
                "nyo" => {
                    applications::nyo::query_nyo(command_written);
                }
                "fl" => filesystem::print_current_dir_files(),
                "go" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 3..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    filesystem::change_flow(name);
                },
                "install" => {
                    if command_written[8] == b'a' {
                        filesystem::install_base_os(true)
                    } else {
                        filesystem::install_base_os(false)
                    }
                },
                "pong" => applications::pong::play(),
                "cat" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 4..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    let file_data = filesystem::read_file(name);
                    file_data.print();
                    file_data.remove();
                },
                "run" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 4..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    filesystem::run_file(name);
                },
                "per" => {
                    let ram_usage = alloc::get_usage();
                    println!("\n   Ram: {:.2}%", (ram_usage.0 as f32 / ram_usage.1 as f32) * 100.0);
                    println!("   {} Bytes / {} Bytes", ram_usage.0, ram_usage.1);
                    println!("   {} KB / {} KB", ram_usage.0 / 1000, ram_usage.1 / 1000);
                    println!("   {} MB / {} MB\n", ram_usage.0 / 1000000, ram_usage.1 / 1000000);
                    println!("   Disk: 0%\n");
                },
                "time" => {
                    clock::print_time();
                },
                "timeset" => {
                    let mut time = [0; 3];
                    let mut time_len = 0;

                    for byte_index in 8..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        time[time_len] = byte as u8;
                        time_len += 1;
                    }

                    let mut time_number = 0;

                    for byte in time.iter().enumerate() {
                        if *byte.1 == 0 { break; }
                        let byte_number = *byte.1 as i32 - 48;
                        time_number += byte_number * 10_i32.pow((time_len - byte.0 - 1) as u32);
                    }

                    clock::set_time(time_number as u8);
                },
                "input" => println!("neh"),
                "char" => println!("Character code: {}", command_written[5]),
                "imagine" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 8..27 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    let image_data = filesystem::read_image(name);
                    if image_data.len() > 0 {
                        render_image(image_data);
                    }
                },
                "imgtest" => {
                    let mut install: [u8; 256] = [0; 256];
                    let mut i = 0;
                    for byte in "install".bytes() {
                        install[i] = byte;
                        i += 1;
                    }
                    let mut go: [u8; 256] = [0; 256];
                    let mut i = 0;
                    for byte in "go images".bytes() {
                        go[i] = byte;
                        i += 1;
                    }
                    let mut imagine: [u8; 256] = [0; 256];
                    let mut i = 0;
                    for byte in "imagine koi".bytes() {
                        imagine[i] = byte;
                        i += 1;
                    }
                    match_commands(install,false);
                    match_commands(go,false);
                    match_commands(imagine,false)
                },
                "blip" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 5..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    blip::open(name);
                },
                "clram" => alloc::clear_ram(),
                "shram" => alloc::toggle_ram_graph(),
                "meram" => alloc::merge_ram(),
                "fsconvtest" => convert_fs_to_bytes().remove(),
                "fswritetest" => write_fs_to_disk(),
                _ => warnln!("This command is unimplemented :C")
            }
        }
    }
    if !command_processed {
        warnln!("This command does not seem to exist :C");
    }

    if user_ran { print!("-> ") }

    // Reset the keys pressed buffer
    {
        let mut text = CURRENT_TEXT.lock();
        let mut text_end = CURRENT_TEXT_END.lock();
        *text = [0; 256];
        *text_end = 0;
    }
}
