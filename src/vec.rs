pub struct FileVec {
    data: [(i32, i32, (u32, u32), [u8; 20]); 65535],
    size: usize,
}
impl FileVec {
    pub fn new() -> Self {
        Self {
            data: [(-1, -1, (0, 0), [0; 20]); 65535],
            size: 0
        }
    }

    pub fn add(&mut self, file: (i32, i32, (u32, u32), [u8; 20])) {
        self.data[self.size] = file;
    }

    pub fn len(&self) -> usize {
        self.size
    }
}