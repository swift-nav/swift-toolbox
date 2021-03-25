use std::collections::HashMap;

// Tracking Tab constants.
pub const NUM_POINTS: usize = 200;
pub const NUM_SATELLITES: usize = 60;
pub const TRK_RATE: f64 = 2.0;
pub const GLO_SLOT_SAT_MAX: u8 = 90;
pub const GLO_FCN_OFFSET: i16 = 8;
pub const SBAS_NEG_OFFSET: i16 = 120;
pub const QZSS_NEG_OFFSET: i16 = 193;
pub const SNR_THRESHOLD: f64 = 15.0;
pub const TRACKING_SIGNALS_PLOT_MAX: f64 = 60.0;
pub const GUI_UPDATE_PERIOD: f64 = 0.2;

pub const CODE_GPS_L1CA: u8 = 0;
pub const CODE_GPS_L2CM: u8 = 1;
pub const CODE_GPS_L2CL: u8 = 7;
pub const CODE_GPS_L2CX: u8 = 8;
pub const CODE_GPS_L1P: u8 = 5;
pub const CODE_GPS_L2P: u8 = 6;
pub const CODE_GPS_L5I: u8 = 9;
pub const CODE_GPS_L5Q: u8 = 10;
pub const CODE_GPS_L5X: u8 = 11;
pub const CODE_GPS_L1CI: u8 = 56;
pub const CODE_GPS_L1CQ: u8 = 57;
pub const CODE_GPS_L1CX: u8 = 58;
pub const CODE_AUX_GPS: u8 = 59;

pub const CODE_GLO_L1OF: u8 = 3;
pub const CODE_GLO_L2OF: u8 = 4;
pub const CODE_GLO_L1P: u8 = 29;
pub const CODE_GLO_L2P: u8 = 30;

pub const CODE_SBAS_L1CA: u8 = 2;
pub const CODE_SBAS_L5I: u8 = 41;
pub const CODE_SBAS_L5Q: u8 = 42;
pub const CODE_SBAS_L5X: u8 = 43;
pub const CODE_AUX_SBAS: u8 = 60;

pub const CODE_BDS2_B1: u8 = 12;
pub const CODE_BDS2_B2: u8 = 13;
pub const CODE_BDS3_B1CI: u8 = 44;
pub const CODE_BDS3_B1CQ: u8 = 45;
pub const CODE_BDS3_B1CX: u8 = 46;
pub const CODE_BDS3_B5I: u8 = 47;
pub const CODE_BDS3_B5Q: u8 = 48;
pub const CODE_BDS3_B5X: u8 = 49;
pub const CODE_BDS3_B7I: u8 = 50;
pub const CODE_BDS3_B7Q: u8 = 51;
pub const CODE_BDS3_B7X: u8 = 52;
pub const CODE_BDS3_B3I: u8 = 53;
pub const CODE_BDS3_B3Q: u8 = 54;
pub const CODE_BDS3_B3X: u8 = 55;

pub const CODE_GAL_E1B: u8 = 14;
pub const CODE_GAL_E1C: u8 = 15;
pub const CODE_GAL_E1X: u8 = 16;
pub const CODE_GAL_E6B: u8 = 17;
pub const CODE_GAL_E6C: u8 = 18;
pub const CODE_GAL_E6X: u8 = 19;
pub const CODE_GAL_E7I: u8 = 20;
pub const CODE_GAL_E7Q: u8 = 21;
pub const CODE_GAL_E7X: u8 = 22;
pub const CODE_GAL_E8I: u8 = 23;
pub const CODE_GAL_E8Q: u8 = 24;
pub const CODE_GAL_E8X: u8 = 25;
pub const CODE_GAL_E5I: u8 = 26;
pub const CODE_GAL_E5Q: u8 = 27;
pub const CODE_GAL_E5X: u8 = 28;
pub const CODE_AUX_GAL: u8 = 61;

