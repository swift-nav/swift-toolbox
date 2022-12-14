// Copyright (c) 2022 Swift Navigation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{
    fmt,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use parking_lot::{Condvar, MappedMutexGuard, Mutex, MutexGuard};

pub struct Watched<T> {
    shared: Arc<Shared<T>>,
}

impl<T: Clone> Watched<T> {
    pub fn new(value: T) -> Watched<T> {
        Watched {
            shared: Arc::new(Shared {
                data: Mutex::new(Value { value, version: 1 }),
                on_update: Condvar::new(),
                closed: AtomicBool::new(false),
                senders: AtomicUsize::new(1),
            }),
        }
    }

    pub fn get(&self) -> T {
        self.shared.data.lock().value.clone()
    }

    pub fn send(&self, value: T) {
        {
            let mut data = self.shared.data.lock();
            data.value = value;
            data.version = data.version.wrapping_add(1);
        }
        self.shared.on_update.notify_all();
    }

    pub fn watch(&self) -> WatchReceiver<T> {
        let version = {
            let data = self.shared.data.lock();
            data.version
        };
        WatchReceiver {
            shared: Arc::clone(&self.shared),
            last_seen: version.wrapping_sub(1),
        }
    }

    pub fn close(&self) {
        self.shared.close();
    }
}

impl<T> Clone for Watched<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::SeqCst);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T: Clone + Default> Default for Watched<T> {
    fn default() -> Self {
        Watched::new(Default::default())
    }
}

impl<T> fmt::Debug for Watched<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Watched").finish()
    }
}

impl<T> Drop for Watched<T> {
    fn drop(&mut self) {
        // if we are the last sender close the channel so things calling
        // `.wait()` will terminate
        if self.shared.senders.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.shared.close();
        }
    }
}

pub struct WatchReceiver<T> {
    shared: Arc<Shared<T>>,
    last_seen: u64,
}

impl<T: Clone> WatchReceiver<T> {
    pub fn get(&mut self) -> Result<T, RecvError> {
        if self.shared.is_closed() {
            return Err(RecvError);
        }
        let data = self.shared.data.lock();
        self.last_seen = data.version;
        Ok(data.value.clone())
    }

    pub fn get_if_new(&mut self) -> Result<Option<T>, RecvError> {
        if self.shared.is_closed() {
            return Err(RecvError);
        }
        let data = self.shared.data.lock();
        if self.last_seen == data.version {
            Ok(None)
        } else {
            self.last_seen = data.version;
            Ok(Some(data.value.clone()))
        }
    }

    pub fn wait(&mut self) -> Result<T, RecvError> {
        let data = self.wait_inner()?;
        Ok(data.value.clone())
    }

    pub fn wait_for(&mut self, duration: Duration) -> Result<Result<T, RecvError>, TimeoutError> {
        let data = self.wait_for_inner(duration)?;
        match data {
            Ok(data) => Ok(Ok(data.value.clone())),
            Err(e) => Ok(Err(e)),
        }
    }

    pub fn wait_mut(&mut self) -> Result<MappedMutexGuard<T>, RecvError> {
        let data = self.wait_inner()?;
        Ok(MutexGuard::map(data, |d| &mut d.value))
    }

    pub fn wait_until<F>(&mut self, mut f: F) -> Result<T, RecvError>
    where
        F: FnMut(&T) -> bool,
    {
        loop {
            let v = self.wait()?;
            if f(&v) {
                return Ok(v);
            }
        }
    }

    pub fn wait_while<F>(&mut self, mut f: F) -> Result<T, RecvError>
    where
        F: FnMut(&T) -> bool,
    {
        self.wait_until(|v| !f(v))
    }

    fn wait_inner(&mut self) -> Result<MutexGuard<Value<T>>, RecvError> {
        if self.shared.is_closed() {
            return Err(RecvError);
        }
        let mut data = self.shared.data.lock();
        while data.version == self.last_seen {
            self.shared.on_update.wait(&mut data);
            if self.shared.is_closed() {
                return Err(RecvError);
            }
        }
        self.last_seen = data.version;
        Ok(data)
    }

