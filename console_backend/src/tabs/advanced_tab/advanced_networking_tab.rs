// Copyright (c) 2022 Swift Navigation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use capnp::message::Builder;
use log::error;
use sbp::messages::{
    observation::{MsgBasePosEcef, MsgBasePosLlh, MsgObs, MsgObsDepA, MsgObsDepB, MsgObsDepC},
    piksi::{MsgNetworkStateReq, MsgNetworkStateResp},
    ConcreteMessage,
};
use sbp::Frame;
use std::collections::HashMap;

use std::net::UdpSocket;

use crate::client_sender::BoxedClientSender;
use crate::constants::WRITE_TO_DEVICE_SENDER_ID;
use crate::shared_state::{AdvancedNetworkingState, SharedState, TabName};
use crate::types::{MsgSender, Result};
use crate::utils::{bytes_to_human_readable, serialize_capnproto_builder};

const DEFAULT_UDP_LOCAL_ADDRESS: &str = "0.0.0.0";
const DEFAULT_UDP_LOCAL_PORT: u16 = 0;
const DEFAULT_UDP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_UDP_PORT: u16 = 13320;
const PPP0_HACK_STR: &str = "---";
const OBS_MSGS: &[u16] = &[
    MsgObs::MESSAGE_TYPE,
    MsgObsDepA::MESSAGE_TYPE,
    MsgObsDepB::MESSAGE_TYPE,
    MsgObsDepC::MESSAGE_TYPE,
    MsgBasePosLlh::MESSAGE_TYPE,
    MsgBasePosEcef::MESSAGE_TYPE,
];

struct NetworkState {
    pub interface_name: String,
    pub ipv4_address: String,
    pub running: bool,
    pub tx_usage: String,
    pub rx_usage: String,
}

/// AdvancedNetworkingTab struct.
pub struct AdvancedNetworkingTab {
    /// Whether or not to broadcast all messages over UDP or only the OBS_MSGS subset.
    all_messages: bool,
    /// The current udp socket connected to for streaming messages, if any.
    client: Option<UdpSocket>,
    /// Client Sender channel for communication from backend to frontend.
    client_sender: BoxedClientSender,
    /// The stored string IP address defaults to DEFAULT_UDP_ADDRESS.
    ip_ad: String,
    /// The stored ip traffic received from the device.
    network_info: HashMap<String, NetworkState>,
    /// The port to send packets over UDP defaults to DEFAULT_UDP_PORT.
    port: u16,
    /// Whether or not UDP streaming is happening, used to inform frontend.
    running: bool,
    /// The shared state for communicating between frontend/backend/other backend tabs.
    shared_state: SharedState,
    /// The MsgSender for sending NetworkState refresh requests to the device.
    writer: MsgSender,
}
impl AdvancedNetworkingTab {
    pub fn new(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
        writer: MsgSender,
    ) -> AdvancedNetworkingTab {
        let tab = AdvancedNetworkingTab {
            all_messages: false,
            client: None,
            client_sender,
            ip_ad: DEFAULT_UDP_ADDRESS.to_string(),
            network_info: HashMap::new(),
            port: DEFAULT_UDP_PORT,
            running: false,
            shared_state: shared_state.clone(),
            writer,
        };
        shared_state.set_advanced_networking_update(AdvancedNetworkingState {
            refresh: true,
            ..Default::default()
        });
        tab
    }

    pub fn handle_network_state_resp(&mut self, msg: MsgNetworkStateResp) {
        let mut tx_usage = bytes_to_human_readable(msg.tx_bytes as u128);
        let mut rx_usage = bytes_to_human_readable(msg.rx_bytes as u128);
        let interface_name = msg
            .interface_name
            .to_string()
            .trim_end_matches('\0')
            .to_string();
        if interface_name.starts_with("ppp0") {
            tx_usage = PPP0_HACK_STR.to_string();
            rx_usage = PPP0_HACK_STR.to_string();
        } else if interface_name.starts_with("lo") || interface_name.starts_with("sit0") {
            return;
        }
        let running = (msg.flags & (1 << 6)) != 0;
        let ipv4_address: Vec<String> = msg.ipv4_address.iter().map(|x| x.to_string()).collect();
        let ipv4_address = ipv4_address.join(".");
        self.network_info.insert(
            interface_name.clone(),
            NetworkState {
                interface_name,
                ipv4_address,
                running,
                tx_usage,
                rx_usage,
            },
        );
        self.send_data();
    }

    /// Refresh Network State.
    fn refresh_network_state(&mut self) -> Result<()> {
        self.network_info.clear();
        self.writer.send(MsgNetworkStateReq {
            sender_id: Some(WRITE_TO_DEVICE_SENDER_ID),
        })?;
        Ok(())
    }

    fn check_update(&mut self) {
        if let Some(update) = self.shared_state.advanced_networking_update() {
            if update.stop {
                self.stop_relay();
            }

            if let Some(ip_address) = update.ip_address {
                self.ip_ad = ip_address;
            }
            if let Some(port) = update.port {
                self.port = port;
            }
            if let Some(all_messages) = update.all_messages {
                self.all_messages = all_messages;
            }

            if update.start {
                if let Err(err) = self.start_relay() {
                    error!("Error starting relay: {}", err);
                }
            }
            if update.refresh {
                if let Err(err) = self.refresh_network_state() {
                    error!("Error refreshing network state: {}", err);
                }
            }
            self.send_data();
        }
    }

