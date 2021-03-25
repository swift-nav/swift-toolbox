use std::{collections::VecDeque, sync::mpsc::Sender};

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
