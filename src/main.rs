#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(FemDOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod input;

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

    FemDOS::init();

    println!("Done initializing components!");

    println!("--------------------------------------");
    println!("| Yippee FemDOS has booted!          |");
    println!("--------------------------------------");

    FemDOS::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    FemDOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    FemDOS::test_panic_handler(info)
}