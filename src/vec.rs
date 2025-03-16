use crate::println;

// the max number of files is 19000
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

pub struct _Vec {

}
impl _Vec {
    pub fn _new() {
        
    }
}