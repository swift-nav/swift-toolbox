use std::{convert::TryInto, marker::PhantomData, sync::Arc, time::Duration};

use crossbeam::channel;
use parking_lot::Mutex;
use sbp::{
    messages::{ConcreteMessage, SBPMessage, SBP},
    time::{GpsTime, GpsTimeError},
};
use slotmap::{DenseSlotMap, HopSlotMap};

type MaybeGpsTime = Option<Result<GpsTime, GpsTimeError>>;

pub struct Broadcaster {
    channels: Arc<Mutex<HopSlotMap<KeyInner, Sender>>>,
}

impl Broadcaster {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HopSlotMap::with_key())),
        }
    }

    pub fn send(&self, message: &SBP, gps_time: MaybeGpsTime) {
        let msg_type = message.get_message_type();
        let mut channels = self.channels.lock();
        channels.retain(|_, chan| {
            if chan.msg_types.iter().any(|ty| ty == &msg_type) {
                chan.inner.send((message.clone(), gps_time.clone())).is_ok()
            } else {
                true
            }
        });
    }

    pub fn subscribe<E>(&self) -> (Receiver<E>, Key)
    where
        E: Event,
    {
        let (tx, rx) = channel::unbounded();
        let mut channels = self.channels.lock();
        let key = channels.insert(Sender {
            inner: tx,
            msg_types: E::MESSAGE_TYPES,
        });
        (
            Receiver {
                inner: rx,
                marker: PhantomData,
            },
            Key { inner: key },
        )
    }

    pub fn unsubscribe(&self, key: Key) {
        let mut channels = self.channels.lock();
        channels.remove(key.inner);
    }

    /// Wait once for a specific message. Returns an error if `dur` elapses.
    pub fn wait<E>(&self, dur: Duration) -> Result<(E, MaybeGpsTime), channel::RecvTimeoutError>
    where
        E: Event,
    {
        let (rx, _) = self.subscribe::<E>();
        rx.recv_timeout(dur)
    }
}

impl Clone for Broadcaster {
    fn clone(&self) -> Self {
        Self {
            channels: Arc::clone(&self.channels),
        }
    }
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

/// A wrapper around a channel sender that knows what message types its receivers expect.
struct Sender {
    inner: channel::Sender<(SBP, MaybeGpsTime)>,
    msg_types: &'static [u16],
}

/// A wrapper around a channel receiver that converts to the appropriate event.
pub struct Receiver<E> {
    inner: channel::Receiver<(SBP, MaybeGpsTime)>,
    marker: PhantomData<E>,
}

slotmap::new_key_type! {
    struct KeyInner;
}

/// Returned along with calls to `subscribe`. Used to unsubscribe to an event.
pub struct Key {
    inner: KeyInner,
}

impl<E> Receiver<E>
where
    E: Event,
{
    pub fn recv(&self) -> Result<(E, MaybeGpsTime), channel::RecvError> {
        let msg = self.inner.recv()?;
        Ok((E::from_sbp(msg.0), msg.1))
    }

    pub fn recv_timeout(
        &self,
        dur: Duration,
    ) -> Result<(E, MaybeGpsTime), channel::RecvTimeoutError> {
        let msg = self.inner.recv_timeout(dur)?;
        Ok((E::from_sbp(msg.0), msg.1))
    }

    pub fn try_recv(&self) -> Result<(E, MaybeGpsTime), channel::TryRecvError> {
        let msg = self.inner.try_recv()?;
        Ok((E::from_sbp(msg.0), msg.1))
    }

    pub fn iter(&self) -> impl Iterator<Item = (E, MaybeGpsTime)> + '_ {
        self.inner.iter().map(|msg| (E::from_sbp(msg.0), msg.1))
    }

    pub fn try_iter(&self) -> impl Iterator<Item = (E, MaybeGpsTime)> + '_ {
        self.inner.try_iter().map(|msg| (E::from_sbp(msg.0), msg.1))
    }

    // other channel methods as needed
}

/// An event you can receive via a `Broadcaster`. It's implemented for all SBP
/// message types, but could also be implemented for composite messages like `Observations`.
pub trait Event {
    /// The message types from which the event can be derived.
    const MESSAGE_TYPES: &'static [u16];

    /// Conversion from SBP. The message type of `msg` is guaranteed to be in
    /// `Self::MESSAGE_TYPES`.
    fn from_sbp(msg: SBP) -> Self;
}

// All concrete message types can be used as events.
impl<T> Event for T
where
    T: ConcreteMessage,
{
    const MESSAGE_TYPES: &'static [u16] = &[T::MESSAGE_TYPE];

    fn from_sbp(msg: SBP) -> Self {
        msg.try_into().unwrap()
    }
}

pub trait Handler<Event, Kind> {
    fn run(&mut self, event: Event, time: MaybeGpsTime);
}

