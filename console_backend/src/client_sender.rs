use std::fmt;
use std::sync::Arc;

use crossbeam::channel::Sender;
use log::error;
use parking_lot::Mutex;

use crate::types::ArcBool;
use crate::utils;

pub type BoxedClientSender = Box<dyn ClientSender + 'static>;

pub trait ClientSender: ClientSenderClone {
    fn send_data(&self, msg_bytes: Vec<u8>);
    fn connected(&self) -> bool;
    fn set_connected(&self, connected: bool);
}

// enables trait object safe cloning
pub trait ClientSenderClone: fmt::Debug + Send + Sync {
    fn clone_box(&self) -> BoxedClientSender;
}

impl<T> ClientSenderClone for T
where
    T: ClientSender + Clone + 'static,
{
    fn clone_box(&self) -> BoxedClientSender {
        Box::new(self.clone())
    }
}

impl Clone for BoxedClientSender {
    fn clone(&self) -> BoxedClientSender {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct ChannelSender {
    inner: Sender<Vec<u8>>,
    connected: ArcBool,
}

impl ChannelSender {
    pub fn new(inner: Sender<Vec<u8>>) -> Self {
        Self {
            inner,
            connected: ArcBool::new_with(true),
        }
    }

    pub fn boxed(inner: Sender<Vec<u8>>) -> BoxedClientSender {
        Box::new(Self::new(inner))
    }
}

impl ClientSender for ChannelSender {
    fn send_data(&self, msg_bytes: Vec<u8>) {
        if self.connected.get() {
            utils::ok_or_log(self.inner.send(msg_bytes), |e| {
                error!("error dispatching data: {e:?}")
            });
        }
    }

    fn connected(&self) -> bool {
        self.connected.get()
    }

    fn set_connected(&self, connected: bool) {
        self.connected.set(connected);
    }
}

#[derive(Debug)]
pub struct TestSender {
    inner: Arc<Mutex<Vec<Vec<u8>>>>,
    connected: ArcBool,
}

impl TestSender {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
            connected: ArcBool::new_with(true),
        }
    }

    pub fn boxed() -> BoxedClientSender {
        Box::new(Self::new())
    }
}

impl Clone for TestSender {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            connected: self.connected.clone(),
        }
    }
}

impl Default for TestSender {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientSender for TestSender {
    fn send_data(&self, msg: Vec<u8>) {
        self.inner.lock().push(msg)
    }

    fn connected(&self) -> bool {
        self.connected.get()
    }

    fn set_connected(&self, connected: bool) {
        self.connected.set(connected);
    }
}
