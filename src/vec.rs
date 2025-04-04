use crate::{alloc, print, println, warnln};

// THIS HAS TO BE REWRITTEN TO USE HEAP SOON!!! :C
pub struct FileVec {
    data: [(u32, i32, (usize, usize, usize), [u8; 20]); 100],
    size: usize,
}
impl FileVec {
    pub fn new() -> Self {
        println!("Created new FileSystem Vector");
        Self {
            data: [(0, -1, (0, 0, 0), [0; 20]); 100],
            size: 1
        }
    }

    pub fn add(&mut self, file: (u32, i32, (usize, usize, usize), [u8; 20])) {
        self.data[self.size] = file;
        self.size += 1;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> [(u32, i32, (usize, usize, usize), [u8; 20]); 100] {
        self.data
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[allow(dead_code)]
pub struct TokenVec {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl TokenVec {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(512);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, token: usize, value: usize) {
        if self.size >= self.heap_size {
            warnln!("Reached vec limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, token);
        alloc::write_byte(self.heap_start + self.size + 8, value);
        self.size += 16;
    }

    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> (usize, usize) {
        if address * 16 >= self.size {
            warnln!("Address {} out of range for reading from token vector :c", address);
            return (0, 0);
        }
        (alloc::read_byte(self.heap_start + address * 16),
         alloc::read_byte(self.heap_start + address * 16 + 8))
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if self.len() == 0 {
            return;
        }
        print!("[");
        for i in 0..self.len() {
            let data_type = alloc::read_byte(self.heap_start + i * 16);

            if data_type == 6 {
                if i < self.len() - 1 {
                    print!("{}, ", alloc::read_byte(self.heap_start + i * 16 + 8) as u8 as char);
                } else {
                    print!("{}", alloc::read_byte(self.heap_start + i * 16 + 8) as u8 as char);
                }
            } else {
                if i < self.len() - 1 {
                    print!("({} {}) ", data_type, alloc::read_byte(self.heap_start + i * 16 + 8));
                } else {
                    print!("({} {})", data_type, alloc::read_byte(self.heap_start + i * 16 + 8));
                }
            }
        }
        print!("]\n");
    }

    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, token: usize, value: usize) {
        if address * 16 >= self.size {
            warnln!("Address out of range for setting in token vector :c");
            return;
        }
        alloc::write_byte(self.heap_start + address * 16, token);
        alloc::write_byte(self.heap_start + address * 16 + 8, value);
    }

    #[allow(dead_code)]
    pub fn shift(&mut self, index: usize, length: usize) {
        for i in index..self.len(){
            if i >= self.len() - length {
                self.set(i, 0, 0);
            } else {
                let next_token = self.get(i + length);
                self.set(i, next_token.0, next_token.1);
            }
        }
        self.size = self.size - length * 16;
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 16
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[allow(dead_code)]
pub struct Vec {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl Vec {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(1024);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, value: usize) {
        if self.size >= self.heap_size {
            warnln!("Reached vec limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value);
        self.size += 8;
    }

    #[allow(dead_code)]
    pub fn get(&mut self, address: usize) -> usize {
        if address * 8 >= self.size {
            warnln!("Address out of range for {} :c", address);
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if self.len() == 0 {
            return;
        }
        print!("[");
        for i in 0..self.len() {
            if i < self.len() - 1 {
                print!("{} ", alloc::read_byte(self.heap_start + i * 8));
            } else {
                print!("{}", alloc::read_byte(self.heap_start + i * 8));
            }
        }
        print!("]\n");
    }

    #[allow(dead_code)]
    pub fn set_as_b64(&mut self, value: [u8; 64]) {
        for i in 0..64 {
            alloc::write_byte(self.heap_start + i * 8, value[i] as usize);
        }
        self.size = 512;
    }

    #[allow(dead_code)]
    pub fn get_as_b64(&self) -> [u8; 64] {
        let mut b64_list = [0; 64];
        for i in 0..self.len() {
            b64_list[i] = alloc::read_byte(self.heap_start + i * 8) as u8;
        }
        b64_list
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }
}