    fn wait_for_inner(
        &mut self,
        duration: Duration,
    ) -> Result<Result<MutexGuard<Value<T>>, RecvError>, TimeoutError> {
        if self.shared.is_closed() {
            return Ok(Err(RecvError));
        }
        let t = Instant::now();
        let mut data = self.shared.data.lock();
        while data.version == self.last_seen {
            let elapsed = t.elapsed();
            if elapsed >= duration {
                return Err(TimeoutError);
            }
            if self
                .shared
                .on_update
                .wait_for(&mut data, duration - elapsed)
                .timed_out()
            {
                return Err(TimeoutError);
            }
            if self.shared.is_closed() {
                return Ok(Err(RecvError));
            }
        }
        self.last_seen = data.version;
        Ok(Ok(data))
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
    data: Mutex<Value<T>>,
    on_update: Condvar,
    closed: AtomicBool,
    senders: AtomicUsize,
}

impl<T> Shared<T> {
    fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.on_update.notify_all();
    }

    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
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

#[derive(Debug)]
pub struct TimeoutError;

impl fmt::Display for TimeoutError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "time out waiting for watched value")
    }
}

impl std::error::Error for TimeoutError {}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crossbeam::channel;

    use super::*;

    const SECOND: Duration = Duration::from_secs(1);

    #[test]
    fn starts_unseen() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();
        assert_eq!(recv.get_if_new().unwrap(), Some(0));
    }

    #[test]
    fn wait() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();

        assert_eq!(watched.get(), 0);
        assert_eq!(recv.get().unwrap(), 0);

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
        assert_eq!(recv.get().unwrap(), 3);
        assert_eq!(watched.get(), 3);
    }

    #[test]
    fn wait_while() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();
        let (s, r) = channel::bounded(0);
        thread::spawn(move || {
            let v = recv.wait_while(|v| *v < 2).unwrap();
            assert_eq!(v, 2);
            s.send(()).unwrap()
        });

        thread::sleep(SECOND);
        assert!(r.try_recv().is_err());

        watched.send(1);
        thread::sleep(SECOND);
        assert!(r.try_recv().is_err());

        watched.send(2);
        thread::sleep(SECOND);
        assert!(r.try_recv().is_ok());
    }

    #[test]
    fn disconnect_watch() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();
        // mark first as seen
        let _ = recv.get();
        thread::spawn(move || {
            thread::sleep(SECOND);
            drop(watched);
        });
        assert!(recv.wait().is_err());
    }

    #[test]
    fn wait_for() {
        let watched = Watched::new(0);
        let mut recv = watched.watch();
        let h = thread::spawn(move || {
            // mark first as seen
            let _ = recv.get();
            let res = recv.wait_for(SECOND / 2);
            assert!(matches!(res, Err(TimeoutError)));
            // wait for rest of the second plus a little bit
            let res = recv.wait_for(SECOND / 2 + Duration::from_millis(100));
            assert!(matches!(res, Ok(Ok(1))));
        });
        thread::sleep(SECOND);
        watched.send(1);
        h.join().unwrap();
    }

    #[test]
    fn multiple_consumers() {
        let watched = Watched::new(0);
        let mut recv1 = watched.watch();
        let mut recv2 = watched.watch();

        watched.send(1);
        assert_eq!(recv1.get_if_new().unwrap(), Some(1));
        assert_eq!(recv2.get_if_new().unwrap(), Some(1));

        watched.send(2);
        assert_eq!(recv1.get_if_new().unwrap(), Some(2));

        watched.send(3);
        assert_eq!(recv1.get_if_new().unwrap(), Some(3));
        assert_eq!(recv2.get_if_new().unwrap(), Some(3));

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
        assert_eq!(recv2.get_if_new().unwrap(), Some(1));

        let mut recv3 = recv2.clone();
        assert_eq!(recv3.get_if_new().unwrap(), None);
        assert_eq!(recv1.get_if_new().unwrap(), Some(1));

        watched.send(2);
        assert_eq!(recv1.get_if_new().unwrap(), Some(2));
        assert_eq!(recv2.get_if_new().unwrap(), Some(2));
    }
}
