use crate::{alloc, applications::{assembly, basic, femc}, data::basefiles, info, print, println, string::BigString, vec::{BigVec, FileVec}, warnln};

use lazy_static::lazy_static;
use spin::Mutex;

pub struct FileSystem {
    files: FileVec,
    flow: i32,
}
impl FileSystem {
    fn _get_item_name(&self, id: u32) -> [u8; 20] {
        self.files.iter()[id as usize].3
    }

    // Print the current path of a file
    fn print_path(&self, id: u32) {
        let file = self.files.iter()[id as usize];
        if file.1 != -1 {
            // Recursively go up the stack of referenced files
            self.print_path(file.1 as u32);
            info!("/");
        }
        for byte in file.3 {
            if byte == 0 { break; }
            info!("{}", byte as char);
        }
    }

    // Create a file and write it's contents to heap (for now, this will convert to disk)
    pub fn create_file(&mut self, parent: i32, filename: [u8; 20], filetype: u8, data: BigVec) {
        let address = alloc::alloc(data.len());

        let mut index = 0;
        for i in 0..data.len() {
            let value = data.get(i);
            if address.0 + index > address.1 {
                break;
            }
            alloc::write_byte(address.0 + index, value as usize);
            index += 8;
        }
        data.remove();

        self.files.add((self.files.len() as u32, parent, (address.0, address.1, index / 8), filename, filetype));

        print!("Created a new file with path ");
        info!("/");
        self.print_path(self.files.len() as u32 - 1);
        print!("\n");
    }

    pub fn set_flow(&mut self, flow: i32) {
        self.flow = flow;
    }
    pub fn flow_back(&mut self) {
        let parent_flow = self.files.get(self.flow as usize).1;
        if parent_flow == -1 { return; }
        self.flow = parent_flow;
    }

    // this being a list of 20 is the max amount of files that will be returned, why 20? sounds good to me tbh :3
    pub fn get_files_from_parent(&self, parent: i32) -> [(u32, i32, (usize, usize, usize), [u8; 20], u8); 20] {
        let mut files_returning = [(0, -1, (0, 1, 0), [1; 20], 0); 20];
        let mut files_returning_len = 0;
        for file in self.files.iter() {
            if file.1 == parent && files_returning_len < 20 {
                files_returning[files_returning_len] = file;
                files_returning_len += 1;
            }
        }
        files_returning
    }

    pub fn get_files_from_current_parent(&self) -> [(u32, i32, (usize, usize, usize), [u8; 20], u8); 20] {
        self.get_files_from_parent(self.flow)
    }

    pub fn get_all_indexes(&self) -> FileVec {
        self.files
    }
}

lazy_static! {
    pub static ref FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem {
        files: FileVec::new(),
        flow: 1
    });
}

// Get all files from the current directory and print those (Used in FL command)
pub fn print_current_dir_files() {
    let files_found = FILESYSTEM.lock().get_files_from_current_parent();
    for file in files_found {
        if file.1 == -1 { continue; }
        for char_byte in file.3 {
            if char_byte == 0 { break; }
            print!("{}", char_byte as char);
        }
        print!("\n");
    }
}

// Change the flow to a named flow in the same flow you're currently in
pub fn change_flow(name: [u8; 20]) {
    let mut back = [0u8; 20];
    back[..4].copy_from_slice(b"back");

    let files = {
        FILESYSTEM.lock().get_files_from_current_parent()
    };
    for file in files {
        if file.3 == name {
            FILESYSTEM.lock().set_flow(file.0 as i32);
        } else if name == back {
            FILESYSTEM.lock().flow_back();
            break;
        }
    }
}

pub fn get_current_flow() -> i32 {
    FILESYSTEM.lock().flow
}

// Check if a certain file exists in the current flow
pub fn file_exists(name: [u8; 20]) -> bool {
    let files = {
        let files = FILESYSTEM.lock().get_files_from_current_parent();
        files.clone()
    };
    for file in files {
        let mut file_name: [u8; 20] = [0; 20];
        for byte in file.3.iter().enumerate() {
            if *byte.1 == 61 { break; }
            file_name[byte.0] = *byte.1;
        }
        if name == file_name {
            return true;
        }
    }
    return false;
}

// Get a certain named file in the current directory by name
pub fn find_file(name: [u8; 20]) -> (u32, i32, (usize, usize, usize), [u8; 20], u8) {
    let files = {
        let files = FILESYSTEM.lock().get_files_from_current_parent();
        files.clone()
    };
    for file in files {
        let mut file_name: [u8; 20] = [0; 20];
        for byte in file.3.iter().enumerate() {
            if *byte.1 == 61 { break; }
            file_name[byte.0] = *byte.1;
        }
        if name == file_name {
            return file
        }
    }
    println!("This file doesn't seem to exist :c");
    return (0, 0, (0, 0, 0), [0; 20], 0)
}

// Write data to already existing file
pub fn update_file(filename: [u8; 20], data: BigVec) {
    if !file_exists(filename) {
        warnln!("This file doesn't exist :c");
        return;
    }
    
    let file = find_file(filename);
    let file_start = file.2.0;
    let file_size = file.2.2;

    for i in 0..file_size {
        alloc::write_byte(file_start + i * 8, data.get(i));
    }
}

