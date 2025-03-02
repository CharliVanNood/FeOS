use crate::{print, println};

pub fn exec(input: [usize; 255]) {
    let tokenized_code = tokenize(input);

    for token in tokenized_code {
        print!("{} ", token);
    }
    print!("\n");
}

fn match_token(token: [i8; 64]) -> i8 {
    let tokens_val = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "print", "+", "-", "/", "*", "(", ")"];
    let tokens_keys = [ 0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   10,       11,  12,  13,  14,  15,  16];

    for command_index in 0..tokens_val.len() {
        let command = tokens_val[command_index];
        let command_bytes = command.bytes();
        let mut is_command = true;

        let mut i = 0;
        for byte in command_bytes {
            if byte != token[i] as u8 {
                is_command = false;
            }
            i += 1;
        }
        if !is_command { continue; }

        return tokens_keys[command_index]
    }

    -1
}

fn tokenize(input: [usize; 255]) -> [i8; 255] {
    let mut tokens: [i8; 255] = [0; 255];
    let mut tokens_index = 0;

    println!("tokenizing code");

    // this creates a max token length of 64
    let mut temp_token = [0; 64];
    let mut temp_token_index = 0;

    for char_index in 7..255 {
        let char = input[char_index];
        if char == 0 { continue; }
        if char == 32 {
            tokens[tokens_index] = match_token(temp_token);
            tokens_index += 1;
            temp_token = [0; 64];
            temp_token_index = 0;
        } else {
            temp_token[temp_token_index] = char as i8;
            temp_token_index += 1;
        }
    }
    tokens[tokens_index] = match_token(temp_token);

    tokens
}