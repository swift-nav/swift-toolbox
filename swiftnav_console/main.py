# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

"""Frontend module for the Swift Console.
"""
import signal
import argparse
from datetime import datetime
import os
import pickle
import platform
import sys
import time

try:
    import sshtunnel  # type: ignore # pylint: disable=unused-import
    from . import ssh_tunnel

    FEATURE_SSHTUNNEL = True
except ImportError:
    FEATURE_SSHTUNNEL = False

from typing import Optional, Tuple

import capnp  # type: ignore

from PySide6.QtWebEngineQuick import QtWebEngineQuick

from PySide6.QtWidgets import QApplication, QSplashScreen  # type: ignore

from PySide6.QtCore import QObject, QUrl, QThread, QTimer, Slot, Signal, Qt, QLocale
from PySide6 import QtCharts  # pylint: disable=unused-import

from PySide6 import QtQml, QtCore

from PySide6.QtGui import QFontDatabase, QIcon, QPixmap

from PySide6.QtQml import QQmlComponent, qmlRegisterType, QQmlDebuggingEnabler

from PySide6.QtQuickControls2 import QQuickStyle

import swiftnav_console.console_resources  # type: ignore # pylint: disable=unused-import

from .constants import ApplicationMetadata, ConnectionState, ConnectionType, Keys, Tabs

from .backend_importer import BackendImporter

from .backend_request_broker import BackendRequestBroker

from .ntrip_status import NtripStatusData

from .log_panel import (
    log_panel_update,
    LogPanelData,
)

from .connection import (
    connection_update,
    ConnectionData,
    ConnectionModel,
)

from .logging_bar import (
    logging_bar_update,
    logging_bar_recording_update,
    LoggingBarData,
    LoggingBarModel,
)

from .advanced_imu_tab import (
    AdvancedImuModel,
    AdvancedImuPoints,
    advanced_imu_tab_update,
)

from .advanced_magnetometer_tab import (
    AdvancedMagnetometerModel,
    AdvancedMagnetometerPoints,
    advanced_magnetometer_tab_update,
)

from .advanced_networking_tab import (
    AdvancedNetworkingModel,
    AdvancedNetworkingData,
    advanced_networking_tab_update,
)

from .advanced_spectrum_analyzer_tab import (
    AdvancedSpectrumAnalyzerModel,
    AdvancedSpectrumAnalyzerPoints,
    advanced_spectrum_analyzer_tab_update,
)

from .advanced_system_monitor_tab import (
    AdvancedSystemMonitorModel,
    AdvancedSystemMonitorData,
    advanced_system_monitor_tab_update,
)

from .fusion_status_flags import (
    FusionStatusFlagsModel,
    FusionStatusFlagsData,
    fusion_status_flags_update,
)

from .baseline_plot import (
    BaselinePlotModel,
    BaselinePlotPoints,
    baseline_plot_update,
)

from .baseline_table import (
    BaselineTableEntries,
    BaselineTableModel,
    baseline_table_update,
)

from .observation_tab import (
    ObservationLocalTableModel,
    ObservationRemoteTableModel,
    observation_update,
    obs_rows_to_dict,
)

from .settings_tab import (
    SettingsTabModel,
    SettingsTabData,
    SettingsTableEntries,
    SettingsTableModel,
    settings_ins_update,
    settings_table_update,
    settings_rows_to_dict,
)

from .solution_position_tab import (
    SolutionPositionModel,
    SolutionPositionPoints,
    solution_position_update,
)

from .solution_table import (
    SolutionTableEntries,
    SolutionTableModel,
    solution_table_update,
)

from .solution_velocity_tab import (
    SolutionVelocityModel,
    SolutionVelocityPoints,
    solution_velocity_update,
)

from .status_bar import (
    status_bar_update,
    StatusBarData,
    StatusBarModel,
)

from .tracking_signals_tab import (
    TrackingSignalsPoints,
    tracking_signals_tab_update,
)

from .tracking_sky_plot_tab import (
    TrackingSkyPlotPoints,
    tracking_sky_plot_update,
)

from .update_tab import (
    update_tab_update,
    UpdateTabData,
    UpdateTabModel,
)

from .file_io import FileIO

CONSOLE_BACKEND_CAPNP_PATH = "console_backend.capnp"

HELP_CLI_ARGS = ["-h", "--help", "help"]

MAIN_INDEX = "MAIN_INDEX"
SUB_INDEX = "SUB_INDEX"

