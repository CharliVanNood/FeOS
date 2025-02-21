#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;

use core::panic::PanicInfo;

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Heyyy we're quickly gonna do {} tests", tests.len());

    for test in tests {
        test.run();
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn(), {
    fn run(&self) {
        print!("{}... ", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    loop {}
}

#[test_case]
fn first_test() {
    assert_eq!(1, 1);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}