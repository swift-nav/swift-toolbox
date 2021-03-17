/// Flags and constants corresponding to GT messages.

pub const TRACKING_STATE_DETAILED: u16 = 0x0011;
pub const TRACKING_STATE_DEP_B: u16 = 0x0013;
pub const THREAD_STATE: u16 = 0x0017;
pub const UART_STATE: u16 = 0x001D;
pub const ACQ_SV_PROFILE_DEP: u16 = 0x001E;
pub const ACQ_RESULT_DEP: u16 = 0x001F;
pub const ACQ_SV_PROFILE: u16 = 0x002E;
pub const ACQ_RESULT: u16 = 0x002F;
pub const TRACKING_STATE: u16 = 0x0041;
pub const BASE_POS_ECEF: u16 = 0x0048;
pub const OBS: u16 = 0x004A;
pub const MEASUREMENT_STATE: u16 = 0x0061;
pub const ALMANAC_GPS_DEP: u16 = 0x0070;
pub const ALMANAC_GPS: u16 = 0x0072;
pub const ALMANAC_GLO: u16 = 0x0073;
pub const GLO_BIASES: u16 = 0x0075;
pub const EPHEMERIS_GPS_DEP_E: u16 = 0x0081;
pub const EPHEMERIS_GPS_DEP_F: u16 = 0x0086;
pub const EPHEMERIS_GLO_DEP_D: u16 = 0x0088;
pub const EPHEMERIS_BDS: u16 = 0x0089;
pub const EPHEMERIS_GPS: u16 = 0x008A;
pub const EPHEMERIS_GLO: u16 = 0x008B;
pub const EPHEMERIS_SBAS: u16 = 0x008C;
pub const EPHEMERIS_GAL: u16 = 0x008D;
pub const EPHEMERIS_QZSS: u16 = 0x008E;
pub const IONO: u16 = 0x0090;
pub const SV_CONFIGURATION_GPS: u16 = 0x0091;
pub const GROUP_DELAY_DEP_A: u16 = 0x0092;
pub const GROUP_DELAY_DEP_B: u16 = 0x0093;
pub const GROUP_DELAY: u16 = 0x0094;
pub const EPHEMERIS_GAL_DEP_A: u16 = 0x0095;
pub const GNSS_CAPB: u16 = 0x0096;
pub const SV_AZ_EL: u16 = 0x0097;
pub const SETTINGS_READ_RESP: u16 = 0x00A5;
pub const SETTINGS_READ_BY_IDX_DONE: u16 = 0x00A6;
pub const SETTINGS_READ_BY_IDX_RESP: u16 = 0x00A7;
pub const SETTINGS_WRITE_RESP: u16 = 0x00AF;
pub const DEVICE_MONITOR: u16 = 0x00B5;
pub const NETWORK_BANDWIDTH_USAGE: u16 = 0x00BD;
pub const CELL_MODEM_STATUS: u16 = 0x00BE;
pub const FRONT_END_GAIN: u16 = 0x00BF;

pub const EXT_EVENT: u16 = 0x0101;
pub const GPS_TIME: u16 = 0x0102;
pub const UTC_TIME: u16 = 0x0103;
pub const GPS_TIME_GNSS: u16 = 0x0104;
pub const UTC_TIME_GNSS: u16 = 0x0105;

pub const DOPS: u16 = 0x0208;
pub const POS_ECEF: u16 = 0x0209;
pub const POS_LLA: u16 = 0x020A;
pub const BASELINE_ECEF: u16 = 0x020B;
pub const BASELINE_NED: u16 = 0x020C;
pub const VEL_ECEF: u16 = 0x020D;
pub const VEL_NED: u16 = 0x020E;
pub const BASELINE_HDG: u16 = 0x020F;
pub const CORR_AGE: u16 = 0x0210;
pub const POS_LLA_COV: u16 = 0x0211;
pub const VEL_NED_COV: u16 = 0x0212;
pub const VEL_BODY: u16 = 0x0213;
pub const POS_ECEF_COV: u16 = 0x0214;
pub const VEL_ECEF_COV: u16 = 0x0215;
pub const PROTECTION_LEVEL: u16 = 0x0216;
pub const ORIENT_QUAT: u16 = 0x0220;
pub const ORIENT_EULER: u16 = 0x0221;
pub const ANGULAR_RATE: u16 = 0x0222;
pub const POS_ECEF_GNSS: u16 = 0x0229;
pub const POS_LLA_GNSS: u16 = 0x022A;
pub const VEL_ECEF_GNSS: u16 = 0x022D;
pub const VEL_NED_GNSS: u16 = 0x022E;
pub const POS_LLA_COV_GNSS: u16 = 0x0231;
pub const VEL_NED_COV_GNSS: u16 = 0x0232;
pub const POS_ECEF_COV_GNSS: u16 = 0x0234;
pub const VEL_ECEF_COV_GNSS: u16 = 0x0235;