pub struct WithTime;

impl<F, E> Handler<E, WithTime> for F
where
    F: FnMut(E, MaybeGpsTime),
    E: Event,
{
    fn run(&mut self, event: E, time: MaybeGpsTime) {
        (self)(event, time)
    }
}

pub struct WithoutTime;

impl<F, E> Handler<E, WithoutTime> for F
where
    F: FnMut(E),
    E: Event,
{
    fn run(&mut self, event: E, _time: MaybeGpsTime) {
        (self)(event)
    }
}

pub struct OnlyTime;

impl<F, E> Handler<E, OnlyTime> for F
where
    F: FnMut(GpsTime),
    E: Event,
{
    fn run(&mut self, _event: E, time: MaybeGpsTime) {
        if let Some(Ok(time)) = time {
            (self)(time)
        }
    }
}

enum Callback<'a> {
    Id {
        func: Box<dyn FnMut(SBP, MaybeGpsTime) + Send + 'a>,
        msg_type: u16,
    },
    Event {
        func: Box<dyn FnMut(SBP, MaybeGpsTime) + Send + 'a>,
        msg_types: &'static [u16],
    },
}

impl<'a> Callback<'a> {
    fn run(&mut self, msg: SBP, time: MaybeGpsTime) {
        match self {
            Callback::Id { func, .. } | Callback::Event { func, .. } => (func)(msg, time),
        }
    }

    fn should_run(&self, msg: u16) -> bool {
        match self {
            Callback::Id { msg_type, .. } => msg_type == &msg,
            Callback::Event { msg_types, .. } => msg_types.contains(&msg),
        }
    }
}

pub fn with_link<'env, F>(f: F)
where
    F: FnOnce(&LinkSource<'env>),
{
    let source: LinkSource<'env> = LinkSource { link: Link::new() };
    f(&source);
}

pub struct LinkSource<'env> {
    link: Link<'env>,
}

impl<'env> LinkSource<'env> {
    pub fn link<'scope>(&self) -> Link<'scope>
    where
        'env: 'scope,
    {
        let link: Link<'scope> = unsafe { std::mem::transmute(self.link.clone()) };
        link
    }

    pub fn send(&self, message: &SBP, gps_time: MaybeGpsTime) -> bool {
        self.link.send(message, gps_time)
    }
}

impl<'env> Drop for LinkSource<'env> {
    fn drop(&mut self) {
        self.link.callbacks.lock().clear();
    }
}

pub struct Link<'a> {
    callbacks: Arc<Mutex<DenseSlotMap<KeyInner, Callback<'a>>>>,
}

impl<'a> Link<'a> {
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(Mutex::new(DenseSlotMap::with_key())),
        }
    }

    pub fn send(&self, message: &SBP, gps_time: MaybeGpsTime) -> bool {
        let msg_type = message.get_message_type();
        let mut cbs = self.callbacks.lock();
        let to_call = cbs.values_mut().filter(|cb| cb.should_run(msg_type));
        let mut called = false;
        for cb in to_call {
            cb.run(message.clone(), gps_time.clone());
            called = true;
        }
        called
    }

    pub fn register_cb_by_id<H>(&self, id: u16, mut handler: H) -> Key
    where
        H: FnMut(SBP) + Send + 'a,
    {
        let mut cbs = self.callbacks.lock();
        let inner = cbs.insert(Callback::Id {
            func: Box::new(move |msg, _time| (handler)(msg)),
            msg_type: id,
        });
        Key { inner }
    }

    pub fn register_cb<H, E, K>(&self, mut handler: H) -> Key
    where
        H: Handler<E, K> + Send + 'a,
        E: Event,
    {
        let mut cbs = self.callbacks.lock();
        let inner = cbs.insert(Callback::Event {
            func: Box::new(move |msg, time| {
                let event = E::from_sbp(msg);
                handler.run(event, time)
            }),
            msg_types: E::MESSAGE_TYPES,
        });
        Key { inner }
    }

    pub fn unregister_cb(&self, key: Key) {
        let mut cbs = self.callbacks.lock();
        cbs.remove(key.inner);
    }
}

impl Clone for Link<'_> {
    fn clone(&self) -> Self {
        Self {
            callbacks: Arc::clone(&self.callbacks),
        }
    }
}

#[cfg(test)]
mod tests {
    use crossbeam::scope;
    use sbp::messages::{
        self,
        observation::{MsgObs, MsgObsDepA, ObservationHeader, ObservationHeaderDep},
    };

    use super::*;

    struct Counter {
        count: usize,
    }

    impl Counter {
        fn obs(&mut self, _: MsgObs) {
            self.count += 1;
        }
    }

