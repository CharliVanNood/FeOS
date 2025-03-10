use crate::{print, println};

fn shift_64b(list: [u8; 64], index: usize, length: usize) -> [u8; 64] {
    let mut return_list = [0; 64];

    for i in 0..64 - length {
        if i < index {
            return_list[i] = list[i];
        } else {
            return_list[i] = list[i + length];
        }
    }

    return_list
}

fn insert_64b(list: [u8; 64], index: usize, value: u8) -> [u8; 64] {
    let mut return_list = [0; 64];

    for i in 0..64 {
        if i < index {
            return_list[i] = list[i];
        } else if i == index {
            println!("INSERTED CHARACTER");
            return_list[i] = value;
        } else {
            return_list[i] = list[i - 1];
        }
    }

    return_list
}

pub fn replace_64b(mut string_in: [u8; 64], key: &str, replacement: &str) -> [u8; 64] {
    let key_bytes = key.bytes();
    let key_bytes_len = key.bytes().count();
    let key_bytes_parsed = {
        let mut key_bytes_parsed = [0; 64];
        let mut index = 0;
        for byte in key_bytes {
            key_bytes_parsed[index] = byte;
            index += 1;
        }
        key_bytes_parsed
    };

    for i in 0..10 {
        if i < string_in.len() - key_bytes_len {
            let mut matches = true;
            for j in 0..key_bytes_len {
                if key_bytes_parsed[j] != string_in[i + j] {
                    matches = false;
                } else {
                    println!("{} to {}", key_bytes_parsed[j], string_in[i + j]);
                }
                //println!("{} to {}", key_bytes_parsed[j], string_in[i + j]);
            }
            if matches {
                for byte in string_in {
                    print!("{}", byte as char);
                }
                print!("\n");
                string_in = shift_64b(string_in, i, 2);
                string_in = insert_64b(string_in, i, 32);
                println!("FOUND A MATCH FOR THIS STRING");
                for byte in string_in {
                    print!("{}", byte as char);
                }
                print!("\n");
            }
        }
    }
    string_in
}