use std::fmt;

use crossbeam::channel::Sender;

use crate::types::ArcBool;

pub type BoxedClientSender = Box<dyn ClientSender + 'static>;

pub trait ClientSender: ClientSenderClone {
    fn send_data(&mut self, msg_bytes: Vec<u8>);
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
    fn send_data(&mut self, msg_bytes: Vec<u8>) {
        if self.connected.get() {
            let _ = self.inner.send(msg_bytes);
        }
    }

    fn connected(&self) -> bool {
        self.connected.get()
    }

    fn set_connected(&self, connected: bool) {
        self.connected.set(connected);
    }
}

#[derive(Debug, Clone)]
pub struct TestSender {
    inner: Vec<Vec<u8>>,
    connected: ArcBool,
}

impl TestSender {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            connected: ArcBool::new_with(true),
        }
    }

    pub fn boxed() -> BoxedClientSender {
        Box::new(Self::new())
    }
}

impl Default for TestSender {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientSender for TestSender {
    fn send_data(&mut self, msg: Vec<u8>) {
        self.inner.push(msg)
    }

    fn connected(&self) -> bool {
        self.connected.get()
    }

    fn set_connected(&self, connected: bool) {
        self.connected.set(connected);
    }
}
