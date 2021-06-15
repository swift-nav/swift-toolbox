use std::{
    convert::TryInto,
    marker::PhantomData,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossbeam::channel;
use sbp::messages::{ConcreteMessage, SBPMessage, SBP};
use slotmap::HopSlotMap;

pub struct Broadcaster {
    channels: Arc<Mutex<HopSlotMap<KeyInner, Sender>>>,
}

impl Broadcaster {
    const CHANNELS_LOCK_FAILURE: &'static str = "failed to aquire lock on channels";

    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HopSlotMap::with_key())),
        }
    }

    pub fn send(&self, message: &SBP) {
        let msg_type = message.get_message_type();
        let mut channels = self.channels.lock().expect(Self::CHANNELS_LOCK_FAILURE);
        channels.retain(|_, chan| {
            if chan.msg_types.iter().any(|ty| ty == &msg_type) {
                chan.inner.send(message.clone()).is_ok()
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
        let mut channels = self.channels.lock().expect(Self::CHANNELS_LOCK_FAILURE);
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
        let mut channels = self.channels.lock().expect(Self::CHANNELS_LOCK_FAILURE);
        channels.remove(key.inner);
    }

    /// Wait once for a specific message. Returns an error if `dur` elapses.
    pub fn wait<E>(&self, dur: Duration) -> Result<E, channel::RecvTimeoutError>
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
    inner: channel::Sender<SBP>,
    msg_types: &'static [u16],
}

/// A wrapper around a channel receiver that converts to the appropriate event.
pub struct Receiver<E> {
    inner: channel::Receiver<SBP>,
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
    pub fn recv(&self) -> Result<E, channel::RecvError> {
        let msg = self.inner.recv()?;
        Ok(E::from_sbp(msg))
    }

    pub fn recv_timeout(&self, dur: Duration) -> Result<E, channel::RecvTimeoutError> {
        let msg = self.inner.recv_timeout(dur)?;
        Ok(E::from_sbp(msg))
    }

    pub fn try_recv(&self) -> Result<E, channel::TryRecvError> {
        let msg = self.inner.try_recv()?;
        Ok(E::from_sbp(msg))
    }

    pub fn iter(&self) -> impl Iterator<Item = E> + '_ {
        self.inner.iter().map(E::from_sbp)
    }

    pub fn try_iter(&self) -> impl Iterator<Item = E> + '_ {
        self.inner.try_iter().map(E::from_sbp)
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

#[cfg(test)]
mod tests {
    use crossbeam::scope;
    use sbp::messages::{
        self,
        observation::{MsgObs, MsgObsDepA, ObservationHeader, ObservationHeaderDep},
    };

    use super::*;

    #[test]
    fn test_broadcaster() {
        let b = Broadcaster::new();
        let (msg_obs, _) = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs());
        b.send(&make_msg_obs_dep_a());

        assert!(msg_obs.try_recv().is_ok());
        assert!(msg_obs.try_recv().is_err());
    }

    #[test]
    fn test_multiple_subs() {
        let b = Broadcaster::new();
        let (msg_obs1, _) = b.subscribe::<MsgObs>();
        let (msg_obs2, _) = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs());
        b.send(&make_msg_obs());

        assert_eq!(msg_obs1.try_iter().count(), 2);
        assert_eq!(msg_obs2.try_iter().count(), 2);
    }

    #[test]
    fn test_unsubscribe() {
        let b = Broadcaster::new();

        let (msg_obs1, key) = b.subscribe::<MsgObs>();
        let (msg_obs2, _) = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs());
        assert_eq!(msg_obs1.try_iter().count(), 1);
        assert_eq!(msg_obs2.try_iter().count(), 1);

        b.unsubscribe(key);

        b.send(&make_msg_obs());
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

            b.send(&make_msg_obs());
            b.send(&make_msg_obs_dep_a());
            b.send(&make_msg_obs());

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
                b.send(&make_msg_obs())
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
                b.send(&make_msg_obs())
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

        b.send(&make_msg_obs());
        b.send(&make_msg_obs_dep_a());

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
