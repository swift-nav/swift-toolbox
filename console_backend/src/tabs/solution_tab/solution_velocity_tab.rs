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

use std::str::FromStr;

use capnp::message::Builder;
use sbp::messages::navigation::MsgVelNed;

use crate::client_sender::BoxedClientSender;
use crate::constants::{HORIZONTAL_COLOR, NUM_POINTS, VERTICAL_COLOR};
use crate::shared_state::{SharedState, TabName};
use crate::types::{RingBuffer, VelocityUnits};
use crate::utils::{euclidean_distance, serialize_capnproto_builder};
use crate::zip;

#[derive(Debug)]
pub struct SolutionVelocityTab {
    /// The available units of measure to send to frontend for selection.
    pub available_units: Vec<&'static str>,
    pub client_sender: BoxedClientSender,
    /// Stored rgb codes for frontend correspond to index of sv_labels.
    pub colors: Vec<String>,
    /// Stored maximum measure of unit used for frontend plot.
    pub max: f64,
    /// Stored minimum measure of unit used for frontend plot.
    pub min: f64,
    /// The current multiplier used to modify points accounting for unit of measure.
    pub multiplier: f64,
    /// The horizontal and vertical velocity points of size, NUM_POINTS, to be sent to frontend.
    pub points: Vec<RingBuffer<(f64, f64)>>,
    /// The shared state for communicating between frontend/backend/other backend tabs.
    pub shared_state: SharedState,
    /// The GPS Time of Week.
    pub tow: f64,
    /// Currently displayed and converted to unit of measure.
    pub unit: VelocityUnits,
}

