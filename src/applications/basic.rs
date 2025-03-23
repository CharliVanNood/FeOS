use crate::{println, string::BigString, vec::{TokenVec, Vec}};

pub fn exec(input: [u8; 512]) {
    let mut input_string = BigString::from_b512(input);
    for _ in 0..32 {
        input_string.replace("\n", " lnnew ");
        input_string.replace(";", " lnnew ");
    }
    let tokenized_code = tokenize(input_string);
    for token in 0..tokenized_code[0].len() {
        let token = tokenized_code[0].get(token);
        println!("{} {}", token.0, token.1);
    }
    //run_tokens(tokenized_code);
}

fn match_token(token: [u8; 64], variables: [Vec; 64]) -> (usize, usize, [Vec; 64]) {
    let tokens_val = [
        "PRINT", "\n", "lnnew", "TRUE", "FALSE"];
    let tokens_keys  = [
         10,      8,    8,       3,      3];

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

        if tokens_val[command_index] == "TRUE" { return (tokens_keys[command_index], 1, variables) }
        return (tokens_keys[command_index], 0, variables)
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

        return (1, int_val as usize, variables)
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

        return (2, int_val as usize, variables)
    }
    
    let mut variables_new = variables;
    for variable in variables.iter().enumerate() {
        if variable.1.get_as_b64() == token {
            return (7, variable.0, variables);
        } else if variable.1.get_as_b64() == [0; 64] {
            variables_new[variable.0].set_as_b64(token);
            return (7, variable.0, variables_new);
        }
    }
    (7, 63, variables)
}

fn tokenize(input: BigString) -> [TokenVec; 128] {
    let mut lines: [TokenVec; 128] = [TokenVec::new(); 128];
    for i in 1..128 {
        lines[i] = TokenVec::new();
    }

    let mut line = 0;

    let mut temp_token = [0; 64];
    let mut temp_token_index = 0;

    let mut variables = [Vec::new(); 64];
    for i in 1..64 {
        variables[i] = Vec::new();
    }

    for char_index in 0..input.len() {
        let char = input.get(char_index);
        if char == 0 { continue; }
        if char == 32 {
            let token = match_token(temp_token, variables);
            variables = token.2;
            if token.0 == 8 {
                line += 1;
                temp_token = [0; 64];
                temp_token_index = 0;
            } else {
                lines[line].add(token.0, token.1);
                temp_token = [0; 64];
                temp_token_index = 0;
            }
        } else {
            temp_token[temp_token_index] = char as u8;
            temp_token_index += 1;
        }
    }
    let token = match_token(temp_token, variables);
    if token.0 != 8 {
        lines[line].add(token.0, token.1);
    }

    lines
}