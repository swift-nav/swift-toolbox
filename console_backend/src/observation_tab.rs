use capnp::message::Builder;

use log::warn;
use std::collections::{BTreeMap, HashMap};

use crate::client_sender::BoxedClientSender;
use crate::shared_state::SharedState;
use crate::types::{ObservationMsg, SignalCodes};
use crate::utils::{compute_doppler, sec_to_ns, serialize_capnproto_builder};

#[derive(Clone, Debug)]
pub struct ObservationTableRow {
    pub sat: i16,
    pub code: String,
    pub pseudo_range: f64,     // (m)
    pub carrier_phase: f64,    // (cycles)
    pub cn0: f64,              // (dB-Hz)
    pub measured_doppler: f64, // (Hz)
    pub computed_doppler: f64, // (Hz)
    pub lock: u16,
    pub flags: u8,
}

impl ObservationTableRow {
    pub fn new() -> ObservationTableRow {
        ObservationTableRow {
            sat: 0,
            code: "".to_string(),
            pseudo_range: 0.0,
            carrier_phase: 0.0,
            cn0: 0.0,
            measured_doppler: 0.0,
            computed_doppler: 0.0,
            lock: 0,
            flags: 0,
        }
    }
}

impl Default for ObservationTableRow {
    fn default() -> Self {
        ObservationTableRow::new()
    }
}

#[derive(Debug)]
pub struct ObservationTable {
    pub is_remote: bool,
    pub gps_tow: f64,
    pub gps_week: u16,
    pub prev_obs_count: u8,
    pub prev_obs_total: u8,
    pub prev_tow: f64,
    pub incoming_obs: BTreeMap<(i16, SignalCodes), ObservationTableRow>,
    pub rows: BTreeMap<(i16, SignalCodes), ObservationTableRow>,
    pub old_carrier_phase: HashMap<(i16, SignalCodes), f64>,
    pub new_carrier_phase: HashMap<(i16, SignalCodes), f64>,
}

impl ObservationTable {
    pub fn new(is_remote: bool) -> ObservationTable {
        ObservationTable {
            is_remote,
            gps_tow: 0.0,
            gps_week: 0,
            prev_obs_count: 0,
            prev_obs_total: 0,
            prev_tow: 0.0,
            incoming_obs: BTreeMap::new(),
            rows: BTreeMap::new(),
            old_carrier_phase: HashMap::new(),
            new_carrier_phase: HashMap::new(),
        }
    }
    pub fn was_packet_dropped(&self, tow: f64, wn: u16, obs_total: u8, obs_count: u8) -> bool {
        (self.gps_tow - tow) > f64::EPSILON
            || self.gps_week != wn
            || self.prev_obs_count + 1 != obs_count
            || self.prev_obs_total != obs_total
    }

    /// Reset observation data in the event of empty observation packet drop.
    ///
    /// # Parameters:
    ///
    /// - `tow`: The GPS time of week.
    /// - `wn`: The current GPS week number.
    /// - `obs_total`: The current observation message total to store.
    pub fn obs_reset(&mut self, tow: f64, wn: u16, obs_total: u8) {
        self.prev_tow = self.gps_tow;
        self.gps_tow = tow;
        self.gps_week = wn;
        self.prev_obs_total = obs_total;
        self.prev_obs_count = 0;
        self.old_carrier_phase = self.new_carrier_phase.clone();
        self.incoming_obs.clear();
        self.new_carrier_phase.clear();
        self.rows.clear();
    }

    pub fn obs_check(&mut self, tow: f64, wn: u16, obs_total: u8, obs_count: u8) -> bool {
        if obs_count == 0 {
            self.obs_reset(tow, wn, obs_total);
            true
        } else if self.was_packet_dropped(tow, wn, obs_total, obs_count) {
            warn!("We dropped a packet. Skipping this ObservationMsg sequence");
            self.obs_reset(tow, wn, obs_total);
            self.prev_obs_count = obs_count;
            false
        } else {
            self.prev_obs_count = obs_count;
            true
        }
    }
}

