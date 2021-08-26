pub mod advanced_ins_tab;
pub mod advanced_magnetometer_tab;
pub mod advanced_spectrum_analyzer_tab;
pub mod baseline_tab;
pub mod cli_options;
pub mod console_backend_capnp {
    include!(concat!(env!("OUT_DIR"), "/console_backend_capnp.rs"));
}
pub mod broadcaster;
pub mod common_constants;
pub mod connection;
pub mod constants;
pub mod date_conv;
pub mod errors;
pub mod fft_monitor;
pub mod fileio;
pub mod formatters;
pub mod fusion_status_flags;
pub mod log_panel;
pub mod main_tab;
pub mod observation_tab;
pub mod output;
pub mod piksi_tools_constants;
pub mod process_messages;
#[cfg(not(test))]
#[cfg(all(not(feature = "benches"), not(feature = "tests"), feature = "pyo3"))]
pub mod server;
pub mod solution_tab;
pub mod solution_velocity_tab;
pub mod status_bar;
pub mod tracking_signals_tab;
pub mod types;
pub mod utils;

use std::cell::RefCell;

use crate::{
    advanced_ins_tab::AdvancedInsTab, advanced_magnetometer_tab::AdvancedMagnetometerTab,
    advanced_spectrum_analyzer_tab::AdvancedSpectrumAnalyzerTab, baseline_tab::BaselineTab,
    main_tab::MainTab, observation_tab::ObservationTab, solution_tab::SolutionTab,
    solution_velocity_tab::SolutionVelocityTab, status_bar::StatusBar,
    tracking_signals_tab::TrackingSignalsTab,
};

struct Tabs<'a, S: types::CapnProtoSender> {
    pub main_tab: RefCell<MainTab<S>>,
    pub advanced_ins_tab: RefCell<AdvancedInsTab<S>>,
    pub advanced_magnetometer_tab: RefCell<AdvancedMagnetometerTab<S>>,
    pub baseline_tab: RefCell<BaselineTab<'a, S>>,
    pub tracking_signals_tab: RefCell<TrackingSignalsTab<S>>,
    pub solution_tab: RefCell<SolutionTab<S>>,
    pub observation_tab: RefCell<ObservationTab<S>>,
    pub solution_velocity_tab: RefCell<SolutionVelocityTab<'a, S>>,
    pub advanced_spectrum_analyzer_tab: RefCell<AdvancedSpectrumAnalyzerTab<S>>,
    pub status_bar: RefCell<StatusBar<S>>,
}

impl<'a, S: types::CapnProtoSender> Tabs<'a, S> {
    fn new(
        shared_state: types::SharedState,
        client_sender: S,
        msg_sender: types::MsgSender,
    ) -> Self {
        Self {
            main_tab: MainTab::new(shared_state.clone(), client_sender.clone()).into(),
            advanced_ins_tab: AdvancedInsTab::new(shared_state.clone(), client_sender.clone())
                .into(),
            advanced_magnetometer_tab: AdvancedMagnetometerTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
            baseline_tab: BaselineTab::new(shared_state.clone(), client_sender.clone(), msg_sender)
                .into(),
            tracking_signals_tab: TrackingSignalsTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
            observation_tab: ObservationTab::new(shared_state.clone(), client_sender.clone())
                .into(),
            solution_tab: SolutionTab::new(shared_state.clone(), client_sender.clone()).into(),
            solution_velocity_tab: SolutionVelocityTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
            advanced_spectrum_analyzer_tab: AdvancedSpectrumAnalyzerTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
            status_bar: StatusBar::new(shared_state.clone(), client_sender.clone()).into(),
        }
    }
}

#[cfg(test)]
pub mod test_common {
    use crate::constants::*;
    use directories::UserDirs;
    use std::{fs, path::PathBuf};

    pub mod data_directories {
        #![allow(dead_code)]
        pub const LINUX: &str = ".local/share/swift_navigation_console";
        pub const MACOS: &str =
            "Library/Application Support/com.swift-nav.swift-nav.swift_navigation_console";
        pub const WINDOWS: &str = "AppData\\Local\\swift-nav\\swift_navigation_console\\data";
    }

    pub fn filename() -> PathBuf {
        let user_dirs = UserDirs::new().unwrap();
        let home_dir = user_dirs.home_dir();
        #[cfg(target_os = "linux")]
        {
            home_dir
                .join(data_directories::LINUX)
                .join(CONNECTION_HISTORY_FILENAME)
        }

        #[cfg(target_os = "macos")]
        {
            home_dir
                .join(data_directories::MACOS)
                .join(CONNECTION_HISTORY_FILENAME)
        }
        #[cfg(target_os = "windows")]
        {
            home_dir
                .join(data_directories::WINDOWS)
                .join(CONNECTION_HISTORY_FILENAME)
        }
    }

    pub fn backup_file(filename: PathBuf) {
        if filename.exists() {
            let mut backup_filename = filename.clone();
            backup_filename.set_extension("backup");
            fs::rename(filename, backup_filename).unwrap();
        }
    }

    pub fn restore_backup_file(filename: PathBuf) {
        let mut backup_filename = filename.clone();
        backup_filename.set_extension("backup");
        if filename.exists() {
            fs::remove_file(filename.clone()).unwrap();
        }
        if backup_filename.exists() {
            fs::rename(backup_filename, filename).unwrap();
        }
    }
}
