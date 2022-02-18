use crate::constants::{
    DGNSS, DGNSS_COLOR, DGNSS_LABEL, DR, DR_COLOR, DR_LABEL, FIXED, FIXED_COLOR, FIXED_LABEL,
    FLOAT, FLOAT_COLOR, FLOAT_LABEL, FLOW_CONTROL_HARDWARE, FLOW_CONTROL_NONE,
    FLOW_CONTROL_SOFTWARE, KPH, MPH, MPS, MPS2KPH, MPS2MPH, NO_FIX, NO_FIX_COLOR, NO_FIX_LABEL,
    RTK, SBAS, SBAS_COLOR, SBAS_LABEL, SPP, SPP_COLOR, SPP_LABEL,
};
use crate::piksi_tools_constants::{
    BDS2_B1_STR, BDS2_B2_STR, BDS3_B1CI_STR, BDS3_B1CQ_STR, BDS3_B1CX_STR, BDS3_B3I_STR,
    BDS3_B3Q_STR, BDS3_B3X_STR, BDS3_B5I_STR, BDS3_B5Q_STR, BDS3_B5X_STR, BDS3_B7I_STR,
    BDS3_B7Q_STR, BDS3_B7X_STR, CODE_NOT_AVAILABLE, GAL_AUX_STR, GAL_E1B_STR, GAL_E1C_STR,
    GAL_E1X_STR, GAL_E5I_STR, GAL_E5Q_STR, GAL_E5X_STR, GAL_E6B_STR, GAL_E6C_STR, GAL_E6X_STR,
    GAL_E7I_STR, GAL_E7Q_STR, GAL_E7X_STR, GAL_E8I_STR, GAL_E8Q_STR, GAL_E8X_STR, GLO_L1OF_STR,
    GLO_L1P_STR, GLO_L2OF_STR, GLO_L2P_STR, GPS_AUX_STR, GPS_L1CA_STR, GPS_L1P_STR, GPS_L2CL_STR,
    GPS_L2CM_STR, GPS_L2CX_STR, GPS_L2P_STR, GPS_L5I_STR, GPS_L5Q_STR, GPS_L5X_STR, QZS_AUX_STR,
    QZS_L1CA_STR, QZS_L2CL_STR, QZS_L2CM_STR, QZS_L2CX_STR, QZS_L5I_STR, QZS_L5Q_STR, QZS_L5X_STR,
    SBAS_AUX_STR, SBAS_L1_STR, SBAS_L5I_STR, SBAS_L5Q_STR, SBAS_L5X_STR,
};

use crate::utils::{mm_to_m, ms_to_sec};
use anyhow::Context;
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use sbp::link::Event;
use sbp::messages::{
    navigation::{
        MsgBaselineNed, MsgBaselineNedDepA, MsgDops, MsgDopsDepA, MsgGpsTime, MsgGpsTimeDepA,
        MsgPosLlh, MsgPosLlhDepA, MsgVelNed, MsgVelNedDepA,
    },
    observation::{
        MsgObs, MsgObsDepB, MsgObsDepC, MsgOsr, PackedObsContent, PackedObsContentDepB,
        PackedObsContentDepC, PackedOsrContent,
    },
    piksi::{Latency, MsgSpecan, MsgSpecanDep, MsgUartState, MsgUartStateDepa, Period},
    ConcreteMessage,
};
use sbp::{Sbp, SbpEncoder, SbpMessage};
use serialport::FlowControl as SPFlowControl;
use std::io;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    fmt,
    fmt::Debug,
    hash::Hash,
    ops::Deref,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering::*},
        Arc, Mutex,
    },
};
pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;
pub type UtcDateTime = DateTime<Utc>;

/// Sends Sbp messages to the connected device
pub struct MsgSender {
    inner: Arc<Mutex<SbpEncoder<Box<dyn io::Write + Send>>>>,
}

impl MsgSender {
    /// 42 is the conventional sender ID intended for messages sent from the host to the device
    const SENDER_ID: u16 = 42;
    const LOCK_FAILURE: &'static str = "failed to aquire sender lock";

