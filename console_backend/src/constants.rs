use chrono::{prelude::*, DateTime, Duration, Local, TimeZone, Utc};
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::Hash,
};

use crate::types::{Error, UtcDateTime};

// Common constants.
pub const NUM_POINTS: usize = 200;

// Tracking Signals Tab constants.
pub const NUM_SATELLITES: usize = 60;
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

// Solution Table.
pub const PLOT_HISTORY_MAX: usize = 1000;
pub const DILUTION_OF_PRECISION_UNITS: f64 = 0.01;
pub const NUM_GNSS_MODES: usize = 6;
pub const SPP: &str = "spp";
pub const DGNSS: &str = "dgnss";
pub const FLOAT: &str = "float";
pub const FIXED: &str = "fixed";
pub const DR: &str = "dr";
pub const SBAS: &str = "sbas";

pub const LAT_SPP: &str = "lat_spp";
pub const LNG_SPP: &str = "lng_spp";
pub const ALT_SPP: &str = "alt_spp";
pub const LAT_DGNSS: &str = "lat_dgnss";
pub const LNG_DGNSS: &str = "lng_dgnss";
pub const ALT_DGNSS: &str = "alt_dgnss";
pub const LAT_FLOAT: &str = "lat_float";
pub const LNG_FLOAT: &str = "lng_float";
pub const ALT_FLOAT: &str = "alt_float";
pub const LAT_FIXED: &str = "lat_fixed";
pub const LNG_FIXED: &str = "lng_fixed";
pub const ALT_FIXED: &str = "alt_fixed";
pub const LAT_DR: &str = "lat_dr";
pub const LNG_DR: &str = "lng_dr";
pub const ALT_DR: &str = "alt_dr";
pub const LAT_SBAS: &str = "lat_sbas";
pub const LNG_SBAS: &str = "lng_sbas";
pub const ALT_SBAS: &str = "alt_sbas";
pub const SOLUTIONS_KEYS: &[&str] = &[
    LAT_SPP, LNG_SPP, ALT_SPP, LAT_DGNSS, LNG_DGNSS, ALT_DGNSS, LAT_FLOAT, LNG_FLOAT, ALT_FLOAT,
    LAT_FIXED, LNG_FIXED, ALT_FIXED, LAT_DR, LNG_DR, ALT_DR, LAT_SBAS, LNG_SBAS, ALT_SBAS,
];
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum GnssModes {
    Spp = 1,
    Dgnss = 2,
    Float = 3,
    Fixed = 4,
    Dr = 5,
    Sbas = 6,
}
impl From<u8> for GnssModes {
    fn from(s: u8) -> Self {
        match s {
            1 => GnssModes::Spp,
            2 => GnssModes::Dgnss,
            3 => GnssModes::Float,
            4 => GnssModes::Fixed,
            5 => GnssModes::Dr,
            6 => GnssModes::Sbas,
            _ => panic!("this u8 does not convert to a gnss mode"),
        }
    }
}
impl ToString for GnssModes {
    /// Retrieve associated string with the provided signal code.
    ///
    /// # Parameters
    ///
    /// - `key`: The code, which is signal code, and satellite constellation-specific satellite identifier.
    fn to_string(&self) -> String {
        let gnss_mode_str = match self {
            GnssModes::Spp => SPP,
            GnssModes::Dgnss => DGNSS,
            GnssModes::Float => FLOAT,
            GnssModes::Fixed => FIXED,
            GnssModes::Dr => DR,
            GnssModes::Sbas => SBAS,
        };
        String::from(gnss_mode_str)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum SignalCodes {
    CodeGpsL1Ca = 0,
    CodeGpsL2Cm = 1,
    CodeGpsL2Cl = 7,
    CodeGpsL2Cx = 8,
    CodeGpsL1P = 5,
    CodeGpsL2P = 6,
    CodeGpsL5I = 9,
    CodeGpsL5Q = 10,
    CodeGpsL5X = 11,
    CodeGpsL1Ci = 56,
    CodeGpsL1Cq = 57,
    CodeGpsL1Cx = 58,
    CodeAuxGps = 59,

    CodeGloL1Of = 3,
    CodeGloL2Of = 4,
    CodeGloL1P = 29,
    CodeGloL2P = 30,

    CodeSbasL1Ca = 2,
    CodeSbasL5I = 41,
    CodeSbasL5Q = 42,
    CodeSbasL5X = 43,
    CodeAuxSbas = 60,

    CodeBds2B1 = 12,
    CodeBds2B2 = 13,
    CodeBds3B1Ci = 44,
    CodeBds3B1Cq = 45,
    CodeBds3B1Cx = 46,
    CodeBds3B5I = 47,
    CodeBds3B5Q = 48,
    CodeBds3B5X = 49,
    CodeBds3B7I = 50,
    CodeBds3B7Q = 51,
    CodeBds3B7X = 52,
    CodeBds3B3I = 53,
    CodeBds3B3Q = 54,
    CodeBds3B3X = 55,

    CodeGalE1B = 14,
    CodeGalE1C = 15,
    CodeGalE1X = 16,
    CodeGalE6B = 17,
    CodeGalE6C = 18,
    CodeGalE6X = 19,
    CodeGalE7I = 20,
    CodeGalE7Q = 21,
    CodeGalE7X = 22,
    CodeGalE8I = 23,
    CodeGalE8Q = 24,
    CodeGalE8X = 25,
    CodeGalE5I = 26,
    CodeGalE5Q = 27,
    CodeGalE5X = 28,
    CodeAuxGal = 61,

    CodeQzsL1Ca = 31,
    CodeQzsL1Ci = 32,
    CodeQzsL1Cq = 33,
    CodeQzsL1Cx = 34,
    CodeQzsL2Cm = 35,
    CodeQzsL2Cl = 36,
    CodeQzsL2Cx = 37,
    CodeQzsL5I = 38,
    CodeQzsL5Q = 39,
    CodeQzsL5X = 40,
    CodeAuxQzs = 62,
    NotAvailable = u8::MAX,
}

impl SignalCodes {
    pub fn code_is_gps(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeGpsL1Ca
                | SignalCodes::CodeGpsL2Cm
                | SignalCodes::CodeGpsL2Cl
                | SignalCodes::CodeGpsL2Cx
                | SignalCodes::CodeGpsL1P
                | SignalCodes::CodeGpsL2P
                | SignalCodes::CodeGpsL5I
                | SignalCodes::CodeGpsL5Q
                | SignalCodes::CodeGpsL5X
                | SignalCodes::CodeAuxGps
        )
    }
    pub fn code_is_glo(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeGloL1Of
                | SignalCodes::CodeGloL2Of
                | SignalCodes::CodeGloL1P
                | SignalCodes::CodeGloL2P
        )
    }
    pub fn code_is_sbas(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeSbasL1Ca
                | SignalCodes::CodeSbasL5I
                | SignalCodes::CodeSbasL5Q
                | SignalCodes::CodeSbasL5X
                | SignalCodes::CodeAuxSbas
        )
    }
    pub fn code_is_bds(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeBds2B1
                | SignalCodes::CodeBds2B2
                | SignalCodes::CodeBds3B1Ci
                | SignalCodes::CodeBds3B1Cq
                | SignalCodes::CodeBds3B1Cx
                | SignalCodes::CodeBds3B5I
                | SignalCodes::CodeBds3B5Q
                | SignalCodes::CodeBds3B5X
                | SignalCodes::CodeBds3B3I
                | SignalCodes::CodeBds3B3Q
                | SignalCodes::CodeBds3B3X
                | SignalCodes::CodeBds3B7I
                | SignalCodes::CodeBds3B7Q
                | SignalCodes::CodeBds3B7X
        )
    }

    pub fn code_is_galileo(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeGalE1B
                | SignalCodes::CodeGalE1C
                | SignalCodes::CodeGalE1X
                | SignalCodes::CodeGalE6B
                | SignalCodes::CodeGalE6C
                | SignalCodes::CodeGalE6X
                | SignalCodes::CodeGalE7I
                | SignalCodes::CodeGalE7Q
                | SignalCodes::CodeGalE7X
                | SignalCodes::CodeGalE8I
                | SignalCodes::CodeGalE8Q
                | SignalCodes::CodeGalE8X
                | SignalCodes::CodeGalE5I
                | SignalCodes::CodeGalE5Q
                | SignalCodes::CodeGalE5X
                | SignalCodes::CodeAuxGal
        )
    }

    pub fn code_is_qzss(&self) -> bool {
        matches!(
            self,
            SignalCodes::CodeQzsL1Ca
                | SignalCodes::CodeQzsL2Cm
                | SignalCodes::CodeQzsL2Cl
                | SignalCodes::CodeQzsL2Cx
                | SignalCodes::CodeQzsL5I
                | SignalCodes::CodeQzsL5Q
                | SignalCodes::CodeQzsL5X
                | SignalCodes::CodeAuxQzs
        )
    }

    pub fn filters(&self) -> Option<String> {
        match self {
            SignalCodes::CodeGpsL1Ca => Some(GPS_L1CA.to_string()),
            SignalCodes::CodeGpsL2Cm => Some(GPS_L2C_M.to_string()),
            SignalCodes::CodeGloL1Of => Some(GLO_L10F.to_string()),
            SignalCodes::CodeGloL2Of => Some(GLO_L20F.to_string()),
            SignalCodes::CodeBds2B1 => Some(BDS2_B1_I.to_string()),
            SignalCodes::CodeBds2B2 => Some(BDS2_B2_I.to_string()),
            SignalCodes::CodeGalE1B => Some(GAL_E1_B.to_string()),
            SignalCodes::CodeGalE7I => Some(GAL_E5B_I.to_string()),
            SignalCodes::CodeQzsL1Ca => Some(QZS_L1CA.to_string()),
            SignalCodes::CodeQzsL2Cm => Some(QZS_L2C_M.to_string()),
            SignalCodes::CodeSbasL1Ca => Some(SBAS_L1.to_string()),
            _ => None,
        }
    }
}