    #[test]
    fn test_dispatcher() {
        let d = Link::new();
        let mut c = Counter { count: 0 };
        d.register_cb(|obs| c.obs(obs));
        d.send(&make_msg_obs(), None);
        d.send(&make_msg_obs_dep_a(), None);
        drop(d);
        assert_eq!(c.count, 1);
    }

    #[test]
    fn test_broadcaster() {
        let b = Broadcaster::new();
        let (msg_obs, _) = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs(), None);
        b.send(&make_msg_obs_dep_a(), None);

        assert!(msg_obs.try_recv().is_ok());
        assert!(msg_obs.try_recv().is_err());
    }

    #[test]
    fn test_multiple_subs() {
        let b = Broadcaster::new();
        let (msg_obs1, _) = b.subscribe::<MsgObs>();
        let (msg_obs2, _) = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs(), None);
        b.send(&make_msg_obs(), None);

        assert_eq!(msg_obs1.try_iter().count(), 2);
        assert_eq!(msg_obs2.try_iter().count(), 2);
    }

    #[test]
    fn test_unsubscribe() {
        let b = Broadcaster::new();

        let (msg_obs1, key) = b.subscribe::<MsgObs>();
        let (msg_obs2, _) = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs(), None);
        assert_eq!(msg_obs1.try_iter().count(), 1);
        assert_eq!(msg_obs2.try_iter().count(), 1);

        b.unsubscribe(key);

        b.send(&make_msg_obs(), None);
        assert_eq!(msg_obs1.try_iter().count(), 0);
        assert_eq!(msg_obs2.try_iter().count(), 1);
    }

    #[test]
    fn test_iter_subscription() {
        let b = Broadcaster::new();

        let (msg_obs, key) = b.subscribe::<MsgObs>();

        scope(|s| {
            s.spawn(|_| {
                // initial non-blocking call should have no messages
                assert_eq!(msg_obs.try_iter().count(), 0);

                // blocking call will get the two messages
                assert_eq!(msg_obs.iter().count(), 2);
            });

            std::thread::sleep(Duration::from_secs(1));

            b.send(&make_msg_obs(), None);
            b.send(&make_msg_obs_dep_a(), None);
            b.send(&make_msg_obs(), None);

            // msg_obs.iter() goes forever if you don't drop the channel
            b.unsubscribe(key);
        })
        .unwrap();
    }

    #[test]
    fn test_wait() {
        let b = Broadcaster::new();

        scope(|s| {
            s.spawn(|_| {
                std::thread::sleep(Duration::from_secs(1));
                b.send(&make_msg_obs(), None)
            });

            assert!(b.wait::<MsgObs>(Duration::from_secs(2)).is_ok());
        })
        .unwrap()
    }

    #[test]
    fn test_wait_timeout() {
        let b = Broadcaster::new();

        scope(|s| {
            s.spawn(|_| {
                std::thread::sleep(Duration::from_secs(2));
                b.send(&make_msg_obs(), None)
            });

            assert!(b.wait::<MsgObs>(Duration::from_secs(1)).is_err());
        })
        .unwrap()
    }

    #[test]
    fn test_custom_event() {
        let b = Broadcaster::new();

        let (obs_msg, _) = b.subscribe::<ObsMsg>();
        let (msg_obs, _) = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs(), None);
        b.send(&make_msg_obs_dep_a(), None);

        // ObsMsg should accept both MsgObs and MsgObsDepA
        assert!(obs_msg.try_recv().is_ok());
        assert!(obs_msg.try_recv().is_ok());
        assert!(obs_msg.try_recv().is_err());

        // just MsgObs
        assert!(msg_obs.try_recv().is_ok());
        assert!(msg_obs.try_recv().is_err());
    }

    enum ObsMsg {
        Obs(MsgObs),
        DepA(MsgObsDepA),
    }

    impl Event for ObsMsg {
        const MESSAGE_TYPES: &'static [u16] = &[MsgObs::MESSAGE_TYPE, MsgObsDepA::MESSAGE_TYPE];

        fn from_sbp(msg: SBP) -> Self {
            match msg {
                SBP::MsgObs(m) => ObsMsg::Obs(m),
                SBP::MsgObsDepA(m) => ObsMsg::DepA(m),
                _ => unreachable!("wrong event keys"),
            }
        }
    }

    fn make_msg_obs() -> SBP {
        MsgObs {
            sender_id: Some(1),
            header: ObservationHeader {
                t: messages::gnss::GPSTime {
                    tow: 1,
                    ns_residual: 1,
                    wn: 1,
                },
                n_obs: 1,
            },
            obs: vec![],
        }
        .into()
    }

    fn make_msg_obs_dep_a() -> SBP {
        MsgObsDepA {
            sender_id: Some(1),
            header: ObservationHeaderDep {
                t: messages::gnss::GPSTimeDep { tow: 1, wn: 1 },
                n_obs: 1,
            },
            obs: vec![],
        }
        .into()
    }
}
