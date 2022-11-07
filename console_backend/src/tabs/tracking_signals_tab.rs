use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use capnp::message::Builder;
use log::warn;
use ordered_float::OrderedFloat;
use sbp::messages::tracking::{MeasurementState, TrackingChannelState};

use crate::client_sender::BoxedClientSender;
use crate::constants::{
    CHART_XMIN_OFFSET_NO_TRACKING, CHART_XMIN_OFFSET_TRACKING, GLO_FCN_OFFSET, GLO_SLOT_SAT_MAX,
    NUM_POINTS, TRACKING_UPDATE_PERIOD, TRK_RATE,
};
use crate::piksi_tools_constants::{
    BDS2_B1_STR, BDS2_B2_STR, GAL_E1B_STR, GAL_E1X_STR, GAL_E7I_STR, GAL_E7Q_STR, GLO_L1OF_STR,
    GLO_L2OF_STR, GPS_L1CA_STR, GPS_L2CM_STR, QZS_L1CA_STR, QZS_L2CM_STR, SBAS_L1_STR,
};
use crate::shared_state::SharedState;
use crate::types::{Cn0Age, Cn0Dict, Deque, ObservationMsg, SignalCodes};
use crate::utils::{serialize_capnproto_builder, signal_key_color, signal_key_label};

/// TrackingSignalsTab struct.
///
/// # Fields:
///
/// - `at_least_one_track_received`: Whether a MsgTrackingState has been received. If so block Obs Msgs from being processed.
/// - `cn0_age`: Main storage of (code, sat) keys corresponding to cn0 age.
/// - `cn0_dict`: Main storage of (code, sat) keys corresponding to cn0 values.
/// - `colors`: Stored rgb codes for frontend correspond to index of sv_labels.
/// - `glo_fcn_dict`:  Storage of glonass sat codes if 100 +[-6, 7] case.
/// - `glo_slot_dict`: Storage of glonass sat codes if [1, 28] slot.
/// - `gps_tow`: The GPS Time of Week.
/// - `gps_week`: The GPS week.
/// - `incoming_obs_cn0`: Map used for accumulating (key, cn0) pairs before performing update_from_obs.
/// - `last_update_time`: Instant monotonic time checking if enough time has passed before performing update_from_obs.
/// - `max`: Stored maximum dB-Hz used for frontend plot.
/// - `min`: Stored minimum dB-Hz used for frontend plot.
/// - `prev_obs_count`: Previous observation count of total expected messages.
/// - `prev_obs_total`: Previous total expected observation messages.
/// - `sats`: Stored satellite data (NUM_SATS, NUM_POINTS) to be sent to frontend.
/// - `sv_labels`: Vector used to store sorted labels before sending to frontend.
/// - `t_init`: Instant monotonic time used as starting reference time.
/// - `time`: Vector of Monotic times stored.
#[derive(Debug)]
pub struct TrackingSignalsTab {
    pub at_least_one_track_received: bool,
    pub check_labels: [&'static str; 13],
    pub client_sender: BoxedClientSender,
    pub cn0_age: Cn0Age,
    pub cn0_dict: Cn0Dict,
    pub colors: Vec<String>,
    pub glo_fcn_dict: HashMap<u8, i16>,
    pub glo_slot_dict: HashMap<i16, i16>,
    pub gps_tow: f64,
    pub gps_week: u16,
    pub incoming_obs_cn0: HashMap<(SignalCodes, i16), f64>,
    pub last_obs_update_time: Instant,
    pub last_update_time: Instant,
    pub prev_obs_count: u8,
    pub prev_obs_total: u8,
    pub received_codes: Vec<SignalCodes>,
    pub sats: Vec<Deque<(OrderedFloat<f64>, f64)>>,
    pub shared_state: SharedState,
    pub sv_labels: Vec<String>,
    pub t_init: Instant,
    pub time: Deque<f64>,
}

impl TrackingSignalsTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> TrackingSignalsTab {
        TrackingSignalsTab {
            at_least_one_track_received: false,
            check_labels: [
                GPS_L1CA_STR,
                GPS_L2CM_STR,
                GLO_L1OF_STR,
                GLO_L2OF_STR,
                BDS2_B1_STR,
                BDS2_B2_STR,
                GAL_E1B_STR,
                GAL_E1X_STR,
                GAL_E7I_STR,
                GAL_E7Q_STR,
                QZS_L1CA_STR,
                QZS_L2CM_STR,
                SBAS_L1_STR,
            ],
            client_sender,
            cn0_dict: Cn0Dict::new(),
            cn0_age: Cn0Age::new(),
            colors: Vec::new(),
            glo_fcn_dict: HashMap::new(),
            glo_slot_dict: HashMap::new(),
            gps_tow: 0.0,
            gps_week: 0,
            incoming_obs_cn0: HashMap::new(),
            last_obs_update_time: Instant::now(),
            last_update_time: Instant::now(),
            prev_obs_count: 0,
            prev_obs_total: 0,
            received_codes: Vec::new(),
            sats: Vec::new(),
            shared_state,
            sv_labels: Vec::new(),
            t_init: Instant::now(),
            time: {
                let mut time = Deque::new(NUM_POINTS);
                for x in (0..(NUM_POINTS as i32)).rev() {
                    time.push((-x as f64) * (1.0 / TRK_RATE));
                }
                time
            },
        }
    }
    /// Push to existing entry in cn0 dict or create Deque and store.
    ///
    /// # Parameters:
    ///
    /// - `key`: The (code, sat) to store.
    /// - `cn0`: The carrier-to-noise density.
    fn push_to_cn0_dict(&mut self, key: (SignalCodes, i16), t: f64, cn0: f64) {
        let cn0_deque = self
            .cn0_dict
            .entry(key)
            .or_insert_with(|| Deque::new(NUM_POINTS));
        cn0_deque.push((OrderedFloat(t), cn0));
    }
    /// Push carrier-to-noise density age to cn0_age with key.
    ///
    /// # Parameters:
    ///
    /// - `key`: The (code, sat) to store.
    /// - `age`: The time elapsed since last start.
    fn push_to_cn0_age(&mut self, key: (SignalCodes, i16), age: f64) {
        let cn0_age = self.cn0_age.entry(key).or_insert(-1.0);
        *cn0_age = age;
    }

