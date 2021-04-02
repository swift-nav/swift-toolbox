use ordered_float::OrderedFloat;
use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
    sync::{mpsc::Sender, Arc, Mutex},
};

use sbp::messages::observation::{
    MsgObs, MsgObsDepB, MsgObsDepC, PackedObsContent, PackedObsContentDepB, PackedObsContentDepC,
};

use crate::constants::{SignalCodes, KPH, MPH, MPS, MPS2KPH, MPS2MPH};

pub type Error = std::boxed::Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Deque<T> {
    d: VecDeque<T>,
    capacity: usize,
}
impl<T> Deque<T> {
    pub fn with_size_limit(capacity: usize) -> Deque<T> {
        Deque {
            d: VecDeque::new(),
            capacity,
        }
    }
    pub fn add(&mut self, ele: T) {
        if self.d.len() == self.capacity {
            self.d.pop_front();
        }
        self.d.push_back(ele);
    }
    pub fn get(&self) -> &VecDeque<T> {
        &self.d
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
        SharedState {
            0: Arc::new(Mutex::new(SharedStateInner::default())),
        }
    }
    pub fn server_is_connected(&self) -> bool {
        {
            let shared_data = self.lock().unwrap();
            (*shared_data).server.connected
        }
    }
    pub fn server_set_connected(&self, set_to: bool) {
        {
            let mut shared_data = self.lock().unwrap();
            (*shared_data).server.connected = set_to;
        }
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
    pub velocity_tab: SolutionVelocityTabState,
}

impl SolutionTabState {
    fn new() -> SolutionTabState {
        SolutionTabState {
            velocity_tab: SolutionVelocityTabState::new(),
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
