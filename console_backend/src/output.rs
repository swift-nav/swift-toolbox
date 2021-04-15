use serde::Serialize;
use std::fs::File;

use crate::types::Result;
/// CsvSerializer for creating and writing to a csv.
/// Taken from ICBINS/src/output.rs.
#[derive(Debug)]
pub struct CsvSerializer {
    writer: csv::Writer<File>,
}

impl CsvSerializer {
    pub fn new(filepath: String) -> Result<CsvSerializer> {
        Ok(CsvSerializer {
            writer: csv::Writer::from_path(filepath)?,
        })
    }
    // TODO(john-michaelburke@) https://swift-nav.atlassian.net/browse/CPP-95
    // Validate Solution Tab logging.
    pub fn serialize<T>(&mut self, ds: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.writer.serialize(ds)?;
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