impl Default for ObservationTable {
    fn default() -> Self {
        ObservationTable::new(false)
    }
}

#[derive(Debug)]
pub struct ObservationTab {
    pub client_sender: BoxedClientSender,
    pub shared_state: SharedState,
    pub remote: ObservationTable,
    pub local: ObservationTable,
}

impl ObservationTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> ObservationTab {
        ObservationTab {
            client_sender,
            shared_state,
            remote: ObservationTable::new(true),
            local: ObservationTable::new(false),
        }
    }

    /// Handle MsgObs, MsgObsDepB, MsgObsDepC and MsgOsr full messages.
    ///
    /// # Parameters:
    ///
    /// - `msg`: The full SBP message cast as an ObservationMsg variant.
    pub fn handle_obs(&mut self, msg: ObservationMsg) {
        let msg_fields = msg.fields();

        let mut is_remote: bool = false;
        if let Some(sender_id) = msg_fields.sender_id {
            if sender_id == 0 {
                is_remote = true;
            }
        }

        let total = msg_fields.n_obs >> 4;
        let count = msg_fields.n_obs & ((1 << 4) - 1);
        if (is_remote
            && !self
                .remote
                .obs_check(msg_fields.tow, msg_fields.wn, total, count))
            || (!is_remote
                && !self
                    .local
                    .obs_check(msg_fields.tow, msg_fields.wn, total, count))
        {
            return;
        }

        let old_carrier_phase = match is_remote {
            true => &self.remote.old_carrier_phase,
            false => &self.local.old_carrier_phase,
        };
        let new_carrier_phase = match is_remote {
            true => &mut self.remote.new_carrier_phase,
            false => &mut self.local.new_carrier_phase,
        };
        let incoming_obs = match is_remote {
            true => &mut self.remote.incoming_obs,
            false => &mut self.local.incoming_obs,
        };

        for state in msg_fields.states.iter() {
            let obs_fields = state.fields();

            let table_key = (obs_fields.sat, obs_fields.code);

            // Bit 0 is Pseudorange valid
            let is_pseudo_range_valid = obs_fields.flags & 0x01 != 0;
            // Bit 1 is Carrier phase valid
            let is_carrier_phase_valid = obs_fields.flags & 0x02 != 0;
            let is_valid = is_pseudo_range_valid && is_carrier_phase_valid;
            let is_deprecated_msg_type = obs_fields.is_deprecated_msg_type;

            if msg_fields.ns_residual != 0 {
                let ns_residual = sec_to_ns(msg_fields.ns_residual as f64);
                if is_remote {
                    self.remote.gps_tow += ns_residual;
                } else {
                    self.local.gps_tow += ns_residual;
                }
            }

            let computed_doppler = match (
                old_carrier_phase.get(&table_key),
                is_deprecated_msg_type || is_valid,
            ) {
                (Some(val), true) => compute_doppler(
                    obs_fields.carrier_phase,
                    *val,
                    if is_remote {
                        self.remote.gps_tow
                    } else {
                        self.local.gps_tow
                    },
                    if is_remote {
                        self.remote.prev_tow
                    } else {
                        self.local.prev_tow
                    },
                    is_deprecated_msg_type,
                ),
                _ => 0 as f64,
            };

            let mut row = ObservationTableRow::new();
            row.code = format!("{}", obs_fields.code);
            row.sat = obs_fields.sat;
            row.pseudo_range = obs_fields.pseudo_range;
            row.carrier_phase = obs_fields.carrier_phase;
            row.cn0 = obs_fields.cn0 / 4.0;
            row.lock = obs_fields.lock;
            row.measured_doppler = obs_fields.measured_doppler;
            row.computed_doppler = computed_doppler;
            // Note: piksi_tools console did not show flags when is_deprecated_msg_type
            row.flags = obs_fields.flags;

            incoming_obs.insert((obs_fields.sat, obs_fields.code), row);
            if is_deprecated_msg_type || is_valid {
                new_carrier_phase.insert(table_key, obs_fields.carrier_phase);
            }
        }

        if count == (total - 1) {
            self.update_from_obs(is_remote);
        }
    }

    /// Update remote or local table using the observation data accumulated by handle_obs.
    ///
    /// # Parameters:
    ///
    /// - `is_remote`: Whether self.incoming_obs was remote or local
    pub fn update_from_obs(&mut self, is_remote: bool) {
        let table: &mut ObservationTable = match is_remote {
            true => &mut self.remote,
            false => &mut self.local,
        };

        for ((sat, code), row) in table.incoming_obs.iter() {
            table.rows.insert((*sat, *code), row.clone());
        }
        self.send_data(is_remote);
    }

    /// Package data into a message buffer and send to frontend.
    ///
    /// # Parameters:
    ///
    /// - `is_remote`: Sender local or remote table
    fn send_data(&mut self, is_remote: bool) {
        let table: &ObservationTable = match is_remote {
            true => &self.remote,
            false => &self.local,
        };
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();

        let mut observation_status = msg.init_observation_status();
        observation_status.set_is_remote(table.is_remote);
        observation_status.set_tow(table.gps_tow);
        observation_status.set_week(table.gps_week);
        let mut rows = observation_status.init_rows(table.rows.len() as u32);

        for (idx, (_key, row)) in table.rows.iter().enumerate() {
            let mut list_item = rows.reborrow().get(idx as u32);

            list_item.set_sat(row.sat);
            list_item.set_code(&row.code);
            list_item.set_pseudo_range(row.pseudo_range);
            list_item.set_carrier_phase(row.carrier_phase);
            list_item.set_cn0(row.cn0);
            list_item.set_measured_doppler(row.measured_doppler);
            list_item.set_computed_doppler(row.computed_doppler);
            list_item.set_lock(row.lock);
            list_item.set_flags(row.flags);
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}

#[cfg(test)]
mod tests {
    const DEFAULT_OBSERVATION_CODE: &str = "GLO L2OF";

    use super::*;
    use crate::client_sender::TestSender;
    use sbp::messages::{
        gnss::{CarrierPhase, GnssSignal, GpsTime},
        observation::{Doppler, MsgObs, ObservationHeader, PackedObsContent},
    };

    // Validate that messages received by handle_obs() populate ObservationTable's incoming obs struct.
    #[test]
    fn handle_obs_msgobs_test() {
        let shared_state = SharedState::new();
        let client_send = TestSender::boxed();
        let mut obs_tab = ObservationTab::new(shared_state, client_send);

        let mut obs_msg = MsgObs {
            sender_id: Some(23),
            obs: Vec::new(),
            header: ObservationHeader {
                t: GpsTime {
                    tow: 0,
                    ns_residual: 0,
                    wn: 1,
                },
                n_obs: 16,
            },
        };
        let signal_code = 4;
        let cn0 = 5;
        let lock = 0;
        let flags = 0b00000001;
        let sat: u8 = 25;
        obs_msg.obs.push(PackedObsContent {
            p: 0_u32,
            l: CarrierPhase { i: 0_i32, f: 0_u8 },
            d: Doppler { i: 0_i16, f: 0_u8 },
            cn0,
            lock,
            flags,
            sid: GnssSignal {
                code: signal_code,
                sat,
            },
        });

        assert_eq!(obs_tab.local.gps_week, 0);
        assert!(obs_tab.local.incoming_obs.is_empty());
        obs_tab.handle_obs(ObservationMsg::MsgObs(obs_msg));
        // Expect identifiers and metadata fields to match obs_msg fields
        for (count, obs) in obs_tab.local.incoming_obs.iter().enumerate() {
            if count > 0 {
                break;
            }
            assert_eq!(obs.1.code, DEFAULT_OBSERVATION_CODE.to_string());
            assert_eq!(obs.1.flags, flags);
            assert_eq!(obs.1.sat as u8, sat);
        }
        assert_eq!(obs_tab.local.gps_week, 1);
    }
}
