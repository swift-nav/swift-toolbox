// 'Universal' constants

pub(crate) const NANOSECONDS_PER_SECOND: f64 = 1.0e+9;
pub(crate) const SECONDS_PER_NANOSECOND: f64 = 1.0e-9;

pub(crate) const APPLICATION_QUALIFIER: &str = "com.swift-nav";
pub(crate) const APPLICATION_ORGANIZATION: &str = "swift-nav";
pub(crate) const APPLICATION_NAME: &str = "swift_navigation_console";
// CLI constants.

// Process Message constants.
pub(crate) const PAUSE_LOOP_SLEEP_DURATION_MS: u64 = 100;

// Logging constants
#[allow(dead_code)]
pub(crate) const LOG_WRITER_BUFFER_MESSAGE_COUNT: usize = 50;

// Main Tab constants.
pub(crate) const VEL_TIME_STR_FILEPATH: &str = "velocity_log_%Y%m%d-%H%M%S.csv";
pub(crate) const POS_LLH_TIME_STR_FILEPATH: &str = "position_log_%Y%m%d-%H%M%S.csv";
pub(crate) const BASELINE_TIME_STR_FILEPATH: &str = "baseline_log_%Y%m%d-%H%M%S.csv";
pub(crate) const SBP_FILEPATH: &str = "swift-gnss-%Y%m%d-%H%M%S.sbp";
pub(crate) const SBP_JSON_FILEPATH: &str = "swift-gnss-%Y%m%d-%H%M%S.sbp.json";
pub(crate) const DEFAULT_LOG_DIRECTORY: &str = "SwiftNav";

// Common constants.
pub(crate) const NUM_POINTS: usize = 200;

// Advanced Ins Tab constants.
pub(crate) const NUM_INS_PLOT_ROWS: usize = 6;
pub(crate) const NUM_INS_FIELDS: usize = 5;

// Navbar constants.
pub(crate) const AVAILABLE_REFRESH_RATES: [u8; 4] = [1, 5, 10, 25];
pub(crate) const AVAILABLE_BAUDRATES: [u32; 6] = [57600, 115200, 230400, 460800, 921600, 1000000];
pub(crate) const FLOW_CONTROL_NONE: &str = "None";
pub(crate) const FLOW_CONTROL_SOFTWARE: &str = "Software XON/XOFF";
pub(crate) const FLOW_CONTROL_HARDWARE: &str = "Hardware RTS/CTS";
pub(crate) const AVAILABLE_FLOWS: &[&str] = &[
    FLOW_CONTROL_NONE,
    FLOW_CONTROL_SOFTWARE,
    FLOW_CONTROL_HARDWARE,
];
pub(crate) const SERIALPORT_READ_TIMEOUT_MS: u64 = 1000;
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

pub(crate) const SHOW_LEGEND: &str = "Show Legend";

// Advanced Magnetometer Tab.
pub(crate) const MAGNETOMETER_Y_AXIS_PADDING_MULTIPLIER: f64 = 0.25;

// Baseline Tab.
pub(crate) const BASELINE_DIRECTION_MAX: f64 = f64::MAX;
pub(crate) const BASELINE_DIRECTION_MIN: f64 = f64::MIN;

pub(crate) const N_FLOAT: &str = "n_Float RTK";
pub(crate) const N_FIXED: &str = "n_Fixed RTK";
pub(crate) const N_DGNSS: &str = "n_DGPS";
pub(crate) const E_FLOAT: &str = "e_Float RTK";
pub(crate) const E_FIXED: &str = "e_Fixed RTK";
pub(crate) const E_DGNSS: &str = "e_DGPS";
pub(crate) const N: &str = "N";
pub(crate) const E: &str = "E";
pub(crate) const D: &str = "D";
pub(crate) const DIST: &str = "Dist.";
pub(crate) const FLAGS: &str = "Flags";
pub(crate) const MODE: &str = "Mode";
pub(crate) const HEADING: &str = "Heading";

pub const BASELINE_DATA_KEYS: &[&str] = &[N_FLOAT, N_FIXED, N_DGNSS, E_FLOAT, E_FIXED, E_DGNSS];

pub const BASELINE_TABLE_KEYS: &[&str] = &[
    GPS_WEEK, GPS_TOW, GPS_TIME, UTC_TIME, UTC_SRC, N, E, D, HORIZ_ACC, VERT_ACC, DIST, SATS_USED,
    FLAGS, MODE, HEADING, CORR_AGE_S,
];

