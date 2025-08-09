use core::arch::asm;

unsafe fn pci_config_read_u32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let address: u32 = (1 << 31)
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC);

    let value: u32;
    asm!(
        "out dx, eax",
        "in eax, dx",
        in("dx") 0xCF8u16,
        in("eax") address,
    );

    asm!(
        "in eax, dx",
        in("dx") 0xCFCu16,
        out("eax") value,
    );

    value
}

pub fn scan_devices() -> [(usize, usize); 255] {
    let mut index = 0;
    let mut devices_found: [(usize, usize); 255] = [(0, 0); 255];

    for bus in 0..=255 {
        for device in 0..32 {
            for function in 0..8 {
                let vendor_device = unsafe { pci_config_read_u32(bus, device, function, 0) };
                let vendor_id = vendor_device & 0xFFFF;
                if vendor_id == 0xFFFF {
                    continue;
                }
                let device_id = (vendor_device >> 16) & 0xFFFF;

                devices_found[index] = (vendor_id as usize, device_id as usize);
                index += 1;
            }
        }
    }

    devices_found
}