// Create a new file
pub fn create_file(parent: i32, mut filename: [u8; 20], filetype: &str, data: BigVec) {
    let mut filename_len = 0;

    let mut back = [0u8; 20];
    back[..4].copy_from_slice(b"back");

    if filename == back {
        warnln!("back is not a valid name");
        return;
    }

    let file_type = {
        match filetype {
            "a" => 1,   // assembly
            "b" => 2,   // basic
            "fc" => 3,  // FemC
            "txt" => 4, // Text
            _ => 0      // Flow
        }
    };

    let is_flow = filetype.bytes().len() == 0;

    for byte in filename {
        if byte == 0 { break; }
        filename_len += 1;
    }

    if !is_flow {
        filename[filename_len] = 61;
        filename_len += 1;
        let filetype_parsed = filetype.bytes();
        for byte in filetype_parsed {
            filename[filename_len] = byte;
            filename_len += 1;
        }
    }

    FILESYSTEM.lock().create_file(parent, filename, file_type, data);
}

// Create a new file from &str, mainly used in the kernel and will be depricated some day
pub fn create_file_from_str(parent: i32, filename: &str, filetype: &str, data: &str) {
    let mut filename_bytes = [0; 20];
    let mut filename_bytes_len = 0;

    if filename == "back" {
        warnln!("{} is not a valid name", filename);
        return;
    }

    let file_type = {
        match filetype {
            "a" => 1,   // assembly
            "b" => 2,   // basic
            "fc" => 3,  // FemC
            "txt" => 4, // Text
            _ => 0      // Flow
        }
    };

    let is_flow = filetype.bytes().len() == 0;

    let filename_parsed = filename.bytes();
    for byte in filename_parsed {
        filename_bytes[filename_bytes_len] = byte;
        filename_bytes_len += 1;
    }

    if !is_flow {
        filename_bytes[filename_bytes_len] = 61;
        filename_bytes_len += 1;
        let filetype_parsed = filetype.bytes();
        for byte in filetype_parsed {
            filename_bytes[filename_bytes_len] = byte;
            filename_bytes_len += 1;
        }
    }

    let mut data_string = BigVec::new();
    for byte in data.bytes() {
        data_string.add(byte as usize);
    }

    println!("halting");
    x86_64::instructions::hlt();

    FILESYSTEM.lock().create_file(parent, filename_bytes, file_type, data_string);
}

// Read a file by name and return it's contents as a big string
pub fn read_file(name: [u8; 20]) -> BigString {
    if !file_exists(name) {
        warnln!("This file doesn't exist :c");
        return BigString::empty();
    }
    
    let file = find_file(name);
    let file_start = file.2.0;
    let file_size = file.2.2;

    let mut data = BigString::new();

    for i in 0..file_size {
        if i > file_start + file_size { break; }
        let byte = alloc::read_byte(file_start + i * 8) as u8;
        data.add(byte);
    }
    
    data
}

// Convert image data to a BigVec and return this
pub fn read_image(name: [u8; 20]) -> BigVec {
    if !file_exists(name) {
        warnln!("Image doesn't exist :C");
        return BigVec::empty();
    }
    let file = find_file(name);
    let file_start = file.2.0;
    let file_size = file.2.2;

    let mut contents = BigVec::new();

    for i in 0..file_size {
        let byte = alloc::read_byte(file_start + i * 8);
        contents.add(byte);
    }
    println!("content size: {} pixels: {}", contents.len(), (contents.len() - 6) / 9);

    contents
}

// Execute a file, this switches executor based on file type
pub fn run_file(name: [u8; 20]) {
    if !file_exists(name) {
        warnln!("This file doesn't exist :c");
        return;
    }

    let file = find_file(name);
    let file_start = file.2.0;
    let file_size = file.2.2;
    let file_type = file.4;

    // Add your file type here to choose the right executor
    match file_type {
        1 => {
            let mut file_data: BigString = BigString::new();
            for i in 0..file_size {
                let byte = alloc::read_byte(file_start + i * 8) as u8;
                file_data.add(byte);
            }
            assembly::exec(file_data);
        }
        2 => {
            let mut file_data: [u8; 512] = [0; 512];
            for i in 0..file_size {
                if i >= 512 {
                    break;
                }
                let byte = alloc::read_byte(file_start + i * 8) as u8;
                file_data[i] = byte;
            }
            basic::exec(file_data);
        }
        3 => {
            let mut file_data: [u8; 256] = [0; 256];
            for i in 0..file_size {
                if i >= 256 {
                    break;
                }
                let byte = alloc::read_byte(file_start + i * 8) as u8;
                file_data[i] = byte;
            }
            femc::exec(file_data);
        }
        _ => warnln!("Unrecognized file type, I can't run this :C")
    }
}

pub fn install_base_os(install_images: bool) {
    println!("Installing FemDOS");
    basefiles::install_files(install_images);
}