    /// Remove cn0 data if age is too old.
    pub fn clean_cn0(&mut self) {
        let mut remove_vec: Vec<(SignalCodes, i16)> = Vec::new();
        for (key, _) in self.cn0_dict.iter_mut() {
            if self.cn0_age[key] < self.time[0] {
                remove_vec.push(*key);
            }
        }
        for key in remove_vec {
            self.cn0_dict.remove(&key);
            self.cn0_age.remove(&key);
        }
    }

    /// Clear and prepare sv_labels, colors, and sats to send data to frontend.
    pub fn update_plot(&mut self) {
        self.sv_labels.clear();
        self.colors.clear();
        self.sats.clear();
        let mut temp_labels = Vec::new();
        let filters = self
            .shared_state
            .lock()
            .tracking_tab
            .signals_tab
            .check_visibility
            .clone();
        let mut tracked_sv_labels = vec![];
        for (key, _) in self.cn0_dict.iter_mut() {
            let (signal_code, _) = key;
            if let Some(filter) = signal_code.filters() {
                if filters.contains(&filter) {
                    continue;
                }
            }
            let (code_lbl, freq_lbl, id_lbl) = signal_key_label(*key, Some(&self.glo_slot_dict));
            let mut label = String::from("");
            if let Some(lbl) = code_lbl {
                label = format!("{label} {lbl}");
            }
            if let Some(lbl) = freq_lbl {
                label = format!("{label} {lbl}");
            }
            if let Some(lbl) = id_lbl {
                tracked_sv_labels.push(lbl.clone());
                label = format!("{label} {lbl}");
            }

            temp_labels.push((label, *key));
        }
        temp_labels.sort_by(|x, y| (x.0).cmp(&(y.0)));

        for (label, key) in temp_labels.iter() {
            self.sv_labels.push(label.clone());
            self.colors.push(String::from(signal_key_color(*key)));
            self.sats.push(self.cn0_dict[key].clone());
        }
        self.shared_state
            .lock()
            .tracking_tab
            .signals_tab
            .tracked_sv_labels = tracked_sv_labels;
    }