pub const CODE_QZS_L1CA: u8 = 31;
pub const CODE_QZS_L1CI: u8 = 32;
pub const CODE_QZS_L1CQ: u8 = 33;
pub const CODE_QZS_L1CX: u8 = 34;
pub const CODE_QZS_L2CM: u8 = 35;
pub const CODE_QZS_L2CL: u8 = 36;
pub const CODE_QZS_L2CX: u8 = 37;
pub const CODE_QZS_L5I: u8 = 38;
pub const CODE_QZS_L5Q: u8 = 39;
pub const CODE_QZS_L5X: u8 = 40;
pub const CODE_AUX_QZS: u8 = 62;

pub fn code_is_gps(code: u8) -> bool {
    matches!(
        code,
        CODE_GPS_L1CA
            | CODE_GPS_L2CM
            | CODE_GPS_L2CL
            | CODE_GPS_L2CX
            | CODE_GPS_L1P
            | CODE_GPS_L2P
            | CODE_GPS_L5I
            | CODE_GPS_L5Q
            | CODE_GPS_L5X
            | CODE_AUX_GPS
    )
}

pub fn code_is_glo(code: u8) -> bool {
    matches!(
        code,
        CODE_GLO_L1OF | CODE_GLO_L2OF | CODE_GLO_L1P | CODE_GLO_L2P
    )
}

pub fn code_is_sbas(code: u8) -> bool {
    matches!(
        code,
        CODE_SBAS_L1CA | CODE_SBAS_L5I | CODE_SBAS_L5Q | CODE_SBAS_L5X | CODE_AUX_SBAS
    )
}

pub fn code_is_bds(code: u8) -> bool {
    matches!(
        code,
        CODE_BDS2_B1
            | CODE_BDS2_B2
            | CODE_BDS3_B1CI
            | CODE_BDS3_B1CQ
            | CODE_BDS3_B1CX
            | CODE_BDS3_B5I
            | CODE_BDS3_B5Q
            | CODE_BDS3_B5X
            | CODE_BDS3_B3I
            | CODE_BDS3_B3Q
            | CODE_BDS3_B3X
            | CODE_BDS3_B7I
            | CODE_BDS3_B7Q
            | CODE_BDS3_B7X
    )
}

pub fn code_is_galileo(code: u8) -> bool {
    matches!(
        code,
        CODE_GAL_E1B
            | CODE_GAL_E1C
            | CODE_GAL_E1X
            | CODE_GAL_E6B
            | CODE_GAL_E6C
            | CODE_GAL_E6X
            | CODE_GAL_E7I
            | CODE_GAL_E7Q
            | CODE_GAL_E7X
            | CODE_GAL_E8I
            | CODE_GAL_E8Q
            | CODE_GAL_E8X
            | CODE_GAL_E5I
            | CODE_GAL_E5Q
            | CODE_GAL_E5X
            | CODE_AUX_GAL
    )
}

pub fn code_is_qzss(code: u8) -> bool {
    matches!(
        code,
        CODE_QZS_L1CA
            | CODE_QZS_L2CM
            | CODE_QZS_L2CL
            | CODE_QZS_L2CX
            | CODE_QZS_L5I
            | CODE_QZS_L5Q
            | CODE_QZS_L5X
            | CODE_AUX_QZS
    )
}

pub fn get_label(
    key: (u8, i16),
    extra: &HashMap<i16, i16>,
) -> (Option<String>, Option<String>, Option<String>) {
    let (code, sat) = key;
    let code_lbl = Some(code_to_str_map(code).to_string());
    let mut freq_lbl = None;
    let id_lbl;

    if code_is_glo(code) {
        let freq_lbl_ = format!("F+{:02}", sat);
        freq_lbl = Some(freq_lbl_);
        if extra.contains_key(&sat) {
            id_lbl = Some(format!("R{:<02}", extra[&sat]));
        } else {
            id_lbl = Some(format!("R{:<02}", sat));
        }
    } else if code_is_sbas(code) {
        id_lbl = Some(format!("S{: >3}", sat));
    } else if code_is_bds(code) {
        id_lbl = Some(format!("C{:0>2}", sat));
    } else if code_is_qzss(code) {
        id_lbl = Some(format!("J{: >3}", sat));
    } else if code_is_galileo(code) {
        id_lbl = Some(format!("E{:0>2}", sat));
    } else {
        id_lbl = Some(format!("G{:0>2}", sat));
    }
    (code_lbl, freq_lbl, id_lbl)
}