TAB_LAYOUT = {
    Tabs.TRACKING_SIGNALS: {
        MAIN_INDEX: 0,
        SUB_INDEX: 0,
    },
    Tabs.TRACKING_SKYPLOT: {
        MAIN_INDEX: 0,
        SUB_INDEX: 1,
    },
    Tabs.SOLUTION_POSITION: {
        MAIN_INDEX: 1,
        SUB_INDEX: 0,
    },
    Tabs.SOLUTION_VELOCITY: {
        MAIN_INDEX: 1,
        SUB_INDEX: 1,
    },
    Tabs.BASELINE: {
        MAIN_INDEX: 2,
        SUB_INDEX: 0,
    },
    Tabs.OBSERVATIONS: {
        MAIN_INDEX: 3,
        SUB_INDEX: 0,
    },
    Tabs.SETTINGS: {
        MAIN_INDEX: 4,
        SUB_INDEX: 0,
    },
    Tabs.UPDATE: {
        MAIN_INDEX: 5,
        SUB_INDEX: 0,
    },
    Tabs.ADVANCED_SYSTEM_MONITOR: {
        MAIN_INDEX: 6,
        SUB_INDEX: 0,
    },
    Tabs.ADVANCED_IMU: {
        MAIN_INDEX: 6,
        SUB_INDEX: 1,
    },
    Tabs.ADVANCED_MAGNETOMETER: {
        MAIN_INDEX: 6,
        SUB_INDEX: 2,
    },
    Tabs.ADVANCED_NETWORKING: {
        MAIN_INDEX: 6,
        SUB_INDEX: 3,
    },
    Tabs.ADVANCED_SPECTRUM_ANALYZER: {
        MAIN_INDEX: 6,
        SUB_INDEX: 4,
    },
    Tabs.ADVANCED_INS: {
        MAIN_INDEX: 6,
        SUB_INDEX: 5,
    },
}

capnp.remove_import_hook()  # pylint: disable=no-member

MAP_ENABLED = [False]
SolutionMap = QObject