impl From<u8> for SignalCodes {
    fn from(s: u8) -> Self {
        match s {
            0 => SignalCodes::CodeGpsL1Ca,
            1 => SignalCodes::CodeGpsL2Cm,
            7 => SignalCodes::CodeGpsL2Cl,
            8 => SignalCodes::CodeGpsL2Cx,
            5 => SignalCodes::CodeGpsL1P,
            6 => SignalCodes::CodeGpsL2P,
            9 => SignalCodes::CodeGpsL5I,
            10 => SignalCodes::CodeGpsL5Q,
            11 => SignalCodes::CodeGpsL5X,
            56 => SignalCodes::CodeGpsL1Ci,
            57 => SignalCodes::CodeGpsL1Cq,
            58 => SignalCodes::CodeGpsL1Cx,
            59 => SignalCodes::CodeAuxGps,

            3 => SignalCodes::CodeGloL1Of,
            4 => SignalCodes::CodeGloL2Of,
            29 => SignalCodes::CodeGloL1P,
            30 => SignalCodes::CodeGloL2P,

            2 => SignalCodes::CodeSbasL1Ca,
            41 => SignalCodes::CodeSbasL5I,
            42 => SignalCodes::CodeSbasL5Q,
            43 => SignalCodes::CodeSbasL5X,
            60 => SignalCodes::CodeAuxSbas,

            12 => SignalCodes::CodeBds2B1,
            13 => SignalCodes::CodeBds2B2,
            44 => SignalCodes::CodeBds3B1Ci,
            45 => SignalCodes::CodeBds3B1Cq,
            46 => SignalCodes::CodeBds3B1Cx,
            47 => SignalCodes::CodeBds3B5I,
            48 => SignalCodes::CodeBds3B5Q,
            49 => SignalCodes::CodeBds3B5X,
            50 => SignalCodes::CodeBds3B7I,
            51 => SignalCodes::CodeBds3B7Q,
            52 => SignalCodes::CodeBds3B7X,
            53 => SignalCodes::CodeBds3B3I,
            54 => SignalCodes::CodeBds3B3Q,
            55 => SignalCodes::CodeBds3B3X,

            14 => SignalCodes::CodeGalE1B,
            15 => SignalCodes::CodeGalE1C,
            16 => SignalCodes::CodeGalE1X,
            17 => SignalCodes::CodeGalE6B,
            18 => SignalCodes::CodeGalE6C,
            19 => SignalCodes::CodeGalE6X,
            20 => SignalCodes::CodeGalE7I,
            21 => SignalCodes::CodeGalE7Q,
            22 => SignalCodes::CodeGalE7X,
            23 => SignalCodes::CodeGalE8I,
            24 => SignalCodes::CodeGalE8Q,
            25 => SignalCodes::CodeGalE8X,
            26 => SignalCodes::CodeGalE5I,
            27 => SignalCodes::CodeGalE5Q,
            28 => SignalCodes::CodeGalE5X,
            61 => SignalCodes::CodeAuxGal,

            31 => SignalCodes::CodeQzsL1Ca,
            32 => SignalCodes::CodeQzsL1Ci,
            33 => SignalCodes::CodeQzsL1Cq,
            34 => SignalCodes::CodeQzsL1Cx,
            35 => SignalCodes::CodeQzsL2Cm,
            36 => SignalCodes::CodeQzsL2Cl,
            37 => SignalCodes::CodeQzsL2Cx,
            38 => SignalCodes::CodeQzsL5I,
            39 => SignalCodes::CodeQzsL5Q,
            40 => SignalCodes::CodeQzsL5X,
            62 => SignalCodes::CodeAuxQzs,
            _ => SignalCodes::NotAvailable,
        }
    }
}

