#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fem_dos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod input;
mod applications;
mod vec;
mod filesystem;
mod disk;
mod string;
mod alloc;
mod clock;

use core::panic::PanicInfo;
use bootloader::BootInfo;

use alloc::{read_byte, write_byte};
use fem_dos::alloc::alloc;

const VERSION: &str = env!("VERSION");

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    vga::clear_screen();
    println!("--------------------------------------");
    println!("| This is my silly operating system: |");
    println!("| FemDOS!                            |");
    println!("|                                    |");
    println!("| Data:                              |");
    println!("| Version: {}                  |", VERSION);
    println!("| Memory offset: 0x{:x}       |", boot_info.physical_memory_offset);
    println!("--------------------------------------");

    // dissabled, this is not working for me yet
    //disk::check_mbr();
    //disk::test();

    alloc::set_heap(boot_info.physical_memory_offset as usize, 0x5000000);
    fem_dos::init(boot_info);

    println!("Done initializing components!");

    #[cfg(test)]
    test_main();

    let address = alloc::alloc(1);
    write_byte(address.0, 255);
    let test_byte = read_byte(address.0);
    if test_byte == 255 {
        infoln!("[YAY] Ram test was successfull :D");
    } else {
        warnln!("[AWW] Ram test failed :c");
    }

    disk::print_ring();

    let mut read_buffer = [0u16; 256];
    let write_buffer = [0xABCDu16; 256];
    disk::write_sector(1, &write_buffer);
    disk::read_sector(1, &mut read_buffer);
    let write_successfull = read_buffer == [0xABCDu16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 1 was successfull :D");
    } else {
        warnln!("[AWW] Disk write sector 1 failed :c");
        for byte in read_buffer.iter().enumerate() {
            if byte.0 > 15 {
                print!("\n");
                break;
            }
            warnln!("{:04x} ", byte.1);
        }
    }

    let write_buffer = [0x1234u16; 256];
    disk::write_sector(2, &write_buffer);
    disk::read_sector(2, &mut read_buffer);
    let write_successfull = read_buffer == [0x1234u16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 2 was successfull :D");
    } else {
        warnln!("[AWW] Disk write sector 2 failed :c");
        for byte in read_buffer.iter().enumerate() {
            if byte.0 > 15 {
                print!("\n");
                break;
            }
            warnln!("{:04x} ", byte.1);
        }
    }

    let write_buffer = [0x5678u16; 256];
    disk::write_sector(3, &write_buffer);
    disk::read_sector(3, &mut read_buffer);
    let write_successfull = read_buffer == [0x5678u16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 3 was successfull :D");
    } else {
        warnln!("[AWW] Disk write sector 3 failed :c");
        for byte in read_buffer.iter().enumerate() {
            if byte.0 > 15 {
                print!("\n");
                break;
            }
            warnln!("{:04x} ", byte.1);
        }
    }

    println!("Done testing!");

    println!("--------------------------------------");
    println!("| Yippee FemDOS has booted!          |");
    println!("--------------------------------------");

    fem_dos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    warnln!("{}", info);
    fem_dos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fem_dos::test_panic_handler(info)
}