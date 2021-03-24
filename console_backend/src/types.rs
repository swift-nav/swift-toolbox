// use chrono::{DateTime, Utc};
// use serde::Serialize;

// use crate::formatters::*;
// use crate::msg_utils::{GnssMode, InsMode, ProtectionLevel};

use std::{borrow::BorrowMut, sync::mpsc::Sender};
use std::{collections::{HashMap, VecDeque}, time::Instant};
use ndarray::{arr0, arr1, arr2, Array, Array0, Array1, Array2, ArrayBase, Axis, concatenate, Dim, OwnedRepr, s, stack};
use ordered_float::OrderedFloat;

use capnp::message::Builder;
use capnp::serialize;
use std::sync::mpsc;

use crate::constants::*;


use sbp::messages::tracking::MeasurementState;
use crate::console_backend_capnp as m;

pub type Error = std::boxed::Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub type Deque<T> = VecDeque<T>;
trait DequeExt<T> {
    fn add(&mut self, ele: T);
}
impl<T> DequeExt<T> for Deque<T> {
    fn add(&mut self, ele: T){
        if self.len() == self.capacity(){
            self.pop_front();
        }
        self.push_back(ele);
    }
}

#[derive(Debug)]
pub struct TrackingSignalsTab {
    pub sats: Vec<Deque<(OrderedFloat<f64>, f64)>>,
    pub sv_labels: Vec<String>,
    pub colors: Vec<String>,
    pub max: f64,
    pub min: f64,
    pub glo_fcn_dict: HashMap<u8, i16>,
    pub glo_slot_dict: HashMap<i16, i16>,
    pub cn0_dict: HashMap<(u8, i16), Deque<(OrderedFloat<f64>, f64)>>,
    pub cn0_age: HashMap<(u8, i16), f64>,
    pub received_codes: Vec<u8>,
    pub t_init: Instant,
    pub time: VecDeque<f64>,
}

impl TrackingSignalsTab {
    pub fn new() -> TrackingSignalsTab {
        TrackingSignalsTab {
            cn0_dict: HashMap::new(),
            cn0_age: HashMap::new(),
            colors: Vec::new(),
            glo_fcn_dict: HashMap::new(),
            glo_slot_dict: HashMap::new(),
            sv_labels: {
                let mut labels: Vec<String> = Vec::new();
                for code in SUPPORTED_CODES {
                    let code_str = code_to_str_map(*code);
                    labels.push(String::from(code_str));
                }
                labels
            },
            max: TRACKING_SIGNALS_PLOT_MAX,
            min: SNR_THRESHOLD,
            received_codes: Vec::new(),
            sats: Vec::new(),
            t_init: Instant::now(),
            time: {
                let mut time: Deque<f64> = Deque::with_capacity(NUM_POINTS);
                for x in (0..(NUM_POINTS as i32)).rev() {
                    time.push_back((-x as f64)*(1.0/TRK_RATE));
                }
                time
            }

        }
    }
    fn push_to_cn0_dict(&mut self, key: (u8, i16), t: f64, cn0: f64) {
        let cn0_deque = self.cn0_dict.entry(key).or_insert(Deque::with_capacity(NUM_POINTS));
        cn0_deque.add((OrderedFloat(t), cn0));
    }
    fn push_to_cn0_age(&mut self, key: (u8, i16), age: f64) {
        let cn0_age = self.cn0_age.entry(key).or_insert(-1.0);
        *cn0_age = age;
    }

    pub fn clean_cn0(&mut self){
        let mut remove_vec: Vec<(u8,i16)> = Vec::new();
        for (key, _) in self.cn0_dict.iter_mut(){
            if self.cn0_age[key] < self.time[0] {
                remove_vec.push(*key);
            }
        }
        for key in remove_vec {
            self.cn0_dict.remove(&key);
            self.cn0_age.remove(&key);
        }        
    }