pub static SUPPORTED_CODES: &[u8] = &[
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

pub const GPS: &str = "GPS";
pub const GLO: &str = "GLO";
pub const GAL: &str = "GAL";
pub const QZS: &str = "QZS";
pub const BDS: &str = "BDS";
pub const SBAS: &str = "SBAS";

pub fn gui_codes(sat_type: &str) -> &'static [u8] {
    match sat_type {
        GPS => &[
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
        ],
        GLO => &[CODE_GLO_L1OF, CODE_GLO_L2OF, CODE_GLO_L1P, CODE_GLO_L2P],
        GAL => &[
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
            CODE_AUX_GAL,
        ],
        QZS => &[
            CODE_QZS_L1CA,
            CODE_QZS_L2CM,
            CODE_QZS_L2CL,
            CODE_QZS_L2CX,
            CODE_QZS_L5I,
            CODE_QZS_L5Q,
            CODE_QZS_L5X,
            CODE_AUX_QZS,
        ],
        BDS => &[
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
        ],
        SBAS => &[
            CODE_SBAS_L1CA,
            CODE_SBAS_L5I,
            CODE_SBAS_L5Q,
            CODE_SBAS_L5X,
            CODE_AUX_SBAS,
        ],
        _ => panic!("Unknown GUI Code."),
    }
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

pub fn code_to_str_map(sat_code: u8) -> &'static str {
    match sat_code {
        CODE_GPS_L1CA => GPS_L1CA_STR,
        CODE_GPS_L2CM => GPS_L2CM_STR,
        CODE_GPS_L2CL => GPS_L2CL_STR,
        CODE_GPS_L2CX => GPS_L2CX_STR,
        CODE_GPS_L1P => GPS_L1P_STR,
        CODE_GPS_L2P => GPS_L2P_STR,
        CODE_GPS_L5I => GPS_L5I_STR,
        CODE_GPS_L5Q => GPS_L5Q_STR,
        CODE_GPS_L5X => GPS_L5X_STR,
        CODE_AUX_GPS => GPS_AUX_STR,

        CODE_GLO_L1OF => GLO_L1OF_STR,
        CODE_GLO_L2OF => GLO_L2OF_STR,
        CODE_GLO_L1P => GLO_L1P_STR,
        CODE_GLO_L2P => GLO_L2P_STR,

        CODE_SBAS_L1CA => SBAS_L1_STR,
        CODE_SBAS_L5I => SBAS_L5I_STR,
        CODE_SBAS_L5Q => SBAS_L5Q_STR,
        CODE_SBAS_L5X => SBAS_L5X_STR,
        CODE_AUX_SBAS => SBAS_AUX_STR,

        CODE_BDS2_B1 => BDS2_B1_STR,
        CODE_BDS2_B2 => BDS2_B2_STR,
        CODE_BDS3_B1CI => BDS3_B1CI_STR,
        CODE_BDS3_B1CQ => BDS3_B1CQ_STR,
        CODE_BDS3_B1CX => BDS3_B1CX_STR,
        CODE_BDS3_B5I => BDS3_B5I_STR,
        CODE_BDS3_B5Q => BDS3_B5Q_STR,
        CODE_BDS3_B5X => BDS3_B5X_STR,
        CODE_BDS3_B7I => BDS3_B7I_STR,
        CODE_BDS3_B7Q => BDS3_B7Q_STR,
        CODE_BDS3_B7X => BDS3_B7X_STR,
        CODE_BDS3_B3I => BDS3_B3I_STR,
        CODE_BDS3_B3Q => BDS3_B3Q_STR,
        CODE_BDS3_B3X => BDS3_B3X_STR,

        CODE_GAL_E1B => GAL_E1B_STR,
        CODE_GAL_E1C => GAL_E1C_STR,
        CODE_GAL_E1X => GAL_E1X_STR,
        CODE_GAL_E6B => GAL_E6B_STR,
        CODE_GAL_E6C => GAL_E6C_STR,
        CODE_GAL_E6X => GAL_E6X_STR,
        CODE_GAL_E7I => GAL_E7I_STR,
        CODE_GAL_E7Q => GAL_E7Q_STR,
        CODE_GAL_E7X => GAL_E7X_STR,
        CODE_GAL_E8I => GAL_E8I_STR,
        CODE_GAL_E8Q => GAL_E8Q_STR,
        CODE_GAL_E8X => GAL_E8X_STR,
        CODE_GAL_E5I => GAL_E5I_STR,
        CODE_GAL_E5Q => GAL_E5Q_STR,
        CODE_GAL_E5X => GAL_E5X_STR,
        CODE_AUX_GAL => GAL_AUX_STR,

        CODE_QZS_L1CA => QZS_L1CA_STR,
        CODE_QZS_L2CM => QZS_L2CM_STR,
        CODE_QZS_L2CL => QZS_L2CL_STR,
        CODE_QZS_L2CX => QZS_L2CX_STR,
        CODE_QZS_L5I => QZS_L5I_STR,
        CODE_QZS_L5Q => QZS_L5Q_STR,
        CODE_QZS_L5X => QZS_L5X_STR,
        _ => CODE_NOT_AVAILABLE,
    }
}