    pub fn new<W>(writer: W) -> Self
    where
        W: io::Write + Send + 'static,
    {
        Self {
            inner: Arc::new(Mutex::new(SbpEncoder::new(Box::new(writer)))),
        }
    }

    pub fn send(&self, msg: impl Into<Sbp>) -> Result<()> {
        let mut msg = msg.into();
        if msg.sender_id().is_none() {
            msg.set_sender_id(Self::SENDER_ID);
        }
        let mut framed = self.inner.lock().expect(Self::LOCK_FAILURE);
        framed.send(&msg).context("while sending a message")?;
        Ok(())
    }
}

impl Clone for MsgSender {
    fn clone(&self) -> Self {
        MsgSender {
            inner: Arc::clone(&self.inner),
        }
    }
}

type Iter<'a, T> = std::iter::Chain<std::slice::Iter<'a, T>, std::slice::Iter<'a, T>>;
type IterMut<'a, T> = std::iter::Chain<std::slice::IterMut<'a, T>, std::slice::IterMut<'a, T>>;

#[derive(Debug, Clone)]
pub struct Deque<T> {
    data: Vec<T>,
    capacity: usize,
    // Index to where the next element will be placed.
    index: usize,
}

impl<T> Deque<T> {
    pub fn new(capacity: usize) -> Deque<T> {
        Deque {
            data: Vec::with_capacity(capacity),
            capacity,
            index: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.is_full() {
            self.data[self.index] = value;
        } else {
            self.data.push(value);
        }
        self.index = (self.index + 1) % self.capacity();
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.index = 0;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_full(&self) -> bool {
        self.capacity() == self.len()
    }

    /// Returns an iterator from the oldest values to the newest.
    pub fn iter(&self) -> Iter<T> {
        let (a, b) = self.data.split_at(self.index);
        b.iter().chain(a.iter())
    }

    /// Returns a mutable iterator from the oldest values to the newest.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (a, b) = self.data.split_at_mut(self.index);
        b.iter_mut().chain(a.iter_mut())
    }
}

impl<T: Clone> Deque<T> {
    pub fn with_fill_value(capacity: usize, fill_value: T) -> Deque<T> {
        Deque {
            data: vec![fill_value; capacity],
            capacity,
            index: 0,
        }
    }
}

impl<T> std::ops::Index<usize> for Deque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let len = self.len();
        assert!(index < len);
        &self.data[(index + self.index) % len]
    }
}

impl<T> std::ops::IndexMut<usize> for Deque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.len();
        assert!(index < len);
        &mut self.data[(index + self.index) % len]
    }
}

impl<'a, T> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Deque<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[macro_export]
macro_rules! unzip {
    ($p:pat => $tup:expr) => {
        |$p| $tup
    };
    ($p:pat => ($($tup:tt)*) , $_iter:expr $(, $tail:expr)*) => {
        $crate::unzip!(
            ($p, b) => ($($tup)*, b) $(, $tail)*
        )
    };
}

// https://docs.rs/itertools/0.10.1/itertools/macro.izip.html
#[macro_export]
macro_rules! zip {
    // one item
    ($x:expr) => {
        ::std::iter::IntoIterator::into_iter($x)
    };

    // two items and an optional trailing comma
    ($x:expr, $y:expr $(,)*) => {
        $crate::zip!($x)
            .zip($crate::zip!($y))
    };

    // three or more items and an optional trailing comma
    ($x:expr $(, $y:expr)* $(,)*) => {
        $crate::zip!($x)
            $(
                .zip($y)
            )*
            .map(
                $crate::unzip!(a => (a) $( , $y )*)
            )
    };
}

