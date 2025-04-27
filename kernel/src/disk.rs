use x86_64::instructions::port::Port;

use crate::{filesystem::FILESYSTEM, infoln, print, println, vec::BigVec, warnln};

// Write to sector by index
pub fn write_sector(lba: u32, data: &[u16]) {
    assert!(data.len() == 256, "ATA sector size must be 512 bytes (256 words) :3");

    let mut data_port = Port::<u16>::new(0x1F0);
    let mut sector_count_port = Port::<u8>::new(0x1F2);
    let mut lba_low_port = Port::<u8>::new(0x1F3);
    let mut lba_mid_port = Port::<u8>::new(0x1F4);
    let mut lba_high_port = Port::<u8>::new(0x1F5);
    let mut drive_head_port = Port::<u8>::new(0x1F6);
    let mut command_port = Port::<u8>::new(0x1F7);
    let mut status_port = Port::<u8>::new(0x1F7);

    unsafe {
        drive_head_port.write(0xE0 | ((lba >> 24) & 0x0F) as u8);

        sector_count_port.write(1);

        // Set the current sector being read from
        lba_low_port.write((lba & 0xFF) as u8);
        lba_mid_port.write(((lba >> 8) & 0xFF) as u8);
        lba_high_port.write(((lba >> 16) & 0xFF) as u8);

        // Send the write command
        command_port.write(0x30);

        // Wait untill ready
        while status_port.read() & 0x80 != 0 {}

        // Write the bytes
        for &word in data {
            data_port.write(word);
        }

        // Issue the flush command to save
        command_port.write(0xE7);
    }

    for _ in 0..100000 { x86_64::instructions::nop(); }
}

// Read from disk
pub fn read_sector(lba: u32, buffer: &mut [u16]) {
    assert!(buffer.len() == 256, "Buffer must hold 512 bytes (256 words):3");

    let mut data_port = Port::<u16>::new(0x1F0);
    let mut sector_count_port = Port::<u8>::new(0x1F2);
    let mut lba_low_port = Port::<u8>::new(0x1F3);
    let mut lba_mid_port = Port::<u8>::new(0x1F4);
    let mut lba_high_port = Port::<u8>::new(0x1F5);
    let mut drive_head_port = Port::<u8>::new(0x1F6);
    let mut command_port = Port::<u8>::new(0x1F7);
    let mut status_port = Port::<u8>::new(0x1F7);

    unsafe {
        drive_head_port.write(0xE0 | ((lba >> 24) & 0x0F) as u8);

        sector_count_port.write(1);

        lba_low_port.write((lba & 0xFF) as u8);
        lba_mid_port.write(((lba >> 8) & 0xFF) as u8);
        lba_high_port.write(((lba >> 16) & 0xFF) as u8);

        command_port.write(0x20);

        while status_port.read() & 0x80 != 0 {}

        for word in buffer.iter_mut() {
            *word = data_port.read();
        }
    }
}

pub fn get_sector_count() -> u32 {
    let mut command_port = Port::<u8>::new(0x1F7);
    let mut data_port = Port::<u16>::new(0x1F0);
    
    unsafe { command_port.write(0xEC) };

    while unsafe { command_port.read() } & 0x80 != 0 {}

    let mut sector_count: u32 = 0;
    for i in 0..100 {
        let word = unsafe { data_port.read() };
        if i == 60 {
            sector_count |= word as u32;
        } else if i == 61 {
            sector_count |= (word as u32) << 16;
        }
    }

    unsafe { command_port.write(0xE7) };
    for _ in 0..100000 { x86_64::instructions::nop(); }

    sector_count
}

#[warn(asm_sub_register)]
fn check_ring() -> i16 {
    let cpl: i16;
    unsafe { core::arch::asm!("mov {0:x}, cs", out(reg) cpl) };
    cpl & 0b11
}

pub fn print_ring() {
    let ring = check_ring();

    match ring {
        0 => infoln!("[YAY] Current ring: 0"),
        3 => warnln!("[AWW] Current ring: 3"),
        _ => warnln!("[AWW] Current ring: {}", ring)
    }
}

pub fn _read_file_indexes() {

}

pub fn print_byte_code(bytes_writing: BigVec) {
    for value in 0..bytes_writing.len() {
        print!("{}", bytes_writing.get(value));
    }
    println!("");
    for value in 0..bytes_writing.len() {
        let byte = bytes_writing.get(value) as u8;
        if byte == 0 { continue; }
        print!("{}", byte as char);
    }
    println!("");
}

pub fn convert_fs_to_bytes() -> BigVec {
    let mut bytes_writing = BigVec::new();
    let file_system_files = FILESYSTEM.lock().get_all_indexes();

    let file_system_size = file_system_files.len() * 26 * 2 + 131;
    let sector_amount = (file_system_size / 512) as usize + 1;
    bytes_writing.add(1);
    bytes_writing.add(sector_amount);

    for _ in 0..128 {
        bytes_writing.add(0);
    }

    for file_index in 0..file_system_files.len() {
        let file = file_system_files.get(file_index);
        if file == (0, -1, (0, 0, 0), [0; 20], 0) { continue; }
        let mut data: [usize; 27] = [0; 27];

        // set the file index
        data[0] = file.0 as usize;

        // set file exists
        if file.1 < 0 {
            data[1] = 1;
            data[2] = 0;
        }
        else {
            data[1] = 0;
            data[2] = file.1 as usize;
        }

        // set file byte offsets and sizes
        data[3] = file.2.0;
        data[4] = file.2.1;
        data[5] = file.2.2;

        // set file name
        for i in 0..file.3.len() {
            data[6 + i] = file.3[i] as usize;
        }

        // set file type
        data[26] = file.4 as usize;

        for byte in data {
            bytes_writing.add(byte & 0xFFFF);
            bytes_writing.add((byte >> 16) & 0xFFFF);
        }
    }

    print_byte_code(bytes_writing);

    bytes_writing
}

pub fn write_fs_to_disk() {
    let mut bytes = convert_fs_to_bytes();

    println!("Amount: {} Used: {}", bytes.get(1), bytes.get(0));

    let mut temp_sector: [u16; 256] = [0; 256];
    let mut temp_sector_index = 0;
    for i in 0..bytes.len() {
        temp_sector[temp_sector_index] = bytes.get(i) as u16;
        temp_sector_index += 1;
        if temp_sector_index == 256 {
            //write_sector(bytes.get(0) as u32, &temp_sector);
            write_sector(bytes.get(0) as u32, &temp_sector);
            print!(".");
            bytes.set(0, bytes.get(0) + 1);
            temp_sector_index = 0;
            temp_sector = [0; 256];
        }
    }

    bytes.remove();
}