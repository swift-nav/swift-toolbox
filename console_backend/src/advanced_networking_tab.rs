use capnp::message::Builder;
use log::error;
use ordered_float::OrderedFloat;
use sbp::messages::piksi::{
    MsgDeviceMonitor, MsgNetworkStateReq, MsgNetworkStateResp, MsgReset, MsgThreadState,
};
use sbp::messages::system::{MsgCsacTelemetry, MsgCsacTelemetryLabels};
use sbp::messages::{SBPMessage, SBP};
use std::collections::HashMap;
use sysinfo::NetworkData;

use std::net::UdpSocket;

use crate::constants::WRITE_TO_DEVICE_SENDER_ID;
use crate::shared_state::{AdvancedNetworkingState, SharedState};
use crate::types::{CapnProtoSender, MsgSender, Result, UartState};
use crate::utils::{bytes_to_human_readable, serialize_capnproto_builder};

const DEFAULT_UDP_LOCAL_ADDRESS: &str = "127.0.0.1";
const DEFAULT_UDP_LOCAL_PORT: u16 = 34254;
const DEFAULT_UDP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_UDP_PORT: u16 = 13320;
const PPP0_HACK_STR: &str = "---";

const OBS_MSGS: &[u16] = &[
    67, /* MsgObsDepB */
    68, /* MsgBasePosLLH */
    72, /* MsgBasePosEcef */
    73, /* MsgObsDepC */
    74, /* MsgObs */
];

struct NetworkState {
    pub interface_name: String,
    pub ipv4_address: String,
    pub running: bool,
    pub tx_usage: String,
    pub rx_usage: String,
}

/// AdvancedNetworkingTab struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
pub struct AdvancedNetworkingTab<S: CapnProtoSender> {
    all_messages: bool,
    client: Option<UdpSocket>,
    client_sender: S,
    ip_ad: String,
    network_info: HashMap<String, NetworkState>,
    port: u16,
    running: bool,
    shared_state: SharedState,
    wtr: MsgSender,
}
impl<S: CapnProtoSender> AdvancedNetworkingTab<S> {
    pub fn new(
        shared_state: SharedState,
        client_sender: S,
        wtr: MsgSender,
    ) -> AdvancedNetworkingTab<S> {
        let tab = AdvancedNetworkingTab {
            all_messages: false,
            client: None,
            client_sender,
            ip_ad: DEFAULT_UDP_ADDRESS.to_string(),
            network_info: HashMap::new(),
            port: DEFAULT_UDP_PORT,
            running: false,
            shared_state: shared_state.clone(),
            wtr,
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
        let msg = MsgNetworkStateReq {
            sender_id: Some(WRITE_TO_DEVICE_SENDER_ID),
        };
        let msg = sbp::messages::SBP::from(msg);
        self.wtr.send(msg)?;
        Ok(())
    }

    fn check_update(&mut self) {
        if let Some(update) = self.shared_state.advanced_networking_update() {
            if update.stop {
                self.stop_relay();
            }

            self.all_messages = update.all_messages;
            if let Some(ip_address) = update.ip_address {
                self.ip_ad = ip_address;
            }
            if let Some(port) = update.port {
                self.port = port;
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

    pub fn handle_sbp(&mut self, msg: &SBP) {
        self.check_update();

        if self.running {
            if let Some(client) = &mut self.client {
                if self.all_messages || OBS_MSGS.contains(&msg.get_message_type()) {
                    if let Ok(frame) = msg.to_frame() {
                        if let Err(err) = client.send(&frame) {
                            error!("Error sending to device: {}", err);
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
            "{}:{}",
            DEFAULT_UDP_LOCAL_ADDRESS, DEFAULT_UDP_LOCAL_PORT
        ))?;
        socket.set_nonblocking(true)?;
        socket.connect(format!("{}:{}", self.ip_ad.as_str(), self.port))?;
        self.client = Some(socket);
        self.running = true;
        Ok(())
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::types::TestSender;
//     use sbp::{
//         messages::piksi::{Latency, MsgUartState, MsgUartStateDepa, Period, UARTChannel},
//         SbpString,
//     };
//     use std::io::sink;

//     #[test]
//     fn handle_uart_state_test() {
//         let shared_state = SharedState::new();
//         let client_send = TestSender { inner: Vec::new() };
//         let wtr = MsgSender::new(sink());
//         let mut tab = AdvancedNetworkingTab::new(shared_state, client_send, wtr);
//         let sender_id = Some(1337);
//         let uart_a = UARTChannel {
//             tx_throughput: 0.0,
//             rx_throughput: 0.0,
//             crc_error_count: 0,
//             io_error_count: 0,
//             tx_buffer_level: 0,
//             rx_buffer_level: 0,
//         };
//         let uart_b = uart_a.clone();
//         let uart_ftdi = uart_a.clone();
//         let avg = 4;
//         let current = 3;
//         let lmin = 2;
//         let lmax = 1;
//         let no_period = 0;
//         let latency = Latency {
//             avg,
//             current,
//             lmin,
//             lmax,
//         };
//         tab.handle_uart_state(UartState::MsgUartStateDepa(MsgUartStateDepa {
//             sender_id,
//             uart_a: uart_a.clone(),
//             uart_b: uart_b.clone(),
//             uart_ftdi: uart_ftdi.clone(),
//             latency,
//         }));
//         assert_eq!(*tab.obs_latency.get(&CURR.to_string()).unwrap(), current);
//         assert_eq!(*tab.obs_latency.get(&AVG.to_string()).unwrap(), avg);
//         assert_eq!(*tab.obs_latency.get(&MIN.to_string()).unwrap(), lmin);
//         assert_eq!(*tab.obs_latency.get(&MAX.to_string()).unwrap(), lmax);
//         assert_eq!(*tab.obs_period.get(&CURR.to_string()).unwrap(), no_period);
//         assert_eq!(*tab.obs_period.get(&AVG.to_string()).unwrap(), no_period);
//         assert_eq!(*tab.obs_period.get(&MIN.to_string()).unwrap(), no_period);
//         assert_eq!(*tab.obs_period.get(&MAX.to_string()).unwrap(), no_period);
//         let avg = 1;
//         let current = 2;
//         let lmin = 3;
//         let lmax = 4;
//         let pmin = 5;
//         let pmax = 6;
//         let latency = Latency {
//             avg,
//             current,
//             lmin,
//             lmax,
//         };
//         let obs_period = Period {
//             pmin,
//             pmax,
//             avg,
//             current,
//         };
//         tab.handle_uart_state(UartState::MsgUartState(MsgUartState {
//             sender_id,
//             uart_a,
//             uart_b,
//             uart_ftdi,
//             latency,
//             obs_period,
//         }));
//         assert_eq!(*tab.obs_latency.get(&CURR.to_string()).unwrap(), current);
//         assert_eq!(*tab.obs_latency.get(&AVG.to_string()).unwrap(), avg);
//         assert_eq!(*tab.obs_latency.get(&MIN.to_string()).unwrap(), lmin);
//         assert_eq!(*tab.obs_latency.get(&MAX.to_string()).unwrap(), lmax);
//         assert_eq!(*tab.obs_period.get(&CURR.to_string()).unwrap(), current);
//         assert_eq!(*tab.obs_period.get(&AVG.to_string()).unwrap(), avg);
//         assert_eq!(*tab.obs_period.get(&MIN.to_string()).unwrap(), pmin);
//         assert_eq!(*tab.obs_period.get(&MAX.to_string()).unwrap(), pmax);
//     }

// }