pub const NDB_EVENT: u16 = 0x0400;
pub const LOG: u16 = 0x0401;
pub const FWD: u16 = 0x0402;

pub const SSR_ORBIT_CLOCK_DEP_A: u16 = 0x05DC;
pub const SSR_ORBIT_CLOCK: u16 = 0x05DD;
pub const SSR_CODE_BIASES: u16 = 0x05E1;
pub const SSR_PHASE_BIASES: u16 = 0x05E6;
pub const SSR_STEC_CORRECTION_DEP_A: u16 = 0x05EB;
pub const SSR_GRIDDED_CORRECTION_NO_STD_DEP_A: u16 = 0x05F0;
pub const SSR_GRIDDED_CORRECTION_DEP_A: u16 = 0x05FA;
pub const SSR_GRID_DEFINITION_DEP_A: u16 = 0x05F5;
pub const SSR_TILE_DEFINITION: u16 = 0x05F6;
pub const SSR_STEC_CORRECTION: u16 = 0x05FB;
pub const SSR_GRIDDED_CORRECTION: u16 = 0x05FC;

pub const OSR: u16 = 0x0640;

pub const IMU_RAW: u16 = 0x0900;
pub const IMU_AUX: u16 = 0x0901;
pub const MAG_RAW: u16 = 0x0902;
pub const ODOMETRY: u16 = 0x0903;
pub const WHEELTICK: u16 = 0x0904;

pub const SBAS_RAW: u16 = 0x7777;

pub const STARTUP: u16 = 0xFF00;
pub const DGNSS_STATUS: u16 = 0xFF02;
pub const INS_STATUS: u16 = 0xFF03;

pub const INS_UPDATES: u16 = 0xFF06;
pub const GNSS_TIME_OFFSET: u16 = 0xFF07;
pub const GROUP_META: u16 = 0xFF0A;
pub const SOLN_META: u16 = 0xFF0E;
pub const SOLN_META_DEP_A: u16 = 0xFF0F;

pub const HEARTBEAT: u16 = 0xFFFF;

// GT_MSG_SBP_TRACKING_STATE_DEP_B --------------------------------------------

pub const TRACKING_STATE_DEP_B_CH_MAX: u8 = 32;
// pub const SIG_CODE_GPS_L1CA: u8 = 0;
// pub const SIG_CODE_GPS_L2CM: u8 = 1;

// GT_MSG_SBP_THREAD_STATE ----------------------------------------------------

// GT_MSG_SBP_UART_STATE ------------------------------------------------------

// GT_MSG_SBP_TRACKING_STATE --------------------------------------------------

pub const TRACKING_STATE_CH_MAX: u8 = 64;

pub const SIG_CODE_GPS_L1CA: u8 = 0;
pub const SIG_CODE_GPS_L2CM: u8 = 1;
pub const SIG_CODE_SBAS_L1CA: u8 = 2;
pub const SIG_CODE_GLO_L1CA: u8 = 3;
pub const SIG_CODE_GLO_L2CA: u8 = 4;
pub const SIG_CODE_GPS_L1P: u8 = 5;
pub const SIG_CODE_GPS_L2P: u8 = 6;
pub const SIG_CODE_GPS_L2CL: u8 = 7;
pub const SIG_CODE_GPS_L2CX: u8 = 8;
pub const SIG_CODE_GPS_L5I: u8 = 9;
pub const SIG_CODE_GPS_L5Q: u8 = 10;
pub const SIG_CODE_GPS_L5X: u8 = 11;
pub const SIG_CODE_BDS_B11: u8 = 12;
pub const SIG_CODE_BDS_B2: u8 = 13;
pub const SIG_CODE_GAL_E1B: u8 = 14;
pub const SIG_CODE_GAL_E1C: u8 = 15;
pub const SIG_CODE_GAL_E1X: u8 = 16;
pub const SIG_CODE_GAL_E6B: u8 = 17;
pub const SIG_CODE_GAL_E6C: u8 = 18;
pub const SIG_CODE_GAL_E6X: u8 = 19;
pub const SIG_CODE_GAL_E7I: u8 = 20;
pub const SIG_CODE_GAL_E7Q: u8 = 21;
pub const SIG_CODE_GAL_E7X: u8 = 22;
pub const SIG_CODE_GAL_E81: u8 = 23;
pub const SIG_CODE_GAL_E8Q: u8 = 24;
pub const SIG_CODE_GAL_E8X: u8 = 25;
pub const SIG_CODE_GAL_E5I: u8 = 26;
pub const SIG_CODE_GAL_E5Q: u8 = 27;
pub const SIG_CODE_BDS_B5Q: u8 = 48;

