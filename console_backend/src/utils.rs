use std::collections::HashMap;

use crate::constants::*;
use crate::types::SignalCodes;

pub fn signal_key_label(
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

/// Retreive the associated color based on provided key.
///
/// # Parameters
///
/// - `key`: The code, which is signal code and satellite constellation-specific satellite identifier.
pub fn signal_key_color(key: (SignalCodes, i16)) -> &'static str {
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

/// Convert milliseconds to seconds.
///
/// # Parameters
/// - `ms`: Value in milliseconds.
///
/// # Returns
/// - Value in seconds.
pub fn ms_to_sec(ms: f64) -> f64 {
    ms / 1.0e+3_f64
}

/// Convert nanoseconds to seconds.
///
/// # Parameters
/// - `ns`: Value in nanoseconds.
///
/// # Returns
/// - Value in seconds.
pub fn ns_to_sec(ns: f64) -> f64 {
    ns / 1.0e+9_f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_signal_key_label_test() {
        let mut extra: HashMap<i16, i16> = HashMap::new();
        extra.insert(
            SignalCodes::CodeGloL2P as i16,
            SignalCodes::CodeGloL2P as i16,
        );

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeGloL2P, SignalCodes::CodeGloL2P as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2P_STR);
        assert_eq!(freq_lbl.unwrap(), "F+30");
        assert_eq!(id_lbl.unwrap(), "R30");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeGloL2Of, SignalCodes::CodeGloL2Of as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), GLO_L2OF_STR);
        assert_eq!(freq_lbl.unwrap(), "F+04");
        assert_eq!(id_lbl.unwrap(), "R04");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeSbasL5Q, SignalCodes::CodeSbasL5Q as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), SBAS_L5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "S 42");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeBds3B5Q, SignalCodes::CodeBds3B5Q as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), BDS3_B5Q_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "C48");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
            (SignalCodes::CodeQzsL2Cx, SignalCodes::CodeQzsL2Cx as i16),
            &extra,
        );
        assert_eq!(code_lbl.unwrap(), QZS_L2CX_STR);
        assert_eq!(freq_lbl, None);
        assert_eq!(id_lbl.unwrap(), "J 37");

        let (code_lbl, freq_lbl, id_lbl) = signal_key_label(
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
        assert!(f64::abs(nano_to_micro_sec(1000_f64) - 1_f64) <= f64::EPSILON);
        assert!(f64::abs(nano_to_micro_sec(1000000_f64) - 1000_f64) <= f64::EPSILON);
        assert!(f64::abs(nano_to_micro_sec(0_f64) - 0_f64) <= f64::EPSILON);
        assert!(f64::abs(nano_to_micro_sec(1337_f64) - 1.337_f64) <= f64::EPSILON);
    }
}