impl std::str::FromStr for SignalCodes {
    type Err = Error;

    /// Retrieve the signal constellation, band and code based off provided string.
    ///
    /// # Parameters
    ///
    /// - `sat_str`: The signal code.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            GPS_L1CA_STR => Ok(SignalCodes::CodeGpsL1Ca),
            GPS_L2CM_STR => Ok(SignalCodes::CodeGpsL2Cm),
            GPS_L2CL_STR => Ok(SignalCodes::CodeGpsL2Cl),
            GPS_L2CX_STR => Ok(SignalCodes::CodeGpsL2Cx),
            GPS_L5I_STR => Ok(SignalCodes::CodeGpsL5I),
            GPS_L5Q_STR => Ok(SignalCodes::CodeGpsL5Q),
            GPS_L5X_STR => Ok(SignalCodes::CodeGpsL5X),
            GPS_L1P_STR => Ok(SignalCodes::CodeGpsL1P),
            GPS_L2P_STR => Ok(SignalCodes::CodeGpsL2P),
            GPS_AUX_STR => Ok(SignalCodes::CodeAuxGps),

            SBAS_L1_STR => Ok(SignalCodes::CodeSbasL1Ca),
            SBAS_L5I_STR => Ok(SignalCodes::CodeSbasL5I),
            SBAS_L5Q_STR => Ok(SignalCodes::CodeSbasL5Q),
            SBAS_L5X_STR => Ok(SignalCodes::CodeSbasL5X),
            SBAS_AUX_STR => Ok(SignalCodes::CodeAuxSbas),

