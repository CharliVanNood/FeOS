#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod vga;
pub mod interrupts;
pub mod gdt;
pub mod input;
pub mod applications;
pub mod filesystem;
pub mod vec;
//pub mod disk;

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
    hlt_loop();
}

#[test_case]
fn first_test() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn hlt_loop() -> ! {
    print!("-> ");
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    filesystem::create_file(-1, (0, 100), "root");
    filesystem::create_file(1, (100, 101), "file1");
    filesystem::create_file(1, (101, 102), "file2");
    filesystem::create_file(1, (102, 103), "file3");

    println!("Enabling Global Descriptor Table");
    gdt::init();
    println!("Enabling CPU interrupts");
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    println!("Interrupts have been initialized :D");
}