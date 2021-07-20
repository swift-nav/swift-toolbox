use log::error;

use capnp::message::Builder;
use sbp::messages::piksi::MsgSpecan;

use crate::fft_monitor::{FftMonitor, AMPLITUDES, FREQUENCIES};
use crate::types::{CapnProtoSender, SharedState};
use crate::utils::serialize_capnproto_builder;

/// AdvancedSpectrumAnalyzerTab struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
pub struct AdvancedSpectrumAnalyzerTab<S: CapnProtoSender> {
    client_send: S,
    pub fft_monitor: FftMonitor,
    most_recent_amplitudes: Vec<f32>,
    most_recent_frequencies: Vec<f32>,
    shared_state: SharedState,
}

impl<S: CapnProtoSender> AdvancedSpectrumAnalyzerTab<S> {
    pub fn new(shared_state: SharedState, client_send: S) -> AdvancedSpectrumAnalyzerTab<S> {
        let mut fft_monitor = FftMonitor::new();
        fft_monitor.enable_channel(None);
        AdvancedSpectrumAnalyzerTab {
            fft_monitor,
            client_send,
            most_recent_amplitudes: vec![],
            most_recent_frequencies: vec![],
            shared_state,
        }
    }

    /// Handler for Specan messages.
    ///
    /// # Parameters
    /// - `msg`: MsgSpecan to extract data from.
    pub fn handle_specan(&mut self, msg: MsgSpecan) {
        if let Err(err) = self.fft_monitor.capture_fft(msg.clone()) {
            error!("{}", err);
            return;
        }
        let channel = 2;
        if let Some(num_ffts) = self.fft_monitor.num_ffts(channel) {
            if num_ffts > 0 {
                if let Some(mut ffts) = self.fft_monitor.get_ffts(channel) {
                    if let Some(most_recent_fft) = ffts.pop() {
                        self.fft_monitor.clear_ffts(None);
                        if let Some(amplitudes) = most_recent_fft.get(&String::from(AMPLITUDES)) {
                            self.most_recent_amplitudes = amplitudes.clone();
                        }
                        if let Some(frequencies) = most_recent_fft.get(&String::from(FREQUENCIES)) {
                            self.most_recent_frequencies = frequencies.clone();
                        }

                        self.send_data();
                    }
                }
            }
        }
    }

    // /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab_status = msg.init_advanced_spectrum_analyzer_status();


        
        let mut xmin = f32::MAX;
        let mut xmax = f32::MIN;
        let mut ymin = f32::MAX;
        let mut ymax = f32::MIN;
        
        let mut point_vals = tab_status.reborrow().init_data(self.most_recent_amplitudes.len() as u32);
        for idx in 0..self.most_recent_amplitudes.len() {
            let mut point_val = point_vals.reborrow().get(idx as u32);
            point_val.set_x(self.most_recent_frequencies[idx] as f64);
            point_val.set_y(self.most_recent_amplitudes[idx] as f64);
            ymin = f32::min(self.most_recent_amplitudes[idx], ymin);
            ymax = f32::max(self.most_recent_amplitudes[idx], ymax);
            xmin = f32::min(self.most_recent_frequencies[idx], xmin);
            xmax = f32::max(self.most_recent_frequencies[idx], xmax);
        }
        
        let channel: u16 = 0;
        tab_status.set_channel(channel);
        tab_status.set_ymin(ymin);
        tab_status.set_ymax(ymax);
        tab_status.set_xmin(xmin);
        tab_status.set_xmax(xmax);

        self.client_send
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TestSender;
    use sbp::messages::imu::{MsgImuAux, MsgImuRaw};

