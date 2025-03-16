use crate::{alloc, println};

// the max number of files is 5000
pub struct FileVec {
    data: [(u32, i32, (u32, u32), [u8; 20]); 5000],
    size: usize,
}
impl FileVec {
    pub fn new() -> Self {
        println!("Created new FileSystem Vector");
        Self {
            data: [(0, -1, (0, 0), [0; 20]); 5000],
            size: 1
        }
    }

    pub fn add(&mut self, file: (u32, i32, (u32, u32), [u8; 20])) {
        self.data[self.size] = file;
        self.size += 1;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> [(u32, i32, (u32, u32), [u8; 20]); 5000] {
        self.data
    }
}

#[allow(dead_code)]
pub struct Vec {
    size: usize,
    heap_start: usize,
    heap_size: usize
}
impl Vec {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(1024);
        Self {
            size: 0,
            heap_start: heap_start,
            heap_size: 1024
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, _value: u8) {
        self.size += 1;
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size
    }
}