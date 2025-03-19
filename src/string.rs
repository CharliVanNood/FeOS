use core::usize;

use crate::{print, println, warnln};
use crate::alloc;

#[allow(dead_code)]
pub struct BigString {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl BigString {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(2048);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from(value: &str) -> Self {
        let heap_start = alloc::alloc(2048);
        let mut size = 0;

        for byte in value.bytes() {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from_b64(value: [u8; 64]) -> Self {
        let heap_start = alloc::alloc(2048);
        let mut size = 0;

        for byte in value {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }
    
    #[allow(dead_code)]
    pub fn from_b256(value: [u8; 256]) -> Self {
        let heap_start = alloc::alloc(2048);
        let mut size = 0;

        for byte in value {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, value: &str) {
        self.size = 0;
        for byte in value.bytes() {
            if self.size >= self.heap_size {
                warnln!("Reached String limit :c");
                return;
            }
            alloc::write_byte(self.heap_start + self.size, byte as usize);
            self.size += 8;
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, value: u8) {
        if self.size >= self.heap_size {
            warnln!("Reached String limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value as usize);
        self.size += 8;
    }

    #[allow(dead_code)]
    pub fn get(&mut self, address: usize) -> usize {
        if address * 8 >= self.size {
            warnln!("Address out of range :c");
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }
}

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

pub fn replace_64b(mut string_in: [u8; 64], key: &str, _replacement: &str) -> [u8; 64] {
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