    pub fn update(&mut self, frame: &Frame) {
        self.check_update();

        if self.running {
            if let Some(client) = &mut self.client {
                if let Some(msg_type) = &frame.msg_type() {
                    if self.all_messages || OBS_MSGS.contains(msg_type) {
                        if let Err(_e) = client.send(frame.as_bytes()) {
                            // Need to squelch error for the case of no client listening.
                        }
                    }
                }
            } else {
                self.running = false;
            }
        }
    }

    fn stop_relay(&mut self) {
        self.client.take();
        self.running = false;
    }

    fn start_relay(&mut self) -> Result<()> {
        let socket = UdpSocket::bind(format!(
            "{DEFAULT_UDP_LOCAL_ADDRESS}:{DEFAULT_UDP_LOCAL_PORT}"
        ))?;
        socket.set_nonblocking(true)?;
        socket.set_broadcast(true)?;
        socket.connect(format!("{}:{}", self.ip_ad.as_str(), self.port))?;
        self.client = Some(socket);
        self.running = true;
        Ok(())
    }

    /// Package data into a message buffer and send to frontend.
    pub fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Advanced {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut status = msg.init_advanced_networking_status();

        let mut entries = status
            .reborrow()
            .init_network_info(self.network_info.len() as u32);
        {
            for (i, (_, val)) in self.network_info.iter().enumerate() {
                let mut entry = entries.reborrow().get(i as u32);
                entry.set_interface_name(&val.interface_name);
                entry.set_ipv4_address(&val.ipv4_address);
                entry.set_running(val.running);
                entry.set_tx_usage(&val.tx_usage);
                entry.set_rx_usage(&val.rx_usage);
            }
        }
        status.set_running(self.running);
        status.set_ip_address(&self.ip_ad);
        status.set_port(self.port);
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sender::TestSender;
    use crate::utils::fixed_sbp_string;
    use sbp::{sbp_string::Unterminated, SbpString};
    use std::io::sink;

    #[test]
    fn handle_network_state_resp_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let writer = MsgSender::new(sink());
        let mut tab = AdvancedNetworkingTab::new(shared_state, client_send, writer);
        let tx_bytes = 1;
        let rx_bytes = 2;
        let sender_id = Some(1337);
        let interface_name_pre = "eth0";
        let interface_name: SbpString<[u8; 16], Unterminated> =
            fixed_sbp_string(interface_name_pre);
        let ipv4_address = [127, 0, 0, 1];
        let ipv4_mask_size = 0;
        let ipv6_address = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let ipv6_mask_size = 0;
        let flags = 0b1000000;
        let msg = MsgNetworkStateResp {
            sender_id,
            interface_name: interface_name.clone(),
            ipv4_address,
            ipv4_mask_size,
            ipv6_address,
            ipv6_mask_size,
            tx_bytes,
            rx_bytes,
            flags,
        };

        assert!(tab.network_info.is_empty());
        tab.handle_network_state_resp(msg);
        assert_eq!(tab.network_info.len(), 1);
        let entry = tab.network_info.get(interface_name_pre).unwrap();
        assert_eq!(entry.ipv4_address, "127.0.0.1");
        assert!(entry.running);
        assert_eq!(entry.tx_usage, format!("{:.1}B", tx_bytes as f64));
        assert_eq!(entry.rx_usage, format!("{:.1}B", rx_bytes as f64));
        let bad_flags = 0b0100000;
        let msg = MsgNetworkStateResp {
            sender_id,
            interface_name,
            ipv4_address,
            ipv4_mask_size,
            ipv6_address,
            ipv6_mask_size,
            tx_bytes,
            rx_bytes,
            flags: bad_flags,
        };

        tab.handle_network_state_resp(msg);
        assert_eq!(tab.network_info.len(), 1);
        let entry = tab.network_info.get(interface_name_pre).unwrap();
        assert!(!entry.running);

        let interface_name_pre = "ppp0";
        let interface_name: SbpString<[u8; 16], Unterminated> =
            fixed_sbp_string(interface_name_pre);
        let ipv4_address = [192, 168, 0, 1];
        let msg = MsgNetworkStateResp {
            sender_id,
            interface_name,
            ipv4_address,
            ipv4_mask_size,
            ipv6_address,
            ipv6_mask_size,
            tx_bytes,
            rx_bytes,
            flags,
        };

        tab.handle_network_state_resp(msg);
        assert_eq!(tab.network_info.len(), 2);
        let entry = tab.network_info.get(interface_name_pre).unwrap();
        assert_eq!(entry.ipv4_address, "192.168.0.1");
        assert!(entry.running);
        assert_eq!(entry.tx_usage, PPP0_HACK_STR.to_string());
        assert_eq!(entry.rx_usage, PPP0_HACK_STR.to_string());

        tab.network_info.clear();
        assert!(tab.network_info.is_empty());

        ["lo", "sit0"].iter_mut().for_each(|interface_name_pre| {
            let interface_name: SbpString<[u8; 16], Unterminated> =
                fixed_sbp_string(interface_name_pre);
            let msg = MsgNetworkStateResp {
                sender_id,
                interface_name,
                ipv4_address,
                ipv4_mask_size,
                ipv6_address,
                ipv6_mask_size,
                tx_bytes,
                rx_bytes,
                flags,
            };

            tab.handle_network_state_resp(msg);
            assert!(tab.network_info.is_empty());
        });
    }
}
