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

use capnp::message::Builder;
use std::collections::BTreeMap;
use std::time::Instant;

use sbp::messages::ssr::{
    MsgSsrCodeBiases, MsgSsrGriddedCorrection, MsgSsrOrbitClock, MsgSsrPhaseBiases,
    MsgSsrStecCorrection, MsgSsrTileDefinition,
};

use crate::client_sender::BoxedClientSender;
use crate::shared_state::{SharedState, TabName};
use crate::tabs::observation_tab::{ObservationTable, ObservationTableRow};
use crate::types::{ObservationMsg, SignalCodes};
use crate::utils::{
    compute_doppler, decode_ssr_update_interval, sec_to_ns, serialize_capnproto_builder,
};

/// Prototype only: the `code`/`value` shown for a satellite's code and phase
/// biases are taken from the first entry in that message's bias list. Each
/// MsgSsrCodeBiases/MsgSsrPhaseBiases can carry biases for several signals per
/// satellite, keyed by an RTCM DF380/381/382/467-encoded signal code that has
/// no decode table in this repo or the sbp crate, so per-signal biases aren't
/// broken out individually here.
#[derive(Clone, Debug, Default)]
pub struct SsrSatCorrectionRow {
    pub radial: i32,
    pub along: i32,
    pub cross: i32,
    pub clock_c0: i32,
    pub code_bias: i16,
    pub phase_bias: i32,
    pub last_seen: Option<Instant>,
}

#[derive(Clone, Debug, Default)]
pub struct SsrTileRow {
    pub corner_nw_lat: f64,
    pub corner_nw_lon: f64,
    pub rows: u16,
    pub cols: u16,
    pub n_sats: u8,
}

/// Tracks arrival of any repeatedly-received SBP message type (SSR message
/// families, but also `MSG_OBS`/`MSG_OSR`), whether or not it carries a
/// declared update interval. Works regardless of how the device got its
/// corrections (console-relayed NTRIP or fetched by the device itself),
/// since it's driven purely by the SBP messages the device already reports.
#[derive(Clone, Debug)]
pub struct SsrStreamRow {
    pub first_seen: Instant,
    pub last_seen: Instant,
    /// The message's own declared update interval (decoded DF391), when it
    /// has one (SSR message families). `None` for message types with no
    /// such field (`MSG_OBS`/`MSG_OSR`) - callers fall back to an
    /// empirically measured average interval in that case.
    pub declared_interval_sec: Option<f64>,
    pub iod_ssr: Option<u8>,
    pub count: u32,
}

#[derive(Debug)]
pub struct CorrectionsTab {
    pub client_sender: BoxedClientSender,
    pub shared_state: SharedState,
    streams: BTreeMap<&'static str, SsrStreamRow>,
    sat_corrections: BTreeMap<(i16, SignalCodes), SsrSatCorrectionRow>,
    tiles: BTreeMap<(u16, u16), SsrTileRow>,
    /// Decoded per-satellite content of the OSR/NXRTK-MSM5 correction
    /// stream, i.e. what used to be the "Remote" section of the
    /// Observations tab (SBP `ObservationMsg` with `sender_id == 0`).
    osr: ObservationTable,
}

/// Decode a North-West corner correction point coordinate.
///
/// See `MsgSsrTileDefinition::corner_nw_lat`/`corner_nw_lon` doc comments in
/// the sbp crate for the encoding.
fn decode_corner(coded: i16, range: f64, bits: u32) -> f64 {
    coded as f64 / (1_u32 << bits) as f64 * range
}

fn obs_stream_name(msg: &ObservationMsg) -> &'static str {
    match msg {
        ObservationMsg::MsgObs(_) => "MSG_OBS",
        ObservationMsg::MsgObsDepB(_) => "MSG_OBS_DEP_B",
        ObservationMsg::MsgObsDepC(_) => "MSG_OBS_DEP_C",
        ObservationMsg::MsgOsr(_) => "MSG_OSR",
    }
}

