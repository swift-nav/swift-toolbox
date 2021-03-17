use capnp::message::Builder;
use capnp::serialize;
use ordered_float::*;
use sbp::messages::SBP;
use std::sync::mpsc;

use crate::{flags::*, types::*};

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
            Ok(SBP::MsgMeasurementState(msg)) => {
                for state in msg.states {
                    if state.cn0 != 0 {
                        let points =
                            match sat_headers.iter().position(|&ele| ele == state.mesid.sat) {
                                Some(idx) => sats.get_mut(idx).unwrap(),
                                _ => {
                                    sat_headers.push(state.mesid.sat);
                                    sats.push(Vec::new());
                                    sats.last_mut().unwrap()
                                }
                            };
                        if points.len() >= 200 {
                            points.remove(0);
                        }
                        points.push((tow, OrderedFloat(state.cn0 as f64 / 4.0)));
                    }
                }
                let mut builder = Builder::new_default();
                let msg = builder.init_root::<m::message::Builder>();

                let mut tracking_status = msg.init_tracking_status();
                tracking_status.set_min(0_f64);
                tracking_status.set_max(60_f64);
                let mut tracking_headers = tracking_status
                    .reborrow()
                    .init_headers(sat_headers.len() as u32);

                for (i, header) in sat_headers.iter().enumerate() {
                    tracking_headers.set(i as u32, *header);
                }

                let mut tracking_points = tracking_status
                    .reborrow()
                    .init_data(sat_headers.len() as u32);
                {
                    for idx in 0..sat_headers.len() {
                        let points = sats.get_mut(idx).unwrap();
                        let mut point_val_idx = tracking_points
                            .reborrow()
                            .init(idx as u32, points.len() as u32);
                        for (i, (x, OrderedFloat(y))) in points.iter().enumerate() {
                            let mut point_val = point_val_idx.reborrow().get(i as u32);
                            point_val.set_x(*x);
                            point_val.set_y(*y);
                        }
                    }
                }
                let mut msg_bytes: Vec<u8> = vec![];
                serialize::write_message(&mut msg_bytes, &builder).unwrap();

                client_send_clone.send(msg_bytes).unwrap();
            }
            Ok(SBP::MsgTrackingState(_msg)) => {}
            Ok(SBP::MsgObs(_msg)) => {}

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