pub fn str_to_code_map(sat_str: &str) -> u8 {
    match sat_str {
        GPS_L1CA_STR => CODE_GPS_L1CA,
        GPS_L2CM_STR => CODE_GPS_L2CM,
        GPS_L2CL_STR => CODE_GPS_L2CL,
        GPS_L2CX_STR => CODE_GPS_L2CX,
        GPS_L5I_STR => CODE_GPS_L5I,
        GPS_L5Q_STR => CODE_GPS_L5Q,
        GPS_L5X_STR => CODE_GPS_L5X,
        GPS_L1P_STR => CODE_GPS_L1P,
        GPS_L2P_STR => CODE_GPS_L2P,
        GPS_AUX_STR => CODE_AUX_GPS,

        SBAS_L1_STR => CODE_SBAS_L1CA,
        SBAS_L5I_STR => CODE_SBAS_L5I,
        SBAS_L5Q_STR => CODE_SBAS_L5Q,
        SBAS_L5X_STR => CODE_SBAS_L5X,
        SBAS_AUX_STR => CODE_AUX_SBAS,

        GLO_L1OF_STR => CODE_GLO_L1OF,
        GLO_L2OF_STR => CODE_GLO_L2OF,
        GLO_L1P_STR => CODE_GLO_L1P,
        GLO_L2P_STR => CODE_GLO_L2P,

        BDS2_B1_STR => CODE_BDS2_B1,
        BDS2_B2_STR => CODE_BDS2_B2,
        BDS3_B1CI_STR => CODE_BDS3_B1CI,
        BDS3_B1CQ_STR => CODE_BDS3_B1CQ,
        BDS3_B1CX_STR => CODE_BDS3_B1CX,
        BDS3_B5I_STR => CODE_BDS3_B5I,
        BDS3_B5Q_STR => CODE_BDS3_B5Q,
        BDS3_B5X_STR => CODE_BDS3_B5X,
        BDS3_B7I_STR => CODE_BDS3_B7I,
        BDS3_B7Q_STR => CODE_BDS3_B7Q,
        BDS3_B7X_STR => CODE_BDS3_B7X,
        BDS3_B3I_STR => CODE_BDS3_B3I,
        BDS3_B3Q_STR => CODE_BDS3_B3Q,
        BDS3_B3X_STR => CODE_BDS3_B3X,

        GAL_E1B_STR => CODE_GAL_E1B,
        GAL_E1C_STR => CODE_GAL_E1C,
        GAL_E1X_STR => CODE_GAL_E1X,
        GAL_E5I_STR => CODE_GAL_E5I,
        GAL_E5Q_STR => CODE_GAL_E5Q,
        GAL_E5X_STR => CODE_GAL_E5X,
        GAL_E6B_STR => CODE_GAL_E6B,
        GAL_E6C_STR => CODE_GAL_E6C,
        GAL_E6X_STR => CODE_GAL_E6X,
        GAL_E7I_STR => CODE_GAL_E7I,
        GAL_E7Q_STR => CODE_GAL_E7Q,
        GAL_E7X_STR => CODE_GAL_E7X,
        GAL_E8I_STR => CODE_GAL_E8I,
        // GAL_E8Q_STR => CODE_GAL_E8Q,  // Unreachable
        // GAL_E8X_STR => CODE_GAL_E8X,  // Unreachable
        GAL_AUX_STR => CODE_AUX_GAL,

        QZS_L1CA_STR => CODE_QZS_L1CA,
        QZS_L2CM_STR => CODE_QZS_L2CM,
        QZS_L2CL_STR => CODE_QZS_L2CL,
        QZS_L2CX_STR => CODE_QZS_L2CX,
        QZS_L5I_STR => CODE_QZS_L5I,
        QZS_L5Q_STR => CODE_QZS_L5Q,
        QZS_L5X_STR => CODE_QZS_L5X,
        QZS_AUX_STR => CODE_AUX_QZS,
        _ => panic!("Unknown sat string!!"),
    }
}

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

