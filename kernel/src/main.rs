#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod window;
mod input;
mod applications;
mod renderer;
mod data;
mod vec;
mod filesystem;
mod disk;
mod string;
mod alloc;
mod clock;

use core::panic::PanicInfo;
use bootloader::{boot_info::MemoryRegionKind, BootInfo};

use alloc::{read_byte, write_byte};
use kernel::alloc::alloc;
use vec::Vec;

const VERSION: &str = env!("VERSION");

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    window::init();
    window::clear_screen();

    println!("-------------------------");
    println!("FemDOS!");
    println!("Data:");
    println!("Version: {}", VERSION);
    println!("Mem Offset: 0x{:x}", boot_info.physical_memory_offset.into_option().unwrap());
    println!("-------------------------");

    // Here we check for the biggest region of ram for the Heap
    let mut biggest_region = (0, 0, 0);
    for region in boot_info.memory_regions.iter() {
        if region.kind == MemoryRegionKind::Usable {
            // set the region to these bounds
            let memory_region_size = region.end - region.start;
            println!("FOUND USABLE size {:x}", memory_region_size);
            if memory_region_size > biggest_region.0 {
                biggest_region.0 = memory_region_size;
                biggest_region.1 = region.start;
                biggest_region.2 = region.end;
            }
        }
    }

    // set the sector bounds for heap :D
    alloc::set_heap(boot_info.physical_memory_offset.into_option().unwrap() as usize + biggest_region.1 as usize, biggest_region.0 as usize);
    kernel::init(boot_info, biggest_region);

    println!("Initialized components!");

    // TESTS START HERE :D
    #[cfg(test)]
    test_main();

    // This test is to make sure a byte gets properly allocated
    let address = alloc::alloc(1);
    write_byte(address.0, 255);
    let test_byte = read_byte(address.0);
    // The byte is set to 255, so it should return 255
    if test_byte == 255 {
        infoln!("[YAY] Ram test");
    } else {
        warnln!("[AWW] Ram test");
    }

    // Check in what level the kernel is operating
    disk::print_ring();

    // Get the size of the disk in sectors of 512 bytes :O
    let sectors = disk::get_sector_count();
    println!("Amount of sectors: {}", sectors);
    println!("Disk size: {} MB", sectors as u64 * 512 / 1024 / 1024);

    // Create a read buffer
    let mut read_buffer = [0u16; 256];
    disk::read_sector(0, &mut read_buffer);

    // Write to sector 1 and make sure it returns correctly
    let write_buffer = [0xABCDu16; 256];
    disk::write_sector(1, &write_buffer);
    disk::read_sector(1, &mut read_buffer);
    let write_successfull = read_buffer == [0xABCDu16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 1");
    } else {
        warnln!("[AWW] Disk write sector 1");
    }

    // Write to sector 2 and make sure it returns correctly
    let write_buffer = [0x1234u16; 256];
    disk::write_sector(2, &write_buffer);
    disk::read_sector(2, &mut read_buffer);
    let write_successfull = read_buffer == [0x1234u16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 2");
    } else {
        warnln!("[AWW] Disk write sector 2");
    }

    // Write to sector 3 and make sure it returns correctly
    let write_buffer = [0x5678u16; 256];
    disk::write_sector(3, &write_buffer);
    disk::read_sector(3, &mut read_buffer);
    let write_successfull = read_buffer == [0x5678u16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 3");
    } else {
        warnln!("[AWW] Disk write sector 3");
    }

    // Write to sector 4 and make sure it DOES NOT returns correctly, this is intended to fail
    let write_buffer = [0x1369u16; 256];
    disk::write_sector(4, &write_buffer);
    disk::read_sector(4, &mut read_buffer);
    let write_successfull = read_buffer == [0xABCDu16; 256];
    if !write_successfull {
        infoln!("[YAY] Disk write sector 4");
    } else {
        warnln!("[AWW] Disk write sector 4");
    }

    // create a test vec and remove it
    let mut test_vec_1 = Vec::new();
    test_vec_1.add(1);
    test_vec_1.add(2);
    test_vec_1.add(3);
    test_vec_1.remove();

    // create another text vec and remove it again
    let mut test_vec_2 = Vec::new();
    test_vec_2.add(4);
    test_vec_2.add(5);
    test_vec_2.add(6);
    test_vec_2.remove();

    // check if the vectors got removed correctly
    let ram_usage = alloc::get_usage();
    if ram_usage.0 == 8 {
        infoln!("[YAY] Heap vectors");
    } else {
        warnln!("[AWW] Heap vectors {}", ram_usage.0);
    }

    println!("Done testing!");

    println!("-------------------------");
    println!("| Yippee FemDOS booted! |");
    println!("-------------------------");

    kernel::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    warnln!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}