// GT_MSG_SBP_BASE_POS_ECEF ---------------------------------------------------

// GT_MSG_SBP_OBS -------------------------------------------------------------

pub const OBS_MAX: u8 = 14;

// GT_MSG_SBP_MEASUREMENT_STATE -----------------------------------------------

pub const MEASUREMENT_STATE_CH_MAX: u8 = 85;

// GT_MSG_SBP_GLO_BIASES ------------------------------------------------------

pub const GLO_BIAS_L1C_MASK: u8 = 0x08;
pub const GLO_BIAS_L1P_MASK: u8 = 0x04;
pub const GLO_BIAS_L2C_MASK: u8 = 0x02;
pub const GLO_BIAS_L2P_MASK: u8 = 0x01;

// GT_MSG_SBP_EPHEMERIS_BDS ---------------------------------------------------

// GT_MSG_SBP_EPHEMERIS_GPS ---------------------------------------------------

// GT_MSG_SBP_EPHEMERIS_GLO ---------------------------------------------------

// GT_MSG_SBP_EPHEMERIS_SBAS --------------------------------------------------

// GT_MSG_SBP_EPHEMERIS_GAL ---------------------------------------------------

// GT_MSG_SBP_EPHEMERIS_GAL_DEP_A ---------------------------------------------

// GT_MSG_SBP_SV_AZ_EL --------------------------------------------------------

pub const AZ_EL_MAX: u8 = 63;

// GT_MSG_SBP_SETTINGS_READ_RESP ----------------------------------------------

// GT_MSG_SBP_SETTINGS_READ_BY_IDX_RESP ---------------------------------------

// GT_MSG_SBP_SETTINGS_WRITE_RESP ---------------------------------------------

// GT_MSG_SBP_DEVICE_MONITOR --------------------------------------------------

// GT_MSG_SBP_NETWORK_BANDWIDTH_USAGE -----------------------------------------

// GT_MSG_SBP_CELL_MODEM_STATUS -----------------------------------------------

// GT_MSG_SBP_FRONT_END_GAIN --------------------------------------------------

pub const FRONT_END_GAIN_INVALID: u8 = 127;

// GT_MSG_SBP_EXT_EVENT -------------------------------------------------------

// GT_MSG_SBP_GPS_TIME --------------------------------------------------------

pub const GPS_TIME_FLAGS_TIME_SOURCE_MASK: u8 = 0x07;
pub const GPS_TIME_FLAGS_TIME_SOURCE_NONE: u8 = 0;
pub const GPS_TIME_FLAGS_TIME_SOURCE_GNSS: u8 = 1;
pub const GPS_TIME_FLAGS_TIME_SOURCE_PROPAGATED: u8 = 2;

// GT_MSG_SBP_UTC_TIME --------------------------------------------------------

pub const UTC_TIME_FLAGS_TIME_SOURCE_MASK: u8 = 0x07;
pub const UTC_TIME_FLAGS_TIME_SOURCE_NONE: u8 = 0;
pub const UTC_TIME_FLAGS_TIME_SOURCE_GNSS: u8 = 1;
pub const UTC_TIME_FLAGS_TIME_SOURCE_PROPAGATED: u8 = 2;

pub const UTC_TIME_FLAGS_OFFSET_SOURCE_MASK: u8 = 0x18;
pub const UTC_TIME_FLAGS_OFFSET_SOURCE_MASK_SHIFT: u8 = 3;
pub const UTC_TIME_FLAGS_OFFSET_SOURCE_FACTORY: u8 = 0;
pub const UTC_TIME_FLAGS_OFFSET_SOURCE_NVM: u8 = 1;
pub const UTC_TIME_FLAGS_OFFSET_SOURCE_DECODED: u8 = 2;