    // #[test]
    // fn handle_imu_raw_test() {
    //     let shared_state = SharedState::new();
    //     let client_send = TestSender { inner: Vec::new() };
    //     let mut ins_tab = AdvancedSpectrumAnalyzerTab::new(shared_state, client_send);
    //     let tow = 1_u32;
    //     let tow_f = 1_u8;
    //     let acc_x = 2_i16;
    //     let acc_y = 3_i16;
    //     let acc_z = 4_i16;
    //     let gyr_x = 5_i16;
    //     let gyr_y = 6_i16;
    //     let gyr_z = 7_i16;
    //     let msg = MsgImuRaw {
    //         sender_id: Some(1337),
    //         tow,
    //         tow_f,
    //         acc_x,
    //         acc_y,
    //         acc_z,
    //         gyr_x,
    //         gyr_y,
    //         gyr_z,
    //     };
    //     let acc_xs = ins_tab.acc_x.get();
    //     let acc_ys = ins_tab.acc_y.get();
    //     let acc_zs = ins_tab.acc_z.get();
    //     let gyro_xs = ins_tab.gyro_x.get();
    //     let gyro_ys = ins_tab.gyro_y.get();
    //     let gyro_zs = ins_tab.gyro_z.get();
    //     for idx in 0..NUM_POINTS {
    //         assert!(f64::abs(acc_xs[idx] - 0_f64) <= f64::EPSILON);
    //         assert!(f64::abs(acc_ys[idx] - 0_f64) <= f64::EPSILON);
    //         assert!(f64::abs(acc_zs[idx] - 0_f64) <= f64::EPSILON);
    //         assert!(f64::abs(gyro_xs[idx] - 0_f64) <= f64::EPSILON);
    //         assert!(f64::abs(gyro_ys[idx] - 0_f64) <= f64::EPSILON);
    //         assert!(f64::abs(gyro_zs[idx] - 0_f64) <= f64::EPSILON);
    //     }
    //     assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) <= f64::EPSILON);
    //     ins_tab.handle_imu_raw(msg);
    //     let acc_xs = ins_tab.acc_x.get();
    //     let acc_ys = ins_tab.acc_y.get();
    //     let acc_zs = ins_tab.acc_z.get();
    //     let gyro_xs = ins_tab.gyro_x.get();
    //     let gyro_ys = ins_tab.gyro_y.get();
    //     let gyro_zs = ins_tab.gyro_z.get();
    //     assert!(f64::abs(acc_xs[NUM_POINTS - 1] - acc_x as f64) <= f64::EPSILON);
    //     assert!(f64::abs(acc_ys[NUM_POINTS - 1] - acc_y as f64) <= f64::EPSILON);
    //     assert!(f64::abs(acc_zs[NUM_POINTS - 1] - acc_z as f64) <= f64::EPSILON);
    //     assert!(f64::abs(gyro_xs[NUM_POINTS - 1] - gyr_x as f64) <= f64::EPSILON);
    //     assert!(f64::abs(gyro_ys[NUM_POINTS - 1] - gyr_y as f64) <= f64::EPSILON);
    //     assert!(f64::abs(gyro_zs[NUM_POINTS - 1] - gyr_z as f64) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) > f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) > f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) > f64::EPSILON);
    // }

    // #[test]
    // fn handle_imu_aux_test() {
    //     let shared_state = SharedState::new();
    //     let client_send = TestSender { inner: Vec::new() };
    //     let mut ins_tab = AdvancedSpectrumAnalyzerTab::new(shared_state.clone(), client_send.clone());
    //     let imu_type_a = 0_u8;
    //     let imu_type_b = 1_u8;
    //     let imu_type_unknown = 2_u8;
    //     let imu_conf = 1_u8;
    //     let temp = 200;
    //     let msg = MsgImuAux {
    //         sender_id: Some(1337),
    //         imu_type: imu_type_unknown,
    //         imu_conf,
    //         temp,
    //     };
    //     assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
    //     assert_eq!(ins_tab.imu_conf, 0_u8);
    //     ins_tab.handle_imu_aux(msg);
    //     assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
    //     assert_ne!(ins_tab.imu_conf, imu_conf);

    //     let mut ins_tab = AdvancedSpectrumAnalyzerTab::new(shared_state.clone(), client_send.clone());
    //     let msg = MsgImuAux {
    //         sender_id: Some(1337),
    //         imu_type: imu_type_a,
    //         imu_conf,
    //         temp,
    //     };
    //     assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
    //     assert_eq!(ins_tab.imu_conf, 0_u8);
    //     ins_tab.handle_imu_aux(msg);
    //     assert!(f64::abs(ins_tab.imu_temp - 23.390625_f64) <= f64::EPSILON);
    //     assert_eq!(ins_tab.imu_conf, imu_conf);

