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
use sbp::messages::piksi::{MsgDeviceMonitor, MsgThreadState};
use std::collections::HashMap;

use crate::client_sender::BoxedClientSender;
use crate::shared_state::{SharedState, TabName};
use crate::types::UartState;
use crate::utils::{cc_to_c, normalize_cpu_usage, serialize_capnproto_builder};

const NO_NAME: &str = "(no name)";
const CURR: &str = "Curr";
const AVG: &str = "Avg";
const MIN: &str = "Min";
const MAX: &str = "Max";
const UART_STATE_KEYS: &[&str] = &[CURR, AVG, MIN, MAX];

struct ThreadStateFields {
    name: String,
    cpu: f64,
    stack_free: u32,
}

pub struct AdvancedSystemMonitorTab {
    shared_state: SharedState,
    /// Client Sender channel for communication from backend to frontend.
    client_sender: BoxedClientSender,
    /// RF frontend temperature reading.
    fe_temp: f64,
    /// UART state latency measurements.
    obs_latency: HashMap<String, i32>,
    /// UART state period measurements.
    obs_period: HashMap<String, i32>,
    /// Vec of, ThreadStateFields, running threads on device containing cpu and memory metric values.
    threads: Vec<ThreadStateFields>,
    /// Vec of ThreadStateFields, sent to frontend after heartbeat received.
    threads_table_list: Vec<ThreadStateFields>,
    /// Zynq SoC temperature reading.
    zynq_temp: f64,
}

impl AdvancedSystemMonitorTab {
    pub fn new(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
    ) -> AdvancedSystemMonitorTab {
        let keys: HashMap<String, i32> = UART_STATE_KEYS
            .iter()
            .map(|&key| (String::from(key), 0))
            .collect();
        AdvancedSystemMonitorTab {
            shared_state,
            client_sender,
            fe_temp: 0.0,
            obs_latency: keys.clone(),
            obs_period: keys,
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
            cpu: normalize_cpu_usage(msg.cpu),
            stack_free: msg.stack_free,
        };
        self.threads.push(thread_state);
    }

    fn update_threads(&mut self) {
        self.threads.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
        self.threads_table_list = std::mem::take(&mut self.threads);
        self.send_data();
    }

    pub fn handle_device_monitor(&mut self, msg: MsgDeviceMonitor) {
        self.zynq_temp = cc_to_c(msg.cpu_temperature);
        self.fe_temp = cc_to_c(msg.fe_temperature);
    }

    pub fn handle_uart_state(&mut self, msg: UartState) {
        let uart_fields = msg.fields();
        let latency = uart_fields.latency;
        self.obs_latency.insert(CURR.to_string(), latency.current);
        self.obs_latency.insert(AVG.to_string(), latency.avg);
        self.obs_latency.insert(MIN.to_string(), latency.lmin);
        self.obs_latency.insert(MAX.to_string(), latency.lmax);
        if let Some(period) = uart_fields.obs_period {
            self.obs_period.insert(CURR.to_string(), period.current);
            self.obs_period.insert(AVG.to_string(), period.avg);
            self.obs_period.insert(MIN.to_string(), period.pmin);
            self.obs_period.insert(MAX.to_string(), period.pmax);
        }
    }

    /// Package data into a message buffer and send to frontend.
    pub fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Advanced {
            return;
        }
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
                entry.set_cpu(val.cpu);
                entry.set_stack_free(val.stack_free);
            }
        }
        status.set_zynq_temp(self.zynq_temp);
        status.set_fe_temp(self.fe_temp);
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
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(shared_state, client_send);
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
    fn handle_device_monitor_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(shared_state, client_send);
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
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(shared_state, client_send);
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
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSystemMonitorTab::new(shared_state, client_send);
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
        assert_eq!(tab.threads_table_list[0].cpu, msg3.cpu as f64 / 10.0);
        assert_eq!(tab.threads_table_list[0].stack_free, msg3.stack_free);
        assert_eq!(
            tab.threads_table_list[1].name,
            name1.to_string().trim_end_matches('\0')
        );
        assert_eq!(tab.threads_table_list[1].cpu, msg1.cpu as f64 / 10.0);
        assert_eq!(tab.threads_table_list[1].stack_free, msg1.stack_free);
        assert_eq!(
            tab.threads_table_list[2].name,
            name2.to_string().trim_end_matches('\0')
        );
        assert_eq!(tab.threads_table_list[2].cpu, msg2.cpu as f64 / 10.0);
        assert_eq!(tab.threads_table_list[2].stack_free, msg2.stack_free);
    }
}
