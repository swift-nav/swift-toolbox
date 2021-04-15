use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use sbp::messages::{
    navigation::{MsgDops, MsgDopsDepA, MsgPosLLH, MsgPosLLHDepA, MsgVelNED, MsgVelNEDDepA},
    observation::{
        MsgObs, MsgObsDepB, MsgObsDepC, PackedObsContent, PackedObsContentDepB,
        PackedObsContentDepC,
    },
};
use serde::Serialize;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    fmt,
    hash::Hash,
    ops::Deref,
    sync::{mpsc::Sender, Arc, Mutex},
    time::Instant,
};

use crate::constants::*;
use crate::formatters::*;

pub type Error = std::boxed::Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub type UtcDateTime = DateTime<Utc>;

#[derive(Debug, Clone)]
pub struct Deque<T> {
    d: Vec<T>,
    capacity: usize,
}
impl<T> Deque<T> {
    pub fn with_size_limit(capacity: usize) -> Deque<T> {
        Deque {
            d: Vec::new(),
            capacity,
        }
    }
    pub fn add(&mut self, ele: T) {
        if self.d.len() == self.capacity {
            self.d.remove(0);
        }
        self.d.push(ele);
    }
    pub fn get(&self) -> &Vec<T> {
        &self.d
    }
    pub fn clear(&mut self) {
        self.d.clear();
    }
}

pub trait MessageSender {
    fn send_data(&mut self, msg_bytes: Vec<u8>);
}

#[derive(Clone)]
pub struct ClientSender {
    pub inner: Sender<Vec<u8>>,
}
impl MessageSender for ClientSender {
    fn send_data(&mut self, msg_bytes: Vec<u8>) {
        self.inner.send(msg_bytes).unwrap();
    }
}

#[derive(Clone)]
pub struct TestSender {
    pub inner: Vec<Vec<u8>>,
}
impl MessageSender for TestSender {
    fn send_data(&mut self, msg: Vec<u8>) {
        self.inner.push(msg)
    }
}

#[derive(Debug)]
pub struct SharedState(Arc<Mutex<SharedStateInner>>);

impl SharedState {
    pub fn new() -> SharedState {
        SharedState(Arc::new(Mutex::new(SharedStateInner::default())))
    }
    pub fn server_is_connected(&self) -> bool {
        let shared_data = self.lock().unwrap();
        (*shared_data).server.connected
    }
    pub fn server_set_connected(&self, set_to: bool) {
        let mut shared_data = self.lock().unwrap();
        (*shared_data).server.connected = set_to;
    }
}

impl Deref for SharedState {
    type Target = Mutex<SharedStateInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SharedState {
    fn clone(&self) -> Self {
        SharedState {
            0: Arc::clone(&self.0),
        }
    }
}

#[derive(Debug)]
pub struct SharedStateInner {
    pub tracking_tab: TrackingTabState,
    pub server: ServerState,
    pub solution_tab: SolutionTabState,
}
impl SharedStateInner {
    pub fn new() -> SharedStateInner {
        SharedStateInner {
            tracking_tab: TrackingTabState::new(),
            server: ServerState::new(),
            solution_tab: SolutionTabState::new(),
        }
    }
}
impl Default for SharedStateInner {
    fn default() -> Self {
        SharedStateInner::new()
    }
}

#[derive(Debug)]
pub struct ServerState {
    pub connected: bool,
}

impl ServerState {
    fn new() -> ServerState {
        ServerState { connected: false }
    }
}

#[derive(Debug)]
pub struct TrackingTabState {
    pub signals_tab: TrackingSignalsTabState,
}

impl TrackingTabState {
    fn new() -> TrackingTabState {
        TrackingTabState {
            signals_tab: TrackingSignalsTabState::new(),
        }
    }
}

#[derive(Debug)]
pub struct TrackingSignalsTabState {
    pub check_visibility: Vec<String>,
}

impl TrackingSignalsTabState {
    fn new() -> TrackingSignalsTabState {
        TrackingSignalsTabState {
            check_visibility: vec![],
        }
    }
}

#[derive(Debug)]
pub struct SolutionTabState {
    pub position_tab: SolutionPositionTabState,
    pub velocity_tab: SolutionVelocityTabState,
}

impl SolutionTabState {
    fn new() -> SolutionTabState {
        SolutionTabState {
            position_tab: SolutionPositionTabState::new(),
            velocity_tab: SolutionVelocityTabState::new(),
        }
    }
}

#[derive(Debug)]
pub struct SolutionPositionTabState {
    pub center: bool,
    pub clear: bool,
    pub ins_status_flags: u32,
    pub last_ins_status_receipt_time: Instant,
    pub last_odo_update_time: Instant,
    pub pause: bool,
    pub unit: String,
    pub zoom: bool,
}

impl SolutionPositionTabState {
    fn new() -> SolutionPositionTabState {
        SolutionPositionTabState {
            center: false,
            clear: false,
            ins_status_flags: 0,
            last_ins_status_receipt_time: Instant::now(),
            last_odo_update_time: Instant::now(),
            pause: false,
            unit: String::from(DEGREES),
            zoom: false,
        }
    }
}

#[derive(Debug)]
pub struct SolutionVelocityTabState {
    pub unit: String,
}

impl SolutionVelocityTabState {
    fn new() -> SolutionVelocityTabState {
        SolutionVelocityTabState {
            unit: String::from(MPS),
        }
    }
}

// Tracking Signals Tab Types.
pub type Cn0Dict = HashMap<(SignalCodes, i16), Deque<(OrderedFloat<f64>, f64)>>;
pub type Cn0Age = HashMap<(SignalCodes, i16), f64>;

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
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
            GAL_E8Q_STR => Ok(SignalCodes::CodeGalE8Q),
            GAL_E8X_STR => Ok(SignalCodes::CodeGalE8X),
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

impl fmt::Display for SignalCodes {
    /// Retrieve associated string with the provided signal code.
    ///
    /// # Parameters
    ///
    /// - `key`: The code, which is signal code, and satellite constellation-specific satellite identifier.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        write!(f, "{}", sat_code_str)
    }
}

