use capnp::message::Builder;
use capnp::serialize;
use ordered_float::OrderedFloat;
use sbp::messages::{SBP, SBPMessage};
use std::sync::mpsc;

use crate::{constants::*, flags::*, types::*};



use crate::console_backend_capnp as m;




pub fn process_messages(
    messages: impl Iterator<Item = sbp::Result<SBP>>,
    client_send_clone: mpsc::Sender<Vec<u8>>,
) {
    let mut hpoints: Vec<(f64, OrderedFloat<f64>)> = vec![];
    let mut vpoints: Vec<(f64, OrderedFloat<f64>)> = vec![];
    let mut sat_headers: Vec<u8> = vec![];
    let mut sats: Vec<Vec<(f64, OrderedFloat<f64>)>> = vec![];
    let mut tow: f64 = 0.0;

    let mut tracking_signals = TrackingSignalsTab::new();

    for message in messages {
        match message {
            Ok(SBP::MsgTrackingState(msg)) => {
                tracking_signals.handle_msg_tracking_state(msg.states.clone(), client_send_clone.clone());
            }
            Ok(SBP::MsgObs(msg)) => {
                tracking_signals.handle_obs(ObservationMsg::MsgObs(msg.clone()), client_send_clone.clone());
            }
            Ok(SBP::MsgMeasurementState(msg)) => {
                tracking_signals.handle_msg_measurement_state(msg.states.clone(), client_send_clone.clone());
            }
            // Ok(SBP::MsgObsDepA(_msg)) => {
            // }
            Ok(SBP::MsgObsDepB(msg)) => {
                tracking_signals.handle_obs(ObservationMsg::MsgObsDepB(msg.clone()), client_send_clone.clone());
            }
            Ok(SBP::MsgObsDepC(msg)) => {
                tracking_signals.handle_obs(ObservationMsg::MsgObsDepC(msg.clone()), client_send_clone.clone());
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

// pub fn is_valid_gps_time(flags: u8) -> bool {
//     UTC_TIME_FLAGS_TIME_SOURCE_NONE != (flags & UTC_TIME_FLAGS_TIME_SOURCE_MASK)
// }

// fn handle_gps_time(
//     wn: u16,   // GPS week number.
//     tow: u32,  // GPS time of week rounded to the nearest millisecond.
//     flags: u8, // Status flags (reserved).
//     outputs: &mut OutputDispatchers,
//     data_sets: &mut DataSets,
// ) -> Result<()> {
//     if is_valid_gps_time(flags) {
//         let gps_tow_secs_new_inner = tow as f64 / 1.0e+3_f64;
//         let gps_tow_secs_new = Some(gps_tow_secs_new_inner);
//         if let Some(gps_tow_secs_inner) = data_sets.dataset.gps_tow_secs {
//             if (gps_tow_secs_new_inner - gps_tow_secs_inner) > TIME_INTERVAL_ERROR_MARGIN_SEC {
//                 data_sets.dataset.delta_tow_ms =
//                     Some((gps_tow_secs_new_inner - gps_tow_secs_inner) * 1.0e+3_f64);
//                 let last_delta_tow_ms = data_sets.dataset.delta_tow_ms;
//                 outputs.output.serialize(&data_sets.dataset)?;
//                 data_sets.dataset_clear();
//                 data_sets.dataset.gps_week = Some(wn as f64);
//                 data_sets.dataset.gps_tow_secs = gps_tow_secs_new;
//                 data_sets.dataset.delta_tow_ms = last_delta_tow_ms;
//             }
//         } else {
//             data_sets.dataset.gps_week = Some(wn as f64);
//             data_sets.dataset.gps_tow_secs = gps_tow_secs_new;
//         }
//     }
//     Ok(())
// }