// GT_MSG_SBP_POS_ECEF --------------------------------------------------------

// GT_MSG_SBP_POS_LLA ---------------------------------------------------------

pub const POS_LLA_FLAGS_FIX_MODE_MASK: u8 = 0x07;
pub const POS_LLA_FLAGS_FIX_MODE_NONE: u8 = 0;
pub const POS_LLA_FLAGS_FIX_MODE_SPS: u8 = 1;
pub const POS_LLA_FLAGS_FIX_MODE_DGPS: u8 = 2;
pub const POS_LLA_FLAGS_FIX_MODE_RTK_FLOAT: u8 = 3;
pub const POS_LLA_FLAGS_FIX_MODE_RTK_FIXED: u8 = 4;
pub const POS_LLA_FLAGS_FIX_MODE_DR: u8 = 5;
pub const POS_LLA_FLAGS_FIX_MODE_SBAS: u8 = 6;

pub const POS_LLA_FLAGS_INS_MODE_BITS_SHIFT: u8 = 3;
pub const POS_LLA_FLAGS_INS_MODE_MASK: u8 = 0x03;
pub const POS_LLA_FLAGS_INS_MODE_NONE: u8 = 0;
pub const POS_LLA_FLAGS_INS_MODE_USED: u8 = 1;

// GT_MSG_SBP_BASELINE_ECEF ---------------------------------------------------

// GT_MSG_SBP_BASELINE_NED ----------------------------------------------------

pub const BASELINE_NED_FLAGS_FIX_MODE_MASK: u8 = 0x07;
pub const BASELINE_NED_FLAGS_FIX_MODE_INVALID: u8 = 0;
pub const BASELINE_NED_FLAGS_FIX_MODE_DGNSS: u8 = 2;
pub const BASELINE_NED_FLAGS_FIX_MODE_RTK_FLOAT: u8 = 3;
pub const BASELINE_NED_FLAGS_FIX_MODE_RTK_FIXED: u8 = 4;

// GT_MSG_SBP_VEL_ECEF --------------------------------------------------------

// GT_MSG_SBP_VEL_NED ---------------------------------------------------------

pub const VEL_NED_FLAGS_VEL_MODE_MASK: u8 = 0x07;
pub const VEL_NED_FLAGS_VEL_MODE_INVALID: u8 = 0;
pub const VEL_NED_FLAGS_VEL_MODE_MEAS_DOPPLER: u8 = 1;
pub const VEL_NED_FLAGS_VEL_MODE_COMP_DOPPLER: u8 = 2;
pub const VEL_NED_FLAGS_VEL_MODE_DR: u8 = 3;

pub const VEL_NED_FLAGS_INS_MODE_BITS_SHIFT: u8 = 3;
pub const VEL_NED_FLAGS_INS_MODE_MASK: u8 = 0x03;
pub const VEL_NED_FLAGS_INS_MODE_NONE: u8 = 0;
pub const VEL_NED_FLAGS_INS_MODE_USED: u8 = 1;

// GT_MSG_SBP_DOPS ------------------------------------------------------------

pub const DOPS_FLAGS_FIX_MODE_MASK: u8 = 0x07;
pub const DOPS_FLAGS_FIX_MODE_NONE: u8 = 0;
pub const DOPS_FLAGS_FIX_MODE_SPS: u8 = 1;
pub const DOPS_FLAGS_FIX_MODE_DGPS: u8 = 2;
pub const DOPS_FLAGS_FIX_MODE_RTK_FLOAT: u8 = 3;
pub const DOPS_FLAGS_FIX_MODE_RTK_FIXED: u8 = 4;
pub const DOPS_FLAGS_FIX_MODE_RESERVED: u8 = 5;
pub const DOPS_FLAGS_FIX_MODE_SBAS: u8 = 6;

// GT_MSG_SBP_BASELINE_HDG ----------------------------------------------------

