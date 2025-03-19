use crate::{alloc, println, warnln};

// the max number of files is 5000
pub struct FileVec {
    data: [(u32, i32, (usize, usize, usize), [u8; 20]); 5000],
    size: usize,
}
impl FileVec {
    pub fn new() -> Self {
        println!("Created new FileSystem Vector");
        Self {
            data: [(0, -1, (0, 0, 0), [0; 20]); 5000],
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

    pub fn iter(&self) -> [(u32, i32, (usize, usize, usize), [u8; 20]); 5000] {
        self.data
    }
}

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
    pub fn add(&mut self, value: u8) {
        if self.size >= self.heap_size {
            warnln!("Reached vec limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value as usize);
        self.size += 1;
    }

    #[allow(dead_code)]
    pub fn get(&mut self, address: usize) -> usize {
        if address > self.size {
            warnln!("Address out of range :c");
            return 0;
        }
        alloc::read_byte(self.heap_start + address)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size
    }
}