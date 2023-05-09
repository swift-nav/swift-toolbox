use capnp::message::Builder;
use ordered_float::OrderedFloat;
use sbp::messages::piksi::{MsgDeviceMonitor, MsgThreadState};
use sbp::messages::system::{MsgCsacTelemetry, MsgCsacTelemetryLabels};
use std::collections::HashMap;

use crate::client_sender::BoxedClientSender;
use crate::types::UartState;
use crate::utils::{cc_to_c, normalize_cpu_usage, serialize_capnproto_builder};

const NO_NAME: &str = "(no name)";
const METRICS_OF_INTEREST: &[&str] = &["Status", "Alarm", "Mode", "Phase", "DiscOK"];
const CURR: &str = "Curr";
const AVG: &str = "Avg";
const MIN: &str = "Min";
const MAX: &str = "Max";
const UART_STATE_KEYS: &[&str] = &[CURR, AVG, MIN, MAX];

struct ThreadStateFields {
    name: String,
    cpu: OrderedFloat<f64>,
    stack_free: u32,
}

/// AdvancedSystemMonitorTab struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `csac_received`: CsacTelemetry received flag.
/// - `csac_telem_list`: Vec of CsacTelemetry metrics and corresponding values.
/// - `fe_temp`: RF frontend temperature reading.
/// - `headers`: Vec of CsacTelemetry metric labels.
/// - `obs_latency`: UART state latency measurements.
/// - `obs_period`: UART state period measurements.
/// - `telem_header_index`: Index of current CsacTelemetry label packet.
/// - `threads`: Vec of, ThreadStateFields, running threads on device containing cpu and memory metric values.
/// - `threads_table_list`: Vec of ThreadStateFields, sent to frontend after heartbeat received.
/// - `zynq_temp`: Zynq SoC temperature reading.
pub struct AdvancedSystemMonitorTab {
    client_sender: BoxedClientSender,
    csac_received: bool,
    csac_telem_list: Vec<(String, String)>,
    fe_temp: f64,
    headers: Vec<String>,
    obs_latency: HashMap<String, i32>,
    obs_period: HashMap<String, i32>,
    telem_header_index: Option<u8>,
    threads: Vec<ThreadStateFields>,
    threads_table_list: Vec<ThreadStateFields>,
    zynq_temp: f64,
}
impl AdvancedSystemMonitorTab {
    pub fn new(client_sender: BoxedClientSender) -> AdvancedSystemMonitorTab {
        AdvancedSystemMonitorTab {
            client_sender,
            csac_received: false,
            csac_telem_list: vec![],
            fe_temp: 0.0,
            headers: vec![],
            obs_latency: {
                UART_STATE_KEYS
                    .iter()
                    .map(|key| (String::from(*key), 0))
                    .collect()
            },
            obs_period: {
                UART_STATE_KEYS
                    .iter()
                    .map(|key| (String::from(*key), 0))
                    .collect()
            },
            telem_header_index: None,
            threads: vec![],
            threads_table_list: vec![],
            zynq_temp: 0.0,
        }
    }

    pub fn handle_heartbeat(&mut self) {
        if !self.threads.is_empty() {
            self.update_threads();
            self.threads.clear();
        }
    }

    pub fn handle_thread_state(&mut self, msg: MsgThreadState) {
        let name = if msg.name.as_bytes().iter().all(|b| b == &0) {
            NO_NAME.to_string()
        } else {
            msg.name.to_string().trim_end_matches('\0').to_string()
        };
        let thread_state = ThreadStateFields {
            name,
            cpu: OrderedFloat::from(normalize_cpu_usage(msg.cpu)),
            stack_free: msg.stack_free,
        };
        self.threads.push(thread_state);
    }

    fn update_threads(&mut self) {
        self.threads.sort_by(|a, b| b.cpu.cmp(&a.cpu));
        self.threads_table_list = std::mem::take(&mut self.threads);
        self.send_data();
    }

    pub fn handle_device_monitor(&mut self, msg: MsgDeviceMonitor) {
        self.zynq_temp = cc_to_c(msg.cpu_temperature);
        self.fe_temp = cc_to_c(msg.fe_temperature);
    }

    pub fn handle_csac_telemetry_labels(&mut self, msg: MsgCsacTelemetryLabels) {
        self.headers = msg
            .telemetry_labels
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        self.telem_header_index = Some(msg.id);
    }