    //     let mut ins_tab = AdvancedSpectrumAnalyzerTab::new(shared_state, client_send);
    //     let msg = MsgImuAux {
    //         sender_id: Some(1337),
    //         imu_type: imu_type_b,
    //         imu_conf,
    //         temp,
    //     };
    //     assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
    //     assert_eq!(ins_tab.imu_conf, 0_u8);
    //     ins_tab.handle_imu_aux(msg);
    //     assert!(f64::abs(ins_tab.imu_temp - 25.78125_f64) <= f64::EPSILON);
    //     assert_eq!(ins_tab.imu_conf, imu_conf);
    // }

    // #[test]
    // fn handle_imu_send_data_test() {
    //     let shared_state = SharedState::new();
    //     let client_send = TestSender { inner: Vec::new() };
    //     let mut ins_tab = AdvancedSpectrumAnalyzerTab::new(shared_state, client_send);

    //     assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) <= f64::EPSILON);

    //     let imu_type = 0_u8;
    //     let imu_conf = 1_u8;
    //     let temp = 200;
    //     let msg = MsgImuAux {
    //         sender_id: Some(1337),
    //         imu_type,
    //         imu_conf,
    //         temp,
    //     };
    //     ins_tab.handle_imu_aux(msg);

    //     let tow = 1_u32;
    //     let tow_f = 1_u8;
    //     let acc_x = 2_i16;
    //     let acc_y = 3_i16;
    //     let acc_z = 4_i16;
    //     let gyr_x = 5_i16;
    //     let gyr_y = 6_i16;
    //     let gyr_z = 7_i16;
    //     let msg = MsgImuRaw {
    //         sender_id: Some(1337),
    //         tow,
    //         tow_f,
    //         acc_x,
    //         acc_y,
    //         acc_z,
    //         gyr_x,
    //         gyr_y,
    //         gyr_z,
    //     };
    //     ins_tab.handle_imu_raw(msg);
    //     let sig_figs = 0.0001220703125_f64;
    //     let acc_x = acc_x as f64;
    //     let acc_y = acc_y as f64;
    //     let acc_z = acc_z as f64;

    //     let rms_acc_x = f64::sqrt((acc_x * acc_x) / NUM_POINTS as f64);
    //     let rms_acc_y = f64::sqrt((acc_y * acc_y) / NUM_POINTS as f64);
    //     let rms_acc_z = f64::sqrt((acc_z * acc_z) / NUM_POINTS as f64);
    //     assert!(f64::abs(ins_tab.rms_acc_x - rms_acc_x * sig_figs) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_y - rms_acc_y * sig_figs) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_z - rms_acc_z * sig_figs) <= f64::EPSILON);

    //     let acc_x = 4_i16;
    //     let acc_y = 6_i16;
    //     let acc_z = 8_i16;
    //     let msg = MsgImuRaw {
    //         sender_id: Some(1337),
    //         tow,
    //         tow_f,
    //         acc_x,
    //         acc_y,
    //         acc_z,
    //         gyr_x,
    //         gyr_y,
    //         gyr_z,
    //     };
    //     ins_tab.handle_imu_raw(msg);
    //     let sig_figs = 0.0001220703125_f64;
    //     let acc_x = acc_x as f64;
    //     let acc_y = acc_y as f64;
    //     let acc_z = acc_z as f64;
    //     let rms_acc_x =
    //         f64::sqrt((acc_x * acc_x + (acc_x / 2_f64) * (acc_x / 2_f64)) / NUM_POINTS as f64);
    //     let rms_acc_y =
    //         f64::sqrt((acc_y * acc_y + (acc_y / 2_f64) * (acc_y / 2_f64)) / NUM_POINTS as f64);
    //     let rms_acc_z =
    //         f64::sqrt((acc_z * acc_z + (acc_z / 2_f64) * (acc_z / 2_f64)) / NUM_POINTS as f64);
    //     assert!(f64::abs(ins_tab.rms_acc_x - rms_acc_x * sig_figs) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_y - rms_acc_y * sig_figs) <= f64::EPSILON);
    //     assert!(f64::abs(ins_tab.rms_acc_z - rms_acc_z * sig_figs) <= f64::EPSILON);
    // }
}
