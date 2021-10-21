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
pub mod server_recv_thread;
pub mod settings_tab;
pub mod shared_state;
pub mod solution_tab;
pub mod solution_velocity_tab;
pub mod status_bar;
pub mod tracking_signals_tab;
pub mod tracking_sky_plot_tab;
pub mod types;
pub mod update_downloader;
pub mod update_tab;
pub mod utils;
pub mod watch;

use std::sync::Mutex;

use crate::{
    advanced_imu_tab::AdvancedImuTab, advanced_magnetometer_tab::AdvancedMagnetometerTab,
    advanced_networking_tab::AdvancedNetworkingTab,
    advanced_spectrum_analyzer_tab::AdvancedSpectrumAnalyzerTab,
    advanced_system_monitor_tab::AdvancedSystemMonitorTab, baseline_tab::BaselineTab,
    main_tab::MainTab, observation_tab::ObservationTab, solution_tab::SolutionTab,
    solution_velocity_tab::SolutionVelocityTab, status_bar::StatusBar,
    tracking_signals_tab::TrackingSignalsTab, tracking_sky_plot_tab::TrackingSkyPlotTab,
    update_tab::UpdateTab,
};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct Tabs<S: types::CapnProtoSender> {
    pub main: Mutex<MainTab<S>>,
    pub advanced_imu: Mutex<AdvancedImuTab<S>>,
    pub advanced_magnetometer: Mutex<AdvancedMagnetometerTab<S>>,
    pub advanced_networking: Mutex<AdvancedNetworkingTab<S>>,
    pub advanced_system_monitor: Mutex<AdvancedSystemMonitorTab<S>>,
    pub baseline: Mutex<BaselineTab<S>>,
    pub tracking_signals: Mutex<TrackingSignalsTab<S>>,
    pub tracking_sky_plot: Mutex<TrackingSkyPlotTab<S>>,
    pub solution: Mutex<SolutionTab<S>>,
    pub observation: Mutex<ObservationTab<S>>,
    pub solution_velocity: Mutex<SolutionVelocityTab<S>>,
    pub advanced_spectrum_analyzer: Mutex<AdvancedSpectrumAnalyzerTab<S>>,
    pub status_bar: Mutex<StatusBar<S>>,
    pub update: Mutex<UpdateTab>,
}

impl<S: types::CapnProtoSender> Tabs<S> {
    fn new(
        shared_state: shared_state::SharedState,
        client_sender: S,
        msg_sender: types::MsgSender,
    ) -> Self {
        Self {
            main: MainTab::new(shared_state.clone(), client_sender.clone()).into(),
            advanced_imu: AdvancedImuTab::new(shared_state.clone(), client_sender.clone()).into(),
            advanced_magnetometer: AdvancedMagnetometerTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
            advanced_networking: AdvancedNetworkingTab::new(
                shared_state.clone(),
                client_sender.clone(),
                msg_sender.clone(),
            )
            .into(),
            advanced_system_monitor: AdvancedSystemMonitorTab::new(
                shared_state.clone(),
                client_sender.clone(),
                msg_sender.clone(),
            )
            .into(),
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
            status_bar: StatusBar::new(shared_state.clone(), client_sender).into(),
            update: UpdateTab::new(shared_state).into(),
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
