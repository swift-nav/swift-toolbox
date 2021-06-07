use log::error;
use sbp::messages::imu::{MsgImuAux, MsgImuRaw};

use capnp::message::Builder;
use capnp::serialize;

use crate::console_backend_capnp as m;
use crate::constants::*;
use crate::errors::{CAP_N_PROTO_SERIALIZATION_FAILURE, GET_MUT_OBJECT_FAILURE};
use crate::fusion_engine_status::FusionEngineStatus;
use crate::types::{Deque, MessageSender, SharedState};

/// AdvancedInsTab struct.
///
/// # Fields:
///
/// - `available_units` - The available units of measure to send to frontend for selection.
/// - `colors`: Stored rgb codes for frontend correspond to index of sv_labels.
/// - `max`: Stored maximum measure of unit used for frontend plot.
/// - `min`: Stored minimum measure of unit used for frontend plot.
/// - `multiplier`: The current multiplier used to modify points accounting for unit of measure.
/// - `points`: The horizontal and vertical velocity points of size, NUM_POINTS, to be sent to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
/// - `tow`: The GPS Time of Week.
/// - `unit`: Currently displayed and converted to unit of measure.
#[derive(Debug)]
pub struct AdvancedInsTab<S: MessageSender> {
    pub client_sender: S,
    pub fusion_engine_status: FusionEngineStatus<S>,
    pub imu_conf: u8,
    pub imu_temp: f64,
    pub rms_acc_x: f64,
    pub rms_acc_y: f64,
    pub rms_acc_z: f64,
    pub acc_x: Deque<f64>,
    pub acc_y: Deque<f64>,
    pub acc_z: Deque<f64>,
    pub gyro_x: Deque<f64>,
    pub gyro_y: Deque<f64>,
    pub gyro_z: Deque<f64>,
    pub shared_state: SharedState,
}

impl<S: MessageSender> AdvancedInsTab<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> AdvancedInsTab<S> {
        let acc_fill_val = Some(0_f64);
        let gyro_fill_val = Some(0_f64);
        AdvancedInsTab {
            fusion_engine_status: FusionEngineStatus::new(
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

    pub fn imu_set_data(&mut self) {
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
    pub fn handle_imu_raw(&mut self, msg: MsgImuRaw) {
        self.acc_x.add(msg.acc_x as f64);
        self.acc_y.add(msg.acc_y as f64);
        self.acc_z.add(msg.acc_z as f64);
        self.gyro_x.add(msg.gyr_x as f64);
        self.gyro_y.add(msg.gyr_y as f64);
        self.gyro_z.add(msg.gyr_z as f64);
        self.imu_set_data();
    }

    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<m::message::Builder>();

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
        let text_data = {
            vec![
                format!("{:.2} C", self.imu_temp),
                format!("{:#04x}", self.imu_conf),
                format!("{:.2} g", self.rms_acc_x),
                format!("{:.2} g", self.rms_acc_y),
                format!("{:.2} g", self.rms_acc_z),
            ]
        };
        let mut text_data_status = tab_status
            .reborrow()
            .init_text_data(NUM_INS_TEXT_FIELDS as u32);

        for (i, datur) in text_data.iter().enumerate() {
            text_data_status.set(i as u32, datur);
        }

        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder)
            .expect(CAP_N_PROTO_SERIALIZATION_FAILURE);
        self.client_sender.send_data(msg_bytes);
    }
}
