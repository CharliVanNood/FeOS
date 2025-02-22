use crate::println;
use crate::print;
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
    if character == 10 {
        match_commands();
        return false;
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

#[allow(dead_code)]
pub fn match_commands() {
    print!("\n");
    let info_command = "info".bytes();
    let command_length = info_command.len();
    let command_written = get_text();
    let mut is_command = true;

    let mut i = 0;
    for byte in info_command {
        if i + 1 == command_length && command_written[i + 1] != 0 {
            is_command = false;
        }
        if byte != command_written[i] as u8 {
            is_command = false;
        }
        i += 1;
    }

    if is_command {
        println!("Niceee you found the info command");
    }
    
    print!("-> ");

    {
        let mut text = CURRENT_TEXT.lock();
        let mut text_end = CURRENT_TEXT_END.lock();
        *text = [0; 255];
        *text_end = 0;
    }
}