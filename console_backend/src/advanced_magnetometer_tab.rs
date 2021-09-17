use sbp::messages::mag::MsgMagRaw;

use capnp::message::Builder;

use crate::constants::{MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER, NUM_POINTS};
use crate::errors::GET_MUT_OBJECT_FAILURE;
use crate::shared_state::SharedState;
use crate::types::{CapnProtoSender, Deque};
use crate::utils::serialize_capnproto_builder;

/// AdvancedMagnetometerTab struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `last_plot_update_time`: The last time the plot values were attempted to send to frontend.
/// - `mag_x`: The stored historic Magnetometer values along x axis.
/// - `mag_y`: The stored historic Magnetometer values along y axis.
/// - `mag_z`: The stored historic Magnetometer values along z axis.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
#[derive(Debug)]
pub struct AdvancedMagnetometerTab<S: CapnProtoSender> {
    client_sender: S,
    mag_x: Deque<f64>,
    mag_y: Deque<f64>,
    mag_z: Deque<f64>,
    shared_state: SharedState,
    ymax: f64,
    ymin: f64,
}

impl<S: CapnProtoSender> AdvancedMagnetometerTab<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> AdvancedMagnetometerTab<S> {
        let mag_fill_val = Some(0_f64);
        AdvancedMagnetometerTab {
            client_sender,
            mag_x: Deque::with_size_limit(NUM_POINTS, mag_fill_val),
            mag_y: Deque::with_size_limit(NUM_POINTS, mag_fill_val),
            mag_z: Deque::with_size_limit(NUM_POINTS, mag_fill_val),
            shared_state,
            ymax: f64::MIN,
            ymin: f64::MAX,
        }
    }

    /// Method for preparing magnetometer data and initiating sending to frontend.
    fn mag_set_data(&mut self) {
        let mag_x = &mut self.mag_x.get();
        let mag_y = &mut self.mag_y.get();
        let mag_z = &mut self.mag_z.get();

        let mut min_ = f64::MAX;
        let mut max_ = f64::MIN;
        for idx in 0..NUM_POINTS {
            min_ = f64::min(mag_x[idx], f64::min(mag_y[idx], f64::min(mag_z[idx], min_)));
            max_ = f64::max(mag_x[idx], f64::max(mag_y[idx], f64::max(mag_z[idx], max_)));
        }
        self.ymin = min_ - f64::abs(min_ * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER);

        self.ymax = max_ + f64::abs(max_ * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER);

        self.send_data();
    }

    /// Handler for Mag Raw messages.
    ///
    /// # Parameters
    /// - `msg`: MsgMagRaw to extract data from.
    pub fn handle_mag_raw(&mut self, msg: MsgMagRaw) {
        self.mag_x.add(msg.mag_x as f64);
        self.mag_y.add(msg.mag_y as f64);
        self.mag_z.add(msg.mag_z as f64);
        self.mag_set_data();
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab_status = msg.init_advanced_magnetometer_status();

        let mut points_vec = vec![self.mag_x.get(), self.mag_y.get(), self.mag_z.get()];

        let mut tab_points = tab_status.reborrow().init_data(points_vec.len() as u32);

        for idx in 0..points_vec.len() {
            let points = points_vec.get_mut(idx).expect(GET_MUT_OBJECT_FAILURE);
            let mut point_val_idx = tab_points.reborrow().init(idx as u32, points.len() as u32);
            for idx in 0..NUM_POINTS {
                let mut point_val = point_val_idx.reborrow().get(idx as u32);
                point_val.set_x(idx as f64);
                point_val.set_y(points[idx]);
            }
        }

        tab_status.set_ymin(self.ymin);
        tab_status.set_ymax(self.ymax);

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::TestSender;

    #[test]
    fn hangle_mag_raw_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut mag_tab = AdvancedMagnetometerTab::new(shared_state, client_send);
        let tow = 1_u32;
        let tow_f = 1_u8;
        let mag_x = 2_i16;
        let mag_y = 3_i16;
        let mag_z = 4_i16;
        let msg = MsgMagRaw {
            sender_id: Some(1337),
            tow,
            tow_f,
            mag_x,
            mag_y,
            mag_z,
        };
        let mag_xs = mag_tab.mag_x.get();
        let mag_ys = mag_tab.mag_y.get();
        let mag_zs = mag_tab.mag_z.get();
        for idx in 0..NUM_POINTS {
            assert!(f64::abs(mag_xs[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(mag_ys[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(mag_zs[idx] - 0_f64) <= f64::EPSILON);
        }
        mag_tab.handle_mag_raw(msg);
        let mag_xs = mag_tab.mag_x.get();
        let mag_ys = mag_tab.mag_y.get();
        let mag_zs = mag_tab.mag_z.get();
        assert!(f64::abs(mag_xs[NUM_POINTS - 1] - mag_x as f64) <= f64::EPSILON);
        assert!(f64::abs(mag_ys[NUM_POINTS - 1] - mag_y as f64) <= f64::EPSILON);
        assert!(f64::abs(mag_zs[NUM_POINTS - 1] - mag_z as f64) <= f64::EPSILON);
    }

    #[test]
    fn handle_imu_send_data_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender { inner: Vec::new() };
        let mut mag_tab = AdvancedMagnetometerTab::new(shared_state, client_send);
        assert!(f64::abs(mag_tab.ymin - f64::MAX) <= f64::EPSILON);
        assert!(f64::abs(mag_tab.ymax - f64::MIN) <= f64::EPSILON);

        let tow = 1_u32;
        let tow_f = 1_u8;
        let mag_x = -2_i16;
        let mag_y = 3_i16;
        let mag_z = 4_i16;
        let msg = MsgMagRaw {
            sender_id: Some(1337),
            tow,
            tow_f,
            mag_x,
            mag_y,
            mag_z,
        };
        mag_tab.handle_mag_raw(msg);
        let mag_x = mag_x as f64;
        let mag_z = mag_z as f64;
        assert!(
            f64::abs(
                mag_tab.ymin - (mag_x - f64::abs(mag_x * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER))
            ) <= f64::EPSILON
        );
        assert!(
            f64::abs(
                mag_tab.ymax - (mag_z + f64::abs(mag_z * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER))
            ) <= f64::EPSILON
        );

        let mag_x_ = 8_i16;
        let mag_y = 6_i16;
        let mag_z_ = 4_i16;
        let msg = MsgMagRaw {
            sender_id: Some(1337),
            tow,
            tow_f,
            mag_x: mag_x_,
            mag_y,
            mag_z: mag_z_,
        };
        mag_tab.handle_mag_raw(msg);
        let mag_x_ = mag_x_ as f64;

        assert!(
            f64::abs(
                mag_tab.ymin - (mag_x - f64::abs(mag_x * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER))
            ) <= f64::EPSILON
        );
        assert!(
            f64::abs(
                mag_tab.ymax - (mag_x_ + f64::abs(mag_x_ * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER))
            ) <= f64::EPSILON
        );
    }
}
