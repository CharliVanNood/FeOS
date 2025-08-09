use lazy_static::lazy_static;
use spin::Mutex;

use crate::{pci, println};

pub struct Network {
    connection_type: usize // 0 = none, 1 = ethernet, 2 = wifi
}
impl Network {
    fn init() -> Self {
        Self {
            connection_type: 0
        }
    }

    fn get_connection_type(&self) -> usize {
        self.connection_type
    }

    fn get_network_devices(&mut self) {
        let devices = pci::scan::scan_devices();

        for i in 0..255 {
            if devices[i].0 == 0 && devices[i].1 == 0 { break; }
            match (devices[i].0, devices[i].1) {
                (32902, 4110) => {
                    println!("Found QEMU network card");
                    self.connection_type = 1;
                    break;
                },
                _ => {}
            }
        }
    }

    fn connect(&mut self) {
        println!("connection attempt");
        self.get_network_devices();
    }
}

lazy_static! {
    pub static ref NETWORK: Mutex<Network> = Mutex::new(Network::init());
}

pub fn get_connection_type() -> usize {
    NETWORK.lock().get_connection_type()
}

pub fn connect() {
    NETWORK.lock().connect();
}