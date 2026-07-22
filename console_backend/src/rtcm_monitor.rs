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

//! Detects which RTCM3 correction bundle is active (generic RTCM, Swift
//! NXRTK-MSM5, Swift OSR, or Swift SSR) by scanning the raw RTCM3 byte
//! stream for message-type IDs and tallying per-ID rates. Bundle boundaries
//! were derived empirically from sample captures of each bundle type, since
//! there's no ID table for them anywhere in this codebase or the RTCM spec
//! (Swift's bundle names are product terms, not RTCM-standardized).

use std::collections::BTreeMap;
use std::time::Instant;

const CRC24_POLY: u32 = 0x1864_CFB;

fn crc24q(data: &[u8]) -> u32 {
    let mut crc: u32 = 0;
    for &b in data {
        crc ^= (b as u32) << 16;
        for _ in 0..8 {
            crc <<= 1;
            if crc & 0x0100_0000 != 0 {
                crc ^= CRC24_POLY;
            }
        }
        crc &= 0x00FF_FFFF;
    }
    crc
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bundle {
    Generic,
    NxrtkMsm5,
    Osr,
    Ssr,
}

impl Bundle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Bundle::Generic => "GENERIC",
            Bundle::NxrtkMsm5 => "NXRTK_MSM5",
            Bundle::Osr => "OSR",
            Bundle::Ssr => "SSR",
        }
    }
}

/// Classify an RTCM3 message-type ID into one of the four correction
/// bundles. Priority matters: message 4062 (Swift's proprietary OSR
/// encoding) has been observed alongside genuine SSR messages (1059/1060)
/// in real SSR captures, so the SSR check must run first or SSR streams
/// would get misclassified as OSR.
fn classify(msg_id: u16) -> Bundle {
    const MSM5_IDS: [u16; 6] = [1075, 1085, 1095, 1105, 1115, 1125];
    const IGS_SSR_ID: u16 = 4076;
    if (1057..=1068).contains(&msg_id) || msg_id == IGS_SSR_ID {
        Bundle::Ssr
    } else if MSM5_IDS.contains(&msg_id) {
        Bundle::NxrtkMsm5
    } else if (4001..=4095).contains(&msg_id) {
        Bundle::Osr
    } else {
        Bundle::Generic
    }
}

#[derive(Debug)]
struct RtcmIdStats {
    first_seen: Instant,
    last_seen: Instant,
    count: u32,
}

#[derive(Debug, Clone)]
pub struct RtcmRow {
    pub msg_id: u16,
    pub rate: f64,
    pub age_sec: f64,
    pub bundle: Bundle,
    pub count: u32,
}

/// Longer than any legal RTCM3 frame (3-byte header + up to 1023-byte
/// payload + 3-byte CRC = 1029 bytes at most). Bounds `buffer`'s growth so
/// a stretch of non-RTCM bytes can't make it grow unbounded.
const MAX_BUFFER_LEN: usize = 4096;

#[derive(Debug, Default)]
pub struct RtcmMonitor {
    stats: BTreeMap<u16, RtcmIdStats>,
    /// Bytes carried over from previous `feed()` calls that didn't yet
    /// resolve to a complete frame. `write_function`/libcurl delivers the
    /// raw NTRIP byte stream in chunks with no relation to RTCM3 frame
    /// boundaries, so most frames longer than a chunk would otherwise never
    /// be seen whole and would silently go uncounted.
    buffer: Vec<u8>,
}