// Enum wrapping around various Observation Message types.
pub enum ObservationMsg {
    MsgObs(MsgObs),
    // MsgObsDepA(MsgObsDepA),
    MsgObsDepB(MsgObsDepB),
    MsgObsDepC(MsgObsDepC),
}
// Enum wrapping around various Observation Message observation types.
pub enum Observations {
    PackedObsContent(PackedObsContent),
    // PackedObsContentDepA(PackedObsContentDepA),
    PackedObsContentDepB(PackedObsContentDepB),
    PackedObsContentDepC(PackedObsContentDepC),
}

// Solution Tab Types.

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum GnssModes {
    NoFix = 0,
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
            0 => GnssModes::NoFix,
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
impl fmt::Display for GnssModes {
    /// Retrieve associated string with the provided signal code.
    ///
    /// # Parameters
    ///
    /// - `key`: The code, which is signal code, and satellite constellation-specific satellite identifier.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gnss_mode_str = match self {
            GnssModes::NoFix => NO_FIX,
            GnssModes::Spp => SPP,
            GnssModes::Dgnss => DGNSS,
            GnssModes::Float => FLOAT,
            GnssModes::Fixed => FIXED,
            GnssModes::Dr => DR,
            GnssModes::Sbas => SBAS,
        };
        write!(f, "{}", gnss_mode_str)
    }
}
impl GnssModes {
    pub fn label(&self) -> String {
        let gnss_mode_label = match self {
            GnssModes::NoFix => NO_FIX_LABEL,
            GnssModes::Spp => SPP_LABEL,
            GnssModes::Dgnss => DGNSS_LABEL,
            GnssModes::Float => FLOAT_LABEL,
            GnssModes::Fixed => FIXED_LABEL,
            GnssModes::Dr => DR_LABEL,
            GnssModes::Sbas => SBAS_LABEL,
        };
        String::from(gnss_mode_label)
    }
    pub fn color(&self) -> String {
        let gnss_mode_color = match self {
            GnssModes::NoFix => NO_FIX_COLOR,
            GnssModes::Spp => SPP_COLOR,
            GnssModes::Dgnss => DGNSS_COLOR,
            GnssModes::Float => FLOAT_COLOR,
            GnssModes::Fixed => FIXED_COLOR,
            GnssModes::Dr => DR_COLOR,
            GnssModes::Sbas => SBAS_COLOR,
        };
        String::from(gnss_mode_color)
    }
}

