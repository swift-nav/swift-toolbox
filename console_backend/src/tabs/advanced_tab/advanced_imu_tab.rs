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

use log::error;
use sbp::messages::imu::{MsgImuAux, MsgImuRaw};

use capnp::message::Builder;
use sbp::messages::imu::msg_imu_aux::ImuType;

use crate::client_sender::BoxedClientSender;
use crate::constants::*;
use crate::fusion_status_flags::FusionStatusFlags;
use crate::shared_state::{SharedState, TabName};
use crate::types::RingBuffer;
use crate::utils::{euclidean_distance, serialize_capnproto_builder};

#[derive(Debug)]
pub struct AdvancedImuTab {
    shared_state: SharedState,
    ///  Client Sender channel for communication from backend to frontend.
    client_sender: BoxedClientSender,
    pub fusion_engine_status_bar: FusionStatusFlags,
    /// Storage for the Imu configuration.
    imu_conf: u8,
    /// Storage for the raw Imu temperature converted to proper units.
    imu_temp: f64,
    /// The calculated root mean squared imu acceleration for x axis.
    rms_acc_x: f64,
    /// The calculated root mean squared imu acceleration for y axis.
    rms_acc_y: f64,
    /// The calculated root mean squared imu acceleration for z axis.
    rms_acc_z: f64,
    /// The stored historic Imu acceleration values along x axis.
    acc_x: RingBuffer<f64>,
    /// The stored historic Imu acceleration values along y axis.
    acc_y: RingBuffer<f64>,
    /// The stored historic Imu acceleration values along z axis.
    acc_z: RingBuffer<f64>,
    /// The stored historic Imu angular rate values along x axis.
    gyro_x: RingBuffer<f64>,
    /// The stored historic Imu angular rate values along y axis.
    gyro_y: RingBuffer<f64>,
    /// The stored historic Imu angular rate values along z axis.
    gyro_z: RingBuffer<f64>,
}

impl AdvancedImuTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> AdvancedImuTab {
        AdvancedImuTab {
            shared_state,
            fusion_engine_status_bar: FusionStatusFlags::new(client_sender.clone()),
            client_sender,
            imu_conf: 0_u8,
            imu_temp: 0_f64,
            rms_acc_x: 0_f64,
            rms_acc_y: 0_f64,
            rms_acc_z: 0_f64,
            acc_x: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            acc_y: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            acc_z: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            gyro_x: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            gyro_y: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            gyro_z: RingBuffer::with_fill_value(NUM_POINTS, 0.),
        }
    }

    /// Method for preparing some rms_acc data and initiating sending of data to frontend.
    fn imu_set_data(&mut self) {
        let acc_range = self.imu_conf & 0xF;
        let sig_figs = f64::powi(2_f64, acc_range as i32 + 1_i32) / f64::powi(2_f64, 15);
        let rms_x = euclidean_distance(self.acc_x.iter()) / f64::sqrt(self.acc_x.len() as f64);
        let rms_y = euclidean_distance(self.acc_y.iter()) / f64::sqrt(self.acc_y.len() as f64);
        let rms_z = euclidean_distance(self.acc_z.iter()) / f64::sqrt(self.acc_z.len() as f64);
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
        match msg.imu_type() {
            Ok(imu) => {
                match imu {
                    ImuType::BoschBmi160 => {
                        self.imu_temp = 23_f64 + msg.temp as f64 / 512_f64;
                    }
                    ImuType::StMicroelectronicsAsm330Llh => {
                        self.imu_temp = 25_f64 + msg.temp as f64 / 256_f64;
                    }
                    ImuType::MurataScha634D03 => {
                        self.imu_temp = 25_f64 + msg.temp as f64 / 30_f64;
                    }
                    ImuType::TdkIam20680Hp => {
                        self.imu_temp = 25_f64 + (msg.temp as f64 - 25_f64) / 326.8;
                    }
                };
                self.imu_conf = msg.imu_conf;
            }
            Err(e) => error!("IMU type {e} not known"),
        }
    }

    /// Handler for Imu Raw messages.
    ///
    /// # Parameters
    /// - `msg`: MsgImuRaw to extract data from.
    pub fn handle_imu_raw(&mut self, msg: MsgImuRaw) {
        self.acc_x.push(msg.acc_x as f64);
        self.acc_y.push(msg.acc_y as f64);
        self.acc_z.push(msg.acc_z as f64);
        self.gyro_x.push(msg.gyr_x as f64);
        self.gyro_y.push(msg.gyr_y as f64);
        self.gyro_z.push(msg.gyr_z as f64);
        self.imu_set_data();
    }

    /// Package data into a message buffer and send to frontend.
    pub fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Advanced {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab_status = msg.init_advanced_imu_status();

        let mut tab_points = tab_status.reborrow().init_data(NUM_INS_PLOT_ROWS as u32);

        let all_points = [
            self.acc_x.iter(),
            self.acc_y.iter(),
            self.acc_z.iter(),
            self.gyro_x.iter(),
            self.gyro_y.iter(),
            self.gyro_z.iter(),
        ];
        for (idx, points) in IntoIterator::into_iter(all_points).enumerate() {
            let mut point_val_idx = tab_points.reborrow().init(idx as u32, NUM_POINTS as u32);
            for (idx, point) in points.enumerate() {
                let mut point_val = point_val_idx.reborrow().get(idx as u32);
                point_val.set_x((NUM_POINTS - idx) as f64);
                point_val.set_y(*point);
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
    use crate::client_sender::TestSender;
    use sbp::messages::imu::{MsgImuAux, MsgImuRaw};

    #[test]
    fn handle_imu_raw_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut ins_tab = AdvancedImuTab::new(shared_state, client_send);
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
        for idx in 0..NUM_POINTS {
            assert!(f64::abs(ins_tab.acc_x[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(ins_tab.acc_y[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(ins_tab.acc_z[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(ins_tab.gyro_x[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(ins_tab.gyro_y[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(ins_tab.gyro_z[idx] - 0_f64) <= f64::EPSILON);
        }
        assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) <= f64::EPSILON);
        ins_tab.handle_imu_raw(msg);
        assert!(f64::abs(ins_tab.acc_x[NUM_POINTS - 1] - acc_x as f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.acc_y[NUM_POINTS - 1] - acc_y as f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.acc_z[NUM_POINTS - 1] - acc_z as f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.gyro_x[NUM_POINTS - 1] - gyr_x as f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.gyro_y[NUM_POINTS - 1] - gyr_y as f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.gyro_z[NUM_POINTS - 1] - gyr_z as f64) <= f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_x - 0_f64) > f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_y - 0_f64) > f64::EPSILON);
        assert!(f64::abs(ins_tab.rms_acc_z - 0_f64) > f64::EPSILON);
    }

    #[test]
    fn handle_imu_aux_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut ins_tab = AdvancedImuTab::new(shared_state.clone(), client_send.clone());
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

        let mut ins_tab = AdvancedImuTab::new(shared_state.clone(), client_send.clone());
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

        let mut ins_tab = AdvancedImuTab::new(shared_state, client_send);
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
        let client_send = TestSender::boxed();
        let mut ins_tab = AdvancedImuTab::new(shared_state, client_send);

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