pub const BASELINE_HDG_FLAGS_FIX_MODE_MASK: u8 = 0x07;
pub const BASELINE_HDG_FLAGS_FIX_MODE_INVALID: u8 = 0;
pub const BASELINE_HDG_FLAGS_FIX_MODE_RESERVED: u8 = 1;
pub const BASELINE_HDG_FLAGS_FIX_MODE_DGNSS: u8 = 2;
pub const BASELINE_HDG_FLAGS_FIX_MODE_RTK_FLOAT: u8 = 3;
pub const BASELINE_HDG_FLAGS_FIX_MODE_RTK_FIXED: u8 = 4;

// GT_MSG_SBP_CORR_AGE --------------------------------------------------------

// GT_MSG_SBP_POS_LLA_COV -----------------------------------------------------

// GT_MSG_SBP_VEL_NED_COV -----------------------------------------------------

// GT_MSG_SBP_VEL_BODY --------------------------------------------------------

pub const VEL_BODY_VEL_MODE_BITS_SHIFT: u8 = 0;
pub const VEL_BODY_VEL_MODE_MASK: u8 = 0x07;
pub const VEL_BODY_VEL_MODE_INVALID: u8 = 0;
pub const VEL_BODY_VEL_MODE_MEAS_DOPPLER: u8 = 1;
pub const VEL_BODY_VEL_MODE_COMP_DOPPLER: u8 = 2;
pub const VEL_BODY_VEL_MODE_DR: u8 = 3;

pub const VEL_BODY_INS_MODE_BITS_SHIFT: u8 = 3;
pub const VEL_BODY_INS_MODE_MASK: u8 = 0x03;
pub const VEL_BODY_INS_MODE_NONE: u8 = 0;
pub const VEL_BODY_INS_MODE_USED: u8 = 1;

// GT_MSG_SBP_POS_ECEF_COV ----------------------------------------------------

// GT_MSG_SBP_VEL_ECEF_COV ----------------------------------------------------

// GT_MSG_SBP_PROTECTION_LEVEL ------------------------------------------------

pub const PROTECTION_LEVEL_TIR_BITS_SHIFT: u8 = 0;
pub const PROTECTION_LEVEL_TIR_MASK: u8 = 0x7;
pub const PROTECTION_LEVEL_TIR_INVALID: u8 = 0;
pub const PROTECTION_LEVEL_TIR_LEVEL1: u8 = 1;
pub const PROTECTION_LEVEL_TIR_LEVEL2: u8 = 2;
pub const PROTECTION_LEVEL_TIR_LEVEL3: u8 = 3;

// GT_MSG_SBP_ORIENT_QUAT -----------------------------------------------------

pub const ORIENT_QUAT_INS_MODE_BITS_SHIFT: u8 = 0;
pub const ORIENT_QUAT_INS_MODE_MASK: u8 = 0x7;
pub const ORIENT_QUAT_INS_MODE_INVALID: u8 = 0;
pub const ORIENT_QUAT_INS_MODE_VALID: u8 = 1;

// GT_MSG_SBP_ORIENT_EULER ----------------------------------------------------

pub const ORIENT_EULER_INS_MODE_BITS_SHIFT: u8 = 0;
pub const ORIENT_EULER_INS_MODE_MASK: u8 = 0x7;
pub const ORIENT_EULER_INS_MODE_INVALID: u8 = 0;
pub const ORIENT_EULER_INS_MODE_VALID: u8 = 1;

// GT_MSG_SBP_ANGULAR_RATE ----------------------------------------------------

pub const ANGULAR_RATE_INS_MODE_BITS_SHIFT: u8 = 0;
pub const ANGULAR_RATE_INS_MODE_MASK: u8 = 0x7;
pub const ANGULAR_RATE_INS_MODE_INVALID: u8 = 0;
pub const ANGULAR_RATE_INS_MODE_VALID: u8 = 1;

// GT_MSG_SBP_IMU_RAW ---------------------------------------------------------

// GT_MSG_SBP_IMU_AUX ---------------------------------------------------------

pub const IMU_AUX_TYPE_BMI160: u8 = 0;

// GT_MSG_SBP_MAG_RAW ---------------------------------------------------------

// GT_MSG_SBP_ODOMETRY --------------------------------------------------------

pub const ODOMETRY_FLAGS_TIME_SOURCE_BITS_SHIFT: u8 = 0;
pub const ODOMETRY_FLAGS_TIME_SOURCE_MASK: u8 = 0x7;
pub const ODOMETRY_FLAGS_TIME_SOURCE_INVALID: u8 = 0;
pub const ODOMETRY_FLAGS_TIME_SOURCE_GNSS: u8 = 1;
pub const ODOMETRY_FLAGS_TIME_SOURCE_CPU: u8 = 2;