#[derive(Debug, Default)]
pub struct ArcBool(Arc<AtomicBool>);
impl ArcBool {
    pub fn new() -> ArcBool {
        ArcBool(Arc::new(AtomicBool::new(false)))
    }
    pub fn new_with(value: bool) -> ArcBool {
        ArcBool(Arc::new(AtomicBool::new(value)))
    }
    pub fn get(&self) -> bool {
        self.load(Acquire)
    }
    pub fn set(&self, set_to: bool) {
        self.store(set_to, Release);
    }
}

impl Deref for ArcBool {
    type Target = AtomicBool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for ArcBool {
    fn clone(&self) -> Self {
        ArcBool {
            0: Arc::clone(&self.0),
        }
    }
}

// Main Tab Types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RealtimeDelay {
    On,
    Off,
}

// Advanced System Monitor Types.
// Struct with shared fields for various UartState Message types.
pub struct UartStateFields {
    pub latency: Latency,
    pub obs_period: Option<Period>,
}

// Enum wrapping around various UartState Message types.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum UartState {
    MsgUartState(MsgUartState),
    MsgUartStateDepa(MsgUartStateDepa),
}

impl UartState {
    pub fn fields(&self) -> UartStateFields {
        let (latency, obs_period) = match self {
            UartState::MsgUartState(msg) => (msg.latency.clone(), Some(msg.obs_period.clone())),

            UartState::MsgUartStateDepa(msg) => (msg.latency.clone(), None),
        };
        UartStateFields {
            latency,
            obs_period,
        }
    }
}

impl Event for UartState {
    const MESSAGE_TYPES: &'static [u16] =
        &[MsgUartState::MESSAGE_TYPE, MsgUartStateDepa::MESSAGE_TYPE];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgUartState(m) => UartState::MsgUartState(m),
            Sbp::MsgUartStateDepa(m) => UartState::MsgUartStateDepa(m),
            _ => unreachable!(),
        }
    }
}

// Enum wrapping around various Vel NED Message types.
#[derive(Debug, Clone, Copy)]
pub struct FlowControl(SPFlowControl);

impl FromStr for FlowControl {
    type Err = String;

    /// Convert flow control string slice to expected serialport FlowControl variant.
    ///
    /// # Parameters
    ///
    /// - `sat_str`: The signal code.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            FLOW_CONTROL_NONE => Ok(FlowControl(SPFlowControl::None)),
            FLOW_CONTROL_SOFTWARE => Ok(FlowControl(SPFlowControl::Software)),
            FLOW_CONTROL_HARDWARE => Ok(FlowControl(SPFlowControl::Hardware)),
            _ => Err(format!(
                "Not a valid flow control option. Choose from [\"{}\", \"{}\", \"{}\"]",
                FLOW_CONTROL_NONE, FLOW_CONTROL_SOFTWARE, FLOW_CONTROL_HARDWARE
            )),
        }
    }
}

impl Deref for FlowControl {
    type Target = SPFlowControl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Tracking Signals Tab Types.
pub type Cn0Dict = HashMap<(SignalCodes, i16), Deque<(OrderedFloat<f64>, f64)>>;
pub type Cn0Age = HashMap<(SignalCodes, i16), f64>;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
            SignalCodes::CodeGpsL1Ca => Some(GPS_L1CA_STR.to_string()),
            SignalCodes::CodeGpsL2Cm => Some(GPS_L2CM_STR.to_string()),
            SignalCodes::CodeGloL1Of => Some(GLO_L1OF_STR.to_string()),
            SignalCodes::CodeGloL2Of => Some(GLO_L2OF_STR.to_string()),
            SignalCodes::CodeBds2B1 => Some(BDS2_B1_STR.to_string()),
            SignalCodes::CodeBds2B2 => Some(BDS2_B2_STR.to_string()),
            SignalCodes::CodeGalE1B => Some(GAL_E1B_STR.to_string()),
            SignalCodes::CodeGalE1X => Some(GAL_E1X_STR.to_string()),
            SignalCodes::CodeGalE7I => Some(GAL_E7I_STR.to_string()),
            SignalCodes::CodeGalE7Q => Some(GAL_E7Q_STR.to_string()),
            SignalCodes::CodeQzsL1Ca => Some(QZS_L1CA_STR.to_string()),
            SignalCodes::CodeQzsL2Cm => Some(QZS_L2CM_STR.to_string()),
            SignalCodes::CodeSbasL1Ca => Some(SBAS_L1_STR.to_string()),
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
            SignalCodes::CodeAuxQzs => QZS_AUX_STR,
            _ => CODE_NOT_AVAILABLE,
        };
        write!(f, "{}", sat_code_str)
    }
}

