use lazy_static::lazy_static;
use spin::Mutex;

/*pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nostack, nomem));
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nostack, nomem));
    result
}

pub unsafe fn outw(port: u16, value: u16) {
    asm!("out dx, ax", in("dx") port, in("ax") value, options(nostack, nomem));
}

pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("in ax, dx", out("ax") result, in("dx") port, options(nostack, nomem));
    result
}

pub unsafe fn ata_write_sector(lba: u32, buffer: &[u16]) {
    while (inb(0x1F7) & 0x80) != 0 {}

    outb(0x1F6, ((lba >> 24) as u8) | 0xE0);

    outb(0x1F2, 1);
    outb(0x1F3, (lba & 0xFF) as u8);
    outb(0x1F4, ((lba >> 8) & 0xFF) as u8);
    outb(0x1F5, ((lba >> 16) & 0xFF) as u8);

    outb(0x1F7, 0x30);

    while (inb(0x1F7) & 0x08) == 0 {}

    for &word in buffer.iter().take(256) {
        outw(0x1F0, word);
    }

    outb(0x1F7, 0xE7);

    while (inb(0x1F7) & 0x80) != 0 {print!(".")}
}

pub unsafe fn ata_read_sector(lba: u32, buffer: &mut [u16]) {
    println!("Waiting for Disk");
    while (inb(0x1F7) & 0x80) == 0 {
        let status = inb(0x1F7);
        if (status & 0x01) != 0 {
            warnln!("Error detected while loading disk!");
            break;
        }
    }
    let status = inb(0x1F7);
    println!("Status after loading: 0x{:X}", status);
    println!("Disk is ready!");

    outb(0x1F6, ((lba >> 24) as u8) | 0xE0);
    println!("Selected drive port 0x1F6");

    outb(0x1F2, 1);
    outb(0x1F3, (lba & 0xFF) as u8);
    outb(0x1F4, ((lba >> 8) & 0xFF) as u8);
    outb(0x1F5, ((lba >> 16) & 0xFF) as u8);
    println!("Set LBA address");
    
    outb(0x1F7, 0x04);
    println!("Resetting drive");
    while (inb(0x1F7) & 0x80) == 0 {}
    while (inb(0x1F7) & 0x08) == 0 {}
    println!("Disk has been reset!");

    outb(0x1F7, 0x20);
    while (inb(0x1F7) & 0x40) != 0 {}

    let status = inb(0x1F7);
    println!("Status read request: 0x{:X}", status);
    println!("Sent read command!");

    println!("Waiting for Data transfer");
    while (inb(0x1F7) & 0x88) != 0x08 {}
    println!("Disk is ready for Data transfer!");

    for word in buffer.iter_mut().take(256) {
        *word = inw(0x1F0);
    }
}


pub fn check_mbr() -> bool {
    println!("Checking for any mbr data!");
    let mut buffer = [0u16; 256];
    unsafe { ata_read_sector(0, &mut buffer) };
    println!("The first sector has been read :D");

    let signature = (buffer[255] >> 8, buffer[255] & 0xFF);

    if signature == (0x55, 0xAA) {
        println!("There is a signature present!");
    } else {
        println!("No signature has been found");
    }

    signature == (0x55, 0xAA)
}*/

pub struct DiskLoaded {
    data: [u8; 10000]
}
impl DiskLoaded {
    pub fn get_byte(&self, index: u32) -> u8 {
        self.data[index as usize]
    }

    pub fn set_byte(&mut self, index: u32, value: u8) {
        self.data[index as usize] = value;
    }
}

lazy_static! {
    pub static ref DISKLOADED: Mutex<DiskLoaded> = Mutex::new(DiskLoaded {
        data: [0; 10000]
    });
}

pub fn get_byte_in_ram(index: u32) -> u8 {
    DISKLOADED.lock().get_byte(index)
}
pub fn set_byte_in_ram(index: u32, value: u8) {
    DISKLOADED.lock().set_byte(index, value);
}