            GLO_L1OF_STR => Ok(SignalCodes::CodeGloL1Of),
            GLO_L2OF_STR => Ok(SignalCodes::CodeGloL2Of),
            GLO_L1P_STR => Ok(SignalCodes::CodeGloL1P),
            GLO_L2P_STR => Ok(SignalCodes::CodeGloL2P),

            BDS2_B1_STR => Ok(SignalCodes::CodeBds2B1),
            BDS2_B2_STR => Ok(SignalCodes::CodeBds2B2),
            BDS3_B1CI_STR => Ok(SignalCodes::CodeBds3B1Ci),
            BDS3_B1CQ_STR => Ok(SignalCodes::CodeBds3B1Cq),
            BDS3_B1CX_STR => Ok(SignalCodes::CodeBds3B1Cx),
            BDS3_B5I_STR => Ok(SignalCodes::CodeBds3B5I),
            BDS3_B5Q_STR => Ok(SignalCodes::CodeBds3B5Q),
            BDS3_B5X_STR => Ok(SignalCodes::CodeBds3B5X),
            BDS3_B7I_STR => Ok(SignalCodes::CodeBds3B7I),
            BDS3_B7Q_STR => Ok(SignalCodes::CodeBds3B7Q),
            BDS3_B7X_STR => Ok(SignalCodes::CodeBds3B7X),
            BDS3_B3I_STR => Ok(SignalCodes::CodeBds3B3I),
            BDS3_B3Q_STR => Ok(SignalCodes::CodeBds3B3Q),
            BDS3_B3X_STR => Ok(SignalCodes::CodeBds3B3X),

            GAL_E1B_STR => Ok(SignalCodes::CodeGalE1B),
            GAL_E1C_STR => Ok(SignalCodes::CodeGalE1C),
            GAL_E1X_STR => Ok(SignalCodes::CodeGalE1X),
            GAL_E5I_STR => Ok(SignalCodes::CodeGalE5I),
            GAL_E5Q_STR => Ok(SignalCodes::CodeGalE5Q),
            GAL_E5X_STR => Ok(SignalCodes::CodeGalE5X),
            GAL_E6B_STR => Ok(SignalCodes::CodeGalE6B),
            GAL_E6C_STR => Ok(SignalCodes::CodeGalE6C),
            GAL_E6X_STR => Ok(SignalCodes::CodeGalE6X),
            GAL_E7I_STR => Ok(SignalCodes::CodeGalE7I),
            GAL_E7Q_STR => Ok(SignalCodes::CodeGalE7Q),
            GAL_E7X_STR => Ok(SignalCodes::CodeGalE7X),
            GAL_E8I_STR => Ok(SignalCodes::CodeGalE8I),
            // GAL_E8Q_STR => SignalCodes::CodeGalE8Q,  // Unreachable
            // GAL_E8X_STR => SignalCodes::CodeGalE8X,  // Unreachable
            GAL_AUX_STR => Ok(SignalCodes::CodeAuxGal),

