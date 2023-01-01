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

use log::error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use sbp::json::to_vec;
use sbp::{Frame, Sbp};
use serde::Serialize;

use crate::common_constants as cc;
use crate::formatters::*;
use crate::types::Result;
use crate::utils::OkOrLog;

pub type CsvLogging = cc::CsvLogging;
impl From<bool> for CsvLogging {
    fn from(logging: bool) -> Self {
        match logging {
            true => CsvLogging::ON,
            false => CsvLogging::OFF,
        }
    }
}
impl CsvLogging {
    pub fn to_bool(&self) -> bool {
        matches!(self, CsvLogging::ON)
    }
}

pub type SbpLogging = cc::SbpLogging;

#[derive(Debug)]
pub struct SbpLogger<W> {
    logger: SbpLogging,
    out: W,
    path: Option<PathBuf>,
}

impl<W: Write> SbpLogger<W> {
    pub fn new(logger: SbpLogging, out: W, path: Option<PathBuf>) -> Self {
        Self { logger, out, path }
    }

    /// Log data into respective outputs, SBP or JSON format
    ///
    /// # Parameters:
    /// - `frame`: The raw frame data that a message must have
    /// - `msg`: Optional SBP depending on validity, only required in JSON format
    ///
    /// # Returns: the bytes serialized and whether path exists.
    /// Error returns should be considered interruptions which breaks the callers flow.
    pub fn serialize(&mut self, frame: &Frame, msg: Option<&Sbp>) -> Result<u16> {
        if msg.is_none() {
            error!("(SBP) message cannot be parsed as SBP, serializing frame instead: {frame:?}");
        }
        let bytes = match &self.logger {
            SbpLogging::SBP_JSON => msg
                .map(to_vec)
                .and_then(|ret| ret.ok_or_log(|_| error!("error serializing SBP to JSON"))),
            SbpLogging::SBP => Some(frame.as_bytes().to_owned()),
        };

        if let Some(path) = &self.path {
            if !path.exists() {
                return Err(format!("serializing path {} does not exist", path.display()).into());
            }
        }

        Ok(bytes
            .map(|b| self.out.write_all(b.as_slice()).map(|_| b))
            .transpose()
            .ok_or_log(|e| error!("{e}"))
            .flatten()
            .map(|b| b.len() as u16)
            .unwrap_or(0))
    }
}

impl SbpLogging {
    pub fn new_logger(&self, path: PathBuf) -> Result<SbpLogger<File>> {
        let out = OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(SbpLogger::new(self.to_owned(), out, Some(path)))
    }
}

/// CsvSerializer for creating and writing to a csv.
/// Taken from ICBINS/src/output.rs.
#[derive(Debug)]
pub struct CsvSerializer {
    writer: csv::Writer<File>,
}

impl CsvSerializer {
    pub fn new(filepath: impl AsRef<Path>) -> Result<CsvSerializer> {
        let writer = csv::Writer::from_path(filepath)?;
        Ok(CsvSerializer { writer })
    }

    pub fn new_option(filepath: impl AsRef<Path> + Copy) -> Option<CsvSerializer> {
        CsvSerializer::new(filepath).ok_or_log(|e| {
            let fname = filepath.as_ref().display();
            error!("issue creating file, {fname:?}, error, {e}")
        })
    }

    pub fn serialize(&mut self, ds: &impl Serialize) -> Result<()> {
        self.writer.serialize(ds)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct BaselineLog {
    pub pc_time: String,
    pub gps_time: Option<String>,
    #[serde(rename = "tow(sec)", with = "float_formatter_3")]
    pub tow_s: Option<f64>,
    #[serde(rename = "north(meters)", with = "float_formatter_6")]
    pub north_m: Option<f64>,
    #[serde(rename = "east(meters)", with = "float_formatter_6")]
    pub east_m: Option<f64>,
    #[serde(rename = "down(meters)", with = "float_formatter_6")]
    pub down_m: Option<f64>,
    #[serde(rename = "h_accuracy(meters)", with = "float_formatter_4")]
    pub h_accuracy_m: Option<f64>,
    #[serde(rename = "v_accuracy(meters)", with = "float_formatter_4")]
    pub v_accuracy_m: Option<f64>,
    #[serde(rename = "distance(meters)", with = "float_formatter_6")]
    pub distance_m: Option<f64>,
    pub num_sats: u8,
    pub flags: u8,
}

#[derive(Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct PosLLHLog {
    pub pc_time: String,
    pub gps_time: Option<String>,
    #[serde(rename = "tow(sec)", with = "float_formatter_3")]
    pub tow_s: Option<f64>,
    #[serde(rename = "latitude(degrees)", with = "float_formatter_10")]
    pub latitude_d: Option<f64>,
    #[serde(rename = "longitude(degrees)", with = "float_formatter_10")]
    pub longitude_d: Option<f64>,
    #[serde(rename = "altitude(meters)", with = "float_formatter_4")]
    pub altitude_m: Option<f64>,
    #[serde(rename = "h_accuracy(meters)", with = "float_formatter_4")]
    pub h_accuracy_m: Option<f64>,
    #[serde(rename = "v_accuracy(meters)", with = "float_formatter_4")]
    pub v_accuracy_m: Option<f64>,
    pub n_sats: u8,
    pub flags: u8,
}

#[derive(Serialize)]
pub struct VelLog {
    pub pc_time: String,
    pub gps_time: Option<String>,
    #[serde(rename = "tow(sec)", with = "float_formatter_3")]
    pub tow_s: Option<f64>,
    #[serde(rename = "north(m/s)", with = "float_formatter_6")]
    pub north_mps: Option<f64>,
    #[serde(rename = "east(m/s)", with = "float_formatter_6")]
    pub east_mps: Option<f64>,
    #[serde(rename = "down(m/s)", with = "float_formatter_6")]
    pub down_mps: Option<f64>,
    #[serde(rename = "speed(m/s)", with = "float_formatter_6")]
    pub speed_mps: Option<f64>,
    pub flags: u8,
    pub num_signals: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_common::msg_to_frame;
    use sbp::messages::{navigation::MsgAgeCorrections, system::MsgInsUpdates};
    use sbp::{Sbp, SbpMessage};
    use serde::Serialize;
    use std::{fs::File, path::Path};
    use tempfile::TempDir;

