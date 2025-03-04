use crate::{println, warnln};

pub fn exec(input: [usize; 255]) {
    let tokenized_code = tokenize(input);
    run_tokens(tokenized_code);
}

fn shift_list(list: [(i8, i32); 255], index: usize, length: usize) -> [(i8, i32); 255] {
    let mut return_list = [(0, 0); 255];

    for i in 0..255 - length {
        if i < index {
            return_list[i] = list[i];
        } else {
            return_list[i] = list[i + length];
        }
    }

    return_list
}

fn run_tokens(tokens: [(i8, i32); 255]) {
    let tokens_after_fact = run_tokens_fact(tokens);
    let tokens_after_math = run_tokens_math(tokens_after_fact);
    let tokens_after_first = run_tokens_first(tokens_after_math);
    let tokens_after_bool = run_tokens_boolean(tokens_after_first);
    let _tokens_after_last = run_tokens_last(tokens_after_bool);
}

fn run_tokens_fact(mut tokens: [(i8, i32); 255]) -> [(i8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            13 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result: f32 = tokens[token_index - 1].1 as f32 / tokens[token_index + 1].1 as f32 * 100.0;
                    tokens[token_index - 1] = (2, operation_result as i32);
                    tokens = shift_list(tokens, token_index, 2);
                    token_index -= 1;
                }
            },
            14 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result = (1, tokens[token_index - 1].1 * tokens[token_index + 1].1);
                    tokens[token_index - 1] = operation_result;
                    tokens = shift_list(tokens, token_index, 2);
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
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let operation_result = (1, tokens[token_index - 1].1 + tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let operation_result = (2, tokens[token_index - 1].1 + tokens[token_index + 1].1 * 100);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 * 100 + tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 + tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            12 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let operation_result = (1, tokens[token_index - 1].1 - tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let operation_result = (2, tokens[token_index - 1].1 - tokens[token_index + 1].1 * 100);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 * 100 - tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 - tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_boolean(mut tokens: [(i8, i32); 255]) -> [(i8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            17 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (3, 3) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            18 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 > tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 > tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 > tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 > tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            19 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 < tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 < tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 < tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 < tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            20 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 >= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 >= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 >= tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 >= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            21 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 <= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 <= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 <= tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 <= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_first(mut tokens: [(i8, i32); 255]) -> [(i8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            22 => {
                if tokens[token_index + 1].0 == 3 {
                    if tokens[token_index + 1].1 == 0 {
                        tokens[token_index] = (3, 1);
                    } else {
                        tokens[token_index] = (3, 0);
                    }
                    tokens = shift_list(tokens, token_index + 1, 1);
                } else {
                    warnln!("This is an unsupported type conversion");
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_last(mut tokens: [(i8, i32); 255]) -> [(i8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            10 => {
                match tokens[token_index + 1].0 {
                    1 => {
                        println!("{}", tokens[token_index + 1].1);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    2 => {
                        println!("{}", tokens[token_index + 1].1 as f32 / 100.0);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    3 => {
                        if tokens[token_index + 1].1 == 0 {
                            println!("false");
                        } else {
                            println!("true");
                        }
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            23 => {
                match tokens[token_index + 1].0 {
                    1 => {
                        warnln!("{}", tokens[token_index + 1].1);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    2 => {
                        warnln!("{}", tokens[token_index + 1].1 as f32 / 100.0);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    3 => {
                        if tokens[token_index + 1].1 == 0 {
                            warnln!("false");
                        } else {
                            warnln!("true");
                        }
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn match_token(token: [i8; 64]) -> (i8, i32) {
    let tokens_val = ["print", "+", "-", "/", "*", "(", ")", "==", ">=", "<=", ">", "<", "true", "false", "not", "warn"];
    let tokens_keys  = [ 10,      11,  12,  13,  14,  15,  16,  17,   20,   21,   18,  19,  3,      3,       22,    23];

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

        if tokens_val[command_index] == "true" { return (tokens_keys[command_index], 1) }
        return (tokens_keys[command_index], 0)
    }

    let mut is_int = true;
    let mut is_float = false;
    let mut int_len = 0;
    for byte in token {
        if byte == 0 { break; }
        if (byte < 48 || byte > 57) && byte != 46 {
            is_int = false;
            break;
        }
        if byte == 46 {
            is_int = false;
            is_float = true;
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
    } else if is_float {
        let mut int_val = 0;
        let mut dec_place = 0;

        for i in 0..int_len {
            if token[i] == 46 {
                dec_place = i;
                break;
            }
        }

        let decimals = (int_len - dec_place) as u32 - 1;

        for i in 0..int_len {
            if token[i] == 46 {
                continue;
            }
            if i > 2 && token[i - 3] == 46 { break; }
            let byte_number = token[i] as i32 - 48;
            if i < dec_place { int_val += byte_number * 10_i32.pow((int_len - i) as u32 - decimals); }
            else { int_val += byte_number * 10_i32.pow((int_len - i) as u32 - (decimals - 1)); }
        }

        return (2, int_val)
    }

    (-1, 0)
}

fn tokenize(input: [usize; 255]) -> [(i8, i32); 255] {
    let mut tokens: [(i8, i32); 255] = [(0, 0); 255];
    let mut tokens_index = 0;

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