            QZS_L1CA_STR => Ok(SignalCodes::CodeQzsL1Ca),
            QZS_L2CM_STR => Ok(SignalCodes::CodeQzsL2Cm),
            QZS_L2CL_STR => Ok(SignalCodes::CodeQzsL2Cl),
            QZS_L2CX_STR => Ok(SignalCodes::CodeQzsL2Cx),
            QZS_L5I_STR => Ok(SignalCodes::CodeQzsL5I),
            QZS_L5Q_STR => Ok(SignalCodes::CodeQzsL5Q),
            QZS_L5X_STR => Ok(SignalCodes::CodeQzsL5X),
            QZS_AUX_STR => Ok(SignalCodes::CodeAuxQzs),
            _ => Ok(SignalCodes::NotAvailable),
        }
    }
}

impl ToString for SignalCodes {
    /// Retrieve associated string with the provided signal code.
    ///
    /// # Parameters
    ///
    /// - `key`: The code, which is signal code, and satellite constellation-specific satellite identifier.
    fn to_string(&self) -> String {
        let sat_code_str = match self {
            SignalCodes::CodeGpsL1Ca => GPS_L1CA_STR,
            SignalCodes::CodeGpsL2Cm => GPS_L2CM_STR,
            SignalCodes::CodeGpsL2Cl => GPS_L2CL_STR,
            SignalCodes::CodeGpsL2Cx => GPS_L2CX_STR,
            SignalCodes::CodeGpsL1P => GPS_L1P_STR,
            SignalCodes::CodeGpsL2P => GPS_L2P_STR,
            SignalCodes::CodeGpsL5I => GPS_L5I_STR,
            SignalCodes::CodeGpsL5Q => GPS_L5Q_STR,
            SignalCodes::CodeGpsL5X => GPS_L5X_STR,
            SignalCodes::CodeAuxGps => GPS_AUX_STR,

            SignalCodes::CodeGloL1Of => GLO_L1OF_STR,
            SignalCodes::CodeGloL2Of => GLO_L2OF_STR,
            SignalCodes::CodeGloL1P => GLO_L1P_STR,
            SignalCodes::CodeGloL2P => GLO_L2P_STR,

            SignalCodes::CodeSbasL1Ca => SBAS_L1_STR,
            SignalCodes::CodeSbasL5I => SBAS_L5I_STR,
            SignalCodes::CodeSbasL5Q => SBAS_L5Q_STR,
            SignalCodes::CodeSbasL5X => SBAS_L5X_STR,
            SignalCodes::CodeAuxSbas => SBAS_AUX_STR,

            SignalCodes::CodeBds2B1 => BDS2_B1_STR,
            SignalCodes::CodeBds2B2 => BDS2_B2_STR,
            SignalCodes::CodeBds3B1Ci => BDS3_B1CI_STR,
            SignalCodes::CodeBds3B1Cq => BDS3_B1CQ_STR,
            SignalCodes::CodeBds3B1Cx => BDS3_B1CX_STR,
            SignalCodes::CodeBds3B5I => BDS3_B5I_STR,
            SignalCodes::CodeBds3B5Q => BDS3_B5Q_STR,
            SignalCodes::CodeBds3B5X => BDS3_B5X_STR,
            SignalCodes::CodeBds3B7I => BDS3_B7I_STR,
            SignalCodes::CodeBds3B7Q => BDS3_B7Q_STR,
            SignalCodes::CodeBds3B7X => BDS3_B7X_STR,
            SignalCodes::CodeBds3B3I => BDS3_B3I_STR,
            SignalCodes::CodeBds3B3Q => BDS3_B3Q_STR,
            SignalCodes::CodeBds3B3X => BDS3_B3X_STR,

            SignalCodes::CodeGalE1B => GAL_E1B_STR,
            SignalCodes::CodeGalE1C => GAL_E1C_STR,
            SignalCodes::CodeGalE1X => GAL_E1X_STR,
            SignalCodes::CodeGalE6B => GAL_E6B_STR,
            SignalCodes::CodeGalE6C => GAL_E6C_STR,
            SignalCodes::CodeGalE6X => GAL_E6X_STR,
            SignalCodes::CodeGalE7I => GAL_E7I_STR,
            SignalCodes::CodeGalE7Q => GAL_E7Q_STR,
            SignalCodes::CodeGalE7X => GAL_E7X_STR,
            SignalCodes::CodeGalE8I => GAL_E8I_STR,
            SignalCodes::CodeGalE8Q => GAL_E8Q_STR,
            SignalCodes::CodeGalE8X => GAL_E8X_STR,
            SignalCodes::CodeGalE5I => GAL_E5I_STR,
            SignalCodes::CodeGalE5Q => GAL_E5Q_STR,
            SignalCodes::CodeGalE5X => GAL_E5X_STR,
            SignalCodes::CodeAuxGal => GAL_AUX_STR,

            SignalCodes::CodeQzsL1Ca => QZS_L1CA_STR,
            SignalCodes::CodeQzsL2Cm => QZS_L2CM_STR,
            SignalCodes::CodeQzsL2Cl => QZS_L2CL_STR,
            SignalCodes::CodeQzsL2Cx => QZS_L2CX_STR,
            SignalCodes::CodeQzsL5I => QZS_L5I_STR,
            SignalCodes::CodeQzsL5Q => QZS_L5Q_STR,
            SignalCodes::CodeQzsL5X => QZS_L5X_STR,
            _ => CODE_NOT_AVAILABLE,
        };
        String::from(sat_code_str)
    }
}

