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
use crate::types::SignalCodes;
use crate::utils::{decode_ssr_update_interval, serialize_capnproto_builder};

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

#[derive(Clone, Debug)]
pub struct SsrStreamRow {
    pub last_seen: Instant,
    pub update_interval_sec: f64,
    pub iod_ssr: u8,
    pub count: u32,
}

#[derive(Debug)]
pub struct CorrectionsTab {
    pub client_sender: BoxedClientSender,
    pub shared_state: SharedState,
    streams: BTreeMap<&'static str, SsrStreamRow>,
    sat_corrections: BTreeMap<(i16, SignalCodes), SsrSatCorrectionRow>,
    tiles: BTreeMap<(u16, u16), SsrTileRow>,
}

/// Decode a North-West corner correction point coordinate.
///
/// See `MsgSsrTileDefinition::corner_nw_lat`/`corner_nw_lon` doc comments in
/// the sbp crate for the encoding.
fn decode_corner(coded: i16, range: f64, bits: u32) -> f64 {
    coded as f64 / (1_u32 << bits) as f64 * range
}

impl CorrectionsTab {
    pub fn new(shared_state: SharedState, client_sender: BoxedClientSender) -> CorrectionsTab {
        CorrectionsTab {
            client_sender,
            shared_state,
            streams: BTreeMap::new(),
            sat_corrections: BTreeMap::new(),
            tiles: BTreeMap::new(),
        }
    }

    fn touch_stream(&mut self, name: &'static str, update_interval_code: u8, iod_ssr: u8) {
        let row = self.streams.entry(name).or_insert_with(|| SsrStreamRow {
            last_seen: Instant::now(),
            update_interval_sec: 0.0,
            iod_ssr,
            count: 0,
        });
        row.last_seen = Instant::now();
        row.update_interval_sec = decode_ssr_update_interval(update_interval_code);
        row.iod_ssr = iod_ssr;
        row.count += 1;
    }

    pub fn handle_orbit_clock(&mut self, msg: MsgSsrOrbitClock) {
        self.touch_stream("MSG_SSR_ORBIT_CLOCK", msg.update_interval, msg.iod_ssr);
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
        self.touch_stream("MSG_SSR_CODE_BIASES", msg.update_interval, msg.iod_ssr);
        let key = (msg.sid.sat as i16, SignalCodes::from(msg.sid.code));
        let row = self.sat_corrections.entry(key).or_default();
        if let Some(first) = msg.biases.first() {
            row.code_bias = first.value;
        }
        row.last_seen = Some(Instant::now());
        self.send_data();
    }

    pub fn handle_phase_biases(&mut self, msg: MsgSsrPhaseBiases) {
        self.touch_stream("MSG_SSR_PHASE_BIASES", msg.update_interval, msg.iod_ssr);
        let key = (msg.sid.sat as i16, SignalCodes::from(msg.sid.code));
        let row = self.sat_corrections.entry(key).or_default();
        if let Some(first) = msg.biases.first() {
            row.phase_bias = first.bias;
        }
        row.last_seen = Some(Instant::now());
        self.send_data();
    }

    pub fn handle_tile_definition(&mut self, msg: MsgSsrTileDefinition) {
        self.touch_stream("MSG_SSR_TILE_DEFINITION", msg.update_interval, msg.iod_atmo);
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
            msg.header.update_interval,
            msg.header.iod_atmo,
        );
        self.send_data();
    }

    pub fn handle_stec_correction(&mut self, msg: MsgSsrStecCorrection) {
        self.touch_stream(
            "MSG_SSR_STEC_CORRECTION",
            msg.header.update_interval,
            msg.ssr_iod_atmo,
        );
        let key = (msg.tile_set_id, msg.tile_id);
        let row = self.tiles.entry(key).or_default();
        row.n_sats = msg.n_sats;
        self.send_data();
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
                list_item.set_update_interval_sec(row.update_interval_sec);
                list_item.set_iod_ssr(row.iod_ssr);
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
