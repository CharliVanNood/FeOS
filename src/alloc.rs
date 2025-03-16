use core::alloc::Layout;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::ptr;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{println, warnln};

pub struct Allocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

impl Allocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: heap_start + heap_size,
            next: AtomicUsize::new(heap_start),
        }
    }

    pub fn set_heap(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = AtomicUsize::new(heap_start);
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();

        let current = self.next.load(Ordering::Relaxed);

        let aligned = (current + (align - 1)) & !(align - 1);

        let new_next = aligned + size;
        if new_next > self.heap_end {
            warnln!("Address 0x{:x} is out of range", new_next);
            return ptr::null_mut();
        }

        self.next.store(new_next, Ordering::Relaxed);

        aligned as *mut u8
    }
}

lazy_static! {
    pub static ref ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new(0, 0));
}

pub fn write_byte(address: usize, value: u8) {
    unsafe {
        let heap_start = { ALLOCATOR.lock().heap_start };
        let heap_end = { ALLOCATOR.lock().heap_end };
        if address + heap_start > heap_end {
            warnln!("Address 0x{:x} is out of range! :C", address + heap_start);
        } else {
            ptr::write((address + heap_start) as *mut u8, value);
        }
    }
}

pub fn read_byte(address: usize) -> u8 {
    unsafe {
        let heap_start = { ALLOCATOR.lock().heap_start };
        let heap_end = { ALLOCATOR.lock().heap_end };
        if address + heap_start > heap_end {
            warnln!("Address 0x{:x} is out of range! :C", address + heap_start);
        } else {
            return ptr::read((address + heap_start) as *mut u8);
        }
    }

    0
}