// Enum wrapping around various PosLLH Message types.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum PosLLH {
    MsgPosLLH(MsgPosLLH),
    MsgPosLLHDepA(MsgPosLLHDepA),
}
impl PosLLH {
    pub fn mode(&self) -> u8 {
        match self {
            PosLLH::MsgPosLLH(msg) => msg.flags & 0x7,
            PosLLH::MsgPosLLHDepA(msg) => {
                let mode = msg.flags & 0x7;
                match mode {
                    0 => 1,
                    1 => 4,
                    2 => 3,
                    _ => mode,
                }
            }
        }
    }
}

// Enum wrapping around various Dops Message types.
#[derive(Debug)]
pub enum Dops {
    MsgDops(MsgDops),
    MsgDopsDepA(MsgDopsDepA),
}

impl Dops {
    pub fn fields(self) -> (u16, u16, u16, u16, u16, u8) {
        match self {
            Dops::MsgDops(msg_) => (
                msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, msg_.flags,
            ),
            Dops::MsgDopsDepA(msg_) => {
                (msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, 1_u8)
            }
        }
    }
}

// Enum wrapping around various Vel NED Message types.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum VelNED {
    MsgVelNED(MsgVelNED),
    MsgVelNEDDepA(MsgVelNEDDepA),
}

impl VelNED {
    pub fn fields(self) -> (u8, f64, i32, i32, i32, u8) {
        match self {
            VelNED::MsgVelNED(msg) => (msg.flags, msg.tow as f64, msg.n, msg.e, msg.d, msg.n_sats),
            VelNED::MsgVelNEDDepA(msg) => (1, msg.tow as f64, msg.n, msg.e, msg.d, msg.n_sats),
        }
    }
}

// Solution Velocity Tab Types.
#[derive(Debug, Clone, PartialEq)]
pub enum VelocityUnits {
    Mps,
    Mph,
    Kph,
}

impl VelocityUnits {
    /// Retrieve the velocity unit as string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            VelocityUnits::Mps => MPS,
            VelocityUnits::Mph => MPH,
            VelocityUnits::Kph => KPH,
        }
    }
    pub fn get_multiplier(&self) -> f64 {
        match self {
            VelocityUnits::Mps => 1.0,
            VelocityUnits::Mph => MPS2MPH,
            VelocityUnits::Kph => MPS2KPH,
        }
    }
}

impl std::str::FromStr for VelocityUnits {
    type Err = Error;
    /// Retrieve the velocity unit from string slice.
    ///
    /// # Parameters
    ///
    /// - `s`: The string slice to convert to VelocityUnits.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            MPS => Ok(VelocityUnits::Mps),
            MPH => Ok(VelocityUnits::Mph),
            KPH => Ok(VelocityUnits::Kph),
            _ => panic!("unable to convert to VelocityUnits"),
        }
    }
}

#[derive(Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct PosLLHLog {
    pub pc_time: String,
    pub gps_time: Option<String>,
    #[serde(rename = "tow(sec)", with = "float_formatter_3")]
    pub tow_s: Option<f64>,
    #[serde(rename = "latitude(degrees)", with = "float_formatter_10")]
    pub latitude_d: Option<f64>,
    #[serde(rename = "longitude(degrees)", with = "float_formatter_10")]
    pub longitude_d: Option<f64>,
    #[serde(rename = "altitude(meters)", with = "float_formatter_4")]
    pub altitude_m: Option<f64>,
    #[serde(rename = "h_accuracy(meters)", with = "float_formatter_4")]
    pub h_accuracy_m: Option<f64>,
    #[serde(rename = "v_accuracy(meters)", with = "float_formatter_4")]
    pub v_accuracy_m: Option<f64>,
    pub n_sats: u8,
    pub flags: u8,
}

#[derive(Serialize)]
pub struct VelLog {
    pub pc_time: String,
    pub gps_time: Option<String>,
    #[serde(rename = "tow(sec)", with = "float_formatter_3")]
    pub tow_s: Option<f64>,
    #[serde(rename = "north(m/s)", with = "float_formatter_6")]
    pub north_mps: Option<f64>,
    #[serde(rename = "east(m/s)", with = "float_formatter_6")]
    pub east_mps: Option<f64>,
    #[serde(rename = "down(m/s)", with = "float_formatter_6")]
    pub down_mps: Option<f64>,
    #[serde(rename = "speed(m/s)", with = "float_formatter_6")]
    pub speed_mps: Option<f64>,
    pub flags: u8,
    pub num_signals: u8,
}
