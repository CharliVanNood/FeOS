use crate::{println, vec::FileVec};

pub struct FileSystem {
    files: FileVec,
    directory: i32
}
impl FileSystem {
    pub fn init() -> Self {
        println!("Initializing the FileSystem");
        Self {
            files: FileVec::new(),
            directory: 0
        }
    }

    pub fn create_file(&mut self, parent: u32, range: (u32, u32), filename: [u8; 20]) {
        self.files.add((self.files.len() as u32, parent, range, filename));
        println!("Created a new file with id {}", self.files.len() as u32 - 1);
    }

    pub fn set_dir(&mut self, dir: i32) {
        self.directory = dir;
    }

    // this being a list of 20 is the max amount of files that will be returned, why 20? sounds good to me tbh :3
    pub fn get_file_from_parent(parent: i32) -> [(u32, u32, (u32, u32), [u8; 20]); 20] {
        let files_returning = [(0, 0, (0, 1), [1; 20]); 20];
        files_returning
    }

    pub fn print_current_dir_files() {
        
    }
}