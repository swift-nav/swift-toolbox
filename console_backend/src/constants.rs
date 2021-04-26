// Server constants.
pub const CLOSE: &str = "CLOSE";

// Process Message constants.
pub const PAUSE_LOOP_SLEEP_DURATION_MS: u64 = 100;

// Logging constants
pub const LOG_WRITER_BUFFER_MESSAGE_COUNT: usize = 50;

// Common constants.
pub const NUM_POINTS: usize = 200;

// Bottom Navbar constants.
pub const AVAILABLE_BAUDRATES: [u32; 6] = [57600, 115200, 230400, 460800, 921600, 1000000];
pub const FLOW_CONTROL_NONE: &str = "None";
pub const FLOW_CONTROL_SOFTWARE: &str = "Software XON/XOFF";
pub const FLOW_CONTROL_HARDWARE: &str = "Hardware RTS/CTS";
pub const AVAILABLE_FLOWS: &[&str] = &[
    FLOW_CONTROL_NONE,
    FLOW_CONTROL_SOFTWARE,
    FLOW_CONTROL_HARDWARE,
];
pub const SERIALPORT_READ_TIMEOUT_MS: u64 = 1000;

// Tracking Signals Tab constants.
pub const NUM_SATELLITES: usize = 60;
pub const NUM_COLORS: u8 = 37;
pub const TRK_RATE: f64 = 2.0;
pub const GLO_SLOT_SAT_MAX: u8 = 90;
pub const GLO_FCN_OFFSET: i16 = 8;
pub const SBAS_NEG_OFFSET: i16 = 120;
pub const QZSS_NEG_OFFSET: i16 = 193;
pub const SNR_THRESHOLD: f64 = 15.0;
pub const TRACKING_SIGNALS_PLOT_MAX: f64 = 60.0;
pub const GUI_UPDATE_PERIOD: f64 = 0.2;

pub const SHOW_LEGEND: &str = "Show Legend";

// Solution Table.
pub const PLOT_HISTORY_MAX: usize = 1000;
pub const DILUTION_OF_PRECISION_UNITS: f64 = 0.01;
pub const NUM_GNSS_MODES: usize = 6;
pub const LAT_MAX: f64 = 90_f64;
pub const LAT_MIN: f64 = -90_f64;
pub const LON_MAX: f64 = 180_f64;
pub const LON_MIN: f64 = -180_f64;
pub const DEGREES: &str = "degrees";
pub const METERS: &str = "meters";
pub const NO_FIX_LABEL: &str = "No Fix";
pub const SPP_LABEL: &str = "SPP";
pub const DGNSS_LABEL: &str = "DGPS";
pub const FLOAT_LABEL: &str = "RTK float";
pub const FIXED_LABEL: &str = "RTK fixed";
pub const DR_LABEL: &str = "DR";
pub const SBAS_LABEL: &str = "SBAS";

pub const NO_FIX: &str = "No Fix";
pub const SPP: &str = "SPP";
pub const DGNSS: &str = "DGPS";
pub const FLOAT: &str = "Float RTK";
pub const FIXED: &str = "Fixed RTK";
pub const DR: &str = "Dead Reckoning";
pub const SBAS: &str = "SBAS";
pub const NO_FIX_COLOR: &str = "#FFFFFF";
pub const SPP_COLOR: &str = "#0000FF";
pub const DGNSS_COLOR: &str = "#00B3FF";
pub const FLOAT_COLOR: &str = "#BF00BF";
pub const FIXED_COLOR: &str = "#FFA500";
pub const DR_COLOR: &str = "#000000";
pub const SBAS_COLOR: &str = "#00FF00";

pub const LAT_SPP: &str = "lat_SPP";
pub const LNG_SPP: &str = "lng_SPP";
pub const ALT_SPP: &str = "alt_SPP";
pub const LAT_DGNSS: &str = "lat_DGPS";
pub const LNG_DGNSS: &str = "lng_DGPS";
pub const ALT_DGNSS: &str = "alt_DGPS";
pub const LAT_FLOAT: &str = "lat_Float RTK";
pub const LNG_FLOAT: &str = "lng_Float RTK";
pub const ALT_FLOAT: &str = "alt_Float RTK";
pub const LAT_FIXED: &str = "lat_Fixed RTK";
pub const LNG_FIXED: &str = "lng_Fixed RTK";
pub const ALT_FIXED: &str = "alt_Fixed RTK";
pub const LAT_DR: &str = "lat_Dead Reckoning";
pub const LNG_DR: &str = "lng_Dead Reckoning";
pub const ALT_DR: &str = "alt_Dead Reckoning";
pub const LAT_SBAS: &str = "lat_SBAS";
pub const LNG_SBAS: &str = "lng_SBAS";
pub const ALT_SBAS: &str = "alt_SBAS";

pub const SOLUTION_DATA_KEYS: &[&str] = &[
    LAT_SPP, LNG_SPP, LAT_DGNSS, LNG_DGNSS, LAT_FLOAT, LNG_FLOAT, LAT_FIXED, LNG_FIXED, LAT_DR,
    LNG_DR, LAT_SBAS, LNG_SBAS,
];

pub const FACTORY_DEFAULT: &str = "Factory Default";
pub const NON_VOLATILE_MEMORY: &str = "Non Volatile Memory";
pub const DECODED_THIS_SESSION: &str = "Decoded this Session";
pub const UNKNOWN: &str = "Unknown";

pub const EMPTY_STR: &str = "--";
pub const GPS_WEEK: &str = "GPS Week";
pub const GPS_TOW: &str = "GPS TOW";
pub const GPS_TIME: &str = "GPS Time";
pub const UTC_TIME: &str = "UTC Time";
pub const UTC_SRC: &str = "UTC Src";
pub const SATS_USED: &str = "Sats Used";
pub const LAT: &str = "Lat";
pub const LNG: &str = "Lng";
pub const HEIGHT: &str = "Height";
pub const HORIZ_ACC: &str = "Horiz Acc";
pub const VERT_ACC: &str = "Vert Acc";
pub const POS_FLAGS: &str = "Pos Flags";
pub const INS_USED: &str = "INS Used";
pub const POS_FIX_MODE: &str = "Pos Fix Mode";
pub const CORR_AGE_S: &str = "Corr. Age [s]";
pub const VEL_N: &str = "Vel. N";
pub const VEL_E: &str = "Vel. E";
pub const VEL_D: &str = "Vel. D";
pub const VEL_FLAGS: &str = "Vel. Flags";
pub const PDOP: &str = "PDOP";
pub const GDOP: &str = "GDOP";
pub const TDOP: &str = "TDOP";
pub const HDOP: &str = "HDOP";
pub const VDOP: &str = "VDOP";
pub const DOPS_FLAGS: &str = "DOPS Flags";
pub const INS_STATUS: &str = "INS Status";

pub const SOLUTION_TABLE_KEYS: &[&str] = &[
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
pub const HORIZONTAL_COLOR: &str = "#E41A1C";
pub const VERTICAL_COLOR: &str = "#377EB8";
pub const MPS: &str = "m/s";
pub const MPH: &str = "mph";
pub const KPH: &str = "kph";
pub const MPS2MPH: f64 = 2.236934;
pub const MPS2KPH: f64 = 3.600000;