impl CorrectionsTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> CorrectionsTab {
        CorrectionsTab {
            client_sender,
            shared_state,
            streams: BTreeMap::new(),
            sat_corrections: BTreeMap::new(),
            tiles: BTreeMap::new(),
            osr: ObservationTable::new(true),
        }
    }

    fn touch_stream(
        &mut self,
        name: &'static str,
        declared_interval_code: Option<u8>,
        iod_ssr: Option<u8>,
    ) {
        let now = Instant::now();
        let row = self.streams.entry(name).or_insert_with(|| SsrStreamRow {
            first_seen: now,
            last_seen: now,
            declared_interval_sec: None,
            iod_ssr: None,
            count: 0,
        });
        row.last_seen = now;
        row.declared_interval_sec = declared_interval_code.map(decode_ssr_update_interval);
        row.iod_ssr = iod_ssr;
        row.count += 1;
    }

    pub fn handle_orbit_clock(&mut self, msg: MsgSsrOrbitClock) {
        self.touch_stream(
            "MSG_SSR_ORBIT_CLOCK",
            Some(msg.update_interval),
            Some(msg.iod_ssr),
        );
        let key = (msg.sid.sat as i16, SignalCodes::from(msg.sid.code));
        let row = self.sat_corrections.entry(key).or_default();
        row.radial = msg.radial;
        row.along = msg.along;
        row.cross = msg.cross;
        row.clock_c0 = msg.c0;
        row.last_seen = Some(Instant::now());
        self.send_data();
    }

    pub fn handle_code_biases(&mut self, msg: MsgSsrCodeBiases) {
        self.touch_stream(
            "MSG_SSR_CODE_BIASES",
            Some(msg.update_interval),
            Some(msg.iod_ssr),
        );
        let key = (msg.sid.sat as i16, SignalCodes::from(msg.sid.code));
        let row = self.sat_corrections.entry(key).or_default();
        if let Some(first) = msg.biases.first() {
            row.code_bias = first.value;
        }
        row.last_seen = Some(Instant::now());
        self.send_data();
    }

    pub fn handle_phase_biases(&mut self, msg: MsgSsrPhaseBiases) {
        self.touch_stream(
            "MSG_SSR_PHASE_BIASES",
            Some(msg.update_interval),
            Some(msg.iod_ssr),
        );
        let key = (msg.sid.sat as i16, SignalCodes::from(msg.sid.code));
        let row = self.sat_corrections.entry(key).or_default();
        if let Some(first) = msg.biases.first() {
            row.phase_bias = first.bias;
        }
        row.last_seen = Some(Instant::now());
        self.send_data();
    }

    pub fn handle_tile_definition(&mut self, msg: MsgSsrTileDefinition) {
        self.touch_stream(
            "MSG_SSR_TILE_DEFINITION",
            Some(msg.update_interval),
            Some(msg.iod_atmo),
        );
        let key = (msg.tile_set_id, msg.tile_id);
        let row = self.tiles.entry(key).or_default();
        row.corner_nw_lat = decode_corner(msg.corner_nw_lat, 90.0, 14);
        row.corner_nw_lon = decode_corner(msg.corner_nw_lon, 180.0, 15);
        row.rows = msg.rows;
        row.cols = msg.cols;
        self.send_data();
    }

    pub fn handle_gridded_correction(&mut self, msg: MsgSsrGriddedCorrection) {
        self.touch_stream(
            "MSG_SSR_GRIDDED_CORRECTION",
            Some(msg.header.update_interval),
            Some(msg.header.iod_atmo),
        );
        self.send_data();
    }

    pub fn handle_stec_correction(&mut self, msg: MsgSsrStecCorrection) {
        self.touch_stream(
            "MSG_SSR_STEC_CORRECTION",
            Some(msg.header.update_interval),
            Some(msg.ssr_iod_atmo),
        );
        let key = (msg.tile_set_id, msg.tile_id);
        let row = self.tiles.entry(key).or_default();
        row.n_sats = msg.n_sats;
        self.send_data();
    }

    /// Handle MsgObs, MsgObsDepB, MsgObsDepC and MsgOsr messages decoded from
    /// the OSR/NXRTK-MSM5 correction stream (`sender_id == 0`), ported from
    /// what used to be `ObservationTab`'s "Remote" handling.
    ///
    /// # Parameters:
    ///
    /// - `msg`: The full SBP message cast as an ObservationMsg variant.
    pub fn handle_osr_obs(&mut self, msg: ObservationMsg) {
        self.touch_stream(obs_stream_name(&msg), None, None);
        // Streams update on every message regardless of the SSR handlers,
        // which are the only other thing that pushes `self.streams` to the
        // frontend - without this, the MSG_OBS/MSG_OSR row would freeze at
        // whatever it looked like the last time an SSR message happened to
        // trigger a send.
        self.send_data();
        let msg_fields = msg.fields();

        let total = msg_fields.n_obs >> 4;
        let count = msg_fields.n_obs & ((1 << 4) - 1);
        if !self
            .osr
            .obs_check(msg_fields.tow, msg_fields.wn, total, count)
        {
            return;
        }

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
                self.osr.gps_tow += sec_to_ns(msg_fields.ns_residual as f64);
            }

            let computed_doppler = match (
                self.osr.old_carrier_phase.get(&table_key),
                is_deprecated_msg_type || is_valid,
            ) {
                (Some(val), true) => compute_doppler(
                    obs_fields.carrier_phase,
                    *val,
                    self.osr.gps_tow,
                    self.osr.prev_tow,
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
            row.flags = obs_fields.flags;

            self.osr
                .incoming_obs
                .insert((obs_fields.sat, obs_fields.code), row);
            if is_deprecated_msg_type || is_valid {
                self.osr
                    .new_carrier_phase
                    .insert(table_key, obs_fields.carrier_phase);
            }
        }

        if count == (total - 1) {
            for ((sat, code), row) in self.osr.incoming_obs.iter() {
                self.osr.rows.insert((*sat, *code), row.clone());
            }
            self.send_osr_data();
        }
    }

    /// Package the decoded OSR/NXRTK-MSM5 observation table into a message
    /// buffer and send to frontend.
    pub fn send_osr_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Corrections {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut osr_status = msg.init_osr_correction_status();
        osr_status.set_tow(self.osr.gps_tow);
        osr_status.set_week(self.osr.gps_week);
        let mut rows = osr_status.init_rows(self.osr.rows.len() as u32);
        for (idx, (_key, row)) in self.osr.rows.iter().enumerate() {
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

    /// Package data into a message buffer and send to frontend.
    pub fn send_data(&mut self) {
        if self.shared_state.current_tab() != TabName::Corrections {
            return;
        }
        let mut builder = Builder::new_default();
        let msg = builder.init_root::<crate::console_backend_capnp::message::Builder>();
        let mut corrections_status = msg.init_corrections_status();

        {
            let mut streams = corrections_status
                .reborrow()
                .init_streams(self.streams.len() as u32);
            for (idx, (name, row)) in self.streams.iter().enumerate() {
                let mut list_item = streams.reborrow().get(idx as u32);
                list_item.set_msg_type(name);
                list_item.set_last_age_sec(row.last_seen.elapsed().as_secs_f64());
                // Prefer the message's own declared interval (SSR families);
                // fall back to an empirically measured average interval for
                // message types with no such field (MSG_OBS/MSG_OSR).
                let interval_sec = row.declared_interval_sec.unwrap_or_else(|| {
                    let elapsed = row.last_seen.duration_since(row.first_seen).as_secs_f64();
                    if row.count > 1 {
                        elapsed / (row.count - 1) as f64
                    } else {
                        0.0
                    }
                });
                list_item.set_update_interval_sec(interval_sec);
                list_item.set_iod_ssr(row.iod_ssr.unwrap_or(0));
                list_item.set_count(row.count);
            }
        }

        {
            let mut sat_corrections = corrections_status
                .reborrow()
                .init_sat_corrections(self.sat_corrections.len() as u32);
            for (idx, ((sat, code), row)) in self.sat_corrections.iter().enumerate() {
                let mut list_item = sat_corrections.reborrow().get(idx as u32);
                list_item.set_sid(&format!("{sat} ({code})"));
                list_item.set_radial(row.radial);
                list_item.set_along(row.along);
                list_item.set_cross(row.cross);
                list_item.set_clock_c0(row.clock_c0);
                list_item.set_code_bias(row.code_bias);
                list_item.set_phase_bias(row.phase_bias);
                list_item.set_age_sec(
                    row.last_seen
                        .map(|t| t.elapsed().as_secs_f64())
                        .unwrap_or(0.0),
                );
            }
        }

        {
            let mut tiles = corrections_status
                .reborrow()
                .init_tiles(self.tiles.len() as u32);
            for (idx, ((tile_set_id, tile_id), row)) in self.tiles.iter().enumerate() {
                let mut list_item = tiles.reborrow().get(idx as u32);
                list_item.set_tile_set_id(*tile_set_id);
                list_item.set_tile_id(*tile_id);
                list_item.set_corner_nw_lat(row.corner_nw_lat);
                list_item.set_corner_nw_lon(row.corner_nw_lon);
                list_item.set_rows(row.rows);
                list_item.set_cols(row.cols);
                list_item.set_n_sats(row.n_sats);
            }
        }

        self.client_sender
            .send_data(serialize_capnproto_builder(builder));
    }
}