pub fn get_label(
    key: (SignalCodes, i16),
    extra: &HashMap<i16, i16>,
) -> (Option<String>, Option<String>, Option<String>) {
    let (code, sat) = key;
    let code_lbl = Some(code.to_string());
    let mut freq_lbl = None;
    let id_lbl;

    if code.code_is_glo() {
        let freq_lbl_ = format!("F+{:02}", sat);
        freq_lbl = Some(freq_lbl_);
        if extra.contains_key(&sat) {
            id_lbl = Some(format!("R{:<02}", extra[&sat]));
        } else {
            id_lbl = Some(format!("R{:<02}", sat));
        }
    } else if code.code_is_sbas() {
        id_lbl = Some(format!("S{: >3}", sat));
    } else if code.code_is_bds() {
        id_lbl = Some(format!("C{:0>2}", sat));
    } else if code.code_is_qzss() {
        id_lbl = Some(format!("J{: >3}", sat));
    } else if code.code_is_galileo() {
        id_lbl = Some(format!("E{:0>2}", sat));
    } else {
        id_lbl = Some(format!("G{:0>2}", sat));
    }
    (code_lbl, freq_lbl, id_lbl)
}

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
pub const GAL_E8I_STR: &str = "GAL AltBOC";
pub const GAL_E8Q_STR: &str = "GAL AltBOC";
pub const GAL_E8X_STR: &str = "GAL AltBOC";
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

/// These colors are distinguishable from one another based on expected codes.
///
/// # Parameters
///
/// - `code`: The signal code.
pub fn color_map(code: i16) -> &'static str {
    match code {
        1 => "#e58a8a",
        2 => "#664949",
        3 => "#590c00",
        4 => "#cc4631",
        5 => "#e56c1c",
        6 => "#4c2a12",
        7 => "#996325",
        8 => "#f2b774",
        9 => "#ffaa00",
        10 => "#ccb993",
        11 => "#997a00",
        12 => "#4c4700",
        13 => "#d0d94e",
        14 => "#aaff00",
        15 => "#4ea614",
        16 => "#123306",
        17 => "#18660c",
        18 => "#6e9974",
        19 => "#8ae6a2",
        20 => "#00ff66",
        21 => "#57f2e8",
        22 => "#1f7980",
        23 => "#263e40",
        24 => "#004d73",
        25 => "#37abe6",
        26 => "#7790a6",
        27 => "#144ea6",
        28 => "#263040",
        29 => "#152859",
        30 => "#1d39f2",
        31 => "#828ed9",
        32 => "#000073",
        33 => "#000066",
        34 => "#8c7aff",
        35 => "#1b0033",
        36 => "#d900ca",
        37 => "#730e6c",
        _ => "#ff0000",
    }
}

const NUM_COLORS: u8 = 37;