pub fn get_color(key: (u8, i16)) -> &'static str {
    let (code, mut sat) = key;

    if code_is_glo(code) {
        sat += GLO_FCN_OFFSET;
    } else if code_is_sbas(code) {
        sat -= SBAS_NEG_OFFSET;
    } else if code_is_qzss(code) {
        sat -= QZSS_NEG_OFFSET;
    }
    if sat > NUM_COLORS as i16 {
        sat %= NUM_COLORS as i16;
    }
    color_map(sat)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_label_test() {
        let mut extra: HashMap<i16, i16> = HashMap::new();
        extra.insert(CODE_GLO_L2P as i16, CODE_GLO_L2P as i16);

        let (code_lbl, freq_lbl, id_lbl) = get_label((CODE_GLO_L2P, CODE_GLO_L2P as i16), &extra);
        assert_eq!(code_lbl.unwrap(), GLO_L2P_STR);
        assert_eq!(freq_lbl.unwrap(), "F+30");
        assert_eq!(id_lbl.unwrap(), "R30");

        let (code_lbl, freq_lbl, id_lbl) = get_label((CODE_GLO_L2OF, CODE_GLO_L2OF as i16), &extra);
        assert_eq!(code_lbl.unwrap(), GLO_L2OF_STR);
        assert_eq!(freq_lbl.unwrap(), "F+04");
        assert_eq!(id_lbl.unwrap(), "R04");

        let (code_lbl, freq_lbl, id_lbl) = get_label((CODE_SBAS_L5Q, CODE_SBAS_L5Q as i16), &extra);
        assert_eq!(code_lbl.unwrap(), SBAS_L5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "S 42");

        let (code_lbl, freq_lbl, id_lbl) = get_label((CODE_BDS3_B5Q, CODE_BDS3_B5Q as i16), &extra);
        assert_eq!(code_lbl.unwrap(), BDS3_B5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "C48");

        let (code_lbl, freq_lbl, id_lbl) = get_label((CODE_QZS_L2CX, CODE_QZS_L2CX as i16), &extra);
        assert_eq!(code_lbl.unwrap(), QZS_L2CX_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "J 37");

        let (code_lbl, freq_lbl, id_lbl) = get_label((CODE_GAL_E8X, CODE_GAL_E8X as i16), &extra);
        assert_eq!(code_lbl.unwrap(), GAL_E8X_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "E25");

        let (code_lbl, freq_lbl, id_lbl) = get_label((255, 255_i16), &extra);
        assert_eq!(code_lbl.unwrap(), CODE_NOT_AVAILABLE);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "G255");
    }
}
