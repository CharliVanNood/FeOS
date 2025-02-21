#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(FemDOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::clear_screen();
    println!("----------------------------------");
    println!("This is my silly operating system:");
    println!("FemDOS!");
    println!("----------------------------------");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    FemDOS::test_panic_handler(info)
}