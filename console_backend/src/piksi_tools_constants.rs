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

//! ```cargo
//! [package]
//! edition = "2018"
//! [dependencies]
//! lazy_static = "*"
//! ```

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::double_parens)] // https://github.com/adsharma/py2many/issues/17
#![allow(clippy::map_identity)]
#![allow(clippy::needless_return)]
#![allow(clippy::print_literal)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::redundant_static_lifetimes)] // https://github.com/adsharma/py2many/issues/266
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::useless_vec)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections;
use std::collections::HashMap;
use std::collections::HashSet;

pub const CODE_GPS_L1CA: i32 = 0;
pub const CODE_GPS_L2CM: i32 = 1;
pub const CODE_GPS_L2CL: i32 = 7;
pub const CODE_GPS_L2CX: i32 = 8;
pub const CODE_GPS_L1P: i32 = 5;
pub const CODE_GPS_L2P: i32 = 6;
pub const CODE_GPS_L5I: i32 = 9;
pub const CODE_GPS_L5Q: i32 = 10;
pub const CODE_GPS_L5X: i32 = 11;
pub const CODE_GPS_L1CI: i32 = 56;
pub const CODE_GPS_L1CQ: i32 = 57;
pub const CODE_GPS_L1CX: i32 = 58;
pub const CODE_AUX_GPS: i32 = 59;
pub const CODE_GLO_L1OF: i32 = 3;
pub const CODE_GLO_L2OF: i32 = 4;
pub const CODE_GLO_L1P: i32 = 29;
pub const CODE_GLO_L2P: i32 = 30;
pub const CODE_SBAS_L1CA: i32 = 2;
pub const CODE_SBAS_L5I: i32 = 41;
pub const CODE_SBAS_L5Q: i32 = 42;
pub const CODE_SBAS_L5X: i32 = 43;
pub const CODE_AUX_SBAS: i32 = 60;
pub const CODE_BDS2_B1: i32 = 12;
pub const CODE_BDS2_B2: i32 = 13;
pub const CODE_BDS3_B1CI: i32 = 44;
pub const CODE_BDS3_B1CQ: i32 = 45;
pub const CODE_BDS3_B1CX: i32 = 46;
pub const CODE_BDS3_B5I: i32 = 47;
pub const CODE_BDS3_B5Q: i32 = 48;
pub const CODE_BDS3_B5X: i32 = 49;
pub const CODE_BDS3_B7I: i32 = 50;
pub const CODE_BDS3_B7Q: i32 = 51;
pub const CODE_BDS3_B7X: i32 = 52;
pub const CODE_BDS3_B3I: i32 = 53;
pub const CODE_BDS3_B3Q: i32 = 54;
pub const CODE_BDS3_B3X: i32 = 55;
pub const CODE_GAL_E1B: i32 = 14;
pub const CODE_GAL_E1C: i32 = 15;
pub const CODE_GAL_E1X: i32 = 16;
pub const CODE_GAL_E6B: i32 = 17;
pub const CODE_GAL_E6C: i32 = 18;
pub const CODE_GAL_E6X: i32 = 19;
pub const CODE_GAL_E7I: i32 = 20;
pub const CODE_GAL_E7Q: i32 = 21;
pub const CODE_GAL_E7X: i32 = 22;
pub const CODE_GAL_E8I: i32 = 23;
pub const CODE_GAL_E8Q: i32 = 24;
pub const CODE_GAL_E8X: i32 = 25;
pub const CODE_GAL_E5I: i32 = 26;
pub const CODE_GAL_E5Q: i32 = 27;
pub const CODE_GAL_E5X: i32 = 28;
pub const CODE_AUX_GAL: i32 = 61;
pub const CODE_QZS_L1CA: i32 = 31;
pub const CODE_QZS_L1CI: i32 = 32;
pub const CODE_QZS_L1CQ: i32 = 33;
pub const CODE_QZS_L1CX: i32 = 34;
pub const CODE_QZS_L2CM: i32 = 35;
pub const CODE_QZS_L2CL: i32 = 36;
pub const CODE_QZS_L2CX: i32 = 37;
pub const CODE_QZS_L5I: i32 = 38;
pub const CODE_QZS_L5Q: i32 = 39;
pub const CODE_QZS_L5X: i32 = 40;
pub const CODE_AUX_QZS: i32 = 62;
pub const SUPPORTED_CODES: &[i32; 60] = &[
    CODE_GPS_L1CA,
    CODE_GPS_L2CM,
    CODE_GPS_L2CL,
    CODE_GPS_L2CX,
    CODE_GPS_L1P,
    CODE_GPS_L2P,
    CODE_GPS_L5I,
    CODE_GPS_L5Q,
    CODE_GPS_L5X,
    CODE_GPS_L1CI,
    CODE_GPS_L1CQ,
    CODE_GPS_L1CX,
    CODE_AUX_GPS,
    CODE_GLO_L1OF,
    CODE_GLO_L2OF,
    CODE_GLO_L1P,
    CODE_GLO_L2P,
    CODE_SBAS_L1CA,
    CODE_SBAS_L5I,
    CODE_SBAS_L5Q,
    CODE_SBAS_L5X,
    CODE_AUX_SBAS,
    CODE_BDS2_B1,
    CODE_BDS2_B2,
    CODE_BDS3_B1CI,
    CODE_BDS3_B1CQ,
    CODE_BDS3_B1CX,
    CODE_BDS3_B5I,
    CODE_BDS3_B5Q,
    CODE_BDS3_B5X,
    CODE_BDS3_B7I,
    CODE_BDS3_B7Q,
    CODE_BDS3_B7X,
    CODE_BDS3_B3I,
    CODE_BDS3_B3Q,
    CODE_BDS3_B3X,
    CODE_GAL_E1B,
    CODE_GAL_E1C,
    CODE_GAL_E1X,
    CODE_GAL_E5I,
    CODE_GAL_E5Q,
    CODE_GAL_E5X,
    CODE_GAL_E6B,
    CODE_GAL_E6C,
    CODE_GAL_E6X,
    CODE_GAL_E7I,
    CODE_GAL_E7Q,
    CODE_GAL_E7X,
    CODE_GAL_E8I,
    CODE_GAL_E8Q,
    CODE_GAL_E8X,
    CODE_AUX_GAL,
    CODE_QZS_L1CA,
    CODE_QZS_L2CM,
    CODE_QZS_L2CL,
    CODE_QZS_L2CX,
    CODE_QZS_L5I,
    CODE_QZS_L5Q,
    CODE_QZS_L5X,
    CODE_AUX_QZS,
];
lazy_static! {
    pub static ref GUI_CODES: HashMap<&'static str, Vec<i32>> = [
        (
            "GPS",
            vec![
                CODE_GPS_L1CA,
                CODE_GPS_L2CM,
                CODE_GPS_L2CL,
                CODE_GPS_L2CX,
                CODE_GPS_L1P,
                CODE_GPS_L2P,
                CODE_GPS_L5I,
                CODE_GPS_L5Q,
                CODE_GPS_L5X,
                CODE_GPS_L1CI,
                CODE_GPS_L1CQ,
                CODE_GPS_L1CX,
                CODE_AUX_GPS
            ]
        ),
        (
            "GLO",
            vec![CODE_GLO_L1OF, CODE_GLO_L2OF, CODE_GLO_L1P, CODE_GLO_L2P]
        ),
        (
            "GAL",
            vec![
                CODE_GAL_E1B,
                CODE_GAL_E1C,
                CODE_GAL_E1X,
                CODE_GAL_E6B,
                CODE_GAL_E6C,
                CODE_GAL_E6X,
                CODE_GAL_E7I,
                CODE_GAL_E7Q,
                CODE_GAL_E7X,
                CODE_GAL_E8I,
                CODE_GAL_E8Q,
                CODE_GAL_E8X,
                CODE_GAL_E5I,
                CODE_GAL_E5Q,
                CODE_GAL_E5X,
                CODE_AUX_GAL
            ]
        ),
        (
            "QZS",
            vec![
                CODE_QZS_L1CA,
                CODE_QZS_L2CM,
                CODE_QZS_L2CL,
                CODE_QZS_L2CX,
                CODE_QZS_L5I,
                CODE_QZS_L5Q,
                CODE_QZS_L5X,
                CODE_AUX_QZS
            ]
        ),
        (
            "BDS",
            vec![
                CODE_BDS2_B1,
                CODE_BDS2_B2,
                CODE_BDS3_B1CI,
                CODE_BDS3_B1CQ,
                CODE_BDS3_B1CX,
                CODE_BDS3_B5I,
                CODE_BDS3_B5Q,
                CODE_BDS3_B5X,
                CODE_BDS3_B7I,
                CODE_BDS3_B7Q,
                CODE_BDS3_B7X,
                CODE_BDS3_B3I,
                CODE_BDS3_B3Q,
                CODE_BDS3_B3X
            ]
        ),
        (
            "SBAS",
            vec![
                CODE_SBAS_L1CA,
                CODE_SBAS_L5I,
                CODE_SBAS_L5Q,
                CODE_SBAS_L5X,
                CODE_AUX_SBAS
            ]
        )
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
pub const GPS_L1CA_STR: &'static str = "GPS L1CA";
pub const GPS_L2CM_STR: &'static str = "GPS L2C M";
pub const GPS_L2CL_STR: &'static str = "GPS L2C L";
pub const GPS_L2CX_STR: &'static str = "GPS L2C M+L";
pub const GPS_L1P_STR: &'static str = "GPS L1P";
pub const GPS_L2P_STR: &'static str = "GPS L2P";
pub const GPS_L5I_STR: &'static str = "GPS L5 I";
pub const GPS_L5Q_STR: &'static str = "GPS L5 Q";
pub const GPS_L5X_STR: &'static str = "GPS L5 I+Q";
pub const GPS_AUX_STR: &'static str = "AUX GPS L1";
pub const SBAS_L1_STR: &'static str = "SBAS L1";
pub const SBAS_L5I_STR: &'static str = "SBAS L5 I";
pub const SBAS_L5Q_STR: &'static str = "SBAS L5 Q";
pub const SBAS_L5X_STR: &'static str = "SBAS L5 I+Q";
pub const SBAS_AUX_STR: &'static str = "AUX SBAS L1";
pub const GLO_L1OF_STR: &'static str = "GLO L1OF";
pub const GLO_L2OF_STR: &'static str = "GLO L2OF";
pub const GLO_L1P_STR: &'static str = "GLO L1P";
pub const GLO_L2P_STR: &'static str = "GLO L2P";
pub const BDS2_B1_STR: &'static str = "BDS2 B1 I";
pub const BDS2_B2_STR: &'static str = "BDS2 B2 I";
pub const BDS3_B1CI_STR: &'static str = "BDS3 B1C I";
pub const BDS3_B1CQ_STR: &'static str = "BDS3 B1C Q";
pub const BDS3_B1CX_STR: &'static str = "BDS3 B1C I+Q";
pub const BDS3_B5I_STR: &'static str = "BDS3 B2a I";
pub const BDS3_B5Q_STR: &'static str = "BDS3 B2a Q";
pub const BDS3_B5X_STR: &'static str = "BDS3 B2a X";
pub const BDS3_B7I_STR: &'static str = "BDS3 B2b I";
pub const BDS3_B7Q_STR: &'static str = "BDS3 B2b Q";
pub const BDS3_B7X_STR: &'static str = "BDS3 B2b X";
pub const BDS3_B3I_STR: &'static str = "BDS3 B3I";
pub const BDS3_B3Q_STR: &'static str = "BDS3 B3Q";
pub const BDS3_B3X_STR: &'static str = "BDS3 B3X";
pub const BDS3_AUX_STR: &'static str = "AUX BDS B1";
pub const GAL_E1B_STR: &'static str = "GAL E1 B";
pub const GAL_E1C_STR: &'static str = "GAL E1 C";
pub const GAL_E1X_STR: &'static str = "GAL E1 B+C";
pub const GAL_E5I_STR: &'static str = "GAL E5a I";
pub const GAL_E5Q_STR: &'static str = "GAL E5a Q";
pub const GAL_E5X_STR: &'static str = "GAL E5a I+Q";
pub const GAL_E6B_STR: &'static str = "GAL E6 B";
pub const GAL_E6C_STR: &'static str = "GAL E6 C";
pub const GAL_E6X_STR: &'static str = "GAL E6 B+C";
pub const GAL_E8I_STR: &'static str = "GAL E5ab I";
pub const GAL_E8Q_STR: &'static str = "GAL E5ab Q";
pub const GAL_E8X_STR: &'static str = "GAL E5ab I+Q";
pub const GAL_E7I_STR: &'static str = "GAL E5b I";
pub const GAL_E7Q_STR: &'static str = "GAL E5b Q";
pub const GAL_E7X_STR: &'static str = "GAL E5b I+Q";
pub const GAL_AUX_STR: &'static str = "AUX GAL E1";
pub const QZS_L1CA_STR: &'static str = "QZS L1CA";
pub const QZS_L2CM_STR: &'static str = "QZS L2C M";
pub const QZS_L2CL_STR: &'static str = "QZS L2C L";
pub const QZS_L2CX_STR: &'static str = "QZS L2C M+L";
pub const QZS_L5I_STR: &'static str = "QZS L5 I";
pub const QZS_L5Q_STR: &'static str = "QZS L5 Q";
pub const QZS_L5X_STR: &'static str = "QZS L5 I+Q";
pub const QZS_AUX_STR: &'static str = "AUX QZS L1";
lazy_static! {
    pub static ref CODE_TO_STR_MAP: HashMap<i32, &'static str> = [
        (CODE_GPS_L1CA, GPS_L1CA_STR),
        (CODE_GPS_L2CM, GPS_L2CM_STR),
        (CODE_GPS_L2CL, GPS_L2CL_STR),
        (CODE_GPS_L2CX, GPS_L2CX_STR),
        (CODE_GPS_L1P, GPS_L1P_STR),
        (CODE_GPS_L2P, GPS_L2P_STR),
        (CODE_GPS_L5I, GPS_L5I_STR),
        (CODE_GPS_L5Q, GPS_L5Q_STR),
        (CODE_GPS_L5X, GPS_L5X_STR),
        (CODE_AUX_GPS, GPS_AUX_STR),
        (CODE_GLO_L1OF, GLO_L1OF_STR),
        (CODE_GLO_L2OF, GLO_L2OF_STR),
        (CODE_GLO_L1P, GLO_L1P_STR),
        (CODE_GLO_L2P, GLO_L2P_STR),
        (CODE_SBAS_L1CA, SBAS_L1_STR),
        (CODE_SBAS_L5I, SBAS_L5I_STR),
        (CODE_SBAS_L5Q, SBAS_L5Q_STR),
        (CODE_SBAS_L5X, SBAS_L5X_STR),
        (CODE_AUX_SBAS, SBAS_AUX_STR),
        (CODE_BDS2_B1, BDS2_B1_STR),
        (CODE_BDS2_B2, BDS2_B2_STR),
        (CODE_BDS3_B1CI, BDS3_B1CI_STR),
        (CODE_BDS3_B1CQ, BDS3_B1CQ_STR),
        (CODE_BDS3_B1CX, BDS3_B1CX_STR),
        (CODE_BDS3_B5I, BDS3_B5I_STR),
        (CODE_BDS3_B5Q, BDS3_B5Q_STR),
        (CODE_BDS3_B5X, BDS3_B5X_STR),
        (CODE_BDS3_B7I, BDS3_B7I_STR),
        (CODE_BDS3_B7Q, BDS3_B7Q_STR),
        (CODE_BDS3_B7X, BDS3_B7X_STR),
        (CODE_BDS3_B3I, BDS3_B3I_STR),
        (CODE_BDS3_B3Q, BDS3_B3Q_STR),
        (CODE_BDS3_B3X, BDS3_B3X_STR),
        (CODE_GAL_E1B, GAL_E1B_STR),
        (CODE_GAL_E1C, GAL_E1C_STR),
        (CODE_GAL_E1X, GAL_E1X_STR),
        (CODE_GAL_E6B, GAL_E6B_STR),
        (CODE_GAL_E6C, GAL_E6C_STR),
        (CODE_GAL_E6X, GAL_E6X_STR),
        (CODE_GAL_E7I, GAL_E7I_STR),
        (CODE_GAL_E7Q, GAL_E7Q_STR),
        (CODE_GAL_E7X, GAL_E7X_STR),
        (CODE_GAL_E8I, GAL_E8I_STR),
        (CODE_GAL_E8Q, GAL_E8Q_STR),
        (CODE_GAL_E8X, GAL_E8X_STR),
        (CODE_GAL_E5I, GAL_E5I_STR),
        (CODE_GAL_E5Q, GAL_E5Q_STR),
        (CODE_GAL_E5X, GAL_E5X_STR),
        (CODE_AUX_GAL, GAL_AUX_STR),
        (CODE_QZS_L1CA, QZS_L1CA_STR),
        (CODE_QZS_L2CM, QZS_L2CM_STR),
        (CODE_QZS_L2CL, QZS_L2CL_STR),
        (CODE_QZS_L2CX, QZS_L2CX_STR),
        (CODE_QZS_L5I, QZS_L5I_STR),
        (CODE_QZS_L5Q, QZS_L5Q_STR),
        (CODE_QZS_L5X, QZS_L5X_STR)
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
lazy_static! {
    pub static ref STR_TO_CODE_MAP: HashMap<&'static str, i32> = [
        (GPS_L1CA_STR, CODE_GPS_L1CA),
        (GPS_L2CM_STR, CODE_GPS_L2CM),
        (GPS_L2CL_STR, CODE_GPS_L2CL),
        (GPS_L2CX_STR, CODE_GPS_L2CX),
        (GPS_L5I_STR, CODE_GPS_L5I),
        (GPS_L5Q_STR, CODE_GPS_L5Q),
        (GPS_L5X_STR, CODE_GPS_L5X),
        (GPS_L1P_STR, CODE_GPS_L1P),
        (GPS_L2P_STR, CODE_GPS_L2P),
        (GPS_AUX_STR, CODE_AUX_GPS),
        (SBAS_L1_STR, CODE_SBAS_L1CA),
        (SBAS_L5I_STR, CODE_SBAS_L5I),
        (SBAS_L5Q_STR, CODE_SBAS_L5Q),
        (SBAS_L5X_STR, CODE_SBAS_L5X),
        (SBAS_AUX_STR, CODE_AUX_SBAS),
        (GLO_L1OF_STR, CODE_GLO_L1OF),
        (GLO_L2OF_STR, CODE_GLO_L2OF),
        (GLO_L1P_STR, CODE_GLO_L1P),
        (GLO_L2P_STR, CODE_GLO_L2P),
        (BDS2_B1_STR, CODE_BDS2_B1),
        (BDS2_B2_STR, CODE_BDS2_B2),
        (BDS3_B1CI_STR, CODE_BDS3_B1CI),
        (BDS3_B1CQ_STR, CODE_BDS3_B1CQ),
        (BDS3_B1CX_STR, CODE_BDS3_B1CX),
        (BDS3_B5I_STR, CODE_BDS3_B5I),
        (BDS3_B5Q_STR, CODE_BDS3_B5Q),
        (BDS3_B5X_STR, CODE_BDS3_B5X),
        (BDS3_B7I_STR, CODE_BDS3_B7I),
        (BDS3_B7Q_STR, CODE_BDS3_B7Q),
        (BDS3_B7X_STR, CODE_BDS3_B7X),
        (BDS3_B3I_STR, CODE_BDS3_B3I),
        (BDS3_B3Q_STR, CODE_BDS3_B3Q),
        (BDS3_B3X_STR, CODE_BDS3_B3X),
        (GAL_E1B_STR, CODE_GAL_E1B),
        (GAL_E1C_STR, CODE_GAL_E1C),
        (GAL_E1X_STR, CODE_GAL_E1X),
        (GAL_E5I_STR, CODE_GAL_E5I),
        (GAL_E5Q_STR, CODE_GAL_E5Q),
        (GAL_E5X_STR, CODE_GAL_E5X),
        (GAL_E6B_STR, CODE_GAL_E6B),
        (GAL_E6C_STR, CODE_GAL_E6C),
        (GAL_E6X_STR, CODE_GAL_E6X),
        (GAL_E7I_STR, CODE_GAL_E7I),
        (GAL_E7Q_STR, CODE_GAL_E7Q),
        (GAL_E7X_STR, CODE_GAL_E7X),
        (GAL_E8I_STR, CODE_GAL_E8I),
        (GAL_E8Q_STR, CODE_GAL_E8Q),
        (GAL_E8X_STR, CODE_GAL_E8X),
        (GAL_AUX_STR, CODE_AUX_GAL),
        (QZS_L1CA_STR, CODE_QZS_L1CA),
        (QZS_L2CM_STR, CODE_QZS_L2CM),
        (QZS_L2CL_STR, CODE_QZS_L2CL),
        (QZS_L2CX_STR, CODE_QZS_L2CX),
        (QZS_L5I_STR, CODE_QZS_L5I),
        (QZS_L5Q_STR, CODE_QZS_L5Q),
        (QZS_L5X_STR, CODE_QZS_L5X),
        (QZS_AUX_STR, CODE_AUX_QZS)
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
pub const CODE_NOT_AVAILABLE: &'static str = "N/A";
pub const EMPTY_STR: &'static str = "--";
pub const SBAS_MODE: i32 = 6;
pub const DR_MODE: i32 = 5;
pub const FIXED_MODE: i32 = 4;
pub const FLOAT_MODE: i32 = 3;
pub const DGNSS_MODE: i32 = 2;
pub const SPP_MODE: i32 = 1;
pub const NO_FIX_MODE: i32 = 0;
pub const DIFFERENTIAL_MODES: &[i32; 3] = &[FIXED_MODE, FLOAT_MODE, DGNSS_MODE];
pub const RTK_MODES: &[i32; 2] = &[FIXED_MODE, FLOAT_MODE];
lazy_static! {
    pub static ref mode_dict: HashMap<i32, &'static str> = [
        (NO_FIX_MODE, "No Fix"),
        (SPP_MODE, "SPP"),
        (DGNSS_MODE, "DGPS"),
        (FLOAT_MODE, "Float RTK"),
        (FIXED_MODE, "Fixed RTK"),
        (DR_MODE, "Dead Reckoning"),
        (SBAS_MODE, "SBAS")
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
lazy_static! {
    pub static ref pos_mode_dict: HashMap<i32, &'static str> = [
        (NO_FIX_MODE, "No Fix"),
        (SPP_MODE, "SPP"),
        (DGNSS_MODE, "DGPS"),
        (FLOAT_MODE, "RTK"),
        (FIXED_MODE, "RTK"),
        (DR_MODE, "DR"),
        (SBAS_MODE, "SBAS")
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
lazy_static! {
    pub static ref rtk_mode_dict: HashMap<i32, &'static str> =
        [(FLOAT_MODE, "Float"), (FIXED_MODE, "Fixed")]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
}
pub const AWAITING_INITIALIZATION: i32 = 0;
pub const DYNAMICALLY_ALIGNING: i32 = 1;
pub const READY: i32 = 2;
pub const GNSS_OUTAGE_MAX: i32 = 3;
lazy_static! {
    pub static ref ins_mode_dict: HashMap<i32, &'static str> = [
        (AWAITING_INITIALIZATION, "Init"),
        (DYNAMICALLY_ALIGNING, "Align"),
        (READY, "Ready"),
        (GNSS_OUTAGE_MAX, "MaxDur")
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
pub const IMU_DATA_ERROR: i32 = 1;
pub const IMU_LICENSE_ERROR: i32 = 2;
pub const IMU_CALIBRATION_ERROR: i32 = 3;
lazy_static! {
    pub static ref ins_error_dict: HashMap<i32, &'static str> = [
        (IMU_DATA_ERROR, "Data Error"),
        (IMU_LICENSE_ERROR, "License Error"),
        (IMU_CALIBRATION_ERROR, "Cal Error")
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
pub const SMOOTHPOSE: i32 = 0;
pub const DR_RUNNER: i32 = 1;
lazy_static! {
    pub static ref ins_type_dict: HashMap<i32, &'static str> =
        [(SMOOTHPOSE, "SP-"), (DR_RUNNER, "")]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
}
pub fn code_to_str(code: i32) -> &'static str {
    if CODE_TO_STR_MAP.keys().any(|&x| x == code) {
        return CODE_TO_STR_MAP[&code] as &'static str;
    } else {
        return CODE_NOT_AVAILABLE as &'static str;
    }
}

lazy_static! {
    pub static ref gps_codes: HashSet<i32> = [
        CODE_GPS_L1CA,
        CODE_GPS_L2CM,
        CODE_GPS_L2CL,
        CODE_GPS_L2CX,
        CODE_GPS_L1P,
        CODE_GPS_L2P,
        CODE_GPS_L5I,
        CODE_GPS_L5Q,
        CODE_GPS_L5X,
        CODE_AUX_GPS
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();
}
pub fn code_is_gps(code: i32) -> bool {
    return gps_codes.iter().any(|&x| x == code);
}

lazy_static! {
    pub static ref glo_codes: HashSet<i32> =
        [CODE_GLO_L1OF, CODE_GLO_L2OF, CODE_GLO_L1P, CODE_GLO_L2P]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
}
pub fn code_is_glo(code: i32) -> bool {
    return glo_codes.iter().any(|&x| x == code);
}

lazy_static! {
    pub static ref sbas_codes: HashSet<i32> = [
        CODE_SBAS_L1CA,
        CODE_SBAS_L5I,
        CODE_SBAS_L5Q,
        CODE_SBAS_L5X,
        CODE_AUX_SBAS
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();
}
pub fn code_is_sbas(code: i32) -> bool {
    return sbas_codes.iter().any(|&x| x == code);
}

lazy_static! {
    pub static ref bds_codes: HashSet<i32> = [
        CODE_BDS2_B1,
        CODE_BDS2_B2,
        CODE_BDS3_B1CI,
        CODE_BDS3_B1CQ,
        CODE_BDS3_B1CX,
        CODE_BDS3_B5I,
        CODE_BDS3_B5Q,
        CODE_BDS3_B5X,
        CODE_BDS3_B3I,
        CODE_BDS3_B3Q,
        CODE_BDS3_B3X,
        CODE_BDS3_B7I,
        CODE_BDS3_B7Q,
        CODE_BDS3_B7X
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();
}
pub fn code_is_bds(code: i32) -> bool {
    return bds_codes.iter().any(|&x| x == code);
}

lazy_static! {
    pub static ref gal_codes: HashSet<i32> = [
        CODE_GAL_E1B,
        CODE_GAL_E1C,
        CODE_GAL_E1X,
        CODE_GAL_E6B,
        CODE_GAL_E6C,
        CODE_GAL_E6X,
        CODE_GAL_E7I,
        CODE_GAL_E7Q,
        CODE_GAL_E7X,
        CODE_GAL_E8I,
        CODE_GAL_E8Q,
        CODE_GAL_E8X,
        CODE_GAL_E5I,
        CODE_GAL_E5Q,
        CODE_GAL_E5X,
        CODE_AUX_GAL
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();
}
pub fn code_is_galileo(code: i32) -> bool {
    return gal_codes.iter().any(|&x| x == code);
}

lazy_static! {
    pub static ref qzss_codes: HashSet<i32> = [
        CODE_QZS_L1CA,
        CODE_QZS_L2CM,
        CODE_QZS_L2CL,
        CODE_QZS_L2CX,
        CODE_QZS_L5I,
        CODE_QZS_L5Q,
        CODE_QZS_L5X,
        CODE_AUX_QZS
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();
}
pub fn code_is_qzss(code: i32) -> bool {
    return qzss_codes.iter().any(|&x| x == code);
}
