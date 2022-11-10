use sbp::messages::mag::MsgMagRaw;

use capnp::message::Builder;

use crate::client_sender::BoxedClientSender;
use crate::constants::{MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER, NUM_POINTS};
use crate::shared_state::{SharedState, TabName};
use crate::types::RingBuffer;
use crate::utils::serialize_capnproto_builder;
use crate::zip;

/// AdvancedMagnetometerTab struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `last_plot_update_time`: The last time the plot values were attempted to send to frontend.
/// - `mag_x`: The stored historic Magnetometer values along x axis.
/// - `mag_y`: The stored historic Magnetometer values along y axis.
/// - `mag_z`: The stored historic Magnetometer values along z axis.
#[derive(Debug)]
pub struct AdvancedMagnetometerTab {
    shared_state: SharedState,
    client_sender: BoxedClientSender,
    mag_x: RingBuffer<f64>,
    mag_y: RingBuffer<f64>,
    mag_z: RingBuffer<f64>,
    ymax: f64,
    ymin: f64,
}

impl AdvancedMagnetometerTab {
    pub fn new(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
    ) -> AdvancedMagnetometerTab {
        AdvancedMagnetometerTab {
            shared_state,
            client_sender,
            mag_x: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            mag_y: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            mag_z: RingBuffer::with_fill_value(NUM_POINTS, 0.),
            ymax: f64::MIN,
            ymin: f64::MAX,
        }
    }

    /// Method for preparing magnetometer data and initiating sending to frontend.
    fn mag_set_data(&mut self) {
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for (x, y, z) in zip!(&self.mag_x, &self.mag_y, &self.mag_z) {
            min = min.min(*x).min(*y).min(*z);
            max = max.max(*x).max(*y).max(*z);
        }
        self.ymin = min - f64::abs(min * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER);
        self.ymax = max + f64::abs(max * MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER);
        self.send_data();
    }

    /// Handler for Mag Raw messages.
    ///
    /// # Parameters
    /// - `msg`: MsgMagRaw to extract data from.
    pub fn handle_mag_raw(&mut self, msg: MsgMagRaw) {
        self.mag_x.push(msg.mag_x as f64);
        self.mag_y.push(msg.mag_y as f64);
        self.mag_z.push(msg.mag_z as f64);
        self.mag_set_data();
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Advanced {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut tab_status = msg.init_advanced_magnetometer_status();

        let all_points = [self.mag_x.iter(), self.mag_y.iter(), self.mag_z.iter()];
        let mut tab_points = tab_status.reborrow().init_data(all_points.len() as u32);

        for (idx, points) in IntoIterator::into_iter(all_points).enumerate() {
            let mut point_val_idx = tab_points.reborrow().init(idx as u32, NUM_POINTS as u32);
            for (idx, point) in points.enumerate() {
                let mut point_val = point_val_idx.reborrow().get(idx as u32);
                point_val.set_x((NUM_POINTS - idx) as f64);
                point_val.set_y(*point);
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
    use crate::client_sender::TestSender;

    #[test]
    fn hangle_mag_raw_test() {
        let client_send = TestSender::boxed();
        let mut mag_tab = AdvancedMagnetometerTab::new(client_send);
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
        for idx in 0..NUM_POINTS {
            assert!(f64::abs(mag_tab.mag_x[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(mag_tab.mag_y[idx] - 0_f64) <= f64::EPSILON);
            assert!(f64::abs(mag_tab.mag_z[idx] - 0_f64) <= f64::EPSILON);
        }
        mag_tab.handle_mag_raw(msg);
        assert!(f64::abs(mag_tab.mag_x[NUM_POINTS - 1] - mag_x as f64) <= f64::EPSILON);
        assert!(f64::abs(mag_tab.mag_y[NUM_POINTS - 1] - mag_y as f64) <= f64::EPSILON);
        assert!(f64::abs(mag_tab.mag_z[NUM_POINTS - 1] - mag_z as f64) <= f64::EPSILON);
    }

    #[test]
    fn handle_imu_send_data_test() {
        let client_send = TestSender::boxed();
        let mut mag_tab = AdvancedMagnetometerTab::new(client_send);
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
