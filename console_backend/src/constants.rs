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

use lazy_static::lazy_static;
use std::time::Duration;

use crate::updater::swift_version::SwiftVersion;

// 'Universal' constants

pub(crate) const NANOSECONDS_PER_SECOND: f64 = 1.0e+9;
pub(crate) const SECONDS_PER_NANOSECOND: f64 = 1.0e-9;

pub(crate) const APPLICATION_QUALIFIER: &str = "com.swift-nav";
pub(crate) const APPLICATION_ORGANIZATION: &str = "swift-nav";
pub(crate) const APPLICATION_NAME: &str = "swift_navigation_console";
// CLI constants.

// Logging constants
#[allow(dead_code)]
pub(crate) const LOG_WRITER_BUFFER_MESSAGE_COUNT: usize = 50;
pub(crate) const LOG_FILENAME: &str = "swift-console-%Y%m%d-%H%M%S.log";

// Main Tab constants.
pub(crate) const WRITE_TO_DEVICE_SENDER_ID: u16 = 1337;
pub(crate) const VEL_TIME_STR_FILEPATH: &str = "velocity_log_%Y%m%d-%H%M%S.csv";
pub(crate) const POS_LLH_TIME_STR_FILEPATH: &str = "position_log_%Y%m%d-%H%M%S.csv";
pub(crate) const BASELINE_TIME_STR_FILEPATH: &str = "baseline_log_%Y%m%d-%H%M%S.csv";
pub(crate) const SBP_FILEPATH: &str = "swift-gnss-%Y%m%d-%H%M%S.sbp";
pub(crate) const SBP_JSON_FILEPATH: &str = "swift-gnss-%Y%m%d-%H%M%S.sbp.json";
pub(crate) const DEFAULT_LOG_DIRECTORY: &str = "SwiftNav";
pub(crate) const DEFAULT_IP_ADDRESS: &str = "192.168.0.222";
pub(crate) const DEFAULT_PORT: u16 = 55555;

// Common constants.
pub(crate) const NUM_POINTS: usize = 200;

// Advanced Ins Tab constants.
pub(crate) const NUM_INS_PLOT_ROWS: usize = 6;
pub(crate) const NUM_INS_FIELDS: usize = 5;

// Navbar constants.
pub(crate) const AVAILABLE_REFRESH_RATES: [u8; 5] = [1, 2, 5, 10, 25];
pub(crate) const AVAILABLE_BAUDRATES: [u32; 5] = [57600, 115200, 230400, 460800, 921600];
pub(crate) const FLOW_CONTROL_NONE: &str = "None";
pub(crate) const FLOW_CONTROL_SOFTWARE: &str = "Software XON/XOFF";
pub(crate) const FLOW_CONTROL_HARDWARE: &str = "Hardware RTS/CTS";
pub(crate) const AVAILABLE_FLOWS: &[&str] = &[
    FLOW_CONTROL_NONE,
    FLOW_CONTROL_SOFTWARE,
    FLOW_CONTROL_HARDWARE,
];
pub(crate) const READER_TIMEOUT: Duration = Duration::from_secs(2);
pub(crate) const CONNECTION_HISTORY_FILENAME: &str = "connection_history.yaml";
pub(crate) const MAX_CONNECTION_HISTORY: i32 = 15;

// Tracking Signals Tab constants.
pub(crate) const NUM_COLORS: u8 = 37;
pub(crate) const TRK_RATE: f64 = 2.0;
pub(crate) const GLO_SLOT_SAT_MAX: u8 = 90;
pub(crate) const GLO_FCN_OFFSET: i16 = 8;
pub(crate) const SBAS_NEG_OFFSET: i16 = 120;
pub(crate) const QZSS_NEG_OFFSET: i16 = 193;
pub(crate) const TRACKING_UPDATE_PERIOD: f64 = 0.2;
pub(crate) const CHART_XMIN_OFFSET_NO_TRACKING: f64 = -45.0;
pub(crate) const CHART_XMIN_OFFSET_TRACKING: f64 = -95.0;

// Advanced Magnetometer Tab.
pub(crate) const MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER: f64 = 0.25;

// Advanced Spectrum Analyzer Tab.
pub const AMPLITUDES: &str = "amplitudes";
pub const FREQUENCIES: &str = "frequencies";
pub const CHANNELS: &[u16] = &[1, 2, 3, 4];
pub const SIGNALS_TOTAL: usize = 512;

// Baseline Tab.
pub(crate) const BASELINE_DIRECTION_MAX: f64 = f64::MAX;
pub(crate) const BASELINE_DIRECTION_MIN: f64 = f64::MIN;

