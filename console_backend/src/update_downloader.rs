use crate::update_tab::UpdateTabContext;
use anyhow::bail;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::{Path, PathBuf},
};

const INDEX_URL: &str =
    "https://s3-us-west-1.amazonaws.com/downloads.swiftnav.com/index_https.json";

const V2_LINK: &str =
    "https://www.swiftnav.com/resource-files/Piksi%20Multi/v2.0.0/Firmware/PiksiMulti-v2.0.0.bin";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PiksiMultiDataConsole {
    version: String,
    darwin_url: String,
    linux2_url: String,
    linux_url: String,
    win32_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PiksiMultiDataFw {
    url: String,
    version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PiksiMultiData {
    console: PiksiMultiDataConsole,
    fw: PiksiMultiDataFw,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct IndexData {
    piksi_multi: PiksiMultiData,
}

/// UpdateDownloader struct.
///
/// # Fields:
///
/// - `index_data`: The deserialized index data containing firmware/console download links and versions.
#[derive(Clone)]
pub struct UpdateDownloader {
    index_data: Option<IndexData>,
}
impl UpdateDownloader {
    pub fn new() -> UpdateDownloader {
        UpdateDownloader { index_data: None }
    }

    pub fn latest_firmware_version(&mut self) -> anyhow::Result<String> {
        self.get_index_data()?;
        if let Some(index_data) = &self.index_data {
            Ok(index_data.piksi_multi.fw.version.clone())
        } else {
            bail!("Unable to fetch latest firmware version.");
        }
    }

    pub fn latest_console_version(&mut self) -> anyhow::Result<String> {
        self.get_index_data()?;
        if let Some(index_data) = &self.index_data {
            Ok(index_data.piksi_multi.console.version.clone())
        } else {
            bail!("Unable to fetch latest console version.");
        }
    }

    pub fn download_multi_v2_firmware(
        &mut self,
        directory: PathBuf,
        update_shared: Option<UpdateTabContext>,
    ) -> anyhow::Result<PathBuf> {
        let filepath_url = String::from(V2_LINK);
        self.download_file_from_url(filepath_url, directory, update_shared)
    }

    pub fn download_multi_firmware(
        &mut self,
        directory: PathBuf,
        update_shared: Option<UpdateTabContext>,
    ) -> anyhow::Result<PathBuf> {
        self.get_index_data()?;
        if let Some(index_data) = &self.index_data {
            let filepath_url = index_data.piksi_multi.fw.url.clone();
            Ok(self.download_file_from_url(filepath_url, directory, update_shared)?)
        } else {
            bail!("Unable to download multi firmware.");
        }
    }

    pub fn get_index_data(&mut self) -> anyhow::Result<()> {
        if self.index_data.is_none() {
            self.index_data = match self.fetch_index_data() {
                Ok(data) => data,
                Err(err) => bail!("Unable to get index data: {}", err),
            };
        }
        Ok(())
    }

    fn fetch_index_data(&mut self) -> anyhow::Result<Option<IndexData>> {
        let client = Client::new();
        let response = client.get(INDEX_URL).send()?;
        let index_data: IndexData = serde_json::from_str(&response.text()?)?;
        Ok(Some(index_data))
    }

    fn download_file_from_url(
        &mut self,
        filepath_url: String,
        directory: PathBuf,
        update_shared: Option<UpdateTabContext>,
    ) -> anyhow::Result<PathBuf> {
        let pathbuf_to_unix_filepath =
            |path: PathBuf| PathBuf::from(path.to_string_lossy().replace("\\", "/"));
        let filename = Path::new(&filepath_url).file_name();
        if let Some(filename_) = filename {
            let filepath = Path::new(&directory).join(filename_);
            if !directory.exists() {
                let msg = format!(
                    "Creating directory: {:?}",
                    pathbuf_to_unix_filepath(directory.clone())
                );
                if let Some(update_shared) = update_shared.clone() {
                    update_shared.fw_log_append(msg);
                }
                create_dir_all(&directory)?;
            }
            let msg = format!("Downloading firmware file: {:?}", filename_);
            if let Some(update_shared) = update_shared.clone() {
                update_shared.fw_log_append(msg);
            }
            let client = Client::new();
            let response = client.get(filepath_url).send()?;
            let mut file = File::create(filepath.clone())?;
            file.write_all(&response.bytes()?)?;
            let msg = format!(
                "Downloaded firmware file to: {:?}",
                pathbuf_to_unix_filepath(filepath.clone())
            );
            if let Some(update_shared) = update_shared {
                update_shared.fw_log_append(msg);
            }
            Ok(filepath)
        } else {
            bail!("Unable to extract filename from index data url.");
        }
    }
}

impl Default for UpdateDownloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use tempfile::TempDir;

    #[test]
    #[ignore]
    fn fetch_index_data_test() {
        let mut downloader = UpdateDownloader::new();
        let index_data = downloader.fetch_index_data().unwrap().unwrap();
        assert!(index_data.piksi_multi.fw.url.contains("https"));
    }
    #[test]
    #[ignore]
    fn download_multi_firmware_test() {
        let mut downloader = UpdateDownloader::new();

        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir = tmp_dir.path().to_path_buf();

        let filename = downloader
            .download_multi_firmware(tmp_dir.clone(), None)
            .unwrap();
        let pattern = tmp_dir.join("PiksiMulti-*");
        let found_filepath = glob(&pattern.to_string_lossy())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        assert_eq!(filename, found_filepath);
    }
}