class BackendMessageReceiver(QObject):  # pylint: disable=too-many-instance-attributes
    _request_quit: Signal = Signal()

    def __init__(
        self,
        app,
        backend,
        messages,
        exit_after_timeout: Optional[float] = None,
        record_file: Optional[str] = None,
        record: bool = False,
    ):
        super().__init__()
        self._app = app
        self._backend = backend
        self._messages = messages
        self._thread = QThread()
        self._thread.started.connect(self._handle_started)  # type: ignore  # pylint: disable=no-member
        self._reader = (
            None if record_file is None else open(str(record_file), "rb")  # pylint: disable=consider-using-with
        )
        filename = f"console-capnp-{datetime.now().strftime('%Y%m%d-%H%M%S')}.pickle"
        self._writer = None if not record else open(filename, "ab")  # pylint: disable=consider-using-with
        self._last_msg_receipt_ns = time.perf_counter_ns()
        self.moveToThread(self._thread)
        self.start_time = None
        self.exit_after_timeout = exit_after_timeout
        self._receive_messages = (
            self._receive_messages_prod
            if (exit_after_timeout is None and record_file is None and not record)
            else self._receive_messages_debug
        )
        self._request_quit.connect(self._app.quit, Qt.QueuedConnection)  # type: ignore
        self._app.aboutToQuit.connect(self._thread.exit)

    @Slot()  # type: ignore
    def _handle_started(self):
        QTimer.singleShot(0, self.receive_messages)

    def start(self):
        self.start_time = time.time()
        self._thread.start()

    def join(self):
        self._thread.wait()

    @Slot()  # type: ignore
    def receive_messages(self):
        if not self._receive_messages():
            self._request_quit.emit()
        else:
            QTimer.singleShot(0, self.receive_messages)

    def _receive_messages_debug(self):
        if self.exit_after_timeout is not None and time.time() - self.start_time > self.exit_after_timeout:
            return False
        msg_receipt_time = time.perf_counter_ns()
        if self._reader is None:
            buffer = self._backend.fetch_message()
        else:
            try:
                msg = pickle.load(self._reader)
            except EOFError:
                print("Pickle file read finished, exiting")
                return False
            buffer = msg["data"]
            if buffer is None:
                return False
            diff = max((msg_receipt_time - self._last_msg_receipt_ns), 0)
            if diff < msg["ns"]:
                time.sleep((msg["ns"] - diff) / 1e9)
        if self._writer is not None:
            pickle.dump({"data": buffer, "ns": msg_receipt_time - self._last_msg_receipt_ns}, self._writer)
        self._last_msg_receipt_ns = msg_receipt_time
        return self._process_message_buffer(buffer)

    def _receive_messages_prod(self):
        buffer = self._backend.fetch_message()
        return self._process_message_buffer(buffer)

    def _process_message_buffer(self, buffer):
        if not buffer:
            print("Terminating GUI loop", file=sys.stderr, flush=True)
            return False
        Message = self._messages.Message
        with Message.from_bytes(buffer) as m:
            if m.which == Message.Union.Status:
                app_state = ConnectionState(m.status.text)
                if app_state == ConnectionState.CLOSED:
                    return False
                if app_state == ConnectionState.DISCONNECTED:
                    data = settings_table_update()
                    SettingsTableEntries.post_data_update(data)
                ConnectionData.post_connection_state_update(app_state)
                if MAP_ENABLED[0]:
                    SolutionMap.clear()
            elif m.which == Message.Union.ConnectionNotification:
                data = m.connectionNotification.message
                ConnectionData.post_connection_message_update(data)
            elif m.which == Message.Union.SolutionPositionStatus:
                data = solution_position_update()
                data[Keys.POINTS][:] = m.solutionPositionStatus.data
                data[Keys.CUR_POINTS][:] = m.solutionPositionStatus.curData
                data[Keys.LAT_MAX] = m.solutionPositionStatus.latMax
                data[Keys.LAT_MIN] = m.solutionPositionStatus.latMin
                data[Keys.LON_MAX] = m.solutionPositionStatus.lonMax
                data[Keys.LON_MIN] = m.solutionPositionStatus.lonMin
                data[Keys.AVAILABLE_UNITS][:] = m.solutionPositionStatus.availableUnits
                data[Keys.SOLUTION_LINE] = m.solutionPositionStatus.lineData

                if MAP_ENABLED[0]:
                    SolutionMap.send_pos(m.solutionPositionStatus)
                SolutionPositionPoints.post_data_update(data)
            elif m.which == Message.Union.SolutionProtectionLevel and MAP_ENABLED[0]:
                SolutionMap.send_prot_lvl(m.solutionProtectionLevel)
            elif m.which == Message.Union.SolutionTableStatus:
                data = solution_table_update()
                data[Keys.ENTRIES][:] = [[entry.key, entry.val] for entry in m.solutionTableStatus.data]
                SolutionTableEntries.post_data_update(data)
            elif m.which == Message.Union.SolutionVelocityStatus:
                data = solution_velocity_update()
                data[Keys.COLORS][:] = m.solutionVelocityStatus.colors
                data[Keys.POINTS][:] = m.solutionVelocityStatus.data
                data[Keys.MAX] = m.solutionVelocityStatus.max
                data[Keys.MIN] = m.solutionVelocityStatus.min
                data[Keys.AVAILABLE_UNITS][:] = m.solutionVelocityStatus.availableUnits
                SolutionVelocityPoints.post_data_update(data)
            elif m.which == Message.Union.BaselinePlotStatus:
                data = baseline_plot_update()
                data[Keys.POINTS][:] = m.baselinePlotStatus.data
                data[Keys.CUR_POINTS][:] = m.baselinePlotStatus.curData
                data[Keys.N_MAX] = m.baselinePlotStatus.nMax
                data[Keys.N_MIN] = m.baselinePlotStatus.nMin
                data[Keys.E_MAX] = m.baselinePlotStatus.eMax
                data[Keys.E_MIN] = m.baselinePlotStatus.eMin
                BaselinePlotPoints.post_data_update(data)
            elif m.which == Message.Union.BaselineTableStatus:
                data = baseline_table_update()
                data[Keys.ENTRIES][:] = [[entry.key, entry.val] for entry in m.baselineTableStatus.data]
                BaselineTableEntries.post_data_update(data)
            elif m.which == Message.Union.AdvancedImuStatus:
                advanced_imu_tab = advanced_imu_tab_update()
                advanced_imu_tab[Keys.FIELDS_DATA][:] = m.advancedImuStatus.fieldsData
                advanced_imu_tab[Keys.POINTS][:] = m.advancedImuStatus.data
                AdvancedImuPoints.post_data_update(advanced_imu_tab)
            elif m.which == Message.Union.AdvancedSpectrumAnalyzerStatus:
                data = advanced_spectrum_analyzer_tab_update()
                data[Keys.CHANNEL] = m.advancedSpectrumAnalyzerStatus.channel
                data[Keys.POINTS][:] = m.advancedSpectrumAnalyzerStatus.data
                data[Keys.YMAX] = m.advancedSpectrumAnalyzerStatus.ymax
                data[Keys.YMIN] = m.advancedSpectrumAnalyzerStatus.ymin
                data[Keys.XMAX] = m.advancedSpectrumAnalyzerStatus.xmax
                data[Keys.XMIN] = m.advancedSpectrumAnalyzerStatus.xmin
                AdvancedSpectrumAnalyzerPoints.post_data_update(data)
            elif m.which == Message.Union.AdvancedNetworkingStatus:
                data = advanced_networking_tab_update()
                data[Keys.RUNNING] = m.advancedNetworkingStatus.running
                data[Keys.IP_ADDRESS] = m.advancedNetworkingStatus.ipAddress
                data[Keys.PORT] = m.advancedNetworkingStatus.port
                data[Keys.NETWORK_INFO][:] = [
                    [entry.interfaceName, entry.ipv4Address, entry.running, entry.txUsage, entry.rxUsage]
                    for entry in m.advancedNetworkingStatus.networkInfo
                ]
                AdvancedNetworkingData.post_data_update(data)
            elif m.which == Message.Union.AdvancedSystemMonitorStatus:
                data = advanced_system_monitor_tab_update()
                data[Keys.OBS_LATENCY][:] = [
                    [entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.obsLatency
                ]
                data[Keys.OBS_PERIOD][:] = [[entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.obsPeriod]
                data[Keys.THREADS_TABLE][:] = [
                    [entry.name, f"{entry.cpu:.1f}", entry.stackFree]
                    for entry in m.advancedSystemMonitorStatus.threadsTable
                ]
                data[Keys.ZYNQ_TEMP] = m.advancedSystemMonitorStatus.zynqTemp
                data[Keys.FE_TEMP] = m.advancedSystemMonitorStatus.feTemp
                AdvancedSystemMonitorData.post_data_update(data)
            elif m.which == Message.Union.AdvancedMagnetometerStatus:
                data = advanced_magnetometer_tab_update()
                data[Keys.YMAX] = m.advancedMagnetometerStatus.ymax
                data[Keys.YMIN] = m.advancedMagnetometerStatus.ymin
                data[Keys.POINTS][:] = m.advancedMagnetometerStatus.data
                AdvancedMagnetometerPoints.post_data_update(data)
            elif m.which == Message.Union.FusionStatusFlagsStatus:
                data = fusion_status_flags_update()
                data[Keys.GNSSPOS] = m.fusionStatusFlagsStatus.gnsspos
                data[Keys.GNSSVEL] = m.fusionStatusFlagsStatus.gnssvel
                data[Keys.WHEELTICKS] = m.fusionStatusFlagsStatus.wheelticks
                data[Keys.SPEED] = m.fusionStatusFlagsStatus.speed
                data[Keys.NHC] = m.fusionStatusFlagsStatus.nhc
                data[Keys.ZEROVEL] = m.fusionStatusFlagsStatus.zerovel
                FusionStatusFlagsData.post_data_update(data)
            elif m.which == Message.Union.TrackingSignalsStatus:
                data = tracking_signals_tab_update()
                data[Keys.CHECK_LABELS][:] = m.trackingSignalsStatus.checkLabels
                data[Keys.LABELS][:] = m.trackingSignalsStatus.labels
                data[Keys.COLORS][:] = m.trackingSignalsStatus.colors
                data[Keys.POINTS][:] = m.trackingSignalsStatus.data
                data[Keys.XMIN_OFFSET] = m.trackingSignalsStatus.xminOffset
                TrackingSignalsPoints.post_data_update(data)
            elif m.which == Message.Union.TrackingSkyPlotStatus:
                data = tracking_sky_plot_update()
                data[Keys.SATS][:] = m.trackingSkyPlotStatus.sats
                data[Keys.LABELS][:] = [
                    list(m.trackingSkyPlotStatus.labels[idx]) for idx in range(len(m.trackingSkyPlotStatus.labels))
                ]
                TrackingSkyPlotPoints.post_data_update(data)
            elif m.which == Message.Union.ObservationStatus:
                data = observation_update()
                data[Keys.TOW] = m.observationStatus.tow
                data[Keys.WEEK] = m.observationStatus.week
                data[Keys.ROWS][:] = obs_rows_to_dict(m.observationStatus.rows)
                if m.observationStatus.isRemote:
                    ObservationRemoteTableModel.post_data_update(data)
                else:
                    ObservationLocalTableModel.post_data_update(data)
            elif m.which == Message.Union.StatusBarStatus:
                data = status_bar_update()
                data[Keys.POS] = m.statusBarStatus.pos
                data[Keys.RTK] = m.statusBarStatus.rtk
                data[Keys.SATS] = m.statusBarStatus.sats
                data[Keys.CORR_AGE] = m.statusBarStatus.corrAge
                data[Keys.INS] = m.statusBarStatus.ins
                data[Keys.DATA_RATE] = m.statusBarStatus.dataRate
                data[Keys.SOLID_CONNECTION] = m.statusBarStatus.solidConnection
                data[Keys.TITLE] = m.statusBarStatus.title
                data[Keys.ANTENNA_STATUS] = m.statusBarStatus.antennaStatus
                up = m.statusBarStatus.ntripUpload
                down = m.statusBarStatus.ntripDownload
                down_units = "B/s"
                if down >= 1000:
                    down /= 1000
                    down = round(down, 1)
                    down_units = "KB/s"
                connected = m.statusBarStatus.ntripConnected
                if connected:
                    data[Keys.NTRIP_DISPLAY] = f"{up}B/s ⬆ {down}{down_units} ⬇"
                    NtripStatusData.post_connected(True)
                else:
                    data[Keys.NTRIP_DISPLAY] = ""
                    NtripStatusData.post_connected(False)
                StatusBarData.post_data_update(data)
            elif m.which == Message.Union.ConnectionStatus:
                data = connection_update()
                data[Keys.AVAILABLE_PORTS][:] = m.connectionStatus.availablePorts
                data[Keys.AVAILABLE_BAUDRATES][:] = m.connectionStatus.availableBaudrates
                data[Keys.AVAILABLE_FLOWS][:] = m.connectionStatus.availableFlows
                data[Keys.PREVIOUS_HOSTS][:] = m.connectionStatus.previousHosts
                data[Keys.PREVIOUS_PORTS][:] = m.connectionStatus.previousPorts
                data[Keys.PREVIOUS_FILES][:] = m.connectionStatus.previousFiles
                data[Keys.LAST_USED_SERIAL_DEVICE] = (
                    m.connectionStatus.lastSerialDevice.port
                    if m.connectionStatus.lastSerialDevice.which() == "port"
                    else None
                )
                data[Keys.PREVIOUS_SERIAL_CONFIGS][:] = [
                    [entry.device, entry.baudrate, entry.flowControl]
                    for entry in m.connectionStatus.previousSerialConfigs
                ]
                data[Keys.CONSOLE_VERSION] = m.connectionStatus.consoleVersion
                data[Keys.PREVIOUS_CONNECTION_TYPE] = ConnectionType(m.connectionStatus.previousConnectionType)
                ConnectionData.post_connection_data_update(data)
            elif m.which == Message.Union.LoggingBarStatus:
                data = logging_bar_update()
                data[Keys.PREVIOUS_FOLDERS][:] = m.loggingBarStatus.previousFolders
                data[Keys.CSV_LOGGING] = m.loggingBarStatus.csvLogging
                data[Keys.SBP_LOGGING] = m.loggingBarStatus.sbpLogging
                data[Keys.SBP_LOGGING_FORMAT] = m.loggingBarStatus.sbpLoggingFormat
                LoggingBarData.post_data_update(data)
            elif m.which == Message.Union.LoggingBarStartRecording:
                data = logging_bar_recording_update()
                data[Keys.RECORDING_SIZE] = None  # reset since name changed => new file
                data[Keys.RECORDING_START_TIME] = time.time()
                data[Keys.RECORDING_FILENAME] = m.loggingBarStartRecording.name
                LoggingBarData.post_recording_data_update(data)
            elif m.which == Message.Union.LoggingBarRecordingSize:
                data = logging_bar_recording_update()
                data[Keys.RECORDING_SIZE] = m.loggingBarRecordingSize.size
                LoggingBarData.post_recording_data_update(data)
            elif m.which == Message.Union.UpdateTabStatus:
                data = update_tab_update()
                data[Keys.HARDWARE_REVISION] = m.updateTabStatus.hardwareRevision
                data[Keys.FW_VERSION_CURRENT] = m.updateTabStatus.fwVersionCurrent
                data[Keys.FW_VERSION_LATEST] = m.updateTabStatus.fwVersionLatest
                data[Keys.FW_LOCAL_FILENAME] = m.updateTabStatus.fwLocalFilename
                data[Keys.DIRECTORY] = m.updateTabStatus.directory
                data[Keys.DOWNLOADING] = m.updateTabStatus.downloading
                data[Keys.UPGRADING] = m.updateTabStatus.upgrading
                data[Keys.FW_TEXT] = m.updateTabStatus.fwText
                data[Keys.FILEIO_LOCAL_FILEPATH] = m.updateTabStatus.fileioLocalFilepath
                data[Keys.FILEIO_DESTINATION_FILEPATH] = m.updateTabStatus.fileioDestinationFilepath
                data[Keys.FW_OUTDATED] = m.updateTabStatus.fwOutdated
                data[Keys.FW_V2_OUTDATED] = m.updateTabStatus.fwV2Outdated
                data[Keys.SERIAL_PROMPT] = m.updateTabStatus.serialPrompt
                data[Keys.CONSOLE_OUTDATED] = m.updateTabStatus.consoleOutdated
                data[Keys.CONSOLE_VERSION_CURRENT] = m.updateTabStatus.consoleVersionCurrent
                data[Keys.CONSOLE_VERSION_LATEST] = m.updateTabStatus.consoleVersionLatest
                UpdateTabData.post_data_update(data)
            elif m.which == Message.Union.LogAppend:
                data = log_panel_update()
                data[Keys.ENTRIES] += [entry.line for entry in m.logAppend.entries]
                data[Keys.LOG_LEVEL] = m.logAppend.logLevel
                LogPanelData.post_data_update(data)
            elif m.which == Message.Union.SettingsTableStatus:
                data = settings_table_update()
                data[Keys.ENTRIES][:] = settings_rows_to_dict(m.settingsTableStatus.data)
                SettingsTableEntries.post_data_update(data)
            elif m.which == Message.Union.SettingsImportResponse:
                SettingsTabData.post_import_status_update(m.settingsImportResponse.status)
            elif m.which == Message.Union.SettingsNotification:
                SettingsTabData.post_notification_update(m.settingsNotification.message)
            elif m.which == Message.Union.InsSettingsChangeResponse:
                data = settings_ins_update()
                data[Keys.RECOMMENDED_INS_SETTINGS][:] = [
                    [entry.settingName, entry.currentValue, entry.recommendedValue]
                    for entry in m.insSettingsChangeResponse.recommendedSettings
                ]
                data[Keys.NEW_INS_CONFIRMATON] = True
                SettingsTabData.post_ins_update(data)
        return True


def is_frozen() -> bool:
    """Check whether the application is frozen.

    Returns:
        bool: Whether the application is frozen.
    """
    me = get_app_dir()
    var_frozen = os.environ.get("SWIFTNAV_CONSOLE_FROZEN", "") != ""
    path_frozen = os.path.exists(os.path.join(me, ".frozen"))
    return var_frozen or path_frozen


def get_app_dir(alt: bool = False) -> str:
    """Fetches the application resources directory.

    Args:
        alt (bool): fetch an alternate data directory (only valid on macOS)

    Returns:
        str: path to the resoure dir (accounting for OS differences)
    """
    var_frozen = os.environ.get("SWIFTNAV_CONSOLE_FROZEN", "")
    if var_frozen != "":
        if platform.system() == "Darwin":
            if alt:
                return var_frozen
            return os.path.join(var_frozen, "../Resources")
        return var_frozen
    return os.path.dirname(sys.executable)


def get_capnp_path() -> str:
    """Get the path to the capnp file based on current installer.

    Returns:
        str: The path to the capnp file.
    """
    path = ""
    if is_frozen():
        path = os.path.join(get_app_dir(), "resources/base", CONSOLE_BACKEND_CAPNP_PATH)
        if not os.path.exists(path):
            path = os.path.join(get_app_dir(alt=True), "resources/base", CONSOLE_BACKEND_CAPNP_PATH)
    else:
        path = os.path.join(
            os.path.dirname(os.path.dirname(__file__)), "src/main/resources/base", CONSOLE_BACKEND_CAPNP_PATH
        )
    return path


def handle_cli_arguments(args: argparse.Namespace, globals_: QObject):
    if args.show_fileio:
        globals_.setProperty("showFileio", True)  # type: ignore
    if args.use_opengl:
        os.environ["QSG_RHI"] = "1"
        os.environ["QSG_RHI_BACKEND"] = "opengl"
        globals_.setProperty("useOpenGL", True)  # type: ignore
    if args.no_antialiasing:
        globals_.setProperty("useAntiAliasing", False)  # type: ignore
    if args.no_prompts:
        globals_.setProperty("showPrompts", False)  # type: ignore
    if args.tab is not None:
        layout_idxs = TAB_LAYOUT[args.tab]
        globals_.setProperty("initialMainTabIndex", layout_idxs[MAIN_INDEX])  # type: ignore
        globals_.setProperty("initialSubTabIndex", layout_idxs[SUB_INDEX])  # type: ignore
    if args.show_csv_log:
        globals_.setProperty("showCsvLog", True)  # type: ignore
    if args.height:
        min_height = globals_.property("minimumHeight")  # type: ignore
        if args.height < min_height:
            print(
                f"WARNING: --height value: {args.height}, is less than minimum: {min_height}. Input will be ignored.",
                file=sys.stderr,
            )
        else:
            globals_.setProperty("height", args.height)  # type: ignore
    if args.width:
        min_width = globals_.property("minimumWidth")  # type: ignore
        if args.width < min_width:
            print(
                f"WARNING: --width value: {args.width}, is less than minimum: {min_width}. Input will be ignored.",
                file=sys.stderr,
            )
        else:
            globals_.setProperty("width", args.width)  # type: ignore
    if args.show_file_connection:
        globals_.setProperty("showFileConnection", True)  # type: ignore
    if args.enable_map:
        globals_.setProperty("enableMap", True)  # type: ignore
        MAP_ENABLED[0] = True
    if args.enable_ntrip:
        globals_.setProperty("enableNtrip", True)  # type: ignore
    try:
        if args.ssh_tunnel:
            ssh_tunnel.setup(args.ssh_tunnel, args.ssh_remote_bind_address)
    except AttributeError:
        pass


def start_splash_linux():
    splash_filename = os.getenv("SWIFTNAV_CONSOLE_SPLASH")
    if not splash_filename:
        return
    try:
        with open(splash_filename, "wb"):
            pass
    except FileNotFoundError:
        pass


def start_splash() -> Optional[QSplashScreen]:
    if platform.system() == "Linux":
        start_splash_linux()
        return None
    pixmap = QPixmap(":/images/splash-version.jpg")
    splash = QSplashScreen(pixmap)
    splash.show()
    return splash


def stop_splash_linux():
    splash_filename = os.getenv("SWIFTNAV_CONSOLE_SPLASH")
    if not splash_filename:
        return
    try:
        os.remove(splash_filename)
        time.sleep(0.200)
    except FileNotFoundError:
        pass


def stop_splash(splash: Optional[QSplashScreen]):
    if platform.system() == "Linux":
        stop_splash_linux()
    else:
        assert splash is not None
        splash.deleteLater()
        splash.close()


def main(passed_args: Optional[Tuple[str, ...]] = None) -> int:
    # Redirect SIGINT to SIG_DFL, so Qt handles Sigint and terminates properly.
    signal.signal(signal.SIGINT, signal.SIG_DFL)

    parser = argparse.ArgumentParser(add_help=False, usage=argparse.SUPPRESS)
    parser.add_argument("--exit-after-timeout", type=int, default=None)
    parser.add_argument("--read-capnp-recording", type=str, default=None)
    parser.add_argument("--record-capnp-recording", action="store_true")
    parser.add_argument("--debug-with-no-backend", action="store_true")
    parser.add_argument("--show-fileio", action="store_true")
    parser.add_argument("--enable-map", action="store_true")
    parser.add_argument("--show-file-connection", action="store_true")
    parser.add_argument("--no-prompts", action="store_true")
    parser.add_argument("--use-opengl", action="store_true")
    parser.add_argument("--no-high-dpi", action="store_true")
    parser.add_argument("--no-antialiasing", action="store_true")
    parser.add_argument("--tab")
    parser.add_argument("--show-csv-log", action="store_true")
    parser.add_argument("--height", type=int)
    parser.add_argument("--width", type=int)
    parser.add_argument("--enable-ntrip", action="store_true")
    parser.add_argument("--qmldebug", action="store_true")
    if FEATURE_SSHTUNNEL:
        parser.add_argument("--ssh-tunnel", type=str, default=None)
        parser.add_argument("--ssh-remote-bind-address", type=str, default=None)

    args_main, unknown_args = parser.parse_known_args()
    for unknown_arg in unknown_args:
        for tunnel_arg in ("--ssh-tunnel", "--ssh-remote-bind-address"):
            if tunnel_arg in unknown_arg:
                parser.error(
                    f"Option {tunnel_arg} unsupported.\n"
                    "The --ssh-tunnel and --ssh-remote-bind-address "
                    "arguments require the `sshtunnel` python module."
                )

    if args_main.debug_with_no_backend and args_main.read_capnp_recording is None:
        parser.error("The --debug-with-no-backend argument requires the --read-capnp-recording argument.")

    if FEATURE_SSHTUNNEL:
        ssh_tunnel.validate(args_main, parser)

    found_help_arg = False
    for arg in unknown_args:
        if arg in HELP_CLI_ARGS:
            found_help_arg = True
    if passed_args is not None:
        for arg in passed_args:
            if arg in HELP_CLI_ARGS:
                found_help_arg = True
        args_main, _ = parser.parse_known_args(passed_args)
    if args_main.no_high_dpi:
        QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_Use96Dpi)  # type: ignore
    if args_main.qmldebug:
        sys.argv.append("-qmljsdebugger=port:10002,block")
        debug = QQmlDebuggingEnabler()  # pylint: disable=unused-variable

    QLocale.setDefault(QLocale.c())
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.ApplicationAttribute.AA_ShareOpenGLContexts)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.ApplicationAttribute.AA_UseDesktopOpenGL)
    QtWebEngineQuick.initialize()
    app = QApplication(sys.argv)
    app.setWindowIcon(QIcon(":/images/icon.ico"))
    app.setOrganizationName(ApplicationMetadata.ORGANIZATION_NAME)
    app.setOrganizationDomain(ApplicationMetadata.ORGANIZATION_DOMAIN)
    app.setApplicationName(ApplicationMetadata.APPLICATION_NAME)
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Regular.ttf")
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Bold.ttf")
    QFontDatabase.addApplicationFont(":/fonts/RobotoCondensed-Regular.ttf")
    QQuickStyle.setStyle("Material")
    # We specifically *don't* want the RobotoCondensed-Bold.ttf font so we get the right look when bolded.

    if MAP_ENABLED[0]:
        global SolutionMap  # pylint: disable=global-statement
        from .solution_map import SolutionMap as SolutionMap_  # pylint: disable=import-outside-toplevel

        SolutionMap = SolutionMap_  # type: ignore

    qmlRegisterType(ConnectionData, "SwiftConsole", 1, 0, "ConnectionData")  # type: ignore
    qmlRegisterType(AdvancedImuPoints, "SwiftConsole", 1, 0, "AdvancedImuPoints")  # type: ignore
    qmlRegisterType(AdvancedMagnetometerPoints, "SwiftConsole", 1, 0, "AdvancedMagnetometerPoints")  # type: ignore
    qmlRegisterType(AdvancedNetworkingData, "SwiftConsole", 1, 0, "AdvancedNetworkingData")  # type: ignore
    qmlRegisterType(
        AdvancedSpectrumAnalyzerPoints, "SwiftConsole", 1, 0, "AdvancedSpectrumAnalyzerPoints"  # type: ignore
    )
    qmlRegisterType(AdvancedSystemMonitorData, "SwiftConsole", 1, 0, "AdvancedSystemMonitorData")  # type: ignore
    qmlRegisterType(FusionStatusFlagsData, "SwiftConsole", 1, 0, "FusionStatusFlagsData")  # type: ignore
    qmlRegisterType(BaselinePlotPoints, "SwiftConsole", 1, 0, "BaselinePlotPoints")  # type: ignore
    qmlRegisterType(BaselineTableEntries, "SwiftConsole", 1, 0, "BaselineTableEntries")  # type: ignore
    qmlRegisterType(SettingsTabData, "SwiftConsole", 1, 0, "SettingsTabData")  # type: ignore
    qmlRegisterType(SettingsTableEntries, "SwiftConsole", 1, 0, "SettingsTableEntries")  # type: ignore
    qmlRegisterType(SolutionMap, "SwiftConsole", 1, 0, "SolutionMap")  # type: ignore
    qmlRegisterType(SolutionPositionPoints, "SwiftConsole", 1, 0, "SolutionPositionPoints")  # type: ignore
    qmlRegisterType(SolutionTableEntries, "SwiftConsole", 1, 0, "SolutionTableEntries")  # type: ignore
    qmlRegisterType(SolutionVelocityPoints, "SwiftConsole", 1, 0, "SolutionVelocityPoints")  # type: ignore
    qmlRegisterType(StatusBarData, "SwiftConsole", 1, 0, "StatusBarData")  # type: ignore
    qmlRegisterType(NtripStatusData, "SwiftConsole", 1, 0, "NtripStatusData")  # type: ignore
    qmlRegisterType(TrackingSignalsPoints, "SwiftConsole", 1, 0, "TrackingSignalsPoints")  # type: ignore
    qmlRegisterType(TrackingSkyPlotPoints, "SwiftConsole", 1, 0, "TrackingSkyPlotPoints")  # type: ignore
    qmlRegisterType(ObservationRemoteTableModel, "SwiftConsole", 1, 0, "ObservationRemoteTableModel")  # type: ignore
    qmlRegisterType(ObservationLocalTableModel, "SwiftConsole", 1, 0, "ObservationLocalTableModel")  # type: ignore
    qmlRegisterType(UpdateTabData, "SwiftConsole", 1, 0, "UpdateTabData")  # type: ignore
    qmlRegisterType(FileIO, "SwiftConsole", 1, 0, "FileIO")  # type: ignore

    engine = QtQml.QQmlApplicationEngine()
    qml_object_created = [False]

    def handle_qml_load_errors(obj, _url):
        qml_object_created[0] = obj is not None

    engine.objectCreated.connect(handle_qml_load_errors)  # type: ignore  # pylint: disable=no-member

    capnp_path = get_capnp_path()
    backend_main = BackendImporter(use_fake=args_main.debug_with_no_backend).Server()
    endpoint_main = backend_main.start()

    if found_help_arg:
        return 0

    # Unfortunately it is not possible to access singletons directly using the PySide6 API.
    # This approach stores the globals somwhere that can be grabbed and manipulated.
    component = QQmlComponent(engine)
    component.setData(
        b'import QtQuick 2.0\nimport "Constants"\nItem{ property var globals: Globals }',  # type: ignore
        QUrl("qrc:/grabGlobals.qml"),
    )
    globals_main = component.create()
    globals_main = globals_main.property("globals")  # type: ignore

    handle_cli_arguments(args_main, globals_main)
    splash = start_splash()

    engine.addImportPath(":/styles")
    engine.load(QUrl("qrc:/view.qml"))
    if not qml_object_created[0]:
        return 1
    messages_main = capnp.load(capnp_path)  # pylint: disable=no-member
    backend_request_broker = BackendRequestBroker(endpoint_main, messages_main)
    connection_model = ConnectionModel()
    advanced_imu_model = AdvancedImuModel()
    advanced_magnetometer_model = AdvancedMagnetometerModel()
    advanced_networking_model = AdvancedNetworkingModel()
    advanced_spectrum_analyzer_model = AdvancedSpectrumAnalyzerModel()
    advanced_system_monitor_model = AdvancedSystemMonitorModel()
    fusion_engine_flags_model = FusionStatusFlagsModel()
    baseline_plot_model = BaselinePlotModel()
    baseline_table_model = BaselineTableModel()
    settings_tab_model = SettingsTabModel()
    settings_table_model = SettingsTableModel()
    solution_position_model = SolutionPositionModel()
    solution_table_model = SolutionTableModel()
    solution_velocity_model = SolutionVelocityModel()
    status_bar_model = StatusBarModel()
    logging_bar_model = LoggingBarModel()
    update_tab_model = UpdateTabModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("connection_model", connection_model)
    root_context.setContextProperty("advanced_imu_model", advanced_imu_model)
    root_context.setContextProperty("advanced_magnetometer_model", advanced_magnetometer_model)
    root_context.setContextProperty("advanced_networking_model", advanced_networking_model)
    root_context.setContextProperty("advanced_spectrum_analyzer_model", advanced_spectrum_analyzer_model)
    root_context.setContextProperty("advanced_system_monitor_model", advanced_system_monitor_model)
    root_context.setContextProperty("fusion_engine_flags_model", fusion_engine_flags_model)
    root_context.setContextProperty("baseline_plot_model", baseline_plot_model)
    root_context.setContextProperty("baseline_table_model", baseline_table_model)
    root_context.setContextProperty("settings_tab_model", settings_tab_model)
    root_context.setContextProperty("settings_table_model", settings_table_model)
    root_context.setContextProperty("solution_position_model", solution_position_model)
    root_context.setContextProperty("solution_table_model", solution_table_model)
    root_context.setContextProperty("solution_velocity_model", solution_velocity_model)
    root_context.setContextProperty("status_bar_model", status_bar_model)
    root_context.setContextProperty("logging_bar_model", logging_bar_model)
    root_context.setContextProperty("update_tab_model", update_tab_model)
    root_context.setContextProperty("backend_request_broker", backend_request_broker)

    backend_msg_receiver = BackendMessageReceiver(
        app,
        backend_main,
        messages_main,
        exit_after_timeout=args_main.exit_after_timeout,
        record_file=args_main.read_capnp_recording,
        record=args_main.record_capnp_recording,
    )
    backend_msg_receiver.start()

    stop_splash(splash)
    app.exec()

    endpoint_main.shutdown()
    backend_msg_receiver.join()
    try:
        # Stop the sshtunnel server if there is one.
        sshtunnel_server.stop()  # type: ignore
    except NameError:
        pass

    return 0


if __name__ == "__main__":
    exit_code = main()
    sys.exit(exit_code)