pub(crate) const N_FLOAT: &str = "n_Float RTK";
pub(crate) const N_FIXED: &str = "n_Fixed RTK";
pub(crate) const N_DGNSS: &str = "n_DGPS";
pub(crate) const E_FLOAT: &str = "e_Float RTK";
pub(crate) const E_FIXED: &str = "e_Fixed RTK";
pub(crate) const E_DGNSS: &str = "e_DGPS";
pub(crate) const NORTH: &str = "N [m]";
pub(crate) const EAST: &str = "E [m]";
pub(crate) const DOWN: &str = "D [m]";
pub(crate) const DIST: &str = "Dist [m]";
pub(crate) const FLAGS: &str = "Flags";
pub(crate) const MODE: &str = "Mode";
pub(crate) const HEADING: &str = "Heading [°]";

pub const BASELINE_DATA_KEYS: &[&str] = &[N_FLOAT, N_FIXED, N_DGNSS, E_FLOAT, E_FIXED, E_DGNSS];

pub const BASELINE_TABLE_KEYS: &[&str] = &[
    GPS_WEEK, GPS_TOW, GPS_TIME, UTC_TIME, UTC_SRC, NORTH, EAST, DOWN, HORIZ_ACC, VERT_ACC, DIST,
    SATS_USED, FLAGS, MODE, HEADING, CORR_AGE_S,
];

// Solution Table.
pub(crate) const PLOT_HISTORY_MAX: usize = 1000;
pub(crate) const DILUTION_OF_PRECISION_UNITS: f64 = 0.01;
pub(crate) const NUM_GNSS_MODES: usize = 6;
pub(crate) const DEGREES: &str = "degrees";
pub(crate) const METERS: &str = "meters";
pub(crate) const NO_FIX_LABEL: &str = "No Fix";
pub(crate) const SPP_LABEL: &str = "SPP";
pub(crate) const DGNSS_LABEL: &str = "DGPS";
pub(crate) const FLOAT_LABEL: &str = "RTK Float";
pub(crate) const FIXED_LABEL: &str = "RTK Fixed";
pub(crate) const DR_LABEL: &str = "DR";
pub(crate) const SBAS_LABEL: &str = "SBAS";

pub(crate) const NO_FIX: &str = "No Fix";
pub(crate) const SPP: &str = "SPP";
pub(crate) const DGNSS: &str = "DGPS";
pub(crate) const RTK: &str = "RTK";
pub(crate) const FLOAT: &str = "Float RTK";
pub(crate) const FIXED: &str = "Fixed RTK";
pub(crate) const DR: &str = "Dead Reckoning";
pub(crate) const SBAS: &str = "SBAS";
pub(crate) const NO_FIX_COLOR: &str = "#FFFFFF";
pub(crate) const SPP_COLOR: &str = "#FF0000";
pub(crate) const DGNSS_COLOR: &str = "#00FFFF";
pub(crate) const FLOAT_COLOR: &str = "#0000FF";
pub(crate) const FIXED_COLOR: &str = "#00FF00";
pub(crate) const DR_COLOR: &str = "#000000";
pub(crate) const SBAS_COLOR: &str = "#FF00FF";

pub(crate) const LAT_SPP: &str = "lat_SPP";
pub(crate) const LON_SPP: &str = "lon_SPP";
pub(crate) const LAT_DGNSS: &str = "lat_DGPS";
pub(crate) const LON_DGNSS: &str = "lon_DGPS";
pub(crate) const LAT_FLOAT: &str = "lat_Float RTK";
pub(crate) const LON_FLOAT: &str = "lon_Float RTK";
pub(crate) const LAT_FIXED: &str = "lat_Fixed RTK";
pub(crate) const LON_FIXED: &str = "lon_Fixed RTK";
pub(crate) const LAT_DR: &str = "lat_Dead Reckoning";
pub(crate) const LON_DR: &str = "lon_Dead Reckoning";
pub(crate) const LAT_SBAS: &str = "lat_SBAS";
pub(crate) const LON_SBAS: &str = "lon_SBAS";

pub(crate) const SOLUTION_DATA_KEYS: &[&str] = &[
    LAT_SPP, LON_SPP, LAT_DGNSS, LON_DGNSS, LAT_FLOAT, LON_FLOAT, LAT_FIXED, LON_FIXED, LAT_DR,
    LON_DR, LAT_SBAS, LON_SBAS,
];

pub(crate) const FACTORY_DEFAULT: &str = "Factory Default";
pub(crate) const NON_VOLATILE_MEMORY: &str = "Non Volatile Memory";
pub(crate) const DECODED_THIS_SESSION: &str = "Decoded this Session";
pub(crate) const UNKNOWN: &str = "Unknown";

