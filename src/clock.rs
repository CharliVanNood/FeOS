fn read_rtc_register(reg: u8) -> u8 {
    unsafe {
        let mut port_70 = Port::<u8>::new(0x70);
        let mut port_71 = Port::<u8>::new(0x71);
        port_70.write(reg);
        port_71.read()
    }
}