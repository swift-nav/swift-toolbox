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

use crate::constants::{DEGREES, METERS};
use std::str::FromStr;

pub mod solution_position_tab;
pub mod solution_velocity_tab;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatLonUnits {
    Degrees,
    Meters,
}

impl LatLonUnits {
    /// Retrieve the velocity unit as string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            LatLonUnits::Degrees => DEGREES,
            LatLonUnits::Meters => METERS,
        }
    }
    pub fn get_sig_figs(&self, mean_lat: f64) -> (f64, f64) {
        match self {
            LatLonUnits::Degrees => (1.0, 1.0),
            LatLonUnits::Meters => LatLonUnits::meters_per_degree(mean_lat),
        }
    }
    /// Convert latitude in degrees to latitude and longitude to meters multipliers.
    ///
    /// Taken from:
    /// <https://github.com/swift-nav/piksi_tools/blob/v3.1.0-release/piksi_tools/console/solution_view.py>
    fn meters_per_degree(lat: f64) -> (f64, f64) {
        let m1 = 111132.92; // latitude calculation term 1
        let m2 = -559.82; // latitude calculation term 2
        let m3 = 1.175; // latitude calculation term 3
        let m4 = -0.0023; // latitude calculation term 4
        let p1 = 111412.84; // longitude calculation term 1
        let p2 = -93.5; // longitude calculation term 2
        let p3 = 0.118; // longitude calculation term 3
        let latlen = m1
            + (m2 * f64::cos(2.0 * f64::to_radians(lat)))
            + (m3 * f64::cos(4.0 * f64::to_radians(lat)))
            + (m4 * f64::cos(6.0 * f64::to_radians(lat)));
        let longlen = (p1 * f64::cos(f64::to_radians(lat)))
            + (p2 * f64::cos(3.0 * f64::to_radians(lat)))
            + (p3 * f64::cos(5.0 * f64::to_radians(lat)));
        (latlen, longlen)
    }
}
impl FromStr for LatLonUnits {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            DEGREES => Ok(LatLonUnits::Degrees),
            METERS => Ok(LatLonUnits::Meters),
            _ => Err(format!("Invalid LatLonUnits: {s}")),
        }
    }
}