pub(crate) const GPS_WEEK: &str = "GPS Week";
pub(crate) const GPS_TOW: &str = "GPS TOW";
pub(crate) const GPS_TIME: &str = "GPS Time";
pub(crate) const UTC_TIME: &str = "UTC Time";
pub(crate) const UTC_SRC: &str = "UTC Src";
pub(crate) const SATS_USED: &str = "Sats Used";
pub(crate) const LAT: &str = "Lat [°]";
pub(crate) const LON: &str = "Lon [°]";
pub(crate) const HEIGHT: &str = "Height [m]";
pub(crate) const HORIZ_ACC: &str = "Horiz Acc [m]";
pub(crate) const VERT_ACC: &str = "Vert Acc [m]";
pub(crate) const POS_FLAGS: &str = "Pos Flags";
pub(crate) const INS_USED: &str = "INS Used";
pub(crate) const POS_FIX_MODE: &str = "Pos Fix Mode";
pub(crate) const CORR_AGE_S: &str = "Corr. Age [s]";
pub(crate) const VEL_N: &str = "Vel. N [m/s]";
pub(crate) const VEL_E: &str = "Vel. E [m/s]";
pub(crate) const VEL_D: &str = "Vel. D [m/s]";
pub(crate) const VEL_TOTAL: &str = "Vel. Total [m/s]";
pub(crate) const VEL_FLAGS: &str = "Vel. Flags";
pub(crate) const PDOP: &str = "PDOP";
pub(crate) const GDOP: &str = "GDOP";
pub(crate) const TDOP: &str = "TDOP";
pub(crate) const HDOP: &str = "HDOP";
pub(crate) const VDOP: &str = "VDOP";
pub(crate) const DOPS_FLAGS: &str = "DOPS Flags";
pub(crate) const INS_STATUS: &str = "INS Status";
pub(crate) const ANG_RATE_X_DEG_P_S: &str = "Ang. Rate X [°/s]";
pub(crate) const ANG_RATE_Y_DEG_P_S: &str = "Ang. Rate Y [°/s]";
pub(crate) const ANG_RATE_Z_DEG_P_S: &str = "Ang. Rate Z [°/s]";
pub(crate) const ROLL: &str = "Roll (Accuracy) [°]";
pub(crate) const PITCH: &str = "Pitch (Accuracy) [°]";
pub(crate) const YAW: &str = "Yaw (Accuracy) [°]";
pub(crate) const COV_N_N: &str = "LLH Cov N-N [m^2]";
pub(crate) const COV_N_E: &str = "LLH Cov N-E [m^2]";
pub(crate) const COV_N_D: &str = "LLH Cov N-D [m^2]";
pub(crate) const COV_E_E: &str = "LLH Cov E-E [m^2]";
pub(crate) const COV_E_D: &str = "LLH Cov E-D [m^2]";
pub(crate) const COV_D_D: &str = "LLH Cov D-D [m^2]";

pub(crate) const SOLUTION_TABLE_KEYS: &[&str] = &[
    GPS_WEEK,
    GPS_TOW,
    GPS_TIME,
    UTC_TIME,
    UTC_SRC,
    SATS_USED,
    LAT,
    LON,
    HEIGHT,
    HORIZ_ACC,
    VERT_ACC,
    POS_FLAGS,
    INS_USED,
    POS_FIX_MODE,
    CORR_AGE_S,
    VEL_N,
    VEL_E,
    VEL_D,
    VEL_TOTAL,
    VEL_FLAGS,
    PDOP,
    GDOP,
    TDOP,
    HDOP,
    VDOP,
    DOPS_FLAGS,
    INS_STATUS,
    ANG_RATE_X_DEG_P_S,
    ANG_RATE_Y_DEG_P_S,
    ANG_RATE_Z_DEG_P_S,
    ROLL,
    PITCH,
    YAW,
    COV_N_N,
    COV_N_E,
    COV_N_D,
    COV_E_E,
    COV_E_D,
    COV_D_D,
];

// Solution Velocity Tab constants.
pub(crate) const HORIZONTAL_COLOR: &str = "#E41A1C";
pub(crate) const VERTICAL_COLOR: &str = "#377EB8";
pub(crate) const MPS: &str = "m/s";
pub(crate) const MPH: &str = "mph";
pub(crate) const KPH: &str = "kph";
pub(crate) const MPS2MPH: f64 = 2.236934;
pub(crate) const MPS2KPH: f64 = 3.600000;
pub(crate) const UDEG2DEG: f64 = 1.0e-6_f64;

// Status Bar constants.
pub(crate) const UPDATE_TOLERANCE_SECONDS: f64 = 1.2;
pub(crate) const UNKNOWN_ERROR: &str = "Unk Error";
pub(crate) const UNKNOWN_ERROR_SHORT: &str = "Unk";
pub(crate) const ODO_POSTFIX: &str = "+Odo";
pub(crate) const INS_POSTFIX: &str = "+INS";

// Update firmware constants.
pub(crate) const HARDWARE_REVISION: &str = "piksi_multi";
pub(crate) const FIRMWARE_V2_VERSION: &str = "v2.0.0";
lazy_static! {
    pub(crate) static ref FIRMWARE_V2: SwiftVersion =
        SwiftVersion::parse(FIRMWARE_V2_VERSION).unwrap();
}

// CLI constants
#[cfg(target_os = "windows")]
pub const EXAMPLE_SERIAL_NAME: &str = "COM1";
#[cfg(target_os = "linux")]
pub const EXAMPLE_SERIAL_NAME: &str = "/dev/ttyUSB0";
#[cfg(target_os = "macos")]
pub const EXAMPLE_SERIAL_NAME: &str = "/dev/cu.usbserial";
