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
    pub fn get(&self, address: usize) -> usize {
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

    #[allow(dead_code)]
    pub fn print(&self) {
        for index in 0..self.len() {
            print!("{}", self.get(index) as u8 as char);
        }
        print!("\n");
    }

    #[allow(dead_code)]
    pub fn includes(&self, needle: &str) -> i8 {
        let first_character = needle.bytes().next().unwrap();
        let needle_length = needle.bytes().len();

        for index in 0..self.len() {
            let character = self.get(index) as u8;
            if character == first_character {
                if needle_length == 1 {
                    return index as i8;
                } else {
                    let mut matches = true;
                    let mut offset = 0;
                    for character in needle.bytes() {
                        let character_self = self.get(index + offset) as u8;
                        if character != character_self {
                            matches = false;
                        }
                        offset += 1;
                    }
                    if matches {
                        return index as i8;
                    }
                }
                println!("FOUND FIRST CHARACTER")
            }
        }
        return -1;
    }

    pub fn replace(&self, needle: &str, value: &str) {
        
    }
}