// Struct with shared fields for various Observation Message types.
pub struct ObservationMsgFields {
    pub n_obs: u8,
    pub tow: f64,
    pub wn: u16,
    pub ns_residual: i32,
    pub states: Vec<Observations>,
    pub sender_id: Option<u16>,
}

// Enum wrapping around various Observation Message types.
#[derive(Debug, Clone)]
pub enum ObservationMsg {
    MsgObs(MsgObs),
    // MsgObsDepA(MsgObsDepA),
    MsgObsDepB(MsgObsDepB),
    MsgObsDepC(MsgObsDepC),
    MsgOsr(MsgOsr),
}

impl ObservationMsg {
    pub fn fields(&self) -> ObservationMsgFields {
        let (n_obs, tow, wn, ns_residual, states, sender_id) = match &self {
            ObservationMsg::MsgObs(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedObsContent)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    obs.header.t.ns_residual,
                    states,
                    obs.sender_id,
                )
            }
            // ObservationMsg::MsgObsDepA(obs)
            ObservationMsg::MsgObsDepB(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedObsContentDepB)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    0_i32,
                    states,
                    obs.sender_id,
                )
            }
            ObservationMsg::MsgObsDepC(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedObsContentDepC)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    0_i32,
                    states,
                    obs.sender_id,
                )
            }

            ObservationMsg::MsgOsr(obs) => {
                let states: Vec<Observations> = obs
                    .obs
                    .clone()
                    .into_iter()
                    .map(Observations::PackedOsrContent)
                    .collect();
                (
                    obs.header.n_obs,
                    ms_to_sec(obs.header.t.tow as f64),
                    obs.header.t.wn,
                    obs.header.t.ns_residual,
                    states,
                    obs.sender_id,
                )
            }
        };
        ObservationMsgFields {
            n_obs,
            tow,
            wn,
            ns_residual,
            states,
            sender_id,
        }
    }
}

