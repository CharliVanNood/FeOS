use crate::{print, println};

pub fn exec(input: [usize; 255]) {
    let tokenized_code = tokenize(input);

    for token in tokenized_code {
        if token.1 == 0 {
            print!("{} ", token.0);
        } else {
            print!("{}({}) ", token.0, token.1);
        }
    }
    print!("\n");

    run_tokens(tokenized_code);
}

fn shift_list(list: [(i8, i32); 255], index: usize) -> [(i8, i32); 255] {
    let mut return_list = [(0, 0); 255];

    for i in 0..255 {
        if i < index {
            return_list[i] = list[i];
        } else if i < 254 {
            return_list[i] = list[i + 1];
        }
    }

    return_list
}

fn run_tokens(tokens: [(i8, i32); 255]) {
    let tokens_after_fact = run_tokens_fact(tokens);

    for token in tokens_after_fact {
        if token.1 == 0 {
            print!("{} ", token.0);
        } else {
            print!("{}({}) ", token.0, token.1);
        }
    }
    print!("\n");

    let tokens_after_math = run_tokens_math(tokens_after_fact);

    for token in tokens_after_math {
        if token.1 == 0 {
            print!("{} ", token.0);
        } else {
            print!("{}({}) ", token.0, token.1);
        }
    }
    print!("\n");
}

fn run_tokens_fact(mut tokens: [(i8, i32); 255]) -> [(i8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            13 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result = (1, tokens[token_index - 1].1 / tokens[token_index + 1].1);
                    tokens[token_index - 1] = operation_result;
                    tokens = shift_list(tokens, token_index);
                    tokens = shift_list(tokens, token_index);
                    token_index -= 1;
                }
            },
            14 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result = (1, tokens[token_index - 1].1 * tokens[token_index + 1].1);
                    tokens[token_index - 1] = operation_result;
                    tokens = shift_list(tokens, token_index);
                    tokens = shift_list(tokens, token_index);
                    token_index -= 1;
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_math(mut tokens: [(i8, i32); 255]) -> [(i8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            11 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result = (1, tokens[token_index - 1].1 + tokens[token_index + 1].1);
                    tokens[token_index - 1] = operation_result;
                    tokens = shift_list(tokens, token_index);
                    tokens = shift_list(tokens, token_index);
                    token_index -= 1;
                }
            },
            12 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result = (1, tokens[token_index - 1].1 - tokens[token_index + 1].1);
                    tokens[token_index - 1] = operation_result;
                    tokens = shift_list(tokens, token_index);
                    tokens = shift_list(tokens, token_index);
                    token_index -= 1;
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn match_token(token: [i8; 64]) -> (i8, i32) {
    let tokens_val = ["print", "+", "-", "/", "*", "(", ")"];
    let tokens_keys  = [ 10,      11,  12,  13,  14,  15,  16];

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

        return (tokens_keys[command_index], 0)
    }

    let mut is_int = true;
    let mut int_len = 0;
    for byte in token {
        if byte == 0 { break; }
        if byte < 48 || byte > 57 {
            is_int = false;
            break;
        }
        int_len += 1;
    }
    if is_int {
        let mut int_val = 0;

        for i in 0..int_len {
            let byte_number = token[i] as i32 - 48;
            int_val += byte_number * 10_i32.pow((int_len - i) as u32 - 1);
        }

        return (1, int_val)
    }

    (-1, 0)
}

fn tokenize(input: [usize; 255]) -> [(i8, i32); 255] {
    let mut tokens: [(i8, i32); 255] = [(0, 0); 255];
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