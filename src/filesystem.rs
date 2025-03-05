use crate::{print, println, vec::FileVec};

use lazy_static::lazy_static;
use spin::Mutex;

pub struct FileSystem {
    files: FileVec,
    flow: i32
}
impl FileSystem {
    pub fn create_file(&mut self, parent: i32, range: (u32, u32), filename: [u8; 20]) {
        self.files.add((self.files.len() as u32, parent, range, filename));
        println!("Created a new file with id {}", self.files.len() as u32 - 1);
    }

    pub fn set_flow(&mut self, flow: i32) {
        self.flow = flow;
    }

    // this being a list of 20 is the max amount of files that will be returned, why 20? sounds good to me tbh :3
    pub fn get_file_from_parent(&self, parent: i32) -> [(u32, i32, (u32, u32), [u8; 20]); 20] {
        let mut files_returning = [(0, -1, (0, 1), [1; 20]); 20];
        let mut files_returning_len = 0;
        for file in self.files.iter() {
            if file.1 == parent && files_returning_len < 20 {
                files_returning[files_returning_len] = file;
                files_returning_len += 1;
            }
        }
        files_returning
    }

    pub fn get_file_from_current_parent(&self) -> [(u32, i32, (u32, u32), [u8; 20]); 20] {
        self.get_file_from_parent(self.flow)
    }
}

lazy_static! {
    pub static ref FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem {
        files: FileVec::new(),
        flow: 1
    });
}

pub fn print_current_dir_files() {
    let files_found = FILESYSTEM.lock().get_file_from_current_parent();
    for file in files_found {
        if file.1 == -1 { continue; }
        for char_byte in file.3 {
            if char_byte == 0 { break; }
            print!("{}", char_byte as char);
        }
        print!("\n");
    }
}

pub fn change_flow(name: [u8; 20]) {
    let files = {
        FILESYSTEM.lock().get_file_from_current_parent()
    };
    for file in files {
        if file.3 == name {
            FILESYSTEM.lock().set_flow(file.0 as i32);
        }
    }
}

#[allow(dead_code)]
pub fn create_file(parent: i32, range: (u32, u32), filename: &str) {
    let mut filename_bytes = [0; 20];
    let mut filename_bytes_len = 0;
    let filename_parsed = filename.bytes();
    for byte in filename_parsed {
        filename_bytes[filename_bytes_len] = byte;
        filename_bytes_len += 1;
    }
    FILESYSTEM.lock().create_file(parent, range, filename_bytes);
}