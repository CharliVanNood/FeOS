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
use fem_dos::{alloc::alloc, string::BigString};

const VERSION: &str = env!("VERSION");

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    vga::clear_screen();
    println!("--------------------------------------");
    println!("| This is my silly operating system: |");
    println!("| FemDOS!                            |");
    println!("|                                    |");
    println!("| Data:                              |");
    println!("| Version: {}                   |", VERSION);
    println!("| Memory offset: 0x{:x}       |", boot_info.physical_memory_offset);
    println!("--------------------------------------");

    // dissabled, this is not working for me yet
    // disk::check_mbr();
    alloc::set_heap(boot_info.physical_memory_offset as usize, 0x5000000);
    fem_dos::init(boot_info);

    let test_string = BigString::from("test a b c");
    if test_string.includes("es") != -1{
        infoln!("String includes es");
    } else {
        warnln!("String doesn't include es");
    }
    test_string.print();

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