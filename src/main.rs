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

use core::panic::PanicInfo;
use bootloader::BootInfo;

use alloc::{read_byte, write_byte};
use fem_dos::vec::Vec;

const VERSION: &str = env!("VERSION");

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    vga::clear_screen();
    println!("--------------------------------------");
    println!("| This is my silly operating system: |");
    println!("| FemDOS!                            |");
    println!("|                                    |");
    println!("| Data:                              |");
    println!("| Version: {}                    |", VERSION);
    println!("| Memory offset: 0x{:x}       |", boot_info.physical_memory_offset);
    println!("--------------------------------------");

    alloc::ALLOCATOR.lock().set_heap(boot_info.physical_memory_offset as usize, 0x5000000);

    #[cfg(test)]
    test_main();

    write_byte(0x5000000, 255);
    let test_byte = read_byte(0x5000000);
    if test_byte == 255 {
        println!("[YAY] Ram test was successfull :D");
    } else {
        warnln!("[AWW] Ram test failed :c");
    }

    let mut test_vec = Vec::new();
    test_vec.add(10);
    println!("test vec length: {}", test_vec.len());

    println!("Done testing!");

    // dissabled, this is not working for me yet
    // disk::check_mbr();
    fem_dos::init();

    println!("Done initializing components!");

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