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

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::clear_screen();
    println!("--------------------------------------");
    println!("| This is my silly operating system: |");
    println!("| FemDOS!                            |");
    println!("--------------------------------------");

    #[cfg(test)]
    test_main();

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