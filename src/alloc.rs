use core::ptr;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{println, warnln};

pub struct Allocator {
    heap_start: usize,
    heap_end: usize,
    heap_size: usize,
    next: usize,
}

impl Allocator {
    pub fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: heap_start + heap_size,
            heap_size: heap_size,
            next: heap_start,
        }
    }

    fn set_heap(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
        self.heap_size = heap_size;
    }

    pub fn alloc(&mut self, size: usize) -> usize {
        println!("ALLOCATING {} to {}", self.next, self.next + size);
        if self.next + size > self.heap_end {
            warnln!("Address 0x{:x} is out of range", self.next + size);
            return 0;
        }

        self.next += size;
        self.next
    }
}

lazy_static! {
    pub static ref ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new(0, 0));
}

pub fn get_usage() -> (usize, usize) {
    let next = { ALLOCATOR.lock().next };
    let heap_start = { ALLOCATOR.lock().heap_start };
    let heap_size = { ALLOCATOR.lock().heap_size };
    (next - heap_start, heap_size)
}

pub fn set_heap(heap_start: usize, heap_size: usize) {
    ALLOCATOR.lock().set_heap(heap_start, heap_size);
}

pub fn alloc(size: usize) -> usize{
    ALLOCATOR.lock().alloc(size)
}

pub fn write_byte(address: usize, value: usize) {
    unsafe {
        let heap_start = { ALLOCATOR.lock().heap_start };
        let heap_end = { ALLOCATOR.lock().heap_end };
        if address + heap_start > heap_end {
            warnln!("Address 0x{:x} is out of range! :C", address + heap_start);
        } else {
            ptr::write((address + heap_start) as *mut usize, value);
        }
    }
}

pub fn read_byte(address: usize) -> usize {
    unsafe {
        let heap_start = { ALLOCATOR.lock().heap_start };
        let heap_end = { ALLOCATOR.lock().heap_end };
        if address + heap_start > heap_end {
            warnln!("Address 0x{:x} is out of range! :C", address + heap_start);
        } else {
            return ptr::read((address + heap_start) as *mut usize);
        }
    }

    0
}