    /// Handle MsgMeasurementState message states.
    ///
    /// # Parameters:
    ///
    /// - `states`: All states contained within the measurementstate message.
    pub fn handle_msg_measurement_state(&mut self, states: Vec<MeasurementState>) {
        self.at_least_one_track_received = true;
        let mut codes_that_came: Vec<(SignalCodes, i16)> = Vec::new();
        let t = (Instant::now()).duration_since(self.t_init).as_secs_f64();
        self.time.push(t);
        for (idx, state) in states.iter().enumerate() {
            let mut sat = state.mesid.sat as i16;
            let signal_code = SignalCodes::from(state.mesid.code);
            if signal_code.code_is_glo() {
                if state.mesid.sat > GLO_SLOT_SAT_MAX {
                    self.glo_fcn_dict
                        .insert(idx as u8, state.mesid.sat as i16 - 100);
                }
                sat = *self.glo_fcn_dict.get(&(idx as u8)).unwrap_or(&(0_i16));

                if state.mesid.sat <= GLO_SLOT_SAT_MAX {
                    self.glo_slot_dict.insert(sat, state.mesid.sat as i16);
                }
            }
            let key = (signal_code, sat);
            codes_that_came.push(key);
            if state.cn0 != 0 {
                self.push_to_cn0_dict(key, t, state.cn0 as f64 / 4.0);
                self.push_to_cn0_age(key, t);
            }
            if !self.received_codes.contains(&signal_code) {
                self.received_codes.push(signal_code);
            }
        }
        for (key, cn0_deque) in self.cn0_dict.iter_mut() {
            if !codes_that_came.contains(key) {
                cn0_deque.push((OrderedFloat(t), 0.0));
            }
        }
        self.clean_cn0();
        self.update_plot();
        self.send_data();
    }
    /// Handle MsgTrackingState message states.
    ///
    /// # Parameters:
    ///
    /// - `states`: All states contained within the trackingstate message.
    pub fn handle_msg_tracking_state(&mut self, states: Vec<TrackingChannelState>) {
        self.at_least_one_track_received = true;
        let mut codes_that_came: Vec<(SignalCodes, i16)> = Vec::new();
        let t = (Instant::now()).duration_since(self.t_init).as_secs_f64();
        self.time.push(t);
        for state in states.iter() {
            let mut sat = state.sid.sat as i16;
            let signal_code = SignalCodes::from(state.sid.code);
            if signal_code.code_is_glo() {
                if state.sid.sat > GLO_SLOT_SAT_MAX {
                    sat -= 100.0 as i16;
                } else {
                    sat -= state.fcn as i16 - GLO_FCN_OFFSET;
                }
                self.glo_slot_dict.insert(sat, state.sid.sat as i16);
            }
            let key = (signal_code, sat);
            codes_that_came.push(key);
            if state.cn0 != 0 {
                self.push_to_cn0_dict(key, t, state.cn0 as f64 / 4.0);
                self.push_to_cn0_age(key, t);
            }
        }
        self.clean_cn0();
        self.update_plot();
        self.send_data();
    }
    /// Handle MsgObs, MsgObsDepB, and MsgObsDepC full messages.
    ///
    /// # Parameters:
    ///
    /// - `msg`: The full SBP message cast as an ObservationMsg variant.
    pub fn handle_obs(&mut self, msg: ObservationMsg) {
        let msg_fields = msg.fields();
        if let Some(sender_id_) = msg_fields.sender_id {
            if sender_id_ == 0_u16 {
                return;
            }
        } else {
            return;
        }

        let total = msg_fields.n_obs >> 4;
        let count = msg_fields.n_obs & ((1 << 4) - 1);

        if count == 0 {
            self.obs_reset(msg_fields.tow, msg_fields.wn, total);
        } else if (self.gps_tow - msg_fields.tow) > f64::EPSILON
            || self.gps_week != msg_fields.wn
            || self.prev_obs_count + 1 != count
            || self.prev_obs_total != total
        {
            warn!("We dropped a packet. Skipping this ObservationMsg sequence");
            self.obs_reset(msg_fields.tow, msg_fields.wn, total);
            self.prev_obs_count = count;
            return;
        } else {
            self.prev_obs_count = count;
        }

        for state in msg_fields.states.iter() {
            let obs_fields = state.fields();
            let (code, sat, cn0) = (obs_fields.code, obs_fields.sat, obs_fields.cn0);
            self.incoming_obs_cn0.insert((code, sat), cn0 / 4.0);
        }

        if count == (total - 1) {
            self.last_update_time = Instant::now();
            self.update_from_obs(self.incoming_obs_cn0.clone());
        }
    }

