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

use crate::client_sender::BoxedClientSender;
use crate::constants::{AMPLITUDES, CHANNELS, FREQUENCIES};
use crate::fft_monitor::FftMonitor;
use crate::shared_state::{SharedState, TabName};
use crate::types::Specan;
use crate::utils::serialize_capnproto_builder;

pub struct AdvancedSpectrumAnalyzerTab {
    /// Client Sender channel for communication from backend to frontend.
    client_sender: BoxedClientSender,
    /// Instance of the FftMonitor struct for handling specan messages.
    fft_monitor: FftMonitor,
    /// Stores the index received from the frontend for which CHANNEL to send to frontend.
    channel_idx: usize,
    /// Stores the currently viewed channel's amplitude values to be sent to frontend.
    most_recent_amplitudes: Vec<f32>,
    /// Stores the currently viewed channel's frequency values to be sent to frontend.
    most_recent_frequencies: Vec<f32>,
    /// The shared state for communicating between frontend/backend/other backend tabs.
    shared_state: SharedState,
}

impl AdvancedSpectrumAnalyzerTab {
    pub fn new(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
    ) -> AdvancedSpectrumAnalyzerTab {
        let mut fft_monitor = FftMonitor::new();
        fft_monitor.enable_channel(None);
        AdvancedSpectrumAnalyzerTab {
            fft_monitor,
            client_sender,
            channel_idx: 0,
            most_recent_amplitudes: vec![],
            most_recent_frequencies: vec![],
            shared_state,
        }
    }

    /// Handler for Specan messages.
    ///
    /// # Parameters
    /// - `msg`: MsgSpecan to extract data from.
    pub fn handle_specan(&mut self, msg: Specan) {
        if let Err(err) = self.fft_monitor.capture_fft(msg) {
            error!("{err}");
            return;
        }
        {
            let shared_data = self.shared_state.lock();
            self.channel_idx = shared_data.advanced_spectrum_analyzer_tab.channel_idx as usize;
        }
        let channel = CHANNELS[self.channel_idx];
        if let Some(num_ffts) = self.fft_monitor.num_ffts(channel) {
            if num_ffts > 0 {
                if let Some(mut ffts) = self.fft_monitor.get_ffts(channel) {
                    if let Some(most_recent_fft) = ffts.pop() {
                        self.fft_monitor.clear_ffts(None);
                        if let Some(amplitudes) = most_recent_fft.get(&String::from(AMPLITUDES)) {
                            self.most_recent_amplitudes.clone_from(amplitudes);
                        }
                        if let Some(frequencies) = most_recent_fft.get(&String::from(FREQUENCIES)) {
                            self.most_recent_frequencies.clone_from(frequencies);
                        }

                        self.send_data();
                    }
                }
            }
        }
    }

    // /// Package data into a message buffer and send to frontend.
    pub fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Advanced {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab_status = msg.init_advanced_spectrum_analyzer_status();

        let mut xmin = f32::MAX;
        let mut xmax = f32::MIN;
        let mut ymin = f32::MAX;
        let mut ymax = f32::MIN;

        let mut point_vals = tab_status
            .reborrow()
            .init_data(self.most_recent_amplitudes.len() as u32);
        for idx in 0..self.most_recent_amplitudes.len() {
            let mut point_val = point_vals.reborrow().get(idx as u32);
            point_val.set_x(self.most_recent_frequencies[idx] as f64);
            point_val.set_y(self.most_recent_amplitudes[idx] as f64);
            ymin = f32::min(self.most_recent_amplitudes[idx], ymin);
            ymax = f32::max(self.most_recent_amplitudes[idx], ymax);
            xmin = f32::min(self.most_recent_frequencies[idx], xmin);
            xmax = f32::max(self.most_recent_frequencies[idx], xmax);
        }

        tab_status.set_channel(self.channel_idx as u16);
        tab_status.set_ymin(ymin);
        tab_status.set_ymax(ymax);
        tab_status.set_xmin(xmin);
        tab_status.set_xmax(xmax);

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client_sender::TestSender, constants::SIGNALS_TOTAL};
    use sbp::messages::{gnss::GpsTime, piksi::MsgSpecan};

    #[test]
    fn handle_specan_empty_amplitude_value_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSpectrumAnalyzerTab::new(shared_state, client_send);
        let channel_tag = 2;
        let wn = 1000;
        let tow = 10001;
        let ns_residual = 100011;
        let t = GpsTime {
            tow,
            ns_residual,
            wn,
        };
        let freq_ref = 1.0;
        let freq_step = 2.0;
        let amplitude_ref = 3.0;
        let amplitude_unit = 4.0;

        let msg = MsgSpecan {
            sender_id: Some(1337),
            channel_tag,
            t,
            freq_ref,
            freq_step,
            amplitude_ref,
            amplitude_unit,
            amplitude_value: vec![],
        };
        assert!(tab.most_recent_amplitudes.is_empty());
        assert!(tab.most_recent_frequencies.is_empty());
        tab.handle_specan(Specan::MsgSpecan(msg));
        // Test against not enough amplitude value samples to trigger an update of current amps/freqs.
        assert!(tab.most_recent_amplitudes.is_empty());
        assert!(tab.most_recent_frequencies.is_empty());
    }
    #[test]
    fn handle_specan_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tab = AdvancedSpectrumAnalyzerTab::new(shared_state, client_send);
        let channel_tag = 1;
        let wn = 1000;
        let tow = 10001;
        let ns_residual = 100011;
        let t = GpsTime {
            tow,
            ns_residual,
            wn,
        };
        let freq_ref = 1.0;
        let freq_step = 2.0;
        let amplitude_ref = 3.0;
        let amplitude_unit = 4.0;

        let msg = MsgSpecan {
            sender_id: Some(1337),
            channel_tag,
            t,
            freq_ref,
            freq_step,
            amplitude_ref,
            amplitude_unit,
            amplitude_value: (0..SIGNALS_TOTAL).map(|i| i as u8).collect(),
        };
        assert!(tab.most_recent_amplitudes.is_empty());
        assert!(tab.most_recent_frequencies.is_empty());
        tab.handle_specan(Specan::MsgSpecan(msg));
        // Test enough samples were provided to trigger an update of current amps/freqs.
        assert_eq!(tab.most_recent_amplitudes.len(), SIGNALS_TOTAL);
        assert_eq!(tab.most_recent_frequencies.len(), SIGNALS_TOTAL);
    }
}
