use std::fmt;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use parking_lot::{Condvar, Mutex};

pub struct Watched<T> {
    shared: Arc<Shared<T>>,
}

impl<T: Clone> Watched<T> {
    pub fn new(value: T) -> Watched<T> {
        Watched {
            shared: Arc::new(Shared {
                lock: Mutex::new(Value { value, version: 1 }),
                on_update: Condvar::new(),
                closed: AtomicBool::new(false),
            }),
        }
    }

    pub fn get(&self) -> T {
        self.shared.lock.lock().value.clone()
    }

    pub fn send(&self, value: T) {
        {
            let mut lock = self.shared.lock.lock();
            lock.value = value;
            lock.version = lock.version.wrapping_add(1);
        }
        self.shared.on_update.notify_all();
    }

    pub fn watch(&self) -> WatchReceiver<T> {
        let version = {
            let lock = self.shared.lock.lock();
            lock.version
        };
        WatchReceiver {
            shared: Arc::clone(&self.shared),
            last_seen: version,
        }
    }
}

impl<T> fmt::Debug for Watched<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Watched").finish()
    }
}

impl<T> Drop for Watched<T> {
    fn drop(&mut self) {
        self.shared.close();
    }
}

pub struct WatchReceiver<T> {
    shared: Arc<Shared<T>>,
    last_seen: u64,
}

impl<T: Clone> WatchReceiver<T> {
    pub fn get(&mut self) -> T {
        let lock = self.shared.lock.lock();
        self.last_seen = lock.version;
        lock.value.clone()
    }

    pub fn get_if_new(&mut self) -> Option<T> {
        let lock = self.shared.lock.lock();
        if self.last_seen == lock.version {
            None
        } else {
            self.last_seen = lock.version;
            Some(lock.value.clone())
        }
    }

    pub fn wait(&mut self) -> Result<T, RecvError> {
        if self.shared.closed() {
            return Err(RecvError);
        }
        let mut lock = self.shared.lock.lock();
        while lock.version == self.last_seen {
            self.shared.on_update.wait(&mut lock);
            if self.shared.closed() {
                return Err(RecvError);
            }
        }
        self.last_seen = lock.version;
        Ok(lock.value.clone())
    }

    pub fn wait_while<F: FnMut(&T) -> bool>(&mut self, mut f: F) -> Result<T, RecvError> {
        loop {
            let v = self.wait()?;
            if !f(&v) {
                return Ok(v);
            }
        }
    }
}

impl<T> Clone for WatchReceiver<T> {
    fn clone(&self) -> WatchReceiver<T> {
        WatchReceiver {
            shared: Arc::clone(&self.shared),
            last_seen: self.last_seen,
        }
    }
}

struct Shared<T> {
    lock: Mutex<Value<T>>,
    on_update: Condvar,
    closed: AtomicBool,
}

impl<T> Shared<T> {
    fn closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.on_update.notify_all();
    }
}

struct Value<T> {
    value: T,
    version: u64,
}

#[derive(Debug)]
pub struct RecvError;

impl fmt::Display for RecvError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "watched value dropped")
    }
}

impl std::error::Error for RecvError {}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crossbeam::channel;

    use super::*;

    #[test]
    fn starts_seen() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();
        assert!(recv.get_if_new().is_none());
    }

    #[test]
    fn wait() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();

        assert_eq!(watched.get(), 0);
        assert_eq!(recv.get(), 0);

        watched.send(1);
        assert_eq!(watched.get(), 1);

        let send_thread = thread::spawn(move || {
            watched.send(2);
            watched
        });
        recv.wait().unwrap();

        let watched = send_thread.join().unwrap();
        let recv_thread = thread::spawn(move || {
            recv.wait().unwrap();
            recv
        });
        watched.send(3);

        let mut recv = recv_thread.join().unwrap();
        assert_eq!(recv.get(), 3);
        assert_eq!(watched.get(), 3);
    }

    #[test]
    fn wait_while() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();
        let (s, r) = channel::bounded(0);
        thread::spawn(move || {
            let v = recv.wait_while(|v| *v <= 2).unwrap();
            assert_eq!(v, 2);
            s.send(()).unwrap()
        });

        thread::sleep(Duration::from_secs(1));
        assert!(r.try_recv().is_err());

        watched.send(1);
        thread::sleep(Duration::from_secs(1));
        assert!(r.try_recv().is_err());

        watched.send(2);
        thread::sleep(Duration::from_secs(1));
        assert!(r.try_recv().is_ok());
    }

    #[test]
    fn disconnect() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            drop(watched);
        });
        assert!(recv.wait().is_err());
    }

    #[test]
    fn multiple_consumers() {
        let watched = Watched::new(0);
        let mut recv1 = watched.watch();
        let mut recv2 = watched.watch();

        watched.send(1);
        assert_eq!(recv1.get_if_new(), Some(1));
        assert_eq!(recv2.get_if_new(), Some(1));

        watched.send(2);
        assert_eq!(recv1.get_if_new(), Some(2));

        watched.send(3);
        assert_eq!(recv1.get_if_new(), Some(3));
        assert_eq!(recv2.get_if_new(), Some(3));

        drop(watched);
        assert!(recv1.wait().is_err());
        assert!(recv2.wait().is_err());
    }

    #[test]
    fn clone_recv() {
        let watched = Watched::new(0);
        let mut recv1 = watched.watch();

        watched.send(1);

        let mut recv2 = recv1.clone();
        assert_eq!(recv2.get_if_new(), Some(1));

        let mut recv3 = recv2.clone();
        assert_eq!(recv3.get_if_new(), None);
        assert_eq!(recv1.get_if_new(), Some(1));

        watched.send(2);
        assert_eq!(recv1.get_if_new(), Some(2));
        assert_eq!(recv2.get_if_new(), Some(2));
    }
}
