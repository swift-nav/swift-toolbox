use sbp::{
    codec::{
        dencode::{FramedWrite, IterSinkExt},
        json::JsonEncoder,
        sbp::SbpEncoder,
    },
    messages::SBP,
};
use serde::Serialize;
use serde_json::ser::CompactFormatter;
use std::{fs::File, path::Path};

use crate::types::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum CsvLogging {
    Off,
    On,
}
impl From<bool> for CsvLogging {
    fn from(logging: bool) -> Self {
        if logging {
            CsvLogging::On
        } else {
            CsvLogging::Off
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SbpLogging {
    Off,
    Sbp,
    Json,
}
impl From<(bool, bool)> for SbpLogging {
    fn from(logging_and_format: (bool, bool)) -> Self {
        let (sbp_logging, sbp_format) = logging_and_format;
        if sbp_logging {
            if sbp_format {
                SbpLogging::Sbp
            } else {
                SbpLogging::Json
            }
        } else {
            SbpLogging::Off
        }
    }
}

pub enum SbpLogger {
    Sbp(FramedWrite<File, SbpEncoder>),
    Json(FramedWrite<File, JsonEncoder<CompactFormatter>>),
}
impl SbpLogger {
    pub fn new_sbp<P: AsRef<Path>>(filepath: P) -> Result<SbpLogger> {
        Ok(SbpLogger::Sbp(SbpEncoder::framed(File::create(filepath)?)))
    }
    pub fn new_sbp_json<P: AsRef<Path>>(filepath: P) -> Result<SbpLogger> {
        Ok(SbpLogger::Json(JsonEncoder::framed(
            File::create(filepath)?,
            CompactFormatter,
        )))
    }
    pub fn serialize(&mut self, msg: &SBP) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::path::Path;
    use tempfile::TempDir;
    const TEST_FILEPATH: &str = "test.csv";

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
            let mut csv_s = CsvSerializer::new(filepath.to_string()).unwrap();
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
}