    /// Update cn0 dict using the observation data accumulated by handle_obs.
    ///
    /// # Parameters:
    ///
    /// - `msg`: The full SBP message cast as an ObservationMsg variant.
    pub fn update_from_obs(&mut self, obs_dict: HashMap<(SignalCodes, i16), f64>) {
        if self.at_least_one_track_received {
            return;
        }
        if Instant::now() - self.last_obs_update_time
            <= Duration::from_secs_f64(TRACKING_UPDATE_PERIOD)
        {
            return;
        }
        self.last_obs_update_time = Instant::now();
        let mut codes_that_came: Vec<(SignalCodes, i16)> = Vec::new();
        let t = (Instant::now()).duration_since(self.t_init).as_secs_f64();
        self.time.push(t);
        for (key, cn0) in obs_dict.iter() {
            let (signal_code, _) = *key;
            codes_that_came.push(*key);
            if *cn0 > 0.0_f64 {
                self.push_to_cn0_dict(*key, t, *cn0);
                self.push_to_cn0_age(*key, t);
            }
            if !self.received_codes.contains(&signal_code) {
                self.received_codes.push(signal_code);
            }
        }
        for (key, cn0_deque) in self.cn0_dict.iter_mut() {
            if !codes_that_came.contains(key) {
                cn0_deque.push((OrderedFloat(t), 0.0));
            }
        }
        self.clean_cn0();
        self.update_plot();
        self.send_data();
    }

    /// Reset observation cn0 data in the event of empty observation packet drop.
    ///
    /// # Parameters:
    ///
    /// - `tow`: The GPS time of week.
    /// - `wn`: The current GPS week number.
    /// - `obs_total`: The current observation message total to store.
    pub fn obs_reset(&mut self, tow: f64, wn: u16, obs_total: u8) {
        self.gps_tow = tow;
        self.gps_week = wn;
        self.prev_obs_total = obs_total;
        self.prev_obs_count = 0;
        self.incoming_obs_cn0.clear()
    }

