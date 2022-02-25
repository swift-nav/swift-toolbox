use std::fs::{File, OpenOptions};
use std::path::Path;

use sbp::json::JsonEncoder;
use sbp::{Sbp, SbpEncoder};
use serde::Serialize;
use serde_json::ser::CompactFormatter;

use crate::common_constants as cc;
use crate::formatters::*;
use crate::types::Result;

pub type CsvLogging = cc::CsvLogging;
impl From<bool> for CsvLogging {
    fn from(logging: bool) -> Self {
        if logging {
            CsvLogging::ON
        } else {
            CsvLogging::OFF
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
pub enum SbpLogger {
    Sbp(SbpEncoder<File>),
    Json(JsonEncoder<File, CompactFormatter>),
}

impl SbpLogger {
    pub fn new_sbp<P: AsRef<Path>>(filepath: P) -> Result<SbpLogger> {
        Ok(SbpLogger::Sbp(SbpEncoder::new(File::create(filepath)?)))
    }
    pub fn open_sbp<P: AsRef<Path>>(filepath: P) -> Result<SbpLogger> {
        Ok(SbpLogger::Sbp(SbpEncoder::new(
            OpenOptions::new().append(true).open(filepath)?,
        )))
    }
    pub fn new_sbp_json<P: AsRef<Path>>(filepath: P) -> Result<SbpLogger> {
        Ok(SbpLogger::Json(JsonEncoder::new(
            File::create(filepath)?,
            CompactFormatter,
        )))
    }
    pub fn open_sbp_json<P: AsRef<Path>>(filepath: P) -> Result<SbpLogger> {
        Ok(SbpLogger::Json(JsonEncoder::new(
            OpenOptions::new().append(true).open(filepath)?,
            CompactFormatter,
        )))
    }
    pub fn serialize(&mut self, msg: &Sbp) -> Result<()> {
        match self {
            SbpLogger::Sbp(logger) => {
                logger.send(msg)?;
            }
            SbpLogger::Json(logger) => {
                logger.send(msg)?;
            }
        }
        Ok(())
    }
}

/// CsvSerializer for creating and writing to a csv.
/// Taken from ICBINS/src/output.rs.
#[derive(Debug)]
pub struct CsvSerializer {
    writer: csv::Writer<File>,
}

impl CsvSerializer {
    pub fn new<P: AsRef<Path>>(filepath: P) -> Result<CsvSerializer> {
        Ok(CsvSerializer {
            writer: csv::Writer::from_path(filepath)?,
        })
    }
    pub fn serialize<T>(&mut self, ds: &T) -> Result<()>
    where
        T: Serialize,
    {
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

    use sbp::messages::{navigation::MsgAgeCorrections, system::MsgInsUpdates};
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
            let mut sbp_logger = SbpLogger::new_sbp(&filepath).unwrap();
            sbp_logger.serialize(&msg_one_wrapped).unwrap();
            sbp_logger.serialize(&msg_two_wrapped).unwrap();
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
