use log::error;
use sbp::messages::imu::{MsgImuAux, MsgImuRaw};

use capnp::message::Builder;

use crate::constants::*;
use crate::errors::GET_MUT_OBJECT_FAILURE;
use crate::fusion_status_flags::FusionStatusFlags;
use crate::types::{CapnProtoSender, Deque, SharedState};
use crate::utils::serialize_capnproto_builder;

/// AdvancedInsTab struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `imu_conf`: Storage for the Imu configuration.
/// - `imu_temp`: Storage for the raw Imu temperature converted to proper units.
/// - `rms_acc_x`: The calculated root mean squared imu acceleration for x axis.
/// - `rms_acc_y`: The calculated root mean squared imu acceleration for y axis.
/// - `rms_acc_z`: The calculated root mean squared imu acceleration for z axis.
/// - `acc_x`: The stored historic Imu acceleration values along x axis.
/// - `acc_y`: The stored historic Imu acceleration values along y axis.
/// - `acc_z`: The stored historic Imu acceleration values along z axis.
/// - `gyro_x`: The stored historic Imu angular rate values along x axis.
/// - `gyro_y`: The stored historic Imu angular rate values along y axis.
/// - `gyro_z`: The stored historic Imu angular rate values along z axis.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
#[derive(Debug)]
pub struct AdvancedInsTab<S: CapnProtoSender> {
    client_sender: S,
    pub fusion_engine_status_bar: FusionStatusFlags<S>,
    imu_conf: u8,
    imu_temp: f64,
    rms_acc_x: f64,
    rms_acc_y: f64,
    rms_acc_z: f64,
    acc_x: Deque<f64>,
    acc_y: Deque<f64>,
    acc_z: Deque<f64>,
    gyro_x: Deque<f64>,
    gyro_y: Deque<f64>,
    gyro_z: Deque<f64>,
    shared_state: SharedState,
}