impl RtcmMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed the next chunk of raw bytes from the correction stream. Safe to
    /// call with arbitrary chunk boundaries: any frame that isn't yet fully
    /// present is buffered and re-checked on the next call rather than
    /// dropped.
    pub fn feed(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let now = Instant::now();
        let n = self.buffer.len();
        let mut consumed = 0;
        while consumed < n {
            let i = consumed;
            if self.buffer[i] != 0xD3 {
                consumed += 1;
                continue;
            }
            if i + 3 > n {
                break; // not enough bytes yet for the length header
            }
            let length =
                (((self.buffer[i + 1] & 0x03) as usize) << 8) | self.buffer[i + 2] as usize;
            let frame_len = 3 + length + 3;
            if i + frame_len > n {
                break; // frame not fully arrived yet
            }
            let header_and_payload = &self.buffer[i..i + 3 + length];
            let crc_given = ((self.buffer[i + 3 + length] as u32) << 16)
                | ((self.buffer[i + 3 + length + 1] as u32) << 8)
                | self.buffer[i + 3 + length + 2] as u32;
            if crc24q(header_and_payload) != crc_given {
                consumed += 1;
                continue;
            }
            let payload = &self.buffer[i + 3..i + 3 + length];
            if payload.len() >= 2 {
                let msg_id = ((payload[0] as u16) << 4) | (payload[1] >> 4) as u16;
                let entry = self.stats.entry(msg_id).or_insert_with(|| RtcmIdStats {
                    first_seen: now,
                    last_seen: now,
                    count: 0,
                });
                entry.last_seen = now;
                entry.count += 1;
            }
            consumed += frame_len;
        }
        self.buffer.drain(0..consumed);
        if self.buffer.len() > MAX_BUFFER_LEN {
            let excess = self.buffer.len() - MAX_BUFFER_LEN;
            self.buffer.drain(0..excess);
        }
    }

    pub fn rows(&self) -> Vec<RtcmRow> {
        let now = Instant::now();
        self.stats
            .iter()
            .map(|(&msg_id, stats)| {
                let elapsed = now.saturating_duration_since(stats.first_seen).as_secs_f64();
                let rate = if elapsed > f64::EPSILON {
                    stats.count as f64 / elapsed
                } else {
                    0.0
                };
                RtcmRow {
                    msg_id,
                    rate,
                    age_sec: now.saturating_duration_since(stats.last_seen).as_secs_f64(),
                    bundle: classify(msg_id),
                    count: stats.count,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real, CRC24Q-valid frames extracted from Swift SQA sample captures
    // (World_sweep/NX.rtcm and World_sweep/SSR.rtcm), used here to prove the
    // parser against genuine RTCM3 bytes rather than self-generated ones.
    const FRAME_1029: &[u8] = &[
        0xd3, 0x00, 0x13, 0x40, 0x51, 0xda, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x6e, 0x78, 0x2d,
        0x76, 0x33, 0x31, 0x2e, 0x30, 0x2e, 0x35, 0x53, 0x2b, 0x77,
    ];
    const FRAME_1059: &[u8] = &[
        0xd3, 0x00, 0x75, 0x42, 0x34, 0xe2, 0xb6, 0x66, 0x00, 0x86, 0x91, 0x62, 0x8c, 0x1f, 0xfc,
        0xaf, 0xff, 0x54, 0x7f, 0xc4, 0x51, 0x00, 0x00, 0x32, 0xc0, 0x0a, 0x40, 0x05, 0xaf, 0x03,
        0xd0, 0xf1, 0x80, 0x00, 0x75, 0x80, 0x16, 0x80, 0x00, 0x50, 0x10, 0x00, 0x4f, 0x58, 0x10,
        0x49, 0x10, 0x00, 0x17, 0x2c, 0x04, 0xc4, 0x00, 0xa6, 0xff, 0xae, 0x97, 0x20, 0x3f, 0xf7,
        0x5f, 0xfe, 0x48, 0xff, 0xfd, 0xff, 0x2c, 0xb0, 0x40, 0x7f, 0xbe, 0xbf, 0xf2, 0x51, 0xfe,
        0x2b, 0xff, 0xcb, 0x68, 0x80, 0x00, 0x75, 0x60, 0x18, 0x20, 0x03, 0xd7, 0x81, 0xd4, 0xd9,
        0x00, 0x00, 0x3a, 0xc0, 0x0b, 0x40, 0x00, 0x0f, 0x01, 0x7d, 0xd1, 0x80, 0x02, 0x25, 0x80,
        0x72, 0x80, 0x0b, 0xa0, 0x20, 0x3f, 0xff, 0x5f, 0xff, 0xc8, 0xff, 0x89, 0xe0, 0x4c, 0x00,
        0x9f, 0x7f, 0xa5,
    ];

    #[test]
    fn classify_test() {
        assert_eq!(classify(1006), Bundle::Generic);
        assert_eq!(classify(1046), Bundle::Generic);
        assert_eq!(classify(1075), Bundle::NxrtkMsm5);
        assert_eq!(classify(1125), Bundle::NxrtkMsm5);
        assert_eq!(classify(4062), Bundle::Osr);
        assert_eq!(classify(1057), Bundle::Ssr);
        assert_eq!(classify(1068), Bundle::Ssr);
        assert_eq!(classify(4076), Bundle::Ssr);
    }

    #[test]
    fn feed_parses_real_frames_and_resyncs() {
        let mut monitor = RtcmMonitor::new();
        // A stray non-sync byte in front proves the scanner resyncs
        // correctly rather than only working when 0xD3 starts at index 0.
        let mut bytes = vec![0x00];
        bytes.extend_from_slice(FRAME_1029);
        bytes.extend_from_slice(FRAME_1059);

        monitor.feed(&bytes);

        let rows = monitor.rows();
        let ids: Vec<u16> = rows.iter().map(|r| r.msg_id).collect();
        assert_eq!(ids, vec![1029, 1059]);

        let row_1029 = rows.iter().find(|r| r.msg_id == 1029).unwrap();
        assert_eq!(row_1029.bundle, Bundle::Generic);
        let row_1059 = rows.iter().find(|r| r.msg_id == 1059).unwrap();
        assert_eq!(row_1059.bundle, Bundle::Ssr);
    }

    #[test]
    fn feed_ignores_corrupt_frame() {
        let mut monitor = RtcmMonitor::new();
        let mut corrupted = FRAME_1029.to_vec();
        *corrupted.last_mut().unwrap() ^= 0xFF; // break the CRC
        monitor.feed(&corrupted);
        assert!(monitor.rows().is_empty());
    }

    #[test]
    fn feed_accumulates_count_across_calls() {
        let mut monitor = RtcmMonitor::new();
        monitor.feed(FRAME_1029);
        monitor.feed(FRAME_1029);
        monitor.feed(FRAME_1029);
        let rows = monitor.rows();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].msg_id, 1029);
        assert_eq!(rows[0].count, 3);
    }

    /// Regression test: `write_function`/libcurl delivers the NTRIP byte
    /// stream in chunks with no relation to RTCM3 frame boundaries. A naive
    /// per-call parser that doesn't buffer a split frame across calls
    /// silently drops any message longer than a chunk - which in practice
    /// meant the very messages that define the OSR/NXRTK-MSM5 bundles
    /// (larger than the short generic messages) were the most likely to
    /// never be counted.
    #[test]
    fn feed_reassembles_frame_split_across_small_chunks() {
        let mut monitor = RtcmMonitor::new();
        let mut bytes = FRAME_1029.to_vec();
        bytes.extend_from_slice(FRAME_1059); // 123 bytes, well over a small chunk
        for chunk in bytes.chunks(16) {
            monitor.feed(chunk);
        }
        let rows = monitor.rows();
        let ids: Vec<u16> = rows.iter().map(|r| r.msg_id).collect();
        assert_eq!(ids, vec![1029, 1059]);
        assert_eq!(rows.iter().find(|r| r.msg_id == 1059).unwrap().count, 1);
    }
}