    const TEST_FILEPATH: &str = "test.csv";
    const TEST_SBP_FILEPATH: &str = "test.sbp";

    #[derive(Serialize)]
    struct TestDataSet {
        float: f64,
        string: String,
        option_int: Option<i32>,
    }

    fn serialize_test_dataset(csv_serializer: &mut CsvSerializer, ds: &TestDataSet) -> Result<()> {
        csv_serializer.writer.serialize(ds)?;
        Ok(())
    }

    #[test]
    fn csv_serializer_test() {
        let tmp_dir = TempDir::new().unwrap();
        let filepath = tmp_dir.path();
        let filepath = filepath.join(TEST_FILEPATH);
        let filepath = filepath.to_str().unwrap();
        {
            let mut csv_s = CsvSerializer::new(filepath).unwrap();
            let dataset_first = TestDataSet {
                float: 13_f64,
                string: String::from("37"),
                option_int: None,
            };
            serialize_test_dataset(&mut csv_s, &dataset_first).unwrap();
            let dataset_second = TestDataSet {
                float: 1_f64,
                string: String::from("3"),
                option_int: Some(-37),
            };
            serialize_test_dataset(&mut csv_s, &dataset_second).unwrap();
        }
        assert!(Path::new(&filepath).is_file());
    }
    #[test]
    fn sbp_serializer_test() {
        let tmp_dir = TempDir::new().unwrap();
        let filepath = tmp_dir.path();
        let filepath = filepath.join(TEST_SBP_FILEPATH);
        let msg_one = MsgAgeCorrections {
            sender_id: Some(1337),
            age: 0xFFFF,
            tow: 0,
        };
        let msg_one_wrapped = Sbp::MsgAgeCorrections(msg_one.clone());
        let msg_two = MsgInsUpdates {
            sender_id: Some(1337),
            gnsspos: 4,
            gnssvel: 4,
            wheelticks: 0xff_u8,
            speed: 0,
            nhc: 0,
            zerovel: 0,
            tow: 0,
        };
        let msg_two_wrapped = Sbp::MsgInsUpdates(msg_two.clone());
        {
            let mut sbp_logger = SbpLogging::SBP.new_logger(filepath.to_owned()).unwrap();
            assert_eq!(
                &msg_one_wrapped.encoded_len(),
                &(sbp_logger.serialize(&msg_to_frame(msg_one_wrapped), None) as usize)
            );
            assert_eq!(
                &msg_two_wrapped.encoded_len(),
                &(sbp_logger.serialize(&msg_to_frame(msg_two_wrapped), None) as usize)
            );
        }
        assert!(&filepath.is_file());
        {
            let file_read = File::open(filepath).unwrap();
            let mut messages = sbp::iter_messages(file_read);
            let msg = messages.next().unwrap().unwrap();
            match msg {
                Sbp::MsgAgeCorrections(msg) => {
                    assert_eq!(msg.sender_id, msg_one.sender_id);
                    assert_eq!(msg.age, msg_one.age);
                    assert_eq!(msg.tow, msg_one.tow);
                }
                _ => panic!("first message does not match"),
            }
            let msg = messages.next().unwrap().unwrap();
            match msg {
                Sbp::MsgInsUpdates(msg) => {
                    assert_eq!(msg.sender_id, msg_two.sender_id);
                    assert_eq!(msg.gnsspos, msg_two.gnsspos);
                    assert_eq!(msg.gnssvel, msg_two.gnssvel);
                    assert_eq!(msg.wheelticks, msg_two.wheelticks);
                    assert_eq!(msg.speed, msg_two.speed);
                    assert_eq!(msg.nhc, msg_two.nhc);
                    assert_eq!(msg.zerovel, msg_two.zerovel);
                    assert_eq!(msg.tow, msg_two.tow);
                }
                _ => panic!("second message does not match"),
            }
        }
    }
}
