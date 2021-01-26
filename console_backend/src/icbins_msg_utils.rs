pub const VEL_NED_FLAGS_VEL_MODE_MASK: u8 = 0x07;
pub const VEL_NED_FLAGS_VEL_MODE_INVALID: u8 = 0;
pub const VEL_NED_FLAGS_VEL_MODE_MEAS_DOPPLER: u8 = 1;
pub const VEL_NED_FLAGS_VEL_MODE_COMP_DOPPLER: u8 = 2;
pub const VEL_NED_FLAGS_VEL_MODE_DR: u8 = 3;

pub const VEL_NED_FLAGS_INS_MODE_BITS_SHIFT: u8 = 3;
pub const VEL_NED_FLAGS_INS_MODE_MASK: u8 = 0x03;
pub const VEL_NED_FLAGS_INS_MODE_NONE: u8 = 0;
pub const VEL_NED_FLAGS_INS_MODE_USED: u8 = 1;

pub fn mm_to_m(mm: f64) -> f64 {
    (mm as f64) / 1.0e+3_f64
}


pub fn is_valid_vel_ned(flags: u8) -> bool {
    VEL_NED_FLAGS_VEL_MODE_INVALID != (flags & VEL_NED_FLAGS_VEL_MODE_MASK)
}

fn ned_to_sog_mps(n: f64, e: f64, d: f64) -> Option<f64> {
    Some(f64::sqrt(
        f64::powi(n, 2) + f64::powi(e, 2) + f64::powi(d, 2),
    ))
}

fn ned_to_cog_deg(n: f64, e: f64) -> Option<f64> {
    let mut radians = f64::atan2(e as f64, n as f64);

    if radians < 0.0 {
        radians += 2.0 * std::f64::consts::PI;
    }

    let degrees = f64::to_degrees(radians);

    if !f64::is_nan(degrees) {
        Some(degrees)
    } else {
        None
    }
}
#[derive(Debug, Clone)]
pub struct VelNedData {
    pub sog_mps: Option<f64>,
    pub cog_deg: Option<f64>,
    pub v_vel_mps: Option<f64>,
}

pub fn is_ins_mode_none_vel_ned(flags: u8) -> bool {
    VEL_NED_FLAGS_INS_MODE_NONE
        == ((flags >> VEL_NED_FLAGS_INS_MODE_BITS_SHIFT) & VEL_NED_FLAGS_INS_MODE_MASK)
}
pub fn is_ins_mode_used_vel_ned(flags: u8) -> bool {
    VEL_NED_FLAGS_INS_MODE_USED
        == ((flags >> VEL_NED_FLAGS_INS_MODE_BITS_SHIFT) & VEL_NED_FLAGS_INS_MODE_MASK)
}

pub fn extract_vel_ned_data(msg_n: i32, msg_e: i32, msg_d: i32) -> VelNedData {
    let (n, e, d) = (
        mm_to_m(msg_n as f64),
        mm_to_m(msg_e as f64),
        mm_to_m(msg_d as f64),
    );

    let sog_mps = ned_to_sog_mps(n, e, d);
    let cog_deg = ned_to_cog_deg(n, e);

    let v_vel_mps = -1.0 * d;

    let v_vel_mps = if !f64::is_nan(v_vel_mps) {
        Some(v_vel_mps)
    } else {
        None
    };

    VelNedData {
        sog_mps,
        cog_deg,
        v_vel_mps,
    }
}
