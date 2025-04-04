use crate::{println, string::BigString, vec::Vec};

pub fn query_nyo(input: [u8; 256]) {
    let input_string = BigString::from_b256(input);
    println!("created string");
    get_closest_string(input_string);
}

pub fn get_closest_string(input: BigString) {
    let mut tokens = Vec::new();

    let mut temp_token = 0;
    for character_index in 0..input.len() {
        let character = input.get(character_index);
        if character != 0 && character != 32 {
            temp_token += character;
        } else {
            tokens.add(temp_token);
            temp_token = 0;
        }
    }

    tokens.print();
}

const RESPONSES: [&str; 4] = [
    "hello", "hi",
    "how are you", "I'm doing great!"
];