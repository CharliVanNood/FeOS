use x86_64::instructions::port::Port;

use crate::println;

fn read_rtc_register(reg: u8) -> u8 {
    unsafe {
        let mut port_70 = Port::<u8>::new(0x70);
        let mut port_71 = Port::<u8>::new(0x71);
        port_70.write(reg);
        port_71.read()
    }
}

fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

pub fn print_time() {
    println!("{:02}:{:02}:{:02}", bcd_to_binary(read_rtc_register(0x04)), bcd_to_binary(read_rtc_register(0x02)), bcd_to_binary(read_rtc_register(0x00)));
}