
use std::{collections::{HashMap, VecDeque}, time::Instant, sync::mpsc::Sender};
use ordered_float::OrderedFloat;

use capnp::message::Builder;
use capnp::serialize;

use sbp::messages::{observation::{MsgObs, MsgObsDepA, MsgObsDepB, MsgObsDepC, PackedObsContent, PackedObsContentDepA, PackedObsContentDepB, PackedObsContentDepC}, tracking::{MeasurementState, TrackingChannelState}};
use crate::constants::*;
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
    pub at_least_one_track_received: bool,
    pub cn0_age: HashMap<(u8, i16), f64>,
    pub cn0_dict: HashMap<(u8, i16), Deque<(OrderedFloat<f64>, f64)>>,
    pub colors: Vec<String>,
    pub glo_fcn_dict: HashMap<u8, i16>,
    pub glo_slot_dict: HashMap<i16, i16>,
    pub gps_tow: f64,
    pub gps_week: u16,
    pub incoming_obs_cn0: HashMap<(u8, i16), f64>,
    pub last_update_time: Instant,
    pub max: f64,
    pub min: f64,
    pub prev_obs_count: u8,
    pub prev_obs_total: u8,
    pub received_codes: Vec<u8>,
    pub sats: Vec<Deque<(OrderedFloat<f64>, f64)>>,
    pub sv_labels: Vec<String>,
    pub t_init: Instant,
    pub time: VecDeque<f64>,
}

impl TrackingSignalsTab {
    pub fn new() -> TrackingSignalsTab {
        TrackingSignalsTab {
            at_least_one_track_received: false,
            cn0_dict: HashMap::new(),
            cn0_age: HashMap::new(),
            colors: Vec::new(),
            glo_fcn_dict: HashMap::new(),
            glo_slot_dict: HashMap::new(),
            gps_tow: 0.0,
            gps_week: 0,
            incoming_obs_cn0: HashMap::new(),
            last_update_time: Instant::now(),
            max: TRACKING_SIGNALS_PLOT_MAX,
            min: SNR_THRESHOLD,
            prev_obs_count: 0,
            prev_obs_total: 0,
            received_codes: Vec::new(),
            sats: Vec::new(),
            sv_labels: {
                let mut labels: Vec<String> = Vec::new();
                for code in SUPPORTED_CODES {
                    let code_str = code_to_str_map(*code);
                    labels.push(String::from(code_str));
                }
                labels
            },
            t_init: Instant::now(),
            time: {
                let mut time: Deque<f64> = Deque::with_capacity(NUM_POINTS);
                for x in (0..(NUM_POINTS as i32)).rev() {
                    time.push_back((-x as f64)*(1.0/TRK_RATE));
                }
                time
            },
            

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
        // for (key, vals) in self.cn0_dict.iter() {
        //     let hey: Vec<f64> = vals.iter().map(|&x| x.1).collect();
        //     println!("{:?} {:?}", key, hey);
        // }        
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
                // println!("{:?}", self.glo_fcn_dict);
                // println!("{:?}>{:?}=={:?}", state.mesid.sat, GLO_SLOT_SAT_MAX, state.mesid.sat>GLO_SLOT_SAT_MAX);
                if state.mesid.sat > GLO_SLOT_SAT_MAX {
                    self.glo_fcn_dict.insert(idx as u8, state.mesid.sat as i16 - 100.0 as i16);
                }
                println!("{:?}", self.glo_fcn_dict);
                sat = *self.glo_fcn_dict.get(&(idx as u8)).unwrap_or(&(0 as i16));
                
                if state.mesid.sat <= GLO_SLOT_SAT_MAX {
                    self.glo_slot_dict.insert(sat, state.mesid.sat as i16);
                }
            }
            
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
        // for (key, cn0_deque) in self.cn0_dict.iter_mut() {
        //     if !codes_that_came.contains(key){
        //         cn0_deque.add((OrderedFloat(t), 0.0));
        //     }
        // }
        self.clean_cn0();
        self.update_plot();
        self.send_data(client_send);
    }
    pub fn handle_msg_tracking_state(&mut self, states: Vec<TrackingChannelState>, client_send: Sender<Vec<u8>>) {
        self.at_least_one_track_received = true;
        let mut codes_that_came: Vec<(u8,i16)> = Vec::new();
        let t = (Instant::now()).duration_since(self.t_init).as_secs_f64();
        self.time.add(t);
        for (idx, state) in states.iter().enumerate() {
            let mut sat = state.sid.sat as i16;
            if code_is_glo(state.sid.code) {
                if state.sid.sat > GLO_SLOT_SAT_MAX {
                    sat = sat - 100.0 as i16;
                } else {
                    sat = state.fcn as i16 - GLO_FCN_OFFSET;
                }
                self.glo_slot_dict.insert(sat, state.sid.sat as i16);
            }
            let key = (state.sid.code, sat);
            codes_that_came.push(key);
            if state.cn0 != 0 {
                self.push_to_cn0_dict(key, t, state.cn0 as f64 / 4.0);
                self.push_to_cn0_age(key, t);
            }
            if !self.received_codes.contains(&state.sid.code){
                self.received_codes.push(state.sid.code);
            }
        }
        // for (key, cn0_deque) in self.cn0_dict.iter_mut() {
        //     if !codes_that_came.contains(key){
        //         cn0_deque.add((OrderedFloat(t), 0.0));
        //     }
        // }
        self.clean_cn0();
        self.update_plot();
        self.send_data(client_send);
    }
    pub fn handle_obs(&mut self, msg: ObservationMsg, client_send: Sender<Vec<u8>>) {
                
        let (seq, tow, wn, states) = match &msg {
            ObservationMsg::MsgObs(obs) => {
                let states: Vec<Observations> = obs.obs.clone().into_iter().map(|x| Observations::PackedObsContent(x)).collect();
                (obs.header.n_obs, obs.header.t.tow as f64 /1000.0_f64, obs.header.t.wn,states)
            }
            // ObservationMsg::MsgObsDepA(obs)
            ObservationMsg::MsgObsDepB(obs) => {
                let states: Vec<Observations> = obs.obs.clone().into_iter().map(|x| Observations::PackedObsContentDepB(x)).collect();
                (obs.header.n_obs, obs.header.t.tow as f64 /1000.0_f64, obs.header.t.wn,states)
            }
            ObservationMsg::MsgObsDepC(obs) => {
                let states: Vec<Observations> = obs.obs.clone().into_iter().map(|x| Observations::PackedObsContentDepC(x)).collect();
                (obs.header.n_obs, obs.header.t.tow as f64 /1000.0_f64, obs.header.t.wn,states)
            }
        };

        // obs_packed_callback
        let total = seq >> 4;
        let count = seq & ((1 << 4) - 1);

        if count == 0 {
            self.obs_reset(tow, wn, total);
        } else if self.gps_tow != tow || self.gps_week != wn || self.prev_obs_count + 1 != count || self.prev_obs_total != total {
            println!("We dropped a packet. Skipping this ObservationMsg sequence");
            self.obs_reset(tow, wn, total);
            self.prev_obs_count = count;
            return;
        } else {
            self.prev_obs_count = count;
        }

        for (idx, state) in states.iter().enumerate() {
            
            let (code, sat, cn0) = match state {
                // Observations::PackedObsContentDepA(obs) => {
                //     let mut sat_ = obs.prn as i16;
                //     if code_is_gps(obs.prn){
                //         sat_ += 1;
                //     }
                //     (sat_ as u8, sat_, obs.cn0)
                // }
                Observations::PackedObsContentDepB(obs) => {
                    let mut sat_ = obs.sid.sat as i16;
                    if code_is_gps(obs.sid.code){
                        sat_ += 1;
                    }
                    (obs.sid.code, sat_, obs.cn0 as f64)
                }
                Observations::PackedObsContentDepC(obs) => {
                    let mut sat_ = obs.sid.sat as i16;
                    if code_is_gps(obs.sid.code){
                        sat_ += 1;
                    }
                    (obs.sid.code, sat_, obs.cn0 as f64)
                }
                Observations::PackedObsContent(obs) => (obs.sid.code, obs.sid.sat as i16, obs.cn0 as f64)
            };
            self.incoming_obs_cn0.insert((code, sat), cn0/ 4.0);
        }

        if count == (total - 1) && (Instant::now()).duration_since(self.last_update_time).as_secs_f64() < GUI_UPDATE_PERIOD{
            self.last_update_time = Instant::now();
            self.update_from_obs(self.incoming_obs_cn0.clone(), client_send);
        }
    }