/// Retreive the associated color based on provided key.
///
/// # Parameters
///
/// - `key`: The code, which is signal code and satellite constellation-specific satellite identifier.
pub fn get_color(key: (SignalCodes, i16)) -> &'static str {
    let (code, mut sat) = key;

    if code.code_is_glo() {
        sat += GLO_FCN_OFFSET;
    } else if code.code_is_sbas() {
        sat -= SBAS_NEG_OFFSET;
    } else if code.code_is_qzss() {
        sat -= QZSS_NEG_OFFSET;
    }
    if sat > NUM_COLORS as i16 {
        sat %= NUM_COLORS as i16;
    }
    color_map(sat)
}

/// Calculate the length of a degree of latitude and longitude in meters.
///
/// # Parameters
///
/// - `lat_deg`: The latitude degree value to convert to lat/lon meters.
pub fn meters_per_deg(lat_deg: f64) -> (f64, f64) {
    let lat_term_1: f64 = 111132.92;
    let lat_term_2: f64 = -559.82;
    let lat_term_3: f64 = 1.175;
    let lat_term_4: f64 = -0.0023;
    let lon_term_1: f64 = 111412.84;
    let lon_term_2: f64 = -93.5;
    let lon_term_3: f64 = 0.118;

    let latlen = lat_term_1
        + (lat_term_2 * f64::cos(2_f64 * f64::to_radians(lat_deg)))
        + (lat_term_3 * f64::cos(4_f64 * f64::to_radians(lat_deg)))
        + (lat_term_4 * f64::cos(6_f64 * f64::to_radians(lat_deg)));
    let lonlen = (lon_term_1 * f64::cos(f64::to_radians(lat_deg)))
        + (lon_term_2 * f64::cos(3_f64 * f64::to_radians(lat_deg)))
        + (lon_term_3 * f64::cos(5_f64 * f64::to_radians(lat_deg)));
    (latlen, lonlen)
}

/// Nanoseconds to Microseconds
///
/// # Parameters
/// - `ns`: The nanoseconds value to be converted.
///
/// # Returns
/// - Newly converted microseconds value.
pub fn nano_to_micro_sec(ns: f64) -> f64 {
    ns / 1000_f64
}

/// Get string corresponding to UTC source.
///
/// # Parameters
/// - `utc_flags`: The utc flags to decipher and extract source.
///
/// # Returns
/// - The string corresponding to the utc source.
pub fn get_utc_source(utc_flags: u8) -> String {
    let source_str = match (utc_flags >> 3) & 0x3 {
        0 => FACTORY_DEFAULT,
        1 => NON_VOLATILE_MEMORY,
        2 => DECODED_THIS_SESSION,
        _ => UNKNOWN,
    };
    String::from(source_str)
}

/// Get UTC date time.
/// Code taken from ICBINS/src/runner.rs.
///
/// # Parameters
/// - `year`: The datetime year.
/// - `month`: The datetime month.
/// - `day`: The datetime day.
/// - `hours`: The datetime hours.
/// - `minutes`: The datetime minutes.
/// - `seconds`: The datetime seconds.
/// - `nanoseconds`: The datetime nanoseconds.
pub fn get_utc_time(
    year: i32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
    nanoseconds: u32,
) -> UtcDateTime {
    Utc.ymd(year, month, day)
        .and_hms_nano(hours, minutes, seconds, nanoseconds)
}

/// Return Utc datetime as date and seconds.
///
/// # Parameters
/// - `datetm`: The datetime to be converted into partial date and seconds strings.
///
/// # Returns:
/// - Partial datetime string and seconds/microseconds string.
pub fn datetime_2_str_utc(datetm: DateTime<Utc>) -> (String, f64) {
    let seconds = datetm.second() as f64 + datetm.nanosecond() as f64 / 1000_f64;
    (datetm.format("%Y-%m-%d %H:%M").to_string(), seconds)
}

/// Return Local datetime as date and seconds.
///
/// # Parameters
/// - `datetm`: The datetime to be converted into partial date and seconds strings.
///
/// # Returns:
/// - Partial datetime string and seconds/microseconds string.
pub fn datetime_2_str_local(datetm: DateTime<Local>) -> (String, f64) {
    let seconds = datetm.second() as f64 + datetm.nanosecond() as f64 / 1000_f64;
    (datetm.format("%Y-%m-%d %H:%M").to_string(), seconds)
}

