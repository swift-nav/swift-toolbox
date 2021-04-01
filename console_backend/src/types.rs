use ordered_float::OrderedFloat;
use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::Sender,
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
    pub fn with_capacity(capacity: usize) -> Deque<T> {
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

// pub trait TabBackend<P: ProtoMsgSender> {
//     fn send_data(&mut self, client_send: P);
// }

pub trait ProtoMsgSender {
    fn send_data(&mut self, msg_bytes: Vec<u8>);
}

#[derive(Clone)]
pub struct ClientSender {
    pub inner: Sender<Vec<u8>>,
}
impl ProtoMsgSender for ClientSender {
    fn send_data(&mut self, msg_bytes: Vec<u8>) {
        self.inner.send(msg_bytes).unwrap();
    }
}

#[derive(Clone)]
pub struct TestSender {
    pub inner: Vec<Vec<u8>>,
}
impl ProtoMsgSender for TestSender {
    fn send_data(&mut self, msg: Vec<u8>) {
        self.inner.push(msg)
    }
}

#[derive(Debug)]
pub struct SharedState {
    pub tracking_tab: TrackingTabState,
    pub solution_tab: SolutionTabState,
}
impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            tracking_tab: TrackingTabState::new(),
            solution_tab: SolutionTabState::new(),
        }
    }
}
impl Default for SharedState {
    fn default() -> Self {
        SharedState::new()
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
