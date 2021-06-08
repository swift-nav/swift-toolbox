use std::{
    convert::TryInto,
    marker::PhantomData,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossbeam::channel;
use sbp::messages::{ConcreteMessage, SBP};

pub struct Broadcaster {
    channels: Arc<Mutex<Vec<channel::Sender<SBP>>>>,
}

impl Broadcaster {
    const CHANNELS_LOCK_FAILURE: &'static str = "failed to aquire lock on channels";

    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn send(&self, message: &SBP) {
        let mut channels = self.channels.lock().expect(Self::CHANNELS_LOCK_FAILURE);
        channels.retain(|chan| chan.send(message.clone()).is_ok());
    }

    pub fn subscribe<E>(&self) -> Receiver<E>
    where
        E: Event,
    {
        let (tx, rx) = channel::unbounded();
        let mut channels = self.channels.lock().expect(Self::CHANNELS_LOCK_FAILURE);
        channels.push(tx);
        Receiver {
            inner: rx,
            marker: PhantomData,
        }
    }

    /// Wait once for a specific message. Returns an error if `dur` elapses.
    pub fn wait<E>(&self, dur: Duration) -> Result<E, channel::RecvTimeoutError>
    where
        E: Event,
    {
        let rx = self.subscribe::<E>();
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

/// A wrapper around a channel receiver that converts to the appropriate event.
pub struct Receiver<E> {
    inner: channel::Receiver<SBP>,
    marker: PhantomData<E>,
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
    use sbp::messages::{
        self,
        observation::{MsgObs, MsgObsDepA, ObservationHeader, ObservationHeaderDep},
    };

    use super::*;

    #[test]
    fn test_broadcaster() {
        let b = Broadcaster::new();
        let msg_obs = b.subscribe::<MsgObs>();

        b.send(&make_msg_obs());
        b.send(&make_msg_obs_dep_a());

        assert!(msg_obs.try_recv().is_ok());
        assert!(msg_obs.try_recv().is_err());
    }

    #[test]
    fn test_custom_event() {
        let b = Broadcaster::new();

        let obs_msg = b.subscribe::<ObsMsg>();
        let msg_obs = b.subscribe::<MsgObs>();

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
