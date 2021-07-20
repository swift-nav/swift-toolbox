// use ordered_float::OrderedFloat;
use sbp::messages::piksi::MsgSpecan;
use serde::Serialize;
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::types::Result;

pub const AMPLITUDES: &str = "amplitudes";
pub const FREQUENCIES: &str = "frequencies";
const CHANNELS: &[u16] = &[1, 2, 3, 4];
const SIGNALS_TOTAL: usize = 512;

#[derive(Clone, Debug, Serialize)]
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
    fn get_frequencies(&self, msg: MsgSpecan) -> Vec<f32> {
        (0..msg.amplitude_value.len())
            .map(|i| msg.freq_ref + msg.freq_step * i as f32)
            .collect()
    }

    fn get_amplitudes(&self, msg: MsgSpecan) -> Vec<f32> {
        msg.amplitude_value
            .iter()
            .map(|&i| msg.amplitude_ref + msg.amplitude_unit * i as f32)
            .collect()
    }
    pub fn capture_fft(&mut self, msg: MsgSpecan) -> Result<()> {
        let channel = msg.channel_tag;
        if let Some(is_enabled) = self.enabled.get(&channel) {
            if *is_enabled {
                let gps_time = GpsTimeHashable::from(msg.t.wn, msg.t.tow, msg.t.ns_residual);
                let mut frequencies = self.get_frequencies(msg.clone());
                let mut amplitudes = self.get_amplitudes(msg);

                if let Some(inc_ffts) = self.incomplete_ffts.get_mut(&channel) {
                    let ffts = inc_ffts.entry(gps_time.clone()).or_insert(MkValue::new());
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
                            if let Some((_, fft)) = inc_ffts.remove_entry(&gps_time.clone()) {
                                if let Some(freqs) = fft.get(FREQUENCIES) {
                                    if let Some(amps) = fft.get(AMPLITUDES) {
                                        if freqs.len() != amps.len() {
                                            return Err(format!("Frequencies length does not match amplitudes length for {:?}", gps_time).into());
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
            channels = self.channels.clone();
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
                if let Some(freqs) = (*val).get(FREQUENCIES) {
                    Some(freqs.len())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn num_ffts(&mut self, channel: u16) -> Option<usize> {
        if let Some(chan) = self.ffts.get(&channel) {
            Some((*chan).len())
        } else {
            None
        }
    }
    pub fn get_ffts(&mut self, channel: u16) -> Option<Vec<MkValue>> {
        if let Some(chan) = self.ffts.get(&channel) {
            Some((*chan).clone())
        } else {
            None
        }
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