impl<S: CapnProtoSender> AdvancedInsTab<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> AdvancedInsTab<S> {
        let acc_fill_val = Some(0_f64);
        let gyro_fill_val = Some(0_f64);
        AdvancedInsTab {
            fusion_engine_status_bar: FusionStatusFlags::new(
                shared_state.clone(),
                client_sender.clone(),
            ),
            client_sender,
            imu_conf: 0_u8,
            imu_temp: 0_f64,
            rms_acc_x: 0_f64,
            rms_acc_y: 0_f64,
            rms_acc_z: 0_f64,
            acc_x: Deque::with_size_limit(NUM_POINTS, acc_fill_val),
            acc_y: Deque::with_size_limit(NUM_POINTS, acc_fill_val),
            acc_z: Deque::with_size_limit(NUM_POINTS, acc_fill_val),
            gyro_x: Deque::with_size_limit(NUM_POINTS, gyro_fill_val),
            gyro_y: Deque::with_size_limit(NUM_POINTS, gyro_fill_val),
            gyro_z: Deque::with_size_limit(NUM_POINTS, gyro_fill_val),
            shared_state,
        }
    }

    /// Method for preparing some rms_acc data and initiating sending of data to frontend.
    fn imu_set_data(&mut self) {
        let acc_x = &mut self.acc_x.get();
        let acc_y = &mut self.acc_y.get();
        let acc_z = &mut self.acc_z.get();
        let acc_range = self.imu_conf & 0xF;
        let sig_figs = f64::powi(2_f64, acc_range as i32 + 1_i32) / f64::powi(2_f64, 15);
        let (rms_x, rms_y, rms_z) = {
            let mut squared_sum_x: f64 = 0_f64;
            let mut squared_sum_y: f64 = 0_f64;
            let mut squared_sum_z: f64 = 0_f64;
            for idx in 0..NUM_POINTS {
                squared_sum_x += f64::powi(acc_x[idx], 2);
                squared_sum_y += f64::powi(acc_y[idx], 2);
                squared_sum_z += f64::powi(acc_z[idx], 2);
            }
            (
                f64::sqrt(squared_sum_x / acc_x.len() as f64),
                f64::sqrt(squared_sum_y / acc_y.len() as f64),
                f64::sqrt(squared_sum_z / acc_z.len() as f64),
            )
        };
        self.rms_acc_x = sig_figs * rms_x;
        self.rms_acc_y = sig_figs * rms_y;
        self.rms_acc_z = sig_figs * rms_z;
        self.send_data();
    }

    /// Handler for Imu Aux messages.
    ///
    /// # Parameters
    /// - `msg`: MsgImuAux to extract data from.
    pub fn handle_imu_aux(&mut self, msg: MsgImuAux) {
        match msg.imu_type {
            0 => {
                self.imu_temp = 23_f64 + msg.temp as f64 / f64::powi(2_f64, 9);
                self.imu_conf = msg.imu_conf;
            }
            1 => {
                self.imu_temp = 25_f64 + msg.temp as f64 / 256_f64;
                self.imu_conf = msg.imu_conf;
            }
            _ => {
                error!("IMU type {} not known.", msg.imu_type);
            }
        }
    }

    /// Handler for Imu Raw messages.
    ///
    /// # Parameters
    /// - `msg`: MsgImuRaw to extract data from.
    pub fn handle_imu_raw(&mut self, msg: MsgImuRaw) {
        self.acc_x.add(msg.acc_x as f64);
        self.acc_y.add(msg.acc_y as f64);
        self.acc_z.add(msg.acc_z as f64);
        self.gyro_x.add(msg.gyr_x as f64);
        self.gyro_y.add(msg.gyr_y as f64);
        self.gyro_z.add(msg.gyr_z as f64);
        self.imu_set_data();
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab_status = msg.init_advanced_ins_status();

        let mut tab_points = tab_status.reborrow().init_data(NUM_INS_PLOT_ROWS as u32);

        let mut points_vec = vec![
            self.acc_x.get(),
            self.acc_y.get(),
            self.acc_z.get(),
            self.gyro_x.get(),
            self.gyro_y.get(),
            self.gyro_z.get(),
        ];
        for idx in 0..NUM_INS_PLOT_ROWS {
            let points = points_vec.get_mut(idx).expect(GET_MUT_OBJECT_FAILURE);
            let mut point_val_idx = tab_points.reborrow().init(idx as u32, points.len() as u32);
            for idx in 0..NUM_POINTS {
                let mut point_val = point_val_idx.reborrow().get(idx as u32);
                point_val.set_x(idx as f64);
                point_val.set_y(points[idx]);
            }
        }
        let fields_data = {
            vec![
                self.imu_temp,
                self.imu_conf as f64,
                self.rms_acc_x,
                self.rms_acc_y,
                self.rms_acc_z,
            ]
        };
        let mut fields_data_status = tab_status
            .reborrow()
            .init_fields_data(NUM_INS_FIELDS as u32);

        for (i, datur) in fields_data.iter().enumerate() {
            fields_data_status.set(i as u32, *datur);
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TestSender;
    use sbp::messages::imu::{MsgImuAux, MsgImuRaw};

    #[test]
    fn handle_imu_raw_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut ins_tab = AdvancedInsTab::new(shared_state, client_send);
        let tow = 1_u32;
        let tow_f = 1_u8;
        let acc_x = 2_i16;
        let acc_y = 3_i16;
        let acc_z = 4_i16;
        let gyr_x = 5_i16;
        let gyr_y = 6_i16;
        let gyr_z = 7_i16;
        let msg = MsgImuRaw {
            sender_id: Some(1337),
            tow,
            tow_f,
            acc_x,
            acc_y,
            acc_z,
            gyr_x,
            gyr_y,
            gyr_z,
        };
        let acc_xs = ins_tab.acc_x.get();
        let acc_ys = ins_tab.acc_y.get();
        let acc_zs = ins_tab.acc_z.get();
        let gyro_xs = ins_tab.gyro_x.get();
        let gyro_ys = ins_tab.gyro_y.get();
        let gyro_zs = ins_tab.gyro_z.get();
        for idx in 0..NUM_POINTS {
            assert!(f64::abs(acc_xs[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(acc_ys[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(acc_zs[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(gyro_xs[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(gyro_ys[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(gyro_zs[idx] - 0_f64) <= f64::EPSILON);
        }
        assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) <= f64::EPSILON);
        ins_tab.handle_imu_raw(msg);
        let acc_xs = ins_tab.acc_x.get();
        let acc_ys = ins_tab.acc_y.get();
        let acc_zs = ins_tab.acc_z.get();
        let gyro_xs = ins_tab.gyro_x.get();
        let gyro_ys = ins_tab.gyro_y.get();
        let gyro_zs = ins_tab.gyro_z.get();
        assert!(f64::abs(acc_xs[NUM_POINTS - 1] - acc_x as f64) <= f64::EPSILON);
        assert!(f64::abs(acc_ys[NUM_POINTS - 1] - acc_y as f64) <= f64::EPSILON);
        assert!(f64::abs(acc_zs[NUM_POINTS - 1] - acc_z as f64) <= f64::EPSILON);
        assert!(f64::abs(gyro_xs[NUM_POINTS - 1] - gyr_x as f64) <= f64::EPSILON);
        assert!(f64::abs(gyro_ys[NUM_POINTS - 1] - gyr_y as f64) <= f64::EPSILON);
        assert!(f64::abs(gyro_zs[NUM_POINTS - 1] - gyr_z as f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) > f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) > f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) > f64::EPSILON);
    }

    #[test]
    fn handle_imu_aux_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut ins_tab = AdvancedInsTab::new(shared_state.clone(), client_send.clone());
        let imu_type_a = 0_u8;
        let imu_type_b = 1_u8;
        let imu_type_unknown = 2_u8;
        let imu_conf = 1_u8;
        let temp = 200;
        let msg = MsgImuAux {
            sender_id: Some(1337),
            imu_type: imu_type_unknown,
            imu_conf,
            temp,
        };
        assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
        assert_eq!(ins_tab.imu_conf, 0_u8);
        ins_tab.handle_imu_aux(msg);
        assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
        assert_ne!(ins_tab.imu_conf, imu_conf);

        let mut ins_tab = AdvancedInsTab::new(shared_state.clone(), client_send.clone());
        let msg = MsgImuAux {
            sender_id: Some(1337),
            imu_type: imu_type_a,
            imu_conf,
            temp,
        };
        assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
        assert_eq!(ins_tab.imu_conf, 0_u8);
        ins_tab.handle_imu_aux(msg);
        assert!(f64::abs(ins_tab.imu_temp - 23.390625_f64) <= f64::EPSILON);
        assert_eq!(ins_tab.imu_conf, imu_conf);

        let mut ins_tab = AdvancedInsTab::new(shared_state, client_send);
        let msg = MsgImuAux {
            sender_id: Some(1337),
            imu_type: imu_type_b,
            imu_conf,
            temp,
        };
        assert!(f64::abs(ins_tab.imu_temp - 0_f64) <= f64::EPSILON);
        assert_eq!(ins_tab.imu_conf, 0_u8);
        ins_tab.handle_imu_aux(msg);
        assert!(f64::abs(ins_tab.imu_temp - 25.78125_f64) <= f64::EPSILON);
        assert_eq!(ins_tab.imu_conf, imu_conf);
    }

    #[test]
    fn handle_imu_send_data_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut ins_tab = AdvancedInsTab::new(shared_state, client_send);

        assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) <= f64::EPSILON);

        let imu_type = 0_u8;
        let imu_conf = 1_u8;
        let temp = 200;
        let msg = MsgImuAux {
            sender_id: Some(1337),
            imu_type,
            imu_conf,
            temp,
        };
        ins_tab.handle_imu_aux(msg);

        let tow = 1_u32;
        let tow_f = 1_u8;
        let acc_x = 2_i16;
        let acc_y = 3_i16;
        let acc_z = 4_i16;
        let gyr_x = 5_i16;
        let gyr_y = 6_i16;
        let gyr_z = 7_i16;
        let msg = MsgImuRaw {
            sender_id: Some(1337),
            tow,
            tow_f,
            acc_x,
            acc_y,
            acc_z,
            gyr_x,
            gyr_y,
            gyr_z,
        };
        ins_tab.handle_imu_raw(msg);
        let sig_figs = 0.0001220703125_f64;
        let acc_x = acc_x as f64;
        let acc_y = acc_y as f64;
        let acc_z = acc_z as f64;

        let rms_acc_x = f64::sqrt((acc_x * acc_x) / NUM_POINTS as f64);
        let rms_acc_y = f64::sqrt((acc_y * acc_y) / NUM_POINTS as f64);
        let rms_acc_z = f64::sqrt((acc_z * acc_z) / NUM_POINTS as f64);
        assert!(f64::abs(ins_tab.rms_acc_x - rms_acc_x * sig_figs) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_y - rms_acc_y * sig_figs) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_z - rms_acc_z * sig_figs) <= f64::EPSILON);

        let acc_x = 4_i16;
        let acc_y = 6_i16;
        let acc_z = 8_i16;
        let msg = MsgImuRaw {
            sender_id: Some(1337),
            tow,
            tow_f,
            acc_x,
            acc_y,
            acc_z,
            gyr_x,
            gyr_y,
            gyr_z,
        };
        ins_tab.handle_imu_raw(msg);
        let sig_figs = 0.0001220703125_f64;
        let acc_x = acc_x as f64;
        let acc_y = acc_y as f64;
        let acc_z = acc_z as f64;
        let rms_acc_x =
            f64::sqrt((acc_x * acc_x + (acc_x / 2_f64) * (acc_x / 2_f64)) / NUM_POINTS as f64);
        let rms_acc_y =
            f64::sqrt((acc_y * acc_y + (acc_y / 2_f64) * (acc_y / 2_f64)) / NUM_POINTS as f64);
        let rms_acc_z =
            f64::sqrt((acc_z * acc_z + (acc_z / 2_f64) * (acc_z / 2_f64)) / NUM_POINTS as f64);
        assert!(f64::abs(ins_tab.rms_acc_x - rms_acc_x * sig_figs) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_y - rms_acc_y * sig_figs) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_z - rms_acc_z * sig_figs) <= f64::EPSILON);
    }
}
