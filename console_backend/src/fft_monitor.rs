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

use crate::constants::{AMPLITUDES, CHANNELS, FREQUENCIES, SIGNALS_TOTAL};
use crate::types::{Result, Specan};
use anyhow::bail;
use serde::Serialize;
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, Default, Serialize)]
pub struct MkValue(HashMap<String, Vec<f32>>);
impl MkValue {
    pub fn new() -> MkValue {
        let hashmap: HashMap<String, Vec<f32>> = [AMPLITUDES, FREQUENCIES]
            .iter()
            .map(|key| (String::from(*key), vec![]))
            .collect();
        MkValue(hashmap)
    }
}
impl Deref for MkValue {
    type Target = HashMap<String, Vec<f32>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MkValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GpsTimeHashable {
    wn: u16,
    tow: u32,
    ns_residual: i32,
}
impl GpsTimeHashable {
    fn from(wn: u16, tow: u32, ns_residual: i32) -> GpsTimeHashable {
        GpsTimeHashable {
            wn,
            tow,
            ns_residual,
        }
    }
}

#[derive(Default)]
pub struct FftMonitor {
    pub channels: Vec<u16>,
    pub ffts: HashMap<u16, Vec<MkValue>>,
    pub incomplete_ffts: HashMap<u16, HashMap<GpsTimeHashable, MkValue>>,
    enabled: HashMap<u16, bool>,
}

impl FftMonitor {
    pub fn new() -> Self {
        FftMonitor {
            channels: vec![1, 2, 3, 4],
            ffts: { CHANNELS.iter().map(|key| (*key, vec![])).collect() },
            incomplete_ffts: { CHANNELS.iter().map(|key| (*key, HashMap::new())).collect() },
            enabled: { CHANNELS.iter().map(|key| (*key, false)).collect() },
        }
    }
    fn get_frequencies(&self, amplitude_value: Vec<u8>, freq_ref: f32, freq_step: f32) -> Vec<f32> {
        (0..amplitude_value.len())
            .map(|i| freq_ref + freq_step * i as f32)
            .collect()
    }

