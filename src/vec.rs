// the max number of files is 19000
pub struct FileVec {
    data: [(u32, u32, (u32, u32), [u8; 20]); 19000],
    size: usize,
}
impl FileVec {
    pub fn new() -> Self {
        Self {
            data: [(0, 0, (0, 0), [0; 20]); 19000],
            size: 1
        }
    }

    pub fn add(&mut self, file: (u32, u32, (u32, u32), [u8; 20])) {
        self.data[self.size] = file;
    }

    pub fn len(&self) -> usize {
        self.size
    }
}