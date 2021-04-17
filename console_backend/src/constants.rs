// Server constants.
pub const CLOSE: &str = "CLOSE";

// Common constants.
pub const NUM_POINTS: usize = 200;

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
pub const GPS_L1CA: &str = "GPS L1CA";
pub const GPS_L2C_M: &str = "GPS L2C M";
pub const GLO_L10F: &str = "GLO L1OF";
pub const GLO_L20F: &str = "GLO L2OF";
pub const BDS2_B1_I: &str = "BDS2 B1 I";
pub const BDS2_B2_I: &str = "BDS2 B2 I";
pub const GAL_E1_B: &str = "GAL E1 B";
pub const GAL_E5B_I: &str = "GAL E5b I";
pub const QZS_L1CA: &str = "QZS L1CA";
pub const QZS_L2C_M: &str = "QZS L2C M";
pub const SBAS_L1: &str = "SBAS L1";
pub const SHOW_LEGEND: &str = "Show Legend";

pub const GPS_L1CA_STR: &str = "GPS L1CA";
pub const GPS_L2CM_STR: &str = "GPS L2C M";
pub const GPS_L2CL_STR: &str = "GPS L2C L";
pub const GPS_L2CX_STR: &str = "GPS L2C M+L";
pub const GPS_L1P_STR: &str = "GPS L1P";
pub const GPS_L2P_STR: &str = "GPS L2P";
pub const GPS_L5I_STR: &str = "GPS L5 I";
pub const GPS_L5Q_STR: &str = "GPS L5 Q";
pub const GPS_L5X_STR: &str = "GPS L5 I+Q";
pub const GPS_AUX_STR: &str = "AUX GPS L1";

pub const SBAS_L1_STR: &str = "SBAS L1";
pub const SBAS_L5I_STR: &str = "SBAS L5 I";
pub const SBAS_L5Q_STR: &str = "SBAS L5 Q";
pub const SBAS_L5X_STR: &str = "SBAS L5 I+Q";
pub const SBAS_AUX_STR: &str = "AUX SBAS L1";

pub const GLO_L1OF_STR: &str = "GLO L1OF";
pub const GLO_L2OF_STR: &str = "GLO L2OF";
pub const GLO_L1P_STR: &str = "GLO L1P";
pub const GLO_L2P_STR: &str = "GLO L2P";

pub const BDS2_B1_STR: &str = "BDS2 B1 I";
pub const BDS2_B2_STR: &str = "BDS2 B2 I";
pub const BDS3_B1CI_STR: &str = "BDS3 B1C I";
pub const BDS3_B1CQ_STR: &str = "BDS3 B1C Q";
pub const BDS3_B1CX_STR: &str = "BDS3 B1C I+Q";
pub const BDS3_B5I_STR: &str = "BDS3 B2a I";
pub const BDS3_B5Q_STR: &str = "BDS3 B2a Q";
pub const BDS3_B5X_STR: &str = "BDS3 B2a X";
pub const BDS3_B7I_STR: &str = "BDS3 B2b I";
pub const BDS3_B7Q_STR: &str = "BDS3 B2b Q";
pub const BDS3_B7X_STR: &str = "BDS3 B2b X";
pub const BDS3_B3I_STR: &str = "BDS3 B3I";
pub const BDS3_B3Q_STR: &str = "BDS3 B3Q";
pub const BDS3_B3X_STR: &str = "BDS3 B3X";
pub const BDS3_AUX_STR: &str = "AUX BDS B1";

pub const GAL_E1B_STR: &str = "GAL E1 B";
pub const GAL_E1C_STR: &str = "GAL E1 C";
pub const GAL_E1X_STR: &str = "GAL E1 B+C";
pub const GAL_E5I_STR: &str = "GAL E5a I";
pub const GAL_E5Q_STR: &str = "GAL E5a Q";
pub const GAL_E5X_STR: &str = "GAL E5a I+Q";
pub const GAL_E6B_STR: &str = "GAL E6 B";
pub const GAL_E6C_STR: &str = "GAL E6 C";
pub const GAL_E6X_STR: &str = "GAL E6 B+C";
pub const GAL_E8I_STR: &str = "GAL E5ab I";
pub const GAL_E8Q_STR: &str = "GAL E5ab Q";
pub const GAL_E8X_STR: &str = "GAL E5ab I+Q";
pub const GAL_E7I_STR: &str = "GAL E5b I";
pub const GAL_E7Q_STR: &str = "GAL E5b Q";
pub const GAL_E7X_STR: &str = "GAL E5b I+Q";
pub const GAL_AUX_STR: &str = "AUX GAL E1";

pub const QZS_L1CA_STR: &str = "QZS L1CA";
pub const QZS_L2CM_STR: &str = "QZS L2C M";
pub const QZS_L2CL_STR: &str = "QZS L2C L";
pub const QZS_L2CX_STR: &str = "QZS L2C M+L";
pub const QZS_L5I_STR: &str = "QZS L5 I";
pub const QZS_L5Q_STR: &str = "QZS L5 Q";
pub const QZS_L5X_STR: &str = "QZS L5 I+Q";
pub const QZS_AUX_STR: &str = "AUX QZS L1";

pub const CODE_NOT_AVAILABLE: &str = "N/A";

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
