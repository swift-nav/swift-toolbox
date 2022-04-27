use std::{path::Path, time::Duration};

use anyhow::anyhow;
use crossbeam::{channel, select};
use lazy_static::lazy_static;
use regex::Regex;
use sbp::{
    link::Link,
    messages::{
        logging::MsgLog,
        piksi::{MsgCommandReq, MsgCommandResp, MsgReset},
    },
    SbpString,
};

use crate::{
    constants::FIRMWARE_V2,
    fileio::{new_sequence, Fileio},
    swift_version::SwiftVersion,
    types::{MsgSender, Result},
};

const UPGRADE_FIRMWARE_REMOTE_DESTINATION: &str = "upgrade.image_set.bin";
const UPGRADE_FIRMWARE_TOOL: &str = "upgrade_tool";
const UPGRADE_WHITELIST: &[&str] = &[
    "ok",
    "writing.*",
    "erasing.*",
    "Error.*",
    "error.*",
    ".*Image.*",
    ".*upgrade.*",
    "Warning:*",
    ".*install.*",
    "upgrade completed successfully",
];

lazy_static! {
    static ref UPGRADE_PROGRESS_RE: Regex = Regex::new(r"\s*[0-9]* % complete").unwrap();
}

const UPGRADE_FIRMWARE_TIMEOUT_SEC: u64 = 600;

pub enum LogOverwriteBehavior {
    DontOverwrite,
    Overwrite,
}

pub fn firmware_update<LogCallback, ProgressCallback>(
    link: Link<'static, ()>,
    msg_sender: MsgSender,
    filepath: &Path,
    current_version: &SwiftVersion,
    log_callback: LogCallback,
    progress_callback: ProgressCallback,
) -> anyhow::Result<()>
where
    LogCallback: Fn(String, LogOverwriteBehavior) + Sync + Send + Clone + 'static,
    ProgressCallback: Fn(f64) + Sync + Send + 'static,
{
    let msg_log_callback = log_callback.clone();
    let key = link.register(move |msg: MsgLog| {
        handle_log_msg(msg, &msg_log_callback);
    });

    // Following is surrounded in a closure, to avoid forgetting to unregister the callback from the link
    let res = (|| {
        log_callback(
            format!("Reading firmware file from path, {}.", filepath.display()),
            LogOverwriteBehavior::DontOverwrite,
        );

        if !filepath.exists() || !filepath.is_file() {
            return Err(anyhow!(
                "Firmware filepath is not a file or does not exist."
            ));
        }
        let update_filename = filepath
            .file_name()
            .ok_or_else(|| anyhow!("Could not get update filename!"))?
            .to_str()
            .ok_or_else(|| anyhow!("Could not convert update filename!"))?;

        let update = SwiftVersion::parse_filename(update_filename)
            .map_err(|e| anyhow!("Failed to parse new firmware version: {:?}", e))?;

        firmware_can_upgrade(current_version, &update)?;

        let firmware_blob = std::fs::File::open(filepath)
            .map_err(|e| anyhow!("Failed to open firmware file: {:?}", e))?;

        log_callback(
            String::from("Transferring image to device..."),
            LogOverwriteBehavior::DontOverwrite,
        );

        let size = firmware_blob.metadata()?.len() as usize;

        let mut bytes_written = 0;
        progress_callback(0.0);

        let mut fileio = Fileio::new(link.clone(), msg_sender.clone());

        fileio.overwrite_with_progress(
            String::from(UPGRADE_FIRMWARE_REMOTE_DESTINATION),
            firmware_blob,
            |n| {
                bytes_written += n;
                let progress = (bytes_written as f64) / (size as f64) * 100.0;
                progress_callback(progress);
            },
        )?;

        log_callback(
            String::from("Image transfer complete."),
            LogOverwriteBehavior::DontOverwrite,
        );

        log_callback(
            String::from("Committing image to flash..."),
            LogOverwriteBehavior::DontOverwrite,
        );

        firmware_upgrade_commit_to_flash(link.clone(), msg_sender.clone())?;
        log_callback(
            String::from("Upgrade Complete."),
            LogOverwriteBehavior::DontOverwrite,
        );
        log_callback(
            String::from("Resetting Piksi..."),
            LogOverwriteBehavior::DontOverwrite,
        );
        msg_sender.send(MsgReset {
            sender_id: None,
            flags: 0,
        })?;

        Ok(())
    })();

    link.unregister(key);

    res
}

