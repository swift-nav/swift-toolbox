use capnp::message::Builder;
use capnp::serialize;
use log::info;
use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

use crate::types::*;
use crate::utils::{compute_doppler, sec_to_ns};

use crate::console_backend_capnp as m;

#[derive(Clone, Debug)]
pub struct ObservationTableRow {
    pub prn: String,
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
            prn: "".to_string(),
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
    pub rows: BTreeMap<(i16, SignalCodes), ObservationTableRow>,
}

impl ObservationTable {
    pub fn new() -> ObservationTable {
        ObservationTable {
            is_remote: false,
            gps_tow: 0.0,
            gps_week: 0,
            rows: BTreeMap::new(),
        }
    }
}

impl Default for ObservationTable {
    fn default() -> Self {
        ObservationTable::new()
    }
}

#[derive(Debug)]
pub struct ObservationTab<S: MessageSender> {
    pub client_sender: S,
    pub shared_state: SharedState,
    pub gps_tow: f64,
    pub gps_week: u16,
    pub incoming_obs: BTreeMap<(i16, SignalCodes), ObservationTableRow>,
    pub old_carrier_phase: HashMap<(i16, SignalCodes), f64>,
    pub new_carrier_phase: HashMap<(i16, SignalCodes), f64>,
    pub last_update_time: Instant,
    pub prev_obs_count: u8,
    pub prev_obs_total: u8,
    pub prev_tow: f64,
    pub remote: ObservationTable,
    pub local: ObservationTable,
}

impl<S: MessageSender> ObservationTab<S> {
    pub fn new(shared_state: SharedState, client_sender: S) -> ObservationTab<S> {
        ObservationTab {
            client_sender,
            shared_state,
            gps_tow: 0.0,
            gps_week: 0,
            incoming_obs: BTreeMap::new(),
            old_carrier_phase: HashMap::new(),
            new_carrier_phase: HashMap::new(),
            last_update_time: Instant::now(),
            prev_obs_count: 0,
            prev_obs_total: 0,
            prev_tow: 0.0,
            remote: ObservationTable::new(),
            local: ObservationTable::new(),
        }
    }

    /// Handle MsgOsr, MsgObs, MsgObsDepB, and MsgObsDepC full messages.
    ///
    /// # Parameters:
    ///
    /// - `msg`: The full SBP message cast as an ObservationMsg variant.
    pub fn handle_obs(&mut self, msg: ObservationMsg) {
        let mut is_remote: bool = false; // TODO(JV): Check logic for local vs remote
                                         // and computed_doppler
        let msg_fields = msg.fields();

        if let Some(sender_id) = msg_fields.sender_id {
            if sender_id == 0 {
                is_remote = true;
            }
        }

        let mut new_carrier_phase: HashMap<(i16, SignalCodes), f64> = HashMap::new();

        let total = msg_fields.n_obs >> 4;
        let count = msg_fields.n_obs & ((1 << 4) - 1);

        if count == 0 {
            self.obs_reset(msg_fields.tow, msg_fields.wn, total);
        } else if (self.gps_tow - msg_fields.tow) > f64::EPSILON
            || self.gps_week != msg_fields.wn
            || self.prev_obs_count + 1 != count
            || self.prev_obs_total != total
        {
            println!("We dropped a packet. Skipping this ObservationMsg sequence");
            self.obs_reset(msg_fields.tow, msg_fields.wn, total);
            self.prev_obs_count = count;
            return;
        } else {
            self.prev_obs_count = count;
        }

        for state in msg_fields.states.iter() {
            let obs_fields = state.fields();

            let table_key = (obs_fields.sat, obs_fields.code);

            // Bit 0 is Pseudorange valid
            let is_pseudo_range_valid = obs_fields.flags & 0x01 != 0;
            // Bit 1 is Carrier phase valid
            let is_carrier_phase_valid = obs_fields.flags & 0x02 != 0;
            let is_valid = is_pseudo_range_valid && is_carrier_phase_valid;

            let is_b_or_c = obs_fields.dep_type == 'B' || obs_fields.dep_type == 'C';
            if is_b_or_c {
                info!("Encounted {} in Observation tab.", obs_fields.dep_type);
            }
            if msg_fields.ns_residual != 0 {
                self.gps_tow += sec_to_ns(msg_fields.ns_residual as f64);
            }

            let computed_doppler = match (
                self.old_carrier_phase.get(&table_key),
                is_b_or_c || is_valid,
            ) {
                (Some(val), true) => compute_doppler(
                    obs_fields.carrier_phase,
                    *val,
                    self.gps_tow,
                    self.prev_tow,
                    is_b_or_c,
                ),
                _ => 0 as f64,
            };

            let mut row = ObservationTableRow::new();
            row.prn = format!("{} ({})", obs_fields.sat, obs_fields.code).to_string();
            row.pseudo_range = obs_fields.pseudo_range;
            row.carrier_phase = obs_fields.carrier_phase;
            row.cn0 = obs_fields.cn0 / 4.0;
            row.lock = obs_fields.lock;
            row.measured_doppler = obs_fields.measured_doppler;
            row.computed_doppler = computed_doppler;
            // Note: piksi_tools console did not show flags when is_b_or_c
            row.flags = obs_fields.flags;

            self.incoming_obs
                .insert((obs_fields.sat, obs_fields.code), row);
            if is_b_or_c || is_valid {
                new_carrier_phase.insert(table_key, obs_fields.carrier_phase);
            }
        }

        self.new_carrier_phase = new_carrier_phase;

        if count == (total - 1) {
            self.last_update_time = Instant::now();
            self.update_from_obs(is_remote);
        }
    }
    /// Update remote or local table using the observation data accumulated by handle_obs.
    ///
    /// # Parameters:
    ///
    /// - `is_remote`: Whether self.incoming_obs was remote or local
    pub fn update_from_obs(&mut self, is_remote: bool) {
        let mut table: ObservationTable = ObservationTable::new();
        table.is_remote = is_remote;
        table.gps_week = self.gps_week;
        table.gps_tow = self.gps_tow;

        for ((sat, code), row) in self.incoming_obs.iter() {
            table.rows.insert((*sat, *code), row.clone());
        }
        if is_remote {
            self.remote = table;
            self.send_data(true);
        } else {
            self.local = table;
            self.send_data(false);
        }
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
        self.incoming_obs.clear();
        self.old_carrier_phase = self.new_carrier_phase.clone();
        self.new_carrier_phase.clear();
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
        let msg = builder.init_root::<m::message::Builder>();

        let mut observation_status = msg.init_observation_status();
        observation_status.set_is_remote(table.is_remote);
        observation_status.set_tow(table.gps_tow);
        observation_status.set_week(table.gps_week);
        let mut rows = observation_status.init_rows(table.rows.len() as u32);

        for (idx, (_key, row)) in table.rows.iter().enumerate() {
            let mut list_item = rows.reborrow().get(idx as u32);

            list_item.set_prn(&row.prn);
            list_item.set_pseudo_range(row.pseudo_range);
            list_item.set_carrier_phase(row.carrier_phase);
            list_item.set_cn0(row.cn0);
            list_item.set_measured_doppler(row.measured_doppler);
            list_item.set_computed_doppler(row.computed_doppler);
            list_item.set_lock(row.lock);
            list_item.set_flags(row.flags);
        }

        let mut msg_bytes: Vec<u8> = vec![];
        serialize::write_message(&mut msg_bytes, &builder).unwrap();

        self.client_sender.send_data(msg_bytes);
    }
}
