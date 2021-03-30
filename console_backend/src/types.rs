use std::{collections::VecDeque, sync::mpsc::Sender};

pub type Error = std::boxed::Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub type Deque<T> = VecDeque<T>;
pub trait DequeExt<T> {
    fn add(&mut self, ele: T);
}
impl<T> DequeExt<T> for Deque<T> {
    fn add(&mut self, ele: T) {
        if self.len() == self.capacity() {
            self.pop_front();
        }
        self.push_back(ele);
    }
}

pub trait TabBackend {
    fn send_data(&mut self, client_send: Sender<Vec<u8>>);
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