impl Event for ObservationMsg {
    const MESSAGE_TYPES: &'static [u16] = &[
        MsgObs::MESSAGE_TYPE,
        MsgObsDepB::MESSAGE_TYPE,
        MsgObsDepC::MESSAGE_TYPE,
        MsgOsr::MESSAGE_TYPE,
    ];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgObs(m) => ObservationMsg::MsgObs(m),
            Sbp::MsgObsDepB(m) => ObservationMsg::MsgObsDepB(m),
            Sbp::MsgObsDepC(m) => ObservationMsg::MsgObsDepC(m),
            Sbp::MsgOsr(m) => ObservationMsg::MsgOsr(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various Observation Contents types.
pub struct ObservationFields {
    pub is_deprecated_msg_type: bool,
    pub code: SignalCodes,
    pub sat: i16,
    pub pseudo_range: f64,
    pub carrier_phase: f64,
    pub cn0: f64,
    pub measured_doppler: f64,
    pub computed_doppler: f64,
    pub lock: u16,
    pub flags: u8,
}

// Enum wrapping around various Observation Contents observation types.
pub enum Observations {
    PackedObsContent(PackedObsContent),
    // PackedObsContentDepA(PackedObsContentDepA),
    PackedObsContentDepB(PackedObsContentDepB),
    PackedObsContentDepC(PackedObsContentDepC),
    PackedOsrContent(PackedOsrContent),
}

impl Observations {
    pub fn fields(&self) -> ObservationFields {
        // DEP_B and DEP_A obs had different pseudorange scaling
        let divisor = match self {
            Observations::PackedObsContentDepB(_) => 1e2,
            _ => 5e1,
        };
        let (
            is_deprecated_msg_type,
            code,
            sat,
            pseudo_range,
            carrier_phase_cycles,
            carrier_phase_fractional,
            cn0,
            measured_doppler,
            lock,
            flags,
        ) = match self {
            Observations::PackedObsContentDepB(obs) => {
                let mut sat_ = obs.sid.sat as i16;
                let signal_code = SignalCodes::from(obs.sid.code);
                if signal_code.code_is_gps() {
                    sat_ += 1;
                }
                (
                    true,
                    signal_code,
                    sat_,
                    obs.p as f64 / divisor,
                    obs.l.i,
                    obs.l.f,
                    obs.cn0 as f64,
                    0_f64, // obs.D
                    obs.lock,
                    0_u8,
                )
            }
            Observations::PackedObsContentDepC(obs) => {
                let mut sat_ = obs.sid.sat as i16;
                let signal_code = SignalCodes::from(obs.sid.code);
                if signal_code.code_is_gps() {
                    sat_ += 1;
                }
                (
                    true,
                    signal_code,
                    sat_,
                    obs.p as f64 / divisor,
                    obs.l.i,
                    obs.l.f,
                    obs.cn0 as f64,
                    0_f64, // obs.D
                    obs.lock,
                    0_u8,
                )
            }
            Observations::PackedObsContent(obs) => (
                false,
                SignalCodes::from(obs.sid.code),
                obs.sid.sat as i16,
                obs.p as f64 / divisor,
                obs.l.i,
                obs.l.f,
                obs.cn0 as f64,
                obs.d.i as f64 + obs.d.f as f64 / ((1 << 8) as f64),
                obs.lock as u16,
                obs.flags,
            ),
            Observations::PackedOsrContent(obs) => (
                false,
                SignalCodes::from(obs.sid.code),
                obs.sid.sat as i16,
                obs.p as f64 / divisor,
                obs.l.i,
                obs.l.f,
                0_f64, // cn0
                0_f64, // obs.D
                obs.lock as u16,
                obs.flags,
            ),
        };
        let carrier_phase =
            carrier_phase_cycles as f64 + carrier_phase_fractional as f64 / ((1 << 8) as f64);
        ObservationFields {
            is_deprecated_msg_type,
            code,
            sat,
            pseudo_range,
            carrier_phase,
            cn0,
            measured_doppler,
            computed_doppler: 0_f64,
            lock,
            flags,
        }
    }
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
    pub fn pos_mode(&self) -> String {
        let gnss_pos_mode = match self {
            GnssModes::NoFix => NO_FIX_LABEL,
            GnssModes::Spp => SPP,
            GnssModes::Dgnss => DGNSS,
            GnssModes::Float => RTK,
            GnssModes::Fixed => RTK,
            GnssModes::Dr => DR_LABEL,
            GnssModes::Sbas => SBAS,
        };
        String::from(gnss_pos_mode)
    }
}

// Struct with shared fields for various PosLLH Message types.
#[allow(clippy::upper_case_acronyms)]
pub struct PosLLHFields {
    pub flags: u8,
    pub h_accuracy: f64,
    pub v_accuracy: f64,
    pub tow: f64,
    pub lat: f64,
    pub lon: f64,
    pub height: f64,
    pub n_sats: u8,
}

// Enum wrapping around various PosLLH Message types.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum PosLLH {
    MsgPosLlh(MsgPosLlh),
    MsgPosLlhDepA(MsgPosLlhDepA),
}

