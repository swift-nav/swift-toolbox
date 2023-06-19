pub mod advanced_imu_tab;
pub mod advanced_magnetometer_tab;
pub mod advanced_networking_tab;
pub mod advanced_spectrum_analyzer_tab;
pub mod advanced_system_monitor_tab;
pub mod baseline_tab;
pub mod cli_options;
pub mod console_backend_capnp {
    include!(concat!(env!("OUT_DIR"), "/console_backend_capnp.rs"));
}
pub mod client_sender;
pub mod common_constants;
pub mod connection;
pub mod constants;
pub mod date_conv;
pub mod errors;
pub mod fft_monitor;
pub mod fileio;
pub mod firmware_update;
pub mod formatters;
pub mod fusion_status_flags;
pub mod log_panel;
pub mod main_tab;
pub mod ntrip_output;
pub mod ntrip_tab;
pub mod observation_tab;
pub mod output;
pub mod piksi_tools_constants;
pub mod process_messages;
#[cfg(not(test))]
#[cfg(all(not(feature = "benches"), not(feature = "tests"), feature = "pyo3"))]
pub mod server;
pub mod server_recv_thread;
pub mod settings_tab;
pub mod shared_state;
pub mod solution_tab;
pub mod solution_velocity_tab;
pub mod status_bar;
pub mod swift_version;
pub mod tracking_signals_tab;
pub mod tracking_sky_plot_tab;
pub mod types;
pub mod update_downloader;
pub mod update_tab;
pub mod utils;
pub mod watch;

use std::sync::Mutex;

use crate::client_sender::BoxedClientSender;
use crate::shared_state::SharedState;
use crate::types::MsgSender;
use crate::{
    advanced_imu_tab::AdvancedImuTab, advanced_magnetometer_tab::AdvancedMagnetometerTab,
    advanced_networking_tab::AdvancedNetworkingTab,
    advanced_spectrum_analyzer_tab::AdvancedSpectrumAnalyzerTab,
    advanced_system_monitor_tab::AdvancedSystemMonitorTab, baseline_tab::BaselineTab,
    main_tab::MainTab, observation_tab::ObservationTab, settings_tab::SettingsTab,
    solution_tab::SolutionTab, solution_velocity_tab::SolutionVelocityTab, status_bar::StatusBar,
    tracking_signals_tab::TrackingSignalsTab, tracking_sky_plot_tab::TrackingSkyPlotTab,
    update_tab::UpdateTab,
};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct Tabs {
    pub main: Mutex<MainTab>,
    pub advanced_imu: Mutex<AdvancedImuTab>,
    pub advanced_magnetometer: Mutex<AdvancedMagnetometerTab>,
    pub advanced_networking: Mutex<AdvancedNetworkingTab>,
    pub advanced_system_monitor: Mutex<AdvancedSystemMonitorTab>,
    pub baseline: Mutex<BaselineTab>,
    pub tracking_signals: Mutex<TrackingSignalsTab>,
    pub tracking_sky_plot: Mutex<TrackingSkyPlotTab>,
    pub solution: Mutex<SolutionTab>,
    pub observation: Mutex<ObservationTab>,
    pub solution_velocity: Mutex<SolutionVelocityTab>,
    pub advanced_spectrum_analyzer: Mutex<AdvancedSpectrumAnalyzerTab>,
    pub status_bar: Mutex<StatusBar>,
    pub update: Mutex<UpdateTab>,
    pub settings: Option<SettingsTab>,
    pub shared_state: SharedState,
}

impl Tabs {
    fn new(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
        msg_sender: MsgSender,
    ) -> Self {
        Self {
            main: MainTab::new(shared_state.clone(), client_sender.clone()).into(),
            advanced_imu: AdvancedImuTab::new(client_sender.clone()).into(),
            advanced_magnetometer: AdvancedMagnetometerTab::new(client_sender.clone()).into(),
            advanced_networking: AdvancedNetworkingTab::new(
                shared_state.clone(),
                client_sender.clone(),
                msg_sender.clone(),
            )
            .into(),
            advanced_system_monitor: AdvancedSystemMonitorTab::new(client_sender.clone()).into(),
            baseline: BaselineTab::new(shared_state.clone(), client_sender.clone(), msg_sender)
                .into(),
            tracking_signals: TrackingSignalsTab::new(shared_state.clone(), client_sender.clone())
                .into(),
            tracking_sky_plot: TrackingSkyPlotTab::new(client_sender.clone(), shared_state.clone())
                .into(),
            observation: ObservationTab::new(shared_state.clone(), client_sender.clone()).into(),
            solution: SolutionTab::new(shared_state.clone(), client_sender.clone()).into(),
            solution_velocity: SolutionVelocityTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
            advanced_spectrum_analyzer: AdvancedSpectrumAnalyzerTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
            status_bar: StatusBar::new(shared_state.clone()).into(),
            update: UpdateTab::new(shared_state.clone()).into(),
            settings: None,
            shared_state,
        }
    }

    fn with_settings(
        shared_state: SharedState,
        client_sender: BoxedClientSender,
        msg_sender: MsgSender,
    ) -> Self {
        let mut tabs = Self::new(
            shared_state.clone(),
            client_sender.clone(),
            msg_sender.clone(),
        );
        tabs.settings = Some(SettingsTab::new(shared_state, client_sender, msg_sender));
        tabs
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