pub const ODOMETRY_FLAGS_VEL_SOURCE_BITS_SHIFT: u8 = 3;
pub const ODOMETRY_FLAGS_VEL_SOURCE_MASK: u8 = 0x3;
pub const ODOMETRY_FLAGS_VEL_SOURCE_0: u8 = 0;
pub const ODOMETRY_FLAGS_VEL_SOURCE_1: u8 = 1;
pub const ODOMETRY_FLAGS_VEL_SOURCE_2: u8 = 2;
pub const ODOMETRY_FLAGS_VEL_SOURCE_3: u8 = 3;

// GT_MSG_SBP_WHEELTICK -------------------------------------------------------

// GT_MSG_SBP_STARTUP ---------------------------------------------------------

// GT_MSG_SBP_DGNSS_STATUS ----------------------------------------------------

// GT_MSG_SBP_INS_STATUS ------------------------------------------------------

pub const INS_STATUS_MODE_BITS_SHIFT: u8 = 0;
pub const INS_STATUS_MODE_MASK: u8 = 0x7;
pub const INS_STATUS_MODE_AWAITING_INIT: u8 = 0;
pub const INS_STATUS_MODE_ALIGNING: u8 = 1;
pub const INS_STATUS_MODE_READY: u8 = 2;
pub const INS_STATUS_MODE_TIMEOUT: u8 = 3;

pub const INS_STATUS_GNSS_FIX_BITS_SHIFT: u8 = 3;
pub const INS_STATUS_GNSS_FIX_MASK: u8 = 0x1;
pub const INS_STATUS_GNSS_FIX_NO: u8 = 0;
pub const INS_STATUS_GNSS_FIX_YES: u8 = 1;

pub const INS_STATUS_INS_ERROR_BITS_SHIFT: u8 = 4;
pub const INS_STATUS_INS_ERROR_MASK: u8 = 0xF;
pub const INS_STATUS_INS_ERROR_NO_ERROR: u8 = 0;
pub const INS_STATUS_INS_ERROR_IMU_DATA: u8 = 1;
pub const INS_STATUS_INS_ERROR_LICENSE: u8 = 2;
pub const INS_STATUS_INS_ERROR_CALIBRATION: u8 = 3;

pub const INS_STATUS_ODO_STAT_BITS_SHIFT: u8 = 8;
pub const INS_STATUS_ODO_STAT_MASK: u8 = 0x3;
pub const INS_STATUS_ODO_STAT_NO_ODO: u8 = 0;
pub const INS_STATUS_ODO_STAT_RECEIVED: u8 = 1;
pub const INS_STATUS_ODO_STAT_RECEIVED_OLD: u8 = 2;

pub const INS_STATUS_ODO_SYNC_BITS_SHIFT: u8 = 10;
pub const INS_STATUS_ODO_SYNC_MASK: u8 = 0x1;
pub const INS_STATUS_ODO_SYNC_OK: u8 = 0;
pub const INS_STATUS_ODO_SYNC_LATE: u8 = 1;

// GT_MSG_SBP_INS_UPDATES -----------------------------------------------------

// GT_MSG_SBP_GROUP_META ------------------------------------------------------

pub const GROUP_META_GROUP_ID_INVALID: u8 = 0;
pub const GROUP_META_GROUP_ID_BEST_POS: u8 = 1;
pub const GROUP_META_GROUP_ID_GNSS: u8 = 2;
pub const GROUP_META_TYPE_BITS_SHIFT: u8 = 0;
pub const GROUP_META_TYPE_MASK: u8 = 0x3;
pub const GROUP_META_TYPE_INVALID: u8 = 0;
pub const GROUP_META_TYPE_GNSS_ONLY: u8 = 1;
pub const GROUP_META_TYPE_GNSS_AND_INS: u8 = 2;
pub const GROUP_META_TYPE_RESERVED: u8 = 3;
pub const GROUP_META_CNT_MAX: u8 = 100;

// GT_MSG_SBP_SOLN_META -------------------------------------------------------

pub const SOLN_META_SENSOR_INFO_MAX: u8 = 20;

// GT_MSG_SBP_HEARTBEAT -------------------------------------------------------