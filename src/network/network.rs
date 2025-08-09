use lazy_static::lazy_static;
use spin::Mutex;

use crate::println;

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

    fn connect(&mut self) {
        println!("connection attempt");
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