    pub fn update_from_obs(&mut self, obs_dict: HashMap<(u8, i16), f64>, client_send: Sender<Vec<u8>>) {
        if self.at_least_one_track_received{
            return;
        }

        let mut codes_that_came: Vec<(u8,i16)> = Vec::new();
        let t = (Instant::now()).duration_since(self.t_init).as_secs_f64();
        self.time.add(t);
        for (key, cn0) in obs_dict.iter() {
            let (code, _) = key;
            codes_that_came.push(*key);
            if *cn0 > 0.0_f64 {
                self.push_to_cn0_dict(*key, t, *cn0);
                self.push_to_cn0_age(*key, t);
            }
            if !self.received_codes.contains(&code){
                self.received_codes.push(code.clone());
            }
        }
        // for (key, cn0_deque) in self.cn0_dict.iter_mut() {
        //     if !codes_that_came.contains(key){
        //         cn0_deque.add((OrderedFloat(t), 0.0));
        //     }
        // }
        self.clean_cn0();
        self.update_plot();
        self.send_data(client_send);
    }
    pub fn obs_reset(&mut self, tow: f64, wn: u16, obs_total: u8) {
        self.gps_tow = tow;
        self.gps_week = wn;
        self.prev_obs_total = obs_total;
        self.prev_obs_count = 0;
        self.incoming_obs_cn0.clear()
    }
}

pub enum ObservationMsg {
    MsgObs(MsgObs),
    // MsgObsDepA(MsgObsDepA),
    MsgObsDepB(MsgObsDepB),
    MsgObsDepC(MsgObsDepC),
}
pub enum Observations {
    PackedObsContent(PackedObsContent),
    // PackedObsContentDepA(PackedObsContentDepA),
    PackedObsContentDepB(PackedObsContentDepB),
    PackedObsContentDepC(PackedObsContentDepC),
}

// #[test]
// fn test_append_tracking_signals_tab() {
//     let mut tst = TrackingSignalsTab::new();
//     assert_eq!(tst.sats.shape(), &[NUM_SATELLITES, NUM_POINTS]);
//     let ele1 = (64.0 as f64, OrderedFloat(999888.0 as f64));
//     tst.append_to_sat(36 as usize, &ele1.clone());
//     assert_eq!(&ele1, tst.sats.get((36, NUM_POINTS-1)).unwrap());
// }