    fn chart_xmin_offset(&self) -> f64 {
        if self.at_least_one_track_received {
            CHART_XMIN_OFFSET_TRACKING
        } else {
            CHART_XMIN_OFFSET_NO_TRACKING
        }
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tracking_signals_status = msg.init_tracking_signals_status();
        let mut labels = tracking_signals_status
            .reborrow()
            .init_labels(self.sv_labels.len() as u32);

        for (i, header) in self.sv_labels.iter().enumerate() {
            labels.set(i as u32, header);
        }

        let mut colors = tracking_signals_status
            .reborrow()
            .init_colors(self.colors.len() as u32);

        for (i, color) in self.colors.iter().enumerate() {
            colors.set(i as u32, color);
        }

        // +1 for the invisible series we always plot
        let num_series = self.sv_labels.len() as u32 + 1;
        let mut tracking_points = tracking_signals_status.reborrow().init_data(num_series);
        {
            for idx in 0..self.sv_labels.len() {
                let points = &mut self.sats[idx];
                let mut point_val_idx = tracking_points
                    .reborrow()
                    .init(idx as u32, points.len() as u32);
                for (i, (OrderedFloat(x), y)) in points.iter().enumerate() {
                    let mut point_val = point_val_idx.reborrow().get(i as u32);
                    point_val.set_x(*x);
                    point_val.set_y(*y);
                }
            }
            // Send a set of points that plots the time we received each update.
            // This way the tracking signals plot always moves even if no sats are selected.
            let mut point_val_idx = tracking_points
                .reborrow()
                .init(num_series - 1, self.time.len() as u32);
            for (i, t) in self.time.iter().enumerate() {
                let mut point_val = point_val_idx.reborrow().get(i as u32);
                point_val.set_x(*t);
                point_val.set_y(0.);
            }
        }
        tracking_signals_status.set_xmin_offset(self.chart_xmin_offset());
        let mut tracking_checkbox_labels = tracking_signals_status
            .reborrow()
            .init_check_labels(self.check_labels.len() as u32);
        for (i, label) in self.check_labels.iter().enumerate() {
            tracking_checkbox_labels.set(i as u32, label);
        }
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;
    use crate::client_sender::TestSender;
    use sbp::messages::{
        gnss::{CarrierPhase, GnssSignal, GpsTime},
        observation::{Doppler, MsgObs, ObservationHeader, PackedObsContent},
    };

    #[test]
    fn cn0_age_and_cn0_dict_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tracking_signals_tab = TrackingSignalsTab::new(shared_state, client_send);
        let t_init = tracking_signals_tab.t_init;
        let signal_code = SignalCodes::from(4_u8);
        let sat = 5_i16;
        let key = (signal_code, sat);

        for idx in 0..NUM_POINTS {
            let mut cn0_age = 0_f64;
            if idx < NUM_POINTS / 2 {
                cn0_age = (NUM_POINTS - idx) as f64;
            }
            let t = (Instant::now()).duration_since(t_init).as_secs_f64();
            tracking_signals_tab.time.push(t);
            tracking_signals_tab.push_to_cn0_dict(key, t, idx as f64);
            tracking_signals_tab.push_to_cn0_age(key, cn0_age);
        }
        assert_eq!(tracking_signals_tab.cn0_dict[&key].len(), NUM_POINTS);
        assert!(tracking_signals_tab.cn0_age[&key] - 1_f64 <= f64::EPSILON);

        let t = (Instant::now()).duration_since(t_init).as_secs_f64();
        tracking_signals_tab.time.push(t);
        let cn0 = 400.0;
        assert!(tracking_signals_tab.cn0_dict[&key][0].1 - 0_f64 <= f64::EPSILON);
        tracking_signals_tab.push_to_cn0_dict(key, t, cn0);

        assert_eq!(tracking_signals_tab.cn0_dict[&key].len(), NUM_POINTS);
        assert!(tracking_signals_tab.cn0_dict[&key][0].1 - 1_f64 <= f64::EPSILON);

        tracking_signals_tab.clean_cn0();
        assert_eq!(tracking_signals_tab.cn0_dict.len(), 0);
    }

    #[test]
    fn handle_msg_measurement_state_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tracking_signals_tab = TrackingSignalsTab::new(shared_state, client_send);

        let mut states: Vec<MeasurementState> = Vec::new();
        let glo_sat_above_one_hundred = 103;
        let glo_sat_under_one_hundred = 25;
        states.push(MeasurementState {
            cn0: 200_u8,
            mesid: GnssSignal {
                sat: glo_sat_above_one_hundred,
                code: 4,
            },
        });
        states.push(MeasurementState {
            cn0: 100_u8,
            mesid: GnssSignal {
                sat: glo_sat_under_one_hundred,
                code: 3,
            },
        });
        states.push(MeasurementState {
            cn0: 100_u8,
            mesid: GnssSignal { sat: 25, code: 5 },
        });