fn firmware_upgrade_commit_to_flash(link: Link<'static, ()>, msg_sender: MsgSender) -> Result<()> {
    let sequence = new_sequence();

    msg_sender.send(MsgCommandReq {
        sender_id: None,
        sequence,
        command: SbpString::from(format!(
            "{UPGRADE_FIRMWARE_TOOL} {UPGRADE_FIRMWARE_REMOTE_DESTINATION}",
        )),
    })?;

    let (finished_tx, finished_rx) = channel::unbounded();

    let key = link.register(move |msg: MsgCommandResp| {
        if msg.sequence == sequence {
            finished_tx.send(msg.code == 0).expect("Sending failed");
        }
    });

    let res = select! {
        recv(finished_rx) -> val => {
            if val.expect("Receiving failed") {
                Ok(())
            } else {
                Err(anyhow!("Failed to commit image to flash."))
            }
        }
        default(Duration::from_secs(UPGRADE_FIRMWARE_TIMEOUT_SEC)) => {
            Err(anyhow!("Firmware upgrade timed out"))
        }
    };

    link.unregister(key);
    res
}

fn handle_log_msg<LogCallback>(msg: MsgLog, log_callback: &LogCallback)
where
    LogCallback: Fn(String, LogOverwriteBehavior) + Sync + Send + Clone + 'static,
{
    let text = msg.text.to_string();

    if UPGRADE_PROGRESS_RE.is_match(&text) {
        log_callback(extract_log_message(&text), LogOverwriteBehavior::Overwrite);
        return;
    }

    for regex in UPGRADE_WHITELIST.iter() {
        if let Ok(reg) = Regex::new(regex) {
            if reg.captures(&text).is_some() {
                log_callback(
                    extract_log_message(&text),
                    LogOverwriteBehavior::DontOverwrite,
                );
            }
        }
    }
}

fn extract_log_message(text: &str) -> String {
    let text: String = text
        .chars()
        .map(|x| match x {
            '\r' => '\n',
            _ => x,
        })
        .collect();
    let text = text.split('\n').collect::<Vec<&str>>();
    let final_text = if text.len() > 1 {
        // upgrade tool delineates lines in stdout with \r, we want penultimate line that is complete to show
        text[text.len() - 2]
    } else {
        // If there is only one line, we show that
        text[text.len() - 1]
    };
    final_text.to_string()
}

fn firmware_can_upgrade(current: &SwiftVersion, update: &SwiftVersion) -> anyhow::Result<()> {
    if current.is_dev() || update.is_dev() {
        return Ok(());
    }

    if *current >= *FIRMWARE_V2 {
        return Ok(());
    }

    if *current < *FIRMWARE_V2 && *update != *FIRMWARE_V2 && *update > *FIRMWARE_V2 {
        return Err(anyhow!(
            "Upgrading to firmware v2.1.0 or later requires that the device be running firmware v2.0.0 or later."
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod firmware_update {
        use std::sync::{Arc, Mutex};

        use super::*;

        #[test]
        fn handle_log_msg_test() {
            let sender_id = Some(1337);
            let level = 1;

            let good_log_messages = vec![
                "ok\r",
                "writing\r",
                "erasing\r",
                "100 % complete\r",
                "Error\r",
                "error\r",
                "Image\r",
                "upgrade\r",
                "Warning:\r",
                "install\r",
                "upgrade completed successfully\r",
            ];

            let message = String::new();
            let last_message = Arc::new(Mutex::new(message));

            let callback_copy = last_message.clone();

            let callback = move |msg, _| {
                let mut current = callback_copy.lock().unwrap();
                *current = msg;
            };

            for log_message in good_log_messages {
                let msg = MsgLog {
                    sender_id,
                    level,
                    text: SbpString::from(log_message.to_string()),
                };
                handle_log_msg(msg, &callback);
                assert_eq!(*last_message.lock().unwrap(), log_message.trim());
            }

            let good_log_messages_one_line = vec![
                "ok",
                "writing",
                "erasing",
                "99 % complete",
                "Error",
                "error",
                "Image",
                "upgrade",
                "Warning",
                "install",
                "upgrade completed successfully",
            ];

            for log_message in good_log_messages_one_line {
                let msg = MsgLog {
                    sender_id,
                    level,
                    text: SbpString::from(log_message.to_string()),
                };
                handle_log_msg(msg, &callback);
                assert_eq!(
                    *last_message.lock().unwrap(),
                    log_message.trim().to_string()
                );
            }

            let bad_log_messages = vec![
                "o1k",
                "wr2iting",
                "era3sing",
                "99 %4 complete",
                "Er5ror",
                "er6ror",
                "Im7age",
                "up8grade",
                "Wa9rning",
                "in10stall",
                "upgr11ade completed successfully",
            ];

            for log_message in bad_log_messages {
                let msg = MsgLog {
                    sender_id,
                    level,
                    text: SbpString::from(log_message.to_string()),
                };
                handle_log_msg(msg, &callback);
                assert_ne!(
                    *last_message.lock().unwrap(),
                    log_message.trim().to_string()
                );
            }
        }
    }
}