    pub fn update_plot(&mut self) {
        self.sv_labels.clear();
        self.colors.clear();
        self.sats.clear();
        for (key, cn0_deque) in self.cn0_dict.iter_mut() {
            let (code_lbl, freq_lbl, id_lbl) = get_label(*key, &self.glo_slot_dict);
            let mut label = String::from("");
            if let Some(lbl) = code_lbl{
                label = format!("{} {}", label, lbl);
            }
            if let Some(lbl) = freq_lbl {
                label = format!("{} {}", label, lbl);
            }
            if let Some(lbl) = id_lbl {
                label = format!("{} {}", label, lbl);
            }
            self.sv_labels.push(label);
            self.colors.push(String::from(get_color(*key)));
            self.sats.push(cn0_deque.clone());
        }
    }

    fn send_data(&mut self, client_send: Sender<Vec<u8>>){
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();

        let mut tracking_status = msg.init_tracking_status();
        tracking_status.set_min(self.min);
        tracking_status.set_max(self.max);
        let mut labels = tracking_status
            .reborrow()
            .init_labels(self.sv_labels.len() as u32);

        for (i, header) in self.sv_labels.iter().enumerate() {
            labels.set(i as u32, header);
        }

        let mut colors = tracking_status
            .reborrow()
            .init_colors(self.colors.len() as u32);
        
        for(i, color) in self.colors.iter().enumerate() {
            colors.set(i as u32, color);
        }

        let mut tracking_points = tracking_status
            .reborrow()
            .init_data(self.sv_labels.len() as u32);
        {
            for idx in 0..self.sv_labels.len() {

                let points = self.sats.get_mut(idx).unwrap();
                let mut point_val_idx = tracking_points
                    .reborrow()
                    .init(idx as u32, points.len() as u32);
                for (i, (OrderedFloat(x), y)) in points.iter().enumerate() {
                    let mut point_val = point_val_idx.reborrow().get(i as u32);
                    point_val.set_x(*x);
                    point_val.set_y(*y);
                }
            }
        }
        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();

        client_send.send(msg_bytes).unwrap();
    }

    pub fn handle_msg_measurement_state(&mut self, states: Vec<MeasurementState>, client_send: Sender<Vec<u8>>) {
        let mut codes_that_came: Vec<(u8,i16)> = Vec::new();
        let t = (Instant::now()).duration_since(self.t_init).as_secs_f64();
        self.time.add(t);
        for (idx, state) in states.iter().enumerate() {
            let mut sat = state.mesid.sat as i16;
            if code_is_glo(state.mesid.code) {
                if state.mesid.sat > GLO_SLOT_SAT_MAX {
                    self.glo_fcn_dict.insert(idx as u8, state.mesid.sat as i16 - 100.0 as i16);
                }
                sat = *self.glo_fcn_dict.get(&(idx as u8)).unwrap_or(&(0 as i16));
                if state.mesid.sat <= GLO_SLOT_SAT_MAX {
                    self.glo_slot_dict.insert(sat, state.mesid.sat as i16);
                }
            }
            // println!("{:?}", self.glo_fcn_dict);
            let key = (state.mesid.code, sat);
            codes_that_came.push(key);
            if state.cn0 != 0 {
                self.push_to_cn0_dict(key, t, state.cn0 as f64 / 4.0);
                self.push_to_cn0_age(key, t);
            }
            if !self.received_codes.contains(&state.mesid.code){
                self.received_codes.push(state.mesid.code);
            }
        }
        for (key, cn0_deque) in self.cn0_dict.iter_mut() {
            if !codes_that_came.contains(key){
                cn0_deque.add((OrderedFloat(t), 0.0));
            }
        }
        self.clean_cn0();
        self.update_plot();
        self.send_data(client_send);
    }
}

// #[test]
// fn test_append_tracking_signals_tab() {
//     let mut tst = TrackingSignalsTab::new();
//     assert_eq!(tst.sats.shape(), &[NUM_SATELLITES, NUM_POINTS]);
//     let ele1 = (64.0 as f64, OrderedFloat(999888.0 as f64));
//     tst.append_to_sat(36 as usize, &ele1.clone());
//     assert_eq!(&ele1, tst.sats.get((36, NUM_POINTS-1)).unwrap());
// }