        tracking_signals_tab.handle_msg_measurement_state(states);
        assert_eq!(tracking_signals_tab.glo_fcn_dict.len(), 1);
        assert_eq!(
            tracking_signals_tab.glo_fcn_dict[&0_u8],
            glo_sat_above_one_hundred as i16 - 100_i16
        );
        assert_eq!(tracking_signals_tab.glo_slot_dict.len(), 1);
        assert_eq!(
            tracking_signals_tab.glo_slot_dict[&0_i16],
            glo_sat_under_one_hundred as i16
        );
        assert_eq!(tracking_signals_tab.cn0_dict.len(), 3);
        assert_eq!(tracking_signals_tab.sv_labels.len(), 3);
        assert_eq!(tracking_signals_tab.colors.len(), 3);
        assert_eq!(tracking_signals_tab.sats.len(), 3);
    }

    #[test]
    fn handle_msg_tracking_state_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tracking_signals_tab = TrackingSignalsTab::new(shared_state, client_send);

        let mut states: Vec<TrackingChannelState> = Vec::new();
        let glo_sat_above_one_hundred = 103;
        let glo_sat_under_one_hundred = 25;
        states.push(TrackingChannelState {
            cn0: 200_u8,
            fcn: 0,
            sid: GnssSignal {
                sat: glo_sat_above_one_hundred,
                code: 4,
            },
        });
        states.push(TrackingChannelState {
            cn0: 100_u8,
            fcn: 10,
            sid: GnssSignal {
                sat: glo_sat_under_one_hundred,
                code: 3,
            },
        });
        states.push(TrackingChannelState {
            cn0: 100_u8,
            fcn: 0,
            sid: GnssSignal { sat: 25, code: 5 },
        });
        tracking_signals_tab.handle_msg_tracking_state(states);

        assert_eq!(tracking_signals_tab.glo_slot_dict.len(), 2);
        assert_eq!(
            tracking_signals_tab.glo_slot_dict[&(glo_sat_above_one_hundred as i16 - 100_i16)],
            glo_sat_above_one_hundred as i16
        );
        assert_eq!(
            tracking_signals_tab.glo_slot_dict
                [&(glo_sat_under_one_hundred as i16 - (10 - GLO_FCN_OFFSET))],
            glo_sat_under_one_hundred as i16
        );
        assert_eq!(tracking_signals_tab.cn0_dict.len(), 3);
        assert_eq!(tracking_signals_tab.sv_labels.len(), 3);
        assert_eq!(tracking_signals_tab.colors.len(), 3);
        assert_eq!(tracking_signals_tab.sats.len(), 3);
    }

    #[test]
    fn handle_msg_obs_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tracking_signals_tab = TrackingSignalsTab::new(shared_state, client_send);
        let mut obs_msg = MsgObs {
            sender_id: Some(5),
            obs: Vec::new(),
            header: ObservationHeader {
                t: GpsTime {
                    tow: 0,
                    ns_residual: 0,
                    wn: 1,
                },
                n_obs: 16,
            },
        };
        let signal_code = 4;
        let sat = 25;
        obs_msg.obs.push(PackedObsContent {
            p: 0_u32,
            l: CarrierPhase { i: 0_i32, f: 0_u8 },
            d: Doppler { i: 0_i16, f: 0_u8 },
            cn0: 5,
            lock: 0,
            flags: 1,
            sid: GnssSignal {
                code: signal_code,
                sat,
            },
        });
        sleep(Duration::from_secs_f64(TRACKING_UPDATE_PERIOD));
        assert_eq!(tracking_signals_tab.cn0_dict.len(), 0);
        tracking_signals_tab.handle_obs(ObservationMsg::MsgObs(obs_msg));
        assert_eq!(tracking_signals_tab.cn0_dict.len(), 1);
    }

    #[test]
    fn update_plot_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tracking_signals_tab = TrackingSignalsTab::new(shared_state, client_send);
        let t = (Instant::now())
            .duration_since(tracking_signals_tab.t_init)
            .as_secs_f64();
        tracking_signals_tab.push_to_cn0_dict((SignalCodes::from(0), 6_i16), t, 5_f64);
        tracking_signals_tab.push_to_cn0_dict((SignalCodes::from(2), 7_i16), t, 6_f64);
        tracking_signals_tab.push_to_cn0_dict((SignalCodes::from(12), 8_i16), t, 7_f64);
        assert_eq!(tracking_signals_tab.sv_labels.len(), 0);
        assert_eq!(tracking_signals_tab.colors.len(), 0);
        assert_eq!(tracking_signals_tab.sats.len(), 0);
        tracking_signals_tab.update_plot();
        assert_eq!(tracking_signals_tab.sv_labels.len(), 3);
        assert_eq!(tracking_signals_tab.colors.len(), 3);
        assert_eq!(tracking_signals_tab.sats.len(), 3);
        assert_eq!(
            tracking_signals_tab.sv_labels,
            vec![" BDS2 B1 I C08", " GPS L1CA G06", " SBAS L1 S  7"]
        );
        tracking_signals_tab
            .shared_state
            .lock()
            .tracking_tab
            .signals_tab
            .check_visibility = vec![String::from(BDS2_B1_STR)];
        tracking_signals_tab.update_plot();
        assert_eq!(tracking_signals_tab.sv_labels.len(), 2);
        assert_eq!(tracking_signals_tab.colors.len(), 2);
        assert_eq!(tracking_signals_tab.sats.len(), 2);
        assert_eq!(
            tracking_signals_tab.sv_labels,
            vec![" GPS L1CA G06", " SBAS L1 S  7"]
        );
    }
}