// Solution Table.
pub(crate) const PLOT_HISTORY_MAX: usize = 1000;
pub(crate) const DILUTION_OF_PRECISION_UNITS: f64 = 0.01;
pub(crate) const NUM_GNSS_MODES: usize = 6;
pub(crate) const LAT_MAX: f64 = 90_f64;
pub(crate) const LAT_MIN: f64 = -90_f64;
pub(crate) const LON_MAX: f64 = 180_f64;
pub(crate) const LON_MIN: f64 = -180_f64;
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
pub(crate) const SPP_COLOR: &str = "#0000FF";
pub(crate) const DGNSS_COLOR: &str = "#00B3FF";
pub(crate) const FLOAT_COLOR: &str = "#BF00BF";
pub(crate) const FIXED_COLOR: &str = "#FFA500";
pub(crate) const DR_COLOR: &str = "#000000";
pub(crate) const SBAS_COLOR: &str = "#00FF00";

pub(crate) const LAT_SPP: &str = "lat_SPP";
pub(crate) const LNG_SPP: &str = "lng_SPP";
pub(crate) const LAT_DGNSS: &str = "lat_DGPS";
pub(crate) const LNG_DGNSS: &str = "lng_DGPS";
pub(crate) const LAT_FLOAT: &str = "lat_Float RTK";
pub(crate) const LNG_FLOAT: &str = "lng_Float RTK";
pub(crate) const LAT_FIXED: &str = "lat_Fixed RTK";
pub(crate) const LNG_FIXED: &str = "lng_Fixed RTK";
pub(crate) const LAT_DR: &str = "lat_Dead Reckoning";
pub(crate) const LNG_DR: &str = "lng_Dead Reckoning";
pub(crate) const LAT_SBAS: &str = "lat_SBAS";
pub(crate) const LNG_SBAS: &str = "lng_SBAS";

pub(crate) const SOLUTION_DATA_KEYS: &[&str] = &[
    LAT_SPP, LNG_SPP, LAT_DGNSS, LNG_DGNSS, LAT_FLOAT, LNG_FLOAT, LAT_FIXED, LNG_FIXED, LAT_DR,
    LNG_DR, LAT_SBAS, LNG_SBAS,
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
pub(crate) const LAT: &str = "Lat";
pub(crate) const LNG: &str = "Lng";
pub(crate) const HEIGHT: &str = "Height";
pub(crate) const HORIZ_ACC: &str = "Horiz Acc";
pub(crate) const VERT_ACC: &str = "Vert Acc";
pub(crate) const POS_FLAGS: &str = "Pos Flags";
pub(crate) const INS_USED: &str = "INS Used";
pub(crate) const POS_FIX_MODE: &str = "Pos Fix Mode";
pub(crate) const CORR_AGE_S: &str = "Corr. Age [s]";
pub(crate) const VEL_N: &str = "Vel. N";
pub(crate) const VEL_E: &str = "Vel. E";
pub(crate) const VEL_D: &str = "Vel. D";
pub(crate) const VEL_FLAGS: &str = "Vel. Flags";
pub(crate) const PDOP: &str = "PDOP";
pub(crate) const GDOP: &str = "GDOP";
pub(crate) const TDOP: &str = "TDOP";
pub(crate) const HDOP: &str = "HDOP";
pub(crate) const VDOP: &str = "VDOP";
pub(crate) const DOPS_FLAGS: &str = "DOPS Flags";
pub(crate) const INS_STATUS: &str = "INS Status";

pub(crate) const SOLUTION_TABLE_KEYS: &[&str] = &[
    GPS_WEEK,
    GPS_TOW,
    GPS_TIME,
    UTC_TIME,
    UTC_SRC,
    SATS_USED,
    LAT,
    LNG,
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
    VEL_FLAGS,
    PDOP,
    GDOP,
    TDOP,
    HDOP,
    VDOP,
    DOPS_FLAGS,
    INS_STATUS,
];

// Solution Velocity Tab constants.
pub(crate) const HORIZONTAL_COLOR: &str = "#E41A1C";
pub(crate) const VERTICAL_COLOR: &str = "#377EB8";
pub(crate) const MPS: &str = "m/s";
pub(crate) const MPH: &str = "mph";
pub(crate) const KPH: &str = "kph";
pub(crate) const MPS2MPH: f64 = 2.236934;
pub(crate) const MPS2KPH: f64 = 3.600000;

// Status Bar constants.
pub(crate) const UPDATE_TOLERANCE_SECONDS: f64 = 1.2;
pub(crate) const UNKNOWN_ERROR: &str = "Unk Error";
pub(crate) const UNKNOWN_ERROR_SHORT: &str = "unk";
pub(crate) const ODO_POSTFIX: &str = "+Odo";
pub(crate) const INS_POSTFIX: &str = "+INS";