/// Returns local time and gps time as a string date and precise seconds string.
///
/// # Parameters
/// - `week`: The week number.
/// - `tow`: The GPS time of week in seconds.
///
/// # Returns
/// - Local Date and Seconds and GPS Date and Seconds.
pub fn log_time_strings(
    week: Option<u16>,
    tow: f64,
) -> ((String, f64), (Option<String>, Option<f64>)) {
    let mut t_gps_date = None;
    let mut t_gps_secs = None;

    if let Some(wn) = week {
        if tow > 0_f64 {
            let t_gps = Utc.ymd(1980, 1, 6).and_hms(0, 0, 0)
                + Duration::weeks(wn as i64)
                + Duration::seconds(tow as i64);
            let (t_gps_date_, t_gps_secs_) = datetime_2_str_utc(t_gps);
            t_gps_date = Some(t_gps_date_);
            t_gps_secs = Some(t_gps_secs_);
        }
    }
    let local_t = Local::now();
    let (t_local_date, t_local_secs) = datetime_2_str_local(local_t);
    ((t_local_date, t_local_secs), (t_gps_date, t_gps_secs))
}

/// Convert millimeters to meters.
/// Taken from ICBINS/src/msg_utils.rs.
///
/// # Parameters
/// - `mm`: Value in millimeters.
///
/// # Returns
/// - Value in meters.
pub fn mm_to_m(mm: f64) -> f64 {
    mm / 1.0e+3_f64
}

/// Convert deciseconds to seconds.
///
/// # Parameters
/// - `ds`: Value in deciseconds.
///
/// # Returns
/// - Value in seconds.
pub fn decisec_to_sec(ds: f64) -> f64 {
    ds / 10_f64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_label_test() {
        let mut extra: HashMap<i16, i16> = HashMap::new();
        extra.insert(
            SignalCodes::CodeGloL2P as i16,
            SignalCodes::CodeGloL2P as i16,
        );

        let (code_lbl, freq_lbl, id_lbl) = get_label(
            (SignalCodes::CodeGloL2P, SignalCodes::CodeGloL2P as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2P_STR);
        assert_eq!(freq_lbl.unwrap(), "F+30");
        assert_eq!(id_lbl.unwrap(), "R30");

        let (code_lbl, freq_lbl, id_lbl) = get_label(
            (SignalCodes::CodeGloL2Of, SignalCodes::CodeGloL2Of as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2OF_STR);
        assert_eq!(freq_lbl.unwrap(), "F+04");
        assert_eq!(id_lbl.unwrap(), "R04");

        let (code_lbl, freq_lbl, id_lbl) = get_label(
            (SignalCodes::CodeSbasL5Q, SignalCodes::CodeSbasL5Q as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), SBAS_L5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "S 42");

        let (code_lbl, freq_lbl, id_lbl) = get_label(
            (SignalCodes::CodeBds3B5Q, SignalCodes::CodeBds3B5Q as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), BDS3_B5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "C48");

        let (code_lbl, freq_lbl, id_lbl) = get_label(
            (SignalCodes::CodeQzsL2Cx, SignalCodes::CodeQzsL2Cx as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), QZS_L2CX_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "J 37");

        let (code_lbl, freq_lbl, id_lbl) = get_label(
            (SignalCodes::CodeGalE8X, SignalCodes::CodeGalE8X as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GAL_E8X_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "E25");
    }

    #[test]
    fn meters_per_deg_test() {
        // Latitude range: [-90, 90]
        assert_eq!(
            meters_per_deg(-90_f64),
            (111693.9173, 6.839280692934427e-12)
        );
        assert_eq!(meters_per_deg(-45_f64), (111131.745, 78846.80572069259));
        assert_eq!(meters_per_deg(0_f64), (110574.2727, 111319.458));
        assert_eq!(meters_per_deg(45_f64), (111131.745, 78846.80572069259));
        assert_eq!(meters_per_deg(90_f64), (111693.9173, 6.839280692934427e-12));
    }

    #[test]
    fn nano_to_micro_sec_test() {
        assert_eq!(nano_to_micro_sec(1000_f64), 1_f64);
        assert_eq!(nano_to_micro_sec(1000000_f64), 1000_f64);
        assert_eq!(nano_to_micro_sec(0_f64), 0_f64);
        assert_eq!(nano_to_micro_sec(1337_f64), 1.337_f64);
    }

    #[test]
    fn get_utc_source_test() {
        assert_eq!(get_utc_source(5_u8), String::from(FACTORY_DEFAULT));
        assert_eq!(get_utc_source(8_u8), String::from(NON_VOLATILE_MEMORY));
        assert_eq!(get_utc_source(16_u8), String::from(DECODED_THIS_SESSION));
        assert_eq!(get_utc_source(255_u8), String::from(UNKNOWN));
    }
}
