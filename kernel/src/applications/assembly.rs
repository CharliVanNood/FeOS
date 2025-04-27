use crate::println;
use crate::string::BigString;
use crate::vec::{TokenVec, Vec};

pub fn exec(input: BigString) {
    let (tokenized_code, functions, values, start_function) = tokenize(&input);
    input.remove();
    run(tokenized_code, functions, values, start_function);
}

fn get_int(numbers: [usize; 3]) -> usize {
    let mut int_val = 0;

    for i in 0..3 {
        let byte_number = numbers[i] as i32 - 48;
        int_val += byte_number * 10_i32.pow((3 - i) as u32 - 1);
    }
    int_val as usize
}

fn tokenize(input: &BigString) -> (Vec, Vec, TokenVec, usize) {
    let mut return_vec = Vec::new();
    let mut functions = Vec::new();
    let mut values = TokenVec::new();

    let mut adr_indexes = 0;

    let start_function = get_int([input.get(0), input.get(1), input.get(2)]);

    for code in 1..input.len() / 3 {
        let a = input.get(code * 3);
        let b = input.get(code * 3 + 1);
        let c = input.get(code * 3 + 2);
        if a == b'a' as usize && b == b'd' as usize && c == b'r' as usize {
            adr_indexes = code * 3 + 3;
            break;
        }
        if a == b'f' as usize && b == b'u' as usize && c == b'n' as usize {
            functions.add(code - 1);
            return_vec.add(1000);
            continue;
        }

        return_vec.add(get_int([a, b, c]));
    }

    let mut temp_characters = [0; 64];
    let mut temp_index = 0;

    for key_index in adr_indexes..input.len() {
        let key = input.get(key_index);

        if key as u8 == b':' {
            let mut int_val = 0;

            for i in 0..temp_index {
                let byte_number = temp_characters[i] as i32 - 48;
                int_val += byte_number * 10_i32.pow((temp_index - i) as u32 - 1);
            }

            values.add(1, int_val as usize);

            temp_characters = [0; 64];
            temp_index = 0;
            continue;
        }

        temp_characters[temp_index] = key as u8;
        temp_index += 1;
    }

    (return_vec, functions, values, start_function)
}

fn run(tokens: Vec, functions: Vec, values: TokenVec, start_function: usize) {
    let mut base_addresses = Vec::new();
    let heap_addresses = Vec::new();

    for token_index in functions.get(start_function)..tokens.len() {
        let token = tokens.get(token_index);
        if token > 100 { continue; }
        match token {
            16 => {
                let address = tokens.get(token_index + 1) - 200;
                let mut value = tokens.get(token_index + 2);
                let value_is_addr = {
                    value < 300
                };

                if value_is_addr {
                    value = value - 200;
                    base_addresses.set_add(address, base_addresses.get_unsafe(value));
                } else {
                    value = value - 300;
                    value = values.get(value).1;
                    base_addresses.set_add(address, value);
                }

                println!("ADDR: {} VAL: {} RES: {}", address, value, base_addresses.get(address));
            }
            _ => {}
        }
    }

    base_addresses.remove();
    heap_addresses.remove();
}