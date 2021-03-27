use capnp::message::Builder;
use capnp::serialize;
use ordered_float::OrderedFloat;
use sbp::messages::SBP;
use std::sync::{mpsc, Arc, Mutex};

use crate::console_backend_capnp as m;

use crate::tracking_tab::*;
use crate::types::SharedState;

pub fn process_messages(
    messages: impl Iterator<Item = sbp::Result<SBP>>,
    shared_state: &Arc<Mutex<SharedState>>,
    client_send_clone: mpsc::Sender<Vec<u8>>,
) {
    let mut hpoints: Vec<(f64, OrderedFloat<f64>)> = vec![];
    let mut vpoints: Vec<(f64, OrderedFloat<f64>)> = vec![];
    let mut tow: f64;
    let shared_state_clone = Arc::clone(&shared_state);
    // let shared_state_clone2 = Arc::clone(&shared_state);
    let mut tracking_signals = TrackingSignalsTab::new(&shared_state_clone);
    // let mut tracking_signals2 = TrackingSignalsTab::new(&shared_state_clone2);

    for message in messages {
        match message {
            Ok(SBP::MsgTrackingState(msg)) => {
                tracking_signals
                    .handle_msg_tracking_state(msg.states.clone(), client_send_clone.clone());
            }
            Ok(SBP::MsgObs(msg)) => {
                tracking_signals.handle_obs(
                    ObservationMsg::MsgObs(msg.clone()),
                    client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgMeasurementState(msg)) => {
                tracking_signals
                    .handle_msg_measurement_state(msg.states.clone(), client_send_clone.clone());
            }
            Ok(SBP::MsgObsDepA(_msg)) => {
                //CPP-85 Unhandled for tracking signals plot tab.
                println!("The message type, MsgObsDepA, is not handled in the Tracking->SignalsPlot tab.");
            }
            Ok(SBP::MsgObsDepB(msg)) => {
                tracking_signals.handle_obs(
                    ObservationMsg::MsgObsDepB(msg.clone()),
                    client_send_clone.clone(),
                );
            }
            Ok(SBP::MsgObsDepC(msg)) => {
                tracking_signals.handle_obs(
                    ObservationMsg::MsgObsDepC(msg.clone()),
                    client_send_clone.clone(),
                );
            }

            Ok(SBP::MsgVelNED(velocity_ned)) => {
                let n = velocity_ned.n as f64;
                let e = velocity_ned.e as f64;
                let d = velocity_ned.d as f64;

                let h_vel = f64::sqrt(f64::powi(n, 2) + f64::powi(e, 2)) / 1000.0;
                let v_vel = (-1.0 * d) / 1000.0;

                tow = velocity_ned.tow as f64 / 1000.0;

                let mut _min = 0.0;
                let mut _max = 1.0;
                {
                    let vmin = vpoints
                        .iter()
                        .min_by_key(|i| i.1)
                        .unwrap_or(&(0.0, OrderedFloat(0.0)));
                    let vmax = vpoints
                        .iter()
                        .max_by_key(|i| i.1)
                        .unwrap_or(&(1.0, OrderedFloat(0.0)));
                    let hmin = hpoints
                        .iter()
                        .min_by_key(|i| i.1)
                        .unwrap_or(&(0.0, OrderedFloat(0.0)));
                    let hmax = hpoints
                        .iter()
                        .max_by_key(|i| i.1)
                        .unwrap_or(&(1.0, OrderedFloat(0.0)));

                    if vmin.1.into_inner() < hmin.1.into_inner() {
                        _min = vmin.1.into_inner();
                    } else {
                        _min = hmin.1.into_inner();
                    }
                    if vmax.1.into_inner() > hmax.1.into_inner() {
                        _max = vmax.1.into_inner();
                    } else {
                        _max = hmax.1.into_inner();
                    }
                }

                if hpoints.len() >= 200 {
                    hpoints.remove(0);
                }
                if vpoints.len() >= 200 {
                    vpoints.remove(0);
                }
                hpoints.push((tow, OrderedFloat(h_vel)));
                vpoints.push((tow, OrderedFloat(v_vel)));

                let mut builder = Builder::new_default();
                let msg = builder.init_root::<m::message::Builder>();

                let mut velocity_status = msg.init_velocity_status();

                velocity_status.set_min(_min);
                velocity_status.set_max(_max);

                {
                    let mut hvel_points = velocity_status
                        .reborrow()
                        .init_hpoints(hpoints.len() as u32);
                    for (i, (x, OrderedFloat(y))) in hpoints.iter().enumerate() {
                        let mut point_val = hvel_points.reborrow().get(i as u32);
                        point_val.set_x(*x);
                        point_val.set_y(*y);
                    }
                }
                {
                    let mut vvel_points = velocity_status
                        .reborrow()
                        .init_vpoints(vpoints.len() as u32);
                    for (i, (x, OrderedFloat(y))) in vpoints.iter().enumerate() {
                        let mut point_val = vvel_points.reborrow().get(i as u32);
                        point_val.set_x(*x);
                        point_val.set_y(*y);
                    }
                }

                let mut msg_bytes: Vec<u8> = vec![];
                serialize::write_message(&mut msg_bytes, &builder).unwrap();

                client_send_clone.send(msg_bytes).unwrap();
            }
            _ => {
                // no-op
            }
        }
    }
}
