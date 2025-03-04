use crate::vec::FileVec;

pub struct FileSystem {
    files: FileVec,
    directory: i32
}
impl FileSystem {
    pub fn init() -> Self {
        Self {
            files: FileVec::new(),
            directory: 0
        }
    }

    pub fn create_file(&mut self, parent: i32, range: (u32, u32), filename: [u8; 20]) {
        self.files.add((self.files.len() as i32, parent, range, filename));
    }

    pub fn set_dir(&mut self, dir: i32) {
        self.directory = dir;
    }
}