impl PosLLH {
    pub fn fields(&self) -> PosLLHFields {
        match self {
            PosLLH::MsgPosLlh(MsgPosLlh {
                flags,
                h_accuracy,
                v_accuracy,
                tow,
                lat,
                lon,
                height,
                n_sats,
                ..
            })
            | PosLLH::MsgPosLlhDepA(MsgPosLlhDepA {
                flags,
                h_accuracy,
                v_accuracy,
                tow,
                lat,
                lon,
                height,
                n_sats,
                ..
            }) => PosLLHFields {
                flags: *flags,
                h_accuracy: mm_to_m(*h_accuracy as f64),
                v_accuracy: mm_to_m(*v_accuracy as f64),
                tow: *tow as f64,
                lat: *lat,
                lon: *lon,
                height: *height,
                n_sats: *n_sats,
            },
        }
    }
    pub fn mode(&self) -> u8 {
        match self {
            PosLLH::MsgPosLlh(msg) => msg.flags & 0x7,
            PosLLH::MsgPosLlhDepA(msg) => {
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

impl Event for PosLLH {
    const MESSAGE_TYPES: &'static [u16] = &[MsgPosLlh::MESSAGE_TYPE, MsgPosLlhDepA::MESSAGE_TYPE];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgPosLlh(m) => PosLLH::MsgPosLlh(m),
            Sbp::MsgPosLlhDepA(m) => PosLLH::MsgPosLlhDepA(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various Dops Message types.
pub struct DopsFields {
    pub pdop: u16,
    pub gdop: u16,
    pub tdop: u16,
    pub hdop: u16,
    pub vdop: u16,
    pub flags: u8,
}
// Enum wrapping around various Dops Message types.
#[derive(Debug, Clone)]
pub enum Dops {
    MsgDops(MsgDops),
    MsgDopsDepA(MsgDopsDepA),
}

impl Dops {
    pub fn fields(self) -> DopsFields {
        let (pdop, gdop, tdop, hdop, vdop, flags) = match self {
            Dops::MsgDops(msg_) => (
                msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, msg_.flags,
            ),
            Dops::MsgDopsDepA(msg_) => {
                (msg_.pdop, msg_.gdop, msg_.tdop, msg_.hdop, msg_.vdop, 1_u8)
            }
        };
        DopsFields {
            pdop,
            gdop,
            tdop,
            hdop,
            vdop,
            flags,
        }
    }
}

impl Event for Dops {
    const MESSAGE_TYPES: &'static [u16] = &[MsgDops::MESSAGE_TYPE, MsgDopsDepA::MESSAGE_TYPE];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgDops(m) => Dops::MsgDops(m),
            Sbp::MsgDopsDepA(m) => Dops::MsgDopsDepA(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various GpsTime Message types.
pub struct GpsTimeFields {
    pub wn: u16,
    pub ns_residual: i32,
    pub flags: u8,
}
// Enum wrapping around various GpsTime Message types.
#[derive(Debug, Clone)]
pub enum GpsTime {
    MsgGpsTime(MsgGpsTime),
    MsgGpsTimeDepA(MsgGpsTimeDepA),
}

impl GpsTime {
    pub fn fields(self) -> GpsTimeFields {
        let (wn, ns_residual, flags) = match self {
            GpsTime::MsgGpsTime(msg_) => (msg_.wn, msg_.ns_residual, msg_.flags),
            GpsTime::MsgGpsTimeDepA(msg_) => (msg_.wn, msg_.ns_residual, msg_.flags),
        };
        GpsTimeFields {
            wn,
            ns_residual,
            flags,
        }
    }
}

impl Event for GpsTime {
    const MESSAGE_TYPES: &'static [u16] = &[MsgGpsTime::MESSAGE_TYPE, MsgGpsTimeDepA::MESSAGE_TYPE];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgGpsTime(m) => GpsTime::MsgGpsTime(m),
            Sbp::MsgGpsTimeDepA(m) => GpsTime::MsgGpsTimeDepA(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various Specan Message types.
pub struct SpecanFields {
    pub wn: u16,
    pub tow: u32,
    pub ns_residual: i32,
    pub amplitude_value: Vec<u8>,
    pub freq_ref: f32,
    pub freq_step: f32,
    pub amplitude_ref: f32,
    pub amplitude_unit: f32,
    pub channel_tag: u16,
}

// Enum wrapping around various Specan Message types.
#[derive(Debug, Clone)]
pub enum Specan {
    MsgSpecan(MsgSpecan),
    MsgSpecanDep(MsgSpecanDep),
}

impl Specan {
    pub fn fields(self) -> SpecanFields {
        let (
            wn,
            tow,
            ns_residual,
            amplitude_value,
            freq_ref,
            freq_step,
            amplitude_ref,
            amplitude_unit,
            channel_tag,
        ) = match self {
            Specan::MsgSpecan(msg_) => (
                msg_.t.wn,
                msg_.t.tow,
                msg_.t.ns_residual,
                msg_.amplitude_value,
                msg_.freq_ref,
                msg_.freq_step,
                msg_.amplitude_ref,
                msg_.amplitude_unit,
                msg_.channel_tag,
            ),
            Specan::MsgSpecanDep(msg_) => (
                msg_.t.wn,
                msg_.t.tow,
                /*msg_.t.ns_residual*/ 0,
                msg_.amplitude_value,
                msg_.freq_ref,
                msg_.freq_step,
                msg_.amplitude_ref,
                msg_.amplitude_unit,
                msg_.channel_tag,
            ),
        };
        SpecanFields {
            wn,
            tow,
            ns_residual,
            amplitude_value,
            freq_ref,
            freq_step,
            amplitude_ref,
            amplitude_unit,
            channel_tag,
        }
    }
}

impl Event for Specan {
    const MESSAGE_TYPES: &'static [u16] = &[MsgSpecan::MESSAGE_TYPE, MsgSpecanDep::MESSAGE_TYPE];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgSpecan(m) => Specan::MsgSpecan(m),
            Sbp::MsgSpecanDep(m) => Specan::MsgSpecanDep(m),
            _ => unreachable!(),
        }
    }
}

// Struct with shared fields for various VelNED Message types.
#[allow(clippy::upper_case_acronyms)]
pub struct VelNEDFields {
    pub flags: u8,
    pub tow: f64,
    pub n: i32,
    pub e: i32,
    pub d: i32,
    pub n_sats: u8,
}

// Enum wrapping around various Vel NED Message types.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum VelNED {
    MsgVelNed(MsgVelNed),
    MsgVelNedDepA(MsgVelNedDepA),
}

impl VelNED {
    pub fn fields(self) -> VelNEDFields {
        let (flags, tow, n, e, d, n_sats) = match self {
            VelNED::MsgVelNed(msg) => (msg.flags, msg.tow as f64, msg.n, msg.e, msg.d, msg.n_sats),
            VelNED::MsgVelNedDepA(msg) => (1, msg.tow as f64, msg.n, msg.e, msg.d, msg.n_sats),
        };
        VelNEDFields {
            flags,
            tow,
            n,
            e,
            d,
            n_sats,
        }
    }
}

impl Event for VelNED {
    const MESSAGE_TYPES: &'static [u16] = &[MsgVelNed::MESSAGE_TYPE, MsgVelNedDepA::MESSAGE_TYPE];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgVelNed(m) => VelNED::MsgVelNed(m),
            Sbp::MsgVelNedDepA(m) => VelNED::MsgVelNedDepA(m),
            _ => unreachable!(),
        }
    }
}

// Baseline Tab Types.

// Struct with shared fields for various BaselineNED Message types.
#[allow(clippy::upper_case_acronyms)]
pub struct BaselineNEDFields {
    pub flags: u8,
    pub tow: f64,
    pub n: i32,
    pub e: i32,
    pub d: i32,
    pub h_accuracy: u16,
    pub v_accuracy: u16,
    pub n_sats: u8,
}
// Enum wrapping around various Baseline NED Message types.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum BaselineNED {
    MsgBaselineNed(MsgBaselineNed),
    MsgBaselineNedDepA(MsgBaselineNedDepA),
}

impl BaselineNED {
    pub fn fields(&self) -> BaselineNEDFields {
        let (flags, tow, n, e, d, h_accuracy, v_accuracy, n_sats) = match self {
            BaselineNED::MsgBaselineNed(msg) => (
                msg.flags,
                msg.tow as f64,
                msg.n,
                msg.e,
                msg.d,
                msg.h_accuracy,
                msg.v_accuracy,
                msg.n_sats,
            ),
            BaselineNED::MsgBaselineNedDepA(msg) => (
                1,
                msg.tow as f64,
                msg.n,
                msg.e,
                msg.d,
                msg.h_accuracy,
                msg.v_accuracy,
                msg.n_sats,
            ),
        };
        BaselineNEDFields {
            flags,
            tow,
            n,
            e,
            d,
            h_accuracy,
            v_accuracy,
            n_sats,
        }
    }
    pub fn mode(&self) -> u8 {
        match self {
            BaselineNED::MsgBaselineNed(MsgBaselineNed { flags, .. })
            | BaselineNED::MsgBaselineNedDepA(MsgBaselineNedDepA { flags, .. }) => *flags & 0x7,
        }
    }
}

impl Event for BaselineNED {
    const MESSAGE_TYPES: &'static [u16] = &[
        MsgBaselineNed::MESSAGE_TYPE,
        MsgBaselineNedDepA::MESSAGE_TYPE,
    ];

    fn from_sbp(msg: Sbp) -> Self {
        match msg {
            Sbp::MsgBaselineNed(m) => BaselineNED::MsgBaselineNed(m),
            Sbp::MsgBaselineNedDepA(m) => BaselineNED::MsgBaselineNedDepA(m),
            _ => unreachable!(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deque_index() {
        let mut d = Deque::new(3);
        d.push(0);
        d.push(1);
        d.push(2);
        assert_eq!(d[0], 0);
        assert_eq!(d[1], 1);
        assert_eq!(d[2], 2);
        d.push(3);
        assert_eq!(d[0], 1);
        assert_eq!(d[1], 2);
        assert_eq!(d[2], 3);
        d.push(4);
        assert_eq!(d[0], 2);
        assert_eq!(d[1], 3);
        assert_eq!(d[2], 4);
        d.push(5);
        assert_eq!(d[0], 3);
        assert_eq!(d[1], 4);
        assert_eq!(d[2], 5);
    }

    #[test]
    fn test_deque_iter() {
        let mut d = Deque::new(3);
        d.push(0);
        d.push(1);
        d.push(2);
        {
            let mut it = d.iter();
            assert_eq!(it.next(), Some(&0));
            assert_eq!(it.next(), Some(&1));
            assert_eq!(it.next(), Some(&2));
            assert!(it.next().is_none());
        }
        d.push(3);
        {
            let mut it = d.iter();
            assert_eq!(it.next(), Some(&1));
            assert_eq!(it.next(), Some(&2));
            assert_eq!(it.next(), Some(&3));
            assert!(it.next().is_none());
        }
        d.push(4);
        {
            let mut it = d.iter();
            assert_eq!(it.next(), Some(&2));
            assert_eq!(it.next(), Some(&3));
            assert_eq!(it.next(), Some(&4));
            assert!(it.next().is_none());
        }
        d.push(5);
        {
            let mut it = d.iter();
            assert_eq!(it.next(), Some(&3));
            assert_eq!(it.next(), Some(&4));
            assert_eq!(it.next(), Some(&5));
            assert!(it.next().is_none());
        }
    }

    #[test]
    fn test_zip() {
        let x = Deque::with_fill_value(3, 0);
        let y = Deque::with_fill_value(3, 1);
        let z = Deque::with_fill_value(3, 2);
        for (x, y, z) in zip!(&x, &y, &z) {
            assert_eq!(*x, 0);
            assert_eq!(*y, 1);
            assert_eq!(*z, 2);
        }
    }

    #[test]
    #[should_panic]
    fn test_index_oob_panic() {
        let mut d = Deque::new(3);
        d.push(1);
        d.push(2);
        d.push(3);
        let _ = d[3];
    }
}
