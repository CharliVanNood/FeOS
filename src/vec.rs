// the max number of files is 19408, this is cause of heap allocation, the memory used here is 698.7kb
pub struct FileVec {
    data: [(i32, i32, (u32, u32), [u8; 20]); 19408],
    size: usize,
}
impl FileVec {
    pub fn new() -> Self {
        Self {
            data: [(-1, -1, (0, 0), [0; 20]); 19408],
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