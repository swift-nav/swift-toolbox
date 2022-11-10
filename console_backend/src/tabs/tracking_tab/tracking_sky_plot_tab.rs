use capnp::message::Builder;
use sbp::messages::observation::MsgSvAzEl;

use crate::client_sender::BoxedClientSender;
use crate::piksi_tools_constants::{
    code_is_bds, code_is_galileo, code_is_glo, code_is_gps, code_is_qzss, code_is_sbas,
};
use crate::shared_state::{SharedState, TabName};
use crate::types::SignalCodes;
use crate::utils::{serialize_capnproto_builder, signal_key_label};

/// SkyPlotObs struct.
///
/// This struct is used to store information about observed satellite locations.
///
/// # Fields
///
/// - `az`: Azimuth angle of the satellite in degrees [0, 360].
/// - `el`: Elevation angle of the satellite in degrees [-90, 90].
/// - `label`: Label of the satellite.
struct SkyPlotObs {
    az: u16,
    el: u16,
    label: String,
}

/// TrackingSkyPlotTab struct.
///
/// # Fields:
///
/// - `client_send`: Client Sender channel for communication from backend to frontend.
/// - `sats`: Storage for Satellite SkyPlotObs to be sent to frontend.
/// - `shared_state`: The shared state for communicating between frontend/backend/other backend tabs.
pub struct TrackingSkyPlotTab {
    client_sender: BoxedClientSender,
    sats: Vec<Vec<SkyPlotObs>>,
    shared_state: SharedState,
}

impl TrackingSkyPlotTab {
    pub fn new(client_sender: BoxedClientSender, shared_state: SharedState) -> TrackingSkyPlotTab {
        TrackingSkyPlotTab {
            client_sender,
            sats: (0_i32..6_i32).map(|_| Vec::new()).collect(),
            shared_state,
        }
    }

    fn clear_sats(&mut self) {
        self.sats.iter_mut().for_each(|v| v.clear());
    }

    /// Handle MsgSvAzEl message states.
    ///
    /// # Parameters:
    ///
    /// - `msg`: The message to update set of points with.
    pub fn handle_sv_az_el(&mut self, msg: MsgSvAzEl) {
        self.clear_sats();
        let svs_tracked = self
            .shared_state
            .lock()
            .tracking_tab
            .signals_tab
            .tracked_sv_labels
            .clone();
        msg.azel.iter().for_each(|azel| {
            let key = (SignalCodes::from(azel.sid.code), azel.sid.sat as i16);
            if let Some(mut label) = signal_key_label(key, None).2 {
                if svs_tracked.contains(&label) {
                    label = format!("{label}*");
                }
                let code = azel.sid.code as i32;
                let obs = SkyPlotObs {
                    az: azel.az as u16 * 2,
                    el: 90 - azel.el as u16,
                    label,
                };
                let idx = if code_is_gps(code) {
                    Some(0)
                } else if code_is_glo(code) {
                    Some(1)
                } else if code_is_galileo(code) {
                    Some(2)
                } else if code_is_bds(code) {
                    Some(3)
                } else if code_is_qzss(code) {
                    Some(4)
                } else if code_is_sbas(code) {
                    Some(5)
                } else {
                    None
                };
                if let Some(idx) = idx {
                    self.sats[idx].push(obs);
                }
            }
        });
        self.send_data();
    }

    /// Package data into a message buffer and send to frontend.
    fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Tracking {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut tab = msg.init_tracking_sky_plot_status();
        let mut sats = tab.reborrow().init_sats(self.sats.len() as u32);
        {
            self.sats.iter_mut().enumerate().for_each(|(idx, sat_row)| {
                let mut point_val_idx = sats.reborrow().init(idx as u32, sat_row.len() as u32);
                for (i, obs) in sat_row.iter().enumerate() {
                    let mut point_val = point_val_idx.reborrow().get(i as u32);
                    point_val.set_az(obs.az);
                    point_val.set_el(obs.el);
                }
            });
        }
        let mut labels = tab.reborrow().init_labels(self.sats.len() as u32);
        {
            self.sats.iter_mut().enumerate().for_each(|(idx, sat_row)| {
                let mut label_val_idx = labels.reborrow().init(idx as u32, sat_row.len() as u32);
                for (i, obs) in sat_row.iter().enumerate() {
                    label_val_idx.reborrow().set(i as u32, &obs.label);
                }
            });
        }
        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    use sbp::messages::{gnss::GnssSignal, observation::SvAzEl};

    use super::*;
    use crate::client_sender::TestSender;

    #[test]
    fn handle_sv_az_el_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut tab = TrackingSkyPlotTab::new(client_send, shared_state.clone());
        let az = 45;
        let el = 40;
        let msg = MsgSvAzEl {
            sender_id: Some(5),
            azel: vec![SvAzEl {
                sid: GnssSignal {
                    code: SignalCodes::CodeGpsL2Cl as u8,
                    sat: 1,
                },
                az,
                el,
            }],
        };
        assert!(tab.sats[0].is_empty());
        tab.handle_sv_az_el(msg);
        assert_eq!(tab.sats[0].len(), 1);
        assert_eq!(tab.sats[0][0].az, az as u16 * 2);
        assert_eq!(tab.sats[0][0].el, 90 - el as u16);
        assert_eq!(tab.sats[0][0].label, "G01");

        let az = 30;
        let el = 30;

        let msg = MsgSvAzEl {
            sender_id: Some(5),
            azel: vec![SvAzEl {
                sid: GnssSignal {
                    code: SignalCodes::CodeQzsL2Cm as u8,
                    sat: 35,
                },
                az,
                el,
            }],
        };
        assert!(tab.sats[4].is_empty());
        let label = "J 35".to_string();
        shared_state
            .lock()
            .tracking_tab
            .signals_tab
            .tracked_sv_labels = vec![label.clone()];
        tab.handle_sv_az_el(msg);
        assert_eq!(tab.sats[4].len(), 1);
        assert_eq!(tab.sats[4][0].az, 30 * 2);
        assert_eq!(tab.sats[4][0].el, 90 - 30);
        assert_eq!(tab.sats[4][0].label, format!("{}*", label));
    }
}
