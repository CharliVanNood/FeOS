use volatile::Volatile;

const BUFFER_SIZE: usize = 320 * 200;

#[repr(transparent)]
struct Buffer {
    pixels: [Volatile<u8>; BUFFER_SIZE],
}

pub fn init() {
    let buffer = unsafe { &mut *(0xa0000 as *mut Buffer) };
    for i in 0..BUFFER_SIZE {
        buffer.pixels[i].write((i / 260) as u8);
    }
}