    pub fn handle_csac_telemetry(&mut self, msg: MsgCsacTelemetry) {
        self.csac_telem_list.clear();
        if let Some(header_index) = self.telem_header_index {
            if msg.id == header_index {
                self.csac_received = true;
                let telems: Vec<String> = msg
                    .telemetry
                    .to_string()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                self.headers
                    .clone()
                    .iter()
                    .enumerate()
                    .for_each(|(i, header)| {
                        if METRICS_OF_INTEREST.contains(&(header.clone().as_str())) {
                            self.csac_telem_list
                                .push((header.clone(), telems[i].clone()));
                        }
                    });
            }
        }
    }

    pub fn handle_uart_state(&mut self, msg: UartState) {
        let uart_fields = msg.fields();
        self.obs_latency
            .insert(CURR.to_string(), uart_fields.latency.current);
        self.obs_latency
            .insert(AVG.to_string(), uart_fields.latency.avg);
        self.obs_latency
            .insert(MIN.to_string(), uart_fields.latency.lmin);
        self.obs_latency
            .insert(MAX.to_string(), uart_fields.latency.lmax);
        if let Some(period) = uart_fields.obs_period {
            self.obs_period.insert(CURR.to_string(), period.current);
            self.obs_period.insert(AVG.to_string(), period.avg);
            self.obs_period.insert(MIN.to_string(), period.pmin);
            self.obs_period.insert(MAX.to_string(), period.pmax);
        }
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut status = msg.init_advanced_system_monitor_status();
        let mut obs_latency_entries = status
            .reborrow()
            .init_obs_latency(self.obs_latency.len() as u32);
        {
            for (i, (key, val)) in self.obs_latency.iter().enumerate() {
                let mut entry = obs_latency_entries.reborrow().get(i as u32);
                entry.set_key(key);
                entry.set_val(*val);
            }
        }
        let mut obs_period_entries = status
            .reborrow()
            .init_obs_period(self.obs_period.len() as u32);
        {
            for (i, (key, val)) in self.obs_period.iter().enumerate() {
                let mut entry = obs_period_entries.reborrow().get(i as u32);
                entry.set_key(key);
                entry.set_val(*val);
            }
        }
        let mut threads_table_entries = status
            .reborrow()
            .init_threads_table(self.threads_table_list.len() as u32);
        {
            for (i, val) in self.threads_table_list.iter().enumerate() {
                let mut entry = threads_table_entries.reborrow().get(i as u32);
                entry.set_name(&val.name.to_string());
                entry.set_cpu(*val.cpu);
                entry.set_stack_free(val.stack_free);
            }
        }
        let mut csac_telem_entries = status
            .reborrow()
            .init_csac_telem_list(self.csac_telem_list.len() as u32);
        {
            for (i, (key, val)) in self.csac_telem_list.iter().enumerate() {
                let mut entry = csac_telem_entries.reborrow().get(i as u32);
                entry.set_key(key);
                entry.set_val(val);
            }
        }
        status.set_zynq_temp(self.zynq_temp);
        status.set_fe_temp(self.fe_temp);
        status.set_csac_received(self.csac_received);
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sender::TestSender;
    use crate::utils::fixed_sbp_string;
    use sbp::{
        messages::piksi::{Latency, MsgUartState, MsgUartStateDepa, Period, UARTChannel},
        sbp_string::NullTerminated,
        SbpString,
    };

    #[test]
    fn handle_uart_state_test() {
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(client_send);
        let sender_id = Some(1337);
        let uart_a = UARTChannel {
            tx_throughput: 0.0,
            rx_throughput: 0.0,
            crc_error_count: 0,
            io_error_count: 0,
            tx_buffer_level: 0,
            rx_buffer_level: 0,
        };
        let uart_b = uart_a.clone();
        let uart_ftdi = uart_a.clone();
        let avg = 4;
        let current = 3;
        let lmin = 2;
        let lmax = 1;
        let no_period = 0;
        let latency = Latency {
            avg,
            current,
            lmin,
            lmax,
        };
        tab.handle_uart_state(UartState::MsgUartStateDepa(MsgUartStateDepa {
            sender_id,
            uart_a: uart_a.clone(),
            uart_b: uart_b.clone(),
            uart_ftdi: uart_ftdi.clone(),
            latency,
        }));
        assert_eq!(*tab.obs_latency.get(&CURR.to_string()).unwrap(), current);
        assert_eq!(*tab.obs_latency.get(&AVG.to_string()).unwrap(), avg);
        assert_eq!(*tab.obs_latency.get(&MIN.to_string()).unwrap(), lmin);
        assert_eq!(*tab.obs_latency.get(&MAX.to_string()).unwrap(), lmax);
        assert_eq!(*tab.obs_period.get(&CURR.to_string()).unwrap(), no_period);
        assert_eq!(*tab.obs_period.get(&AVG.to_string()).unwrap(), no_period);
        assert_eq!(*tab.obs_period.get(&MIN.to_string()).unwrap(), no_period);
        assert_eq!(*tab.obs_period.get(&MAX.to_string()).unwrap(), no_period);
        let avg = 1;
        let current = 2;
        let lmin = 3;
        let lmax = 4;
        let pmin = 5;
        let pmax = 6;
        let latency = Latency {
            avg,
            current,
            lmin,
            lmax,
        };
        let obs_period = Period {
            pmin,
            pmax,
            avg,
            current,
        };
        tab.handle_uart_state(UartState::MsgUartState(MsgUartState {
            sender_id,
            uart_a,
            uart_b,
            uart_ftdi,
            latency,
            obs_period,
        }));
        assert_eq!(*tab.obs_latency.get(&CURR.to_string()).unwrap(), current);
        assert_eq!(*tab.obs_latency.get(&AVG.to_string()).unwrap(), avg);
        assert_eq!(*tab.obs_latency.get(&MIN.to_string()).unwrap(), lmin);
        assert_eq!(*tab.obs_latency.get(&MAX.to_string()).unwrap(), lmax);
        assert_eq!(*tab.obs_period.get(&CURR.to_string()).unwrap(), current);
        assert_eq!(*tab.obs_period.get(&AVG.to_string()).unwrap(), avg);
        assert_eq!(*tab.obs_period.get(&MIN.to_string()).unwrap(), pmin);
        assert_eq!(*tab.obs_period.get(&MAX.to_string()).unwrap(), pmax);
    }

    #[test]
    fn handle_csac_telemetry_test() {
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(client_send);
        let id = 13;
        let headers: Vec<String> = METRICS_OF_INTEREST.iter().map(|s| s.to_string()).collect();
        tab.headers = headers.clone();
        let telemetry_pre: Vec<String> = "4,3,2,1,0".split(',').map(|s| s.to_string()).collect();
        let telemetry = SbpString::from(telemetry_pre.join(","));
        let csac_telem = MsgCsacTelemetry {
            sender_id: Some(1337),
            id,
            telemetry,
        };
        tab.handle_csac_telemetry(csac_telem.clone());
        assert!(tab.csac_telem_list.is_empty());
        let bad_id = 31;
        tab.telem_header_index = Some(bad_id);
        assert!(!tab.csac_received);
        tab.handle_csac_telemetry(csac_telem.clone());
        assert!(tab.csac_telem_list.is_empty());
        tab.telem_header_index = Some(id);
        assert!(!tab.csac_received);
        tab.handle_csac_telemetry(csac_telem);
        assert!(!tab.csac_telem_list.is_empty());
        assert!(tab.csac_received);
        let telem_table_list: Vec<(String, String)> = headers
            .iter()
            .zip(telemetry_pre)
            .map(|(h, t)| (h.clone(), t))
            .collect();
        assert_eq!(tab.csac_telem_list, telem_table_list);
    }
    #[test]
    fn handle_csac_telemetry_labels_test() {
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(client_send);
        let id = 13;
        let headers_pre: Vec<String> = "mcdonald,had,a,farm"
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let telemetry_labels = SbpString::from(headers_pre.join(","));
        let csac_telem_labels = MsgCsacTelemetryLabels {
            sender_id: Some(1337),
            id,
            telemetry_labels,
        };
        assert!(tab.telem_header_index.is_none());
        tab.handle_csac_telemetry_labels(csac_telem_labels);
        assert_eq!(tab.telem_header_index, Some(id));
        assert_eq!(tab.headers, headers_pre);
    }
    #[test]
    fn handle_device_monitor_test() {
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(client_send);
        let cpu_temperature = 3333;
        let fe_temperature = 4444;
        let msg = MsgDeviceMonitor {
            sender_id: Some(1337),
            dev_vin: 0,
            cpu_vint: 0,
            cpu_vaux: 0,
            cpu_temperature,
            fe_temperature,
        };
        assert!(f64::abs(tab.zynq_temp - 0.0) < f64::EPSILON);
        assert!(f64::abs(tab.fe_temp - 0.0) < f64::EPSILON);
        tab.handle_device_monitor(msg);
        assert!(f64::abs(tab.zynq_temp - (cpu_temperature as f64 / 100.0)) < f64::EPSILON);
        assert!(f64::abs(tab.fe_temp - (fe_temperature as f64 / 100.0)) < f64::EPSILON);
    }
    #[test]
    fn handle_thread_state_test() {
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(client_send);
        let name1: SbpString<[u8; 20], NullTerminated> = fixed_sbp_string("mcdonald");
        let msg1 = MsgThreadState {
            sender_id: Some(1337),
            name: name1.clone(),
            cpu: 66,
            stack_free: 13,
        };
        assert!(tab.threads.is_empty());
        tab.handle_thread_state(msg1);
        assert_eq!(tab.threads.len(), 1);
        let name2: SbpString<[u8; 20], NullTerminated> = fixed_sbp_string(NO_NAME);
        let msg2 = MsgThreadState {
            sender_id: Some(1337),
            name: SbpString::new([0u8; 20]),
            cpu: 6,
            stack_free: 133,
        };
        tab.handle_thread_state(msg2);
        assert_eq!(tab.threads.len(), 2);
        let name3: SbpString<[u8; 20], NullTerminated> = fixed_sbp_string("farm");
        let msg3 = MsgThreadState {
            sender_id: Some(1337),
            name: name3.clone(),
            cpu: 667,
            stack_free: 133,
        };
        tab.handle_thread_state(msg3);
        assert_eq!(tab.threads.len(), 3);
        assert_eq!(
            tab.threads[0].name,
            name1.to_string().trim_end_matches('\0')
        );
        assert_eq!(
            tab.threads[1].name,
            name2.to_string().trim_end_matches('\0')
        );
        assert_eq!(
            tab.threads[2].name,
            name3.to_string().trim_end_matches('\0')
        );
    }

    #[test]
    fn handle_heartbeat_test() {
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(client_send);
        assert!(tab.threads_table_list.is_empty());
        tab.handle_heartbeat();
        assert!(tab.threads_table_list.is_empty());
        let name1: SbpString<[u8; 20], NullTerminated> = fixed_sbp_string("mcdonald");
        let msg1 = MsgThreadState {
            sender_id: Some(1337),
            name: name1.clone(),
            cpu: 66,
            stack_free: 13,
        };
        tab.handle_thread_state(msg1.clone());
        let name2: SbpString<[u8; 20], NullTerminated> = fixed_sbp_string(NO_NAME);
        let msg2 = MsgThreadState {
            sender_id: Some(1337),
            name: SbpString::new([0u8; 20]),
            cpu: 6,
            stack_free: 133,
        };
        tab.handle_thread_state(msg2.clone());
        let name3: SbpString<[u8; 20], NullTerminated> = fixed_sbp_string("farm");
        let msg3 = MsgThreadState {
            sender_id: Some(1337),
            name: name3.clone(),
            cpu: 667,
            stack_free: 133,
        };
        tab.handle_thread_state(msg3.clone());
        assert!(tab.threads_table_list.is_empty());
        assert_eq!(tab.threads.len(), 3);
        tab.handle_heartbeat();
        assert!(tab.threads.is_empty());
        assert!(!tab.threads_table_list.is_empty());
        assert_eq!(
            tab.threads_table_list[0].name,
            name3.to_string().trim_end_matches('\0')
        );
        assert_eq!(
            tab.threads_table_list[0].cpu,
            OrderedFloat(msg3.cpu as f64 / 10.0)
        );
        assert_eq!(tab.threads_table_list[0].stack_free, msg3.stack_free);
        assert_eq!(
            tab.threads_table_list[1].name,
            name1.to_string().trim_end_matches('\0')
        );
        assert_eq!(
            tab.threads_table_list[1].cpu,
            OrderedFloat(msg1.cpu as f64 / 10.0)
        );
        assert_eq!(tab.threads_table_list[1].stack_free, msg1.stack_free);
        assert_eq!(
            tab.threads_table_list[2].name,
            name2.to_string().trim_end_matches('\0')
        );
        assert_eq!(
            tab.threads_table_list[2].cpu,
            OrderedFloat(msg2.cpu as f64 / 10.0)
        );
        assert_eq!(tab.threads_table_list[2].stack_free, msg2.stack_free);
    }
}