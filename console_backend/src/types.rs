use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{mpsc::Sender, Arc, Mutex},
    time::Instant,
};

use sbp::messages::{
    navigation::{MsgDops, MsgDopsDepA, MsgPosLLH, MsgPosLLHDepA, MsgVelNED, MsgVelNEDDepA},
    observation::{
        MsgObs, MsgObsDepB, MsgObsDepC, PackedObsContent, PackedObsContentDepB,
        PackedObsContentDepC,
    },
};

use crate::constants::{SignalCodes, DEGREES, KPH, MPH, MPS, MPS2KPH, MPS2MPH};
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

// Enum wrapping around various Vel NED Message types.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum VelNED {
    MsgVelNED(MsgVelNED),
    MsgVelNEDDepA(MsgVelNEDDepA),
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
