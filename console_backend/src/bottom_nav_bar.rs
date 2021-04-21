
use capnp::message::Builder;
use capnp::serialize;
use serialport::{available_ports, SerialPortInfo};

use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;


use crate::console_backend_capnp as m;
use crate::types::{MessageSender, SharedState};

/// BottomNavBar struct.
///
/// # Fields:
///
/// - `available_ports` - The available serial ports to send to frontend for selection.
/// - `available_baudrates` - The available units of measure to send to frontend for selection.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
#[derive(Debug)]
pub struct BottomNavBar<S: MessageSender> {
    pub available_ports: Vec<SerialPortInfo>,
    pub available_baudrates: Vec<u32>,
    pub client_sender: S,
    pub shared_state: SharedState,
}
impl <S: MessageSender> BottomNavBar <S> {
    pub fn new(
        shared_state: SharedState,
        client_sender: S,
    ) -> BottomNavBar<S> {
        BottomNavBar {
            available_baudrates: vec![],
            available_ports: vec![],
            client_sender,
            shared_state,
        }
    }

    fn get_ports(&mut self) {
        if let Ok(ports) = available_ports() {
            self.available_ports = ports;
        }        
    }

    pub fn refresh_ports(&mut self) {
        self.get_ports();
        self.send_data();
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();

        let mut bottom_navbar_status = msg.init_bottom_navbar_status();
        
        let mut available_ports = bottom_navbar_status
            .reborrow()
            .init_available_ports(self.available_ports.len() as u32);

        for (i, serialportinfo) in self.available_ports.iter().enumerate() {
            available_ports.set(i as u32, &(*serialportinfo).port_name);
        }

        let mut available_baudrates = bottom_navbar_status
            .reborrow()
            .init_available_baudrates(self.available_baudrates.len() as u32);

        for (i, baudrate) in self.available_baudrates.iter().enumerate() {
            available_baudrates.set(i as u32, *baudrate);
        }
        

        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();

        self.client_sender.send_data(msg_bytes);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TestSender;

    #[test]
    fn get_ports_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut bottom_navbar = BottomNavBar::new(shared_state, client_send);
        bottom_navbar.get_ports();
        
    }
}