impl SolutionVelocityTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> SolutionVelocityTab {
        SolutionVelocityTab {
            available_units: vec![
                VelocityUnits::Mps.as_str(),
                VelocityUnits::Mph.as_str(),
                VelocityUnits::Kph.as_str(),
            ],
            client_sender,
            colors: vec![String::from(HORIZONTAL_COLOR), String::from(VERTICAL_COLOR)],
            max: 0_f64,
            min: 0_f64,
            multiplier: VelocityUnits::Mps.get_multiplier(),
            points: vec![RingBuffer::new(NUM_POINTS), RingBuffer::new(NUM_POINTS)],
            shared_state,
            tow: 0_f64,
            unit: VelocityUnits::Mps,
        }
    }

    fn convert_points(&mut self, new_unit: VelocityUnits) {
        let new_mult = new_unit.get_multiplier();
        let mut hpoints: RingBuffer<(f64, f64)> = RingBuffer::new(NUM_POINTS);
        let mut vpoints: RingBuffer<(f64, f64)> = RingBuffer::new(NUM_POINTS);
        let mult_conversion = new_mult / self.multiplier;
        for idx in 0..self.points[0].len() {
            let mut hpoint = self.points[0][idx];
            hpoint.1 *= mult_conversion;
            hpoints.push(hpoint);
            let mut vpoint = self.points[1][idx];
            vpoint.1 *= mult_conversion;
            vpoints.push(vpoint);
        }
        self.multiplier = new_mult;
        self.points = vec![hpoints, vpoints];
        self.unit = new_unit;
    }

    /// Handle MsgVelNed message states.
    ///
    /// # Parameters:
    ///
    /// - `msg`: The message to update set of points with.
    pub fn handle_vel_ned(&mut self, msg: MsgVelNed) {
        let n = msg.n as f64;
        let e = msg.e as f64;
        let d = msg.d as f64;

        let h_vel = euclidean_distance([n, e].iter()) / 1000.0;
        let v_vel = (-1.0 * d) / 1000.0;

        self.tow = msg.tow as f64 / 1000.0;

        self.points[0].push((self.tow, h_vel * self.multiplier));
        self.points[1].push((self.tow, v_vel * self.multiplier));

        let mut new_unit = self.unit.clone();
        {
            if let Ok(unit) = VelocityUnits::from_str(
                self.shared_state
                    .lock()
                    .solution_tab
                    .velocity_tab
                    .unit
                    .as_str(),
            ) {
                new_unit = unit;
            }
        }
        if new_unit != self.unit {
            self.convert_points(new_unit);
        }
        let hpoints = &self.points[0];
        let vpoints = &self.points[1];
        let mut min = hpoints[0].1;
        let mut max = hpoints[0].1;
        for (h, v) in zip!(hpoints, vpoints) {
            min = f64::min(h.1, f64::min(v.1, min));
            max = f64::max(h.1, f64::max(v.1, max));
        }
        self.min = min;
        self.max = max;
        self.send_data();
    }

    /// Package data into a message buffer and send to frontend.
    pub fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Solution {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut velocity_status = msg.init_solution_velocity_status();
        velocity_status.set_min(self.min);
        velocity_status.set_max(self.max);

        let mut velocity_points = velocity_status
            .reborrow()
            .init_data(self.points.len() as u32);
        for idx in 0..self.points.len() {
            let points = &mut self.points[idx];
            let mut point_val_idx = velocity_points
                .reborrow()
                .init(idx as u32, points.len() as u32);
            for (i, (x, y)) in points.iter().enumerate() {
                let mut point_val = point_val_idx.reborrow().get(i as u32);
                point_val.set_x(*x);
                point_val.set_y(*y);
            }
        }
        let mut available_units = velocity_status
            .reborrow()
            .init_available_units(self.available_units.len() as u32);

        for (i, unit) in self.available_units.iter().enumerate() {
            available_units.set(i as u32, unit);
        }
        let mut colors = velocity_status
            .reborrow()
            .init_colors(self.colors.len() as u32);

        for (i, color) in self.colors.iter().enumerate() {
            colors.set(i as u32, color);
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_sender::TestSender;
    use crate::constants::{MPS2KPH, MPS2MPH};

    #[test]
    fn handle_vel_ned_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut solution_velocity_tab = SolutionVelocityTab::new(shared_state, client_send);

        let msg: MsgVelNed = MsgVelNed {
            sender_id: Some(5),
            n: 6,
            e: 66,
            d: 666,
            tow: 1001_u32,
            h_accuracy: 0,
            v_accuracy: 0,
            flags: 1,
            n_sats: 1,
        };

        solution_velocity_tab.handle_vel_ned(msg);
        assert_eq!(solution_velocity_tab.points.len(), 2);
        let hpoints = &solution_velocity_tab.points[0];
        let vpoints = &solution_velocity_tab.points[1];
        assert_eq!(hpoints.len(), 1);
        assert_eq!(vpoints.len(), 1);
        assert!((hpoints[0].1 - 0.06627216610312357) <= f64::EPSILON);
        assert!((vpoints[0].1 - (-0.666)) <= f64::EPSILON);
        let msg = MsgVelNed {
            sender_id: Some(5),
            n: 1,
            e: 133,
            d: 1337,
            tow: 1002_u32,
            h_accuracy: 0,
            v_accuracy: 0,
            flags: 1,
            n_sats: 1,
        };
        solution_velocity_tab.handle_vel_ned(msg);
        let hpoints = &solution_velocity_tab.points[0];
        let vpoints = &solution_velocity_tab.points[1];
        assert_eq!(hpoints.len(), 2);
        assert_eq!(vpoints.len(), 2);
        assert!(f64::abs(hpoints[1].1 - 0.13300375934536587) <= f64::EPSILON);
        assert!(f64::abs(vpoints[1].1 - (-1.337)) <= f64::EPSILON);
        let msg = MsgVelNed {
            sender_id: Some(5),
            n: 7,
            e: 67,
            d: 667,
            tow: 1003_u32,
            h_accuracy: 0,
            v_accuracy: 0,
            flags: 1,
            n_sats: 1,
        };
        solution_velocity_tab.handle_vel_ned(msg);
        let hpoints = &solution_velocity_tab.points[0];
        let vpoints = &solution_velocity_tab.points[1];
        assert_eq!(hpoints.len(), 3);
        assert_eq!(vpoints.len(), 3);
        assert!(f64::abs(hpoints[1].1 - solution_velocity_tab.max) <= f64::EPSILON);
        assert!(f64::abs(vpoints[1].1 - solution_velocity_tab.min) <= f64::EPSILON);
    }

    #[test]
    fn test_convert_points() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut solution_velocity_tab = SolutionVelocityTab::new(shared_state, client_send);

        let mut msg: MsgVelNed = MsgVelNed {
            sender_id: Some(5),
            n: 6,
            e: 66,
            d: 666,
            tow: 1001_u32,
            h_accuracy: 0,
            v_accuracy: 0,
            flags: 1,
            n_sats: 1,
        };

        solution_velocity_tab.handle_vel_ned(msg);
        msg = MsgVelNed {
            sender_id: Some(5),
            n: 1,
            e: 133,
            d: 1337,
            tow: 1002_u32,
            h_accuracy: 0,
            v_accuracy: 0,
            flags: 1,
            n_sats: 1,
        };
        solution_velocity_tab.handle_vel_ned(msg);
        let hpoints = solution_velocity_tab.points[0].clone();
        let vpoints = solution_velocity_tab.points[1].clone();

        let new_unit = VelocityUnits::Mps;
        solution_velocity_tab.convert_points(new_unit);
        let new_hpoints = &solution_velocity_tab.points[0];
        let new_vpoints = &solution_velocity_tab.points[1];
        for idx in 0..hpoints.len() {
            assert!(f64::abs(hpoints[idx].1 - new_hpoints[idx].1) <= f64::EPSILON);
            assert!(f64::abs(vpoints[idx].1 - new_vpoints[idx].1) <= f64::EPSILON);
        }

        let hpoints = solution_velocity_tab.points[0].clone();
        let vpoints = solution_velocity_tab.points[1].clone();

        let new_unit = VelocityUnits::Mph;
        solution_velocity_tab.convert_points(new_unit);
        let new_hpoints = &solution_velocity_tab.points[0];
        let new_vpoints = &solution_velocity_tab.points[1];
        for idx in 0..hpoints.len() {
            assert!(f64::abs((hpoints[idx].1 * MPS2MPH) - new_hpoints[idx].1) <= f64::EPSILON);
            assert!(f64::abs((vpoints[idx].1 * MPS2MPH) - new_vpoints[idx].1) <= f64::EPSILON);
        }

        let hpoints = solution_velocity_tab.points[0].clone();
        let vpoints = solution_velocity_tab.points[1].clone();
        let new_unit = VelocityUnits::Kph;
        solution_velocity_tab.convert_points(new_unit);
        let new_hpoints = &solution_velocity_tab.points[0];
        let new_vpoints = &solution_velocity_tab.points[1];

        for idx in 0..hpoints.len() {
            assert!(
                f64::abs(hpoints[idx].1 * (MPS2KPH / MPS2MPH) - new_hpoints[idx].1) <= f64::EPSILON
            );
            assert!(
                f64::abs(vpoints[idx].1 * (MPS2KPH / MPS2MPH) - new_vpoints[idx].1) <= f64::EPSILON
            );
        }
    }
}
