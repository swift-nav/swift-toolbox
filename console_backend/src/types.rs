use std::{collections::VecDeque, sync::mpsc::Sender};

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

pub trait TabBackend<P: ProtoMsgSender> {
    fn send_data(&mut self, client_send: P);
}

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
}
impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            tracking_tab: TrackingTabState::new(),
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
    pub check_visibility: Vec<String>,
}

impl TrackingTabState {
    fn new() -> TrackingTabState {
        TrackingTabState {
            check_visibility: vec![],
        }
    }
}
