pub mod cli_options;
pub mod console_backend_capnp {
    include!(concat!(env!("OUT_DIR"), "/console_backend_capnp.rs"));
}
pub mod client_sender;
pub mod common_constants;
pub mod connection;
pub mod constants;
pub mod errors;
pub mod fft_monitor;
pub mod fileio;
pub mod formatters;
pub mod fusion_status_flags;
pub mod log_panel;
pub mod output;
pub mod piksi_tools_constants;
pub mod process_messages;
#[cfg(not(test))]
#[cfg(all(not(feature = "benches"), not(feature = "tests"), feature = "pyo3"))]
pub mod server;
pub mod server_recv_thread;
pub mod shared_state;
pub mod status_bar;
pub mod tabs;
pub mod types;
pub mod updater;
pub mod utils;
pub mod watch;

use crate::status_bar::StatusBar;
use std::sync::Mutex;

use crate::tabs::{
    advanced_tab::{
        advanced_imu_tab::AdvancedImuTab, advanced_magnetometer_tab::AdvancedMagnetometerTab,
        advanced_networking_tab::AdvancedNetworkingTab,
        advanced_spectrum_analyzer_tab::AdvancedSpectrumAnalyzerTab,
        advanced_system_monitor_tab::AdvancedSystemMonitorTab,
    },
    baseline_tab::BaselineTab,
    main_tab::MainTab,
    observation_tab::ObservationTab,
    settings_tab::SettingsTab,
    solution_tab::{
        solution_position_tab::SolutionPositionTab, solution_velocity_tab::SolutionVelocityTab,
    },
    tracking_tab::{
        tracking_signals_tab::TrackingSignalsTab, tracking_sky_plot_tab::TrackingSkyPlotTab,
    },
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
    pub solution_position: Mutex<SolutionPositionTab>,
    pub observation: Mutex<ObservationTab>,
    pub solution_velocity: Mutex<SolutionVelocityTab>,
    pub advanced_spectrum_analyzer: Mutex<AdvancedSpectrumAnalyzerTab>,
    pub status_bar: Mutex<StatusBar>,
    pub update: Mutex<UpdateTab>,
    pub settings: Option<SettingsTab>, // settings only enabled on TCP / Serial
}

impl Tabs {
    fn new(
        shared_state: shared_state::SharedState,
        client_sender: client_sender::BoxedClientSender,
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
            )
            .into(),
            baseline: BaselineTab::new(shared_state.clone(), client_sender.clone(), msg_sender)
                .into(),
            tracking_signals: TrackingSignalsTab::new(shared_state.clone(), client_sender.clone())
                .into(),
            tracking_sky_plot: TrackingSkyPlotTab::new(client_sender.clone(), shared_state.clone())
                .into(),
            observation: ObservationTab::new(shared_state.clone(), client_sender.clone()).into(),
            solution_position: SolutionPositionTab::new(
                shared_state.clone(),
                client_sender.clone(),
            )
            .into(),
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
            update: UpdateTab::new(shared_state).into(),
            settings: None,
        }
    }

    fn with_settings(
        shared_state: shared_state::SharedState,
        client_sender: client_sender::BoxedClientSender,
        msg_sender: types::MsgSender,
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

    pub fn msg_to_frame(msg: impl sbp::SbpMessage) -> sbp::Frame {
        let vec = sbp::to_vec(&msg).unwrap();
        let bytes = vec.as_slice();
        let mut msgs = sbp::iter_frames(bytes);
        msgs.next().unwrap().unwrap()
    }
}
