use crate::{alloc, applications::femc, disk, info, print, println, vec::FileVec};

use lazy_static::lazy_static;
use spin::Mutex;

pub struct FileSystem {
    files: FileVec,
    flow: i32
}
impl FileSystem {
    fn _get_item_name(&self, id: u32) -> [u8; 20] {
        self.files.iter()[id as usize].3
    }

    fn print_path(&self, id: u32) {
        let file = self.files.iter()[id as usize];
        if file.1 != -1 {
            self.print_path(file.1 as u32);
            info!("/");
        }
        for byte in file.3 {
            if byte == 0 { break; }
            info!("{}", byte as char);
        }
    }

    pub fn create_file(&mut self, parent: i32, range: (u32, u32), filename: [u8; 20], data: &str) {
        self.files.add((self.files.len() as u32, parent, range, filename));
        let range_len = range.1 - range.0;
        let data_bytes = data.bytes();
        let mut index = 0;
        for i in data_bytes {
            if index > range_len { break; }
            disk::set_byte_in_ram(index + range.0, i);
            index += 1;
        }
        print!("Created a new file with path ");
        info!("/");
        self.print_path(self.files.len() as u32 - 1);
        print!("\n");
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

pub fn find_file(name: [u8; 20]) -> (u32, i32, (u32, u32), [u8; 20]) {
    let files = {
        FILESYSTEM.lock().get_file_from_current_parent()
    };
    for file in files {
        if file.3 == name {
            return file
        }
    }
    println!("This file doesn't seem to exist :c");
    return (0, 0, (0, 0,), [0; 20])
}

pub fn create_file(parent: i32, range: (u32, u32), filename: &str, data: &str) {
    let mut filename_bytes = [0; 20];
    let mut filename_bytes_len = 0;
    let filename_parsed = filename.bytes();
    for byte in filename_parsed {
        filename_bytes[filename_bytes_len] = byte;
        filename_bytes_len += 1;
    }
    FILESYSTEM.lock().create_file(parent, range, filename_bytes, data);
}

pub fn read_file(name: [u8; 20]) {
    let file = find_file(name);
    let file_start = file.2.0;
    let file_end = file.2.1;

    for i in file_start..file_end {
        let byte = disk::get_byte_in_ram(i);
        print!("{}", byte as char);
    }
    print!("\n");
}

pub fn run_file(name: [u8; 20]) {
    let file = find_file(name);
    let file_start = file.2.0;
    let file_end = file.2.1;

    let mut file_data: [usize; 255] = [0; 255];
    let mut file_index = 0;

    for i in file_start..file_end {
        let byte = disk::get_byte_in_ram(i);
        file_data[file_index] = byte as usize;
        file_index += 1;
    }

    femc::exec(file_data);
}

pub fn install_base_os() {
    println!("Installing FemDOS");
    create_file(1, (100, 111), "file1", "Hello world");
    create_file(1, (111, 112), "file2", "This is amazing");
    create_file(1, (112, 113), "file3", "This is a text file");
    create_file(1, (0, 0), "flow1", "");
    create_file(1, (0, 0), "flow2", "");
    create_file(6, (113, 114), "hidden file", "WOW YOU FOUND ME");
    create_file(1, (114, 115), "hidden file", "print 1 + 1");
    create_file(1, (115, 130), "python1", "print 1 + 10 * 10\nprint 10 + 10");
}