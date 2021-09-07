use crate::update_tab::UpdateShared;
use anyhow::bail;
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::{Path, PathBuf},
};

const INDEX_URL: &str =
    "https://s3-us-west-1.amazonaws.com/downloads.swiftnav.com/index_https.json";

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

    pub fn download_multi_firmware(
        &mut self,
        directory: PathBuf,
        update_shared: Option<UpdateShared>,
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
            self.index_data = match fetch_index_data() {
                Ok(data) => data,
                Err(err) => bail!("Unable to get index data: {}", err),
            };
        }
        Ok(())
    }

    fn download_file_from_url(
        &mut self,
        filepath_url: String,
        directory: PathBuf,
        update_shared: Option<UpdateShared>,
    ) -> anyhow::Result<PathBuf> {
        let filename = Path::new(&filepath_url).file_name();
        if let Some(filename_) = filename {
            let filepath = Path::new(&directory).join(filename_);
            if !directory.exists() {
                let msg = format!("Creating directory: {:?}", directory);
                debug!("{}", msg);
                if let Some(update_shared) = update_shared.clone() {
                    update_shared.fw_log_append(msg);
                }
                create_dir_all(&directory)?;
            }
            if !filepath.exists() {
                let msg = format!("Downloading firmware file: {}", filepath_url);
                debug!("{}", msg);
                if let Some(update_shared) = update_shared.clone() {
                    update_shared.fw_log_append(msg);
                }
                let response = minreq::get(filepath_url).send()?;
                let mut file = File::create(filepath.clone())?;
                file.write_all(&response.into_bytes())?;
                let msg = format!("Downloaded firmware file to: {:?}", filepath);
                debug!("{}", msg);
                if let Some(update_shared) = update_shared {
                    update_shared.fw_log_append(msg);
                }
            } else {
                let msg = format!("Firmware file already exists: {:?}", filepath);
                debug!("{}", msg);
                if let Some(update_shared) = update_shared {
                    update_shared.fw_log_append(msg);
                }
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

fn fetch_index_data() -> anyhow::Result<Option<IndexData>> {
    let response = minreq::get(INDEX_URL).send()?;
    let index_data: IndexData = serde_json::from_str(response.as_str()?)?;
    Ok(Some(index_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use tempfile::TempDir;

    #[test]
    fn fetch_index_data_test() {
        let index_data = fetch_index_data().unwrap().unwrap();
        assert!(index_data.piksi_multi.fw.url.contains("https"));
    }
    #[test]
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