    fn get_amplitudes(
        &self,
        amplitude_value: Vec<u8>,
        amplitude_ref: f32,
        amplitude_unit: f32,
    ) -> Vec<f32> {
        amplitude_value
            .iter()
            .map(|&i| amplitude_ref + amplitude_unit * i as f32)
            .collect()
    }
    pub fn capture_fft(&mut self, msg: Specan) -> Result<()> {
        let msg = msg.fields();
        let channel = msg.channel_tag;
        if let Some(is_enabled) = self.enabled.get(&channel) {
            if *is_enabled {
                let gps_time = GpsTimeHashable::from(msg.wn, msg.tow, msg.ns_residual);
                let mut frequencies =
                    self.get_frequencies(msg.amplitude_value.clone(), msg.freq_ref, msg.freq_step);
                let mut amplitudes =
                    self.get_amplitudes(msg.amplitude_value, msg.amplitude_ref, msg.amplitude_unit);

                if let Some(inc_ffts) = self.incomplete_ffts.get_mut(&channel) {
                    let ffts = inc_ffts
                        .entry(gps_time.clone())
                        .or_insert_with(MkValue::new);
                    if let Some(freqs) = (*ffts).get_mut(&String::from(FREQUENCIES)) {
                        freqs.append(&mut frequencies);
                    }
                    if let Some(amps) = (*ffts).get_mut(&String::from(AMPLITUDES)) {
                        amps.append(&mut amplitudes);
                    }
                }
                if let Some(ffts_chan_len) =
                    self.num_incomplete_freqs_for_timestamp(channel, gps_time.clone())
                {
                    if ffts_chan_len == SIGNALS_TOTAL {
                        if let Some(inc_ffts) = self.incomplete_ffts.get_mut(&channel) {
                            if let Some((_, fft)) = inc_ffts.remove_entry(&gps_time) {
                                if let Some(freqs) = fft.get(FREQUENCIES) {
                                    if let Some(amps) = fft.get(AMPLITUDES) {
                                        if freqs.len() != amps.len() {
                                            bail!("Frequencies length does not match amplitudes length for {:?}", gps_time);
                                        }
                                        if let Some(chan_ffts) = self.ffts.get_mut(&channel) {
                                            chan_ffts.append(&mut vec![fft]);
                                            inc_ffts.clear();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn enable(&mut self, en: bool, channel: Option<u16>) {
        let mut channels = vec![];
        if let Some(chan) = channel {
            channels.append(&mut vec![chan]);
        } else {
            channels.clone_from(&self.channels);
        }
        for chan in channels {
            if let Some(chan_en) = self.enabled.get_mut(&chan) {
                *chan_en = en;
            }
        }
    }
    pub fn enable_channel(&mut self, channel: Option<u16>) {
        self.clear_ffts(channel);
        self.enable(true, channel);
    }
    pub fn disable_channel(&mut self, channel: Option<u16>) {
        self.clear_ffts(channel);
        self.enable(false, channel);
    }
    pub fn num_incomplete_freqs_for_timestamp(
        &mut self,
        channel: u16,
        timestamp: GpsTimeHashable,
    ) -> Option<usize> {
        if let Some(chan) = self.incomplete_ffts.get(&channel) {
            if let Some(val) = (*chan).get(&timestamp) {
                (*val).get(FREQUENCIES).map(|freqs| freqs.len())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn num_ffts(&mut self, channel: u16) -> Option<usize> {
        self.ffts.get(&channel).map(|chan| (*chan).len())
    }
    pub fn get_ffts(&mut self, channel: u16) -> Option<Vec<MkValue>> {
        self.ffts.get(&channel).map(|chan| (*chan).clone())
    }
    pub fn clear_ffts(&mut self, channel: Option<u16>) {
        let mut channels = vec![];
        if let Some(chan) = channel {
            channels.append(&mut vec![chan]);
        }
        for chan in channels {
            if let Some(chan_en) = self.ffts.get_mut(&chan) {
                (*chan_en).clear();
            }
            if let Some(chan_en) = self.incomplete_ffts.get_mut(&chan) {
                (*chan_en).clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SIGNALS_TOTAL;
    use sbp::messages::{gnss::GpsTime, piksi::MsgSpecan};

    #[test]
    fn get_frequencies_test() {
        let fftmon = FftMonitor::new();
        let freq_ref = 1.0;
        let freq_step = 2.0;
        let amplitude_value = (0..SIGNALS_TOTAL).map(|i| i as u8).collect();

        let freqs = fftmon.get_frequencies(amplitude_value, freq_ref, freq_step);
        let expected_freqs: Vec<f32> = (0..SIGNALS_TOTAL)
            .map(|i| freq_ref + freq_step * i as f32)
            .collect();
        assert_eq!(freqs, expected_freqs);
    }
    #[test]
    fn get_amplitudes_test() {
        let fftmon = FftMonitor::new();
        let amplitude_ref = 3.0;
        let amplitude_unit = 4.0;
        let amplitude_value = (0..SIGNALS_TOTAL).map(|i| i as u8).collect();

        let freqs = fftmon.get_frequencies(amplitude_value, amplitude_ref, amplitude_unit);
        let expected_freqs: Vec<f32> = (0..SIGNALS_TOTAL)
            .map(|i| amplitude_ref + amplitude_unit * i as f32)
            .collect();
        assert_eq!(freqs, expected_freqs);
    }

    fn get_specan_msg(channel_tag: u16, wn: u16, incomplete: bool) -> Specan {
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
        let amplitude_value: Vec<u8> = if incomplete {
            (0..(SIGNALS_TOTAL / 2)).map(|i| i as u8).collect()
        } else {
            (0..SIGNALS_TOTAL).map(|i| i as u8).collect()
        };
        let msg = MsgSpecan {
            sender_id: Some(1337),
            channel_tag,
            t,
            freq_ref,
            freq_step,
            amplitude_ref,
            amplitude_unit,
            amplitude_value,
        };
        Specan::MsgSpecan(msg)
    }
    #[test]
    fn clear_ffts_test() {
        let mut fftmon = FftMonitor::new();
        fftmon.enable_channel(None);
        let channel_tag = 1;
        let wn = 1000;
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.ffts.get(&channel_tag).unwrap();
        assert_eq!(ffts.len(), 1);
        let fft_mkval = ffts.first().unwrap();
        let freqs = fft_mkval.get(FREQUENCIES).unwrap();
        let amps = fft_mkval.get(AMPLITUDES).unwrap();
        assert_eq!(freqs.len(), SIGNALS_TOTAL);
        assert_eq!(amps.len(), SIGNALS_TOTAL);
        fftmon.clear_ffts(Some(channel_tag));
        let ffts = fftmon.ffts.get(&channel_tag).unwrap();
        assert_eq!(ffts.len(), 0);
    }

    #[test]
    fn enable_channel_test() {
        let mut fftmon = FftMonitor::new();
        let channel_tag = 1;
        let wn = 1000;
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.ffts.get(&channel_tag).unwrap();
        assert_eq!(ffts.len(), 0);

        fftmon.enable_channel(Some(channel_tag));
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.ffts.get(&channel_tag).unwrap();
        assert_eq!(ffts.len(), 1);
        let fft_mkval = ffts.first().unwrap();
        let freqs = fft_mkval.get(FREQUENCIES).unwrap();
        let amps = fft_mkval.get(AMPLITUDES).unwrap();
        assert_eq!(freqs.len(), SIGNALS_TOTAL);
        assert_eq!(amps.len(), SIGNALS_TOTAL);
    }
    #[test]
    fn get_ffts_test() {
        let mut fftmon = FftMonitor::new();
        fftmon.enable_channel(None);
        let channel_tag = 1;
        let wn = 1000;
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();

        let ffts = fftmon.get_ffts(channel_tag).unwrap();
        assert_eq!(ffts.len(), 1);
        let fft_mkval = ffts.first().unwrap();
        let freqs = fft_mkval.get(FREQUENCIES).unwrap();
        let amps = fft_mkval.get(AMPLITUDES).unwrap();
        assert_eq!(freqs.len(), SIGNALS_TOTAL);
        assert_eq!(amps.len(), SIGNALS_TOTAL);
    }
    #[test]
    fn num_ffts_test() {
        let mut fftmon = FftMonitor::new();
        fftmon.enable_channel(None);
        let channel_tag = 1;
        let wn = 1000;
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.num_ffts(channel_tag).unwrap();
        assert_eq!(ffts, 1);
        let wn = 1001;
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.num_ffts(channel_tag).unwrap();
        assert_eq!(ffts, 2);
    }
    #[test]
    fn disable_channel_test() {
        let mut fftmon = FftMonitor::new();
        fftmon.enable_channel(None);
        let channel_tag = 1;
        let wn = 1000;
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.num_ffts(channel_tag).unwrap();
        assert_eq!(ffts, 1);
        fftmon.disable_channel(None);
        let wn = 1001;
        let msg = get_specan_msg(channel_tag, wn, false);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.num_ffts(channel_tag).unwrap();
        assert_eq!(ffts, 1);
    }

    #[test]
    fn incomplete_ffts_test() {
        let mut fftmon = FftMonitor::new();
        fftmon.enable_channel(None);
        let channel_tag = 1;
        let wn = 1000;
        let incomplete = true;
        let msg = get_specan_msg(channel_tag, wn, incomplete);
        fftmon.capture_fft(msg).unwrap();
        let ffts = fftmon.ffts.get(&channel_tag).unwrap();
        assert_eq!(ffts.len(), 0);
        let inc_ffts = fftmon.incomplete_ffts.get(&channel_tag).unwrap();
        assert_eq!(inc_ffts.len(), 1);
        let timestamp = fftmon
            .incomplete_ffts
            .get(&channel_tag)
            .unwrap()
            .keys()
            .next()
            .unwrap()
            .clone();
        let num_inc_freqs = fftmon
            .num_incomplete_freqs_for_timestamp(channel_tag, timestamp)
            .unwrap();
        assert_eq!(num_inc_freqs, SIGNALS_TOTAL / 2);
    }
}
