"""Frontend module for the Swift Console.
"""
import argparse
import os
import sys

from typing import Optional, Tuple

import capnp  # type: ignore

from PySide2.QtWidgets import QApplication  # type: ignore

from PySide2.QtCore import QObject, QUrl, QPointF, QThread, QTimer, Slot
from PySide2.QtCharts import QtCharts  # pylint: disable=unused-import

from PySide2 import QtQml, QtCore

from PySide2.QtGui import QFontDatabase, QIcon

from PySide2.QtQml import QQmlComponent, qmlRegisterType

import swiftnav_console.console_resources  # type: ignore # pylint: disable=unused-import

import console_backend.server  # type: ignore  # pylint: disable=import-error,no-name-in-module

from .constants import ApplicationMetadata, ConnectionState, ConnectionType, Keys, Tabs

from .data_model import DataModel

from .log_panel import (
    log_panel_update,
    log_panel_lock,
    LogPanelData,
    LogPanelModel,
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
    obs_rows_to_json,
)

from .settings_tab import (
    SettingsTabModel,
    SettingsTabData,
    SettingsTableEntries,
    SettingsTableModel,
    settings_ins_update,
    settings_table_update,
    settings_rows_to_json,
)

from .solution_position_tab import (
    SolutionPositionModel,
    SolutionPositionPoints,
    solution_position_update,
)

from .solution_table import (
    SolutionTableEntries,
    SolutionTableModel,
    SOLUTION_TABLE,
)

from .solution_velocity_tab import (
    SolutionVelocityModel,
    SolutionVelocityPoints,
    SOLUTION_VELOCITY_TAB,
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
    TRACKING_SKY_PLOT_TAB,
)

from .update_tab import (
    UPDATE_TAB,
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


class BackendMessageReceiver(QObject):
    def __init__(self, app, backend, messages):
        super().__init__()
        self._app = app
        self._backend = backend
        self._messages = messages
        self._thread = QThread()
        self._thread.started.connect(self._handle_started)  # pylint: disable=no-member
        self.moveToThread(self._thread)

    def _handle_started(self):
        QTimer.singleShot(0, self.receive_messages)

    def start(self):
        self._thread.start()

    def join(self):
        self._thread.wait()

    @Slot()  # type: ignore
    def receive_messages(self):
        if not self._receive_messages():
            self._thread.exit()
        else:
            QTimer.singleShot(0, self.receive_messages)

    def _receive_messages(self):
        buffer = self._backend.fetch_message()
        if not buffer:
            print("terminating GUI loop", file=sys.stderr)
            return False
        Message = self._messages.Message
        m = Message.from_bytes(buffer)
        if m.which == Message.Union.Status:
            app_state = ConnectionState(m.status.text)
            if app_state == ConnectionState.CLOSED:
                self._app.quit()
                return True
            if app_state == ConnectionState.DISCONNECTED:
                data = settings_table_update()
                SettingsTableEntries.post_data_update(data)
            ConnectionData.post_connection_state_update(app_state)
        elif m.which == Message.Union.ConnectionNotification:
            data = m.connectionNotification.message
            ConnectionData.post_connection_message_update(data)
        elif m.which == Message.Union.SolutionPositionStatus:
            data = solution_position_update()
            data[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionPositionStatus.data[idx]]
                for idx in range(len(m.solutionPositionStatus.data))
            ]
            data[Keys.CUR_POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionPositionStatus.curData[idx]]
                for idx in range(len(m.solutionPositionStatus.curData))
            ]
            data[Keys.LAT_MAX] = m.solutionPositionStatus.latMax
            data[Keys.LAT_MIN] = m.solutionPositionStatus.latMin
            data[Keys.LON_MAX] = m.solutionPositionStatus.lonMax
            data[Keys.LON_MIN] = m.solutionPositionStatus.lonMin
            data[Keys.AVAILABLE_UNITS][:] = m.solutionPositionStatus.availableUnits
            SolutionPositionPoints.post_data_update(data)
        elif m.which == Message.Union.SolutionTableStatus:
            SOLUTION_TABLE[Keys.ENTRIES][:] = [[entry.key, entry.val] for entry in m.solutionTableStatus.data]
        elif m.which == Message.Union.SolutionVelocityStatus:
            SOLUTION_VELOCITY_TAB[Keys.COLORS][:] = m.solutionVelocityStatus.colors
            SOLUTION_VELOCITY_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionVelocityStatus.data[idx]]
                for idx in range(len(m.solutionVelocityStatus.data))
            ]
            SOLUTION_VELOCITY_TAB[Keys.MAX] = m.solutionVelocityStatus.max
            SOLUTION_VELOCITY_TAB[Keys.MIN] = m.solutionVelocityStatus.min
            SOLUTION_VELOCITY_TAB[Keys.AVAILABLE_UNITS][:] = m.solutionVelocityStatus.availableUnits
        elif m.which == Message.Union.BaselinePlotStatus:
            data = baseline_plot_update()
            data[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.baselinePlotStatus.data[idx]]
                for idx in range(len(m.baselinePlotStatus.data))
            ]
            data[Keys.CUR_POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.baselinePlotStatus.curData[idx]]
                for idx in range(len(m.baselinePlotStatus.curData))
            ]
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
            advanced_imu_tab[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.advancedImuStatus.data[idx]]
                for idx in range(len(m.advancedImuStatus.data))
            ]
            AdvancedImuPoints.post_data_update(advanced_imu_tab)
        elif m.which == Message.Union.AdvancedSpectrumAnalyzerStatus:
            data = advanced_spectrum_analyzer_tab_update()
            data[Keys.CHANNEL] = m.advancedSpectrumAnalyzerStatus.channel
            data[Keys.POINTS][:] = [QPointF(point.x, point.y) for point in m.advancedSpectrumAnalyzerStatus.data]
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
            data[Keys.OBS_LATENCY][:] = [[entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.obsLatency]
            data[Keys.OBS_PERIOD][:] = [[entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.obsPeriod]
            data[Keys.THREADS_TABLE][:] = [
                [entry.name, entry.cpu, entry.stackFree] for entry in m.advancedSystemMonitorStatus.threadsTable
            ]
            data[Keys.CSAC_TELEM_LIST][:] = [
                [entry.key, entry.val] for entry in m.advancedSystemMonitorStatus.csacTelemList
            ]
            data[Keys.CSAC_RECEIVED] = m.advancedSystemMonitorStatus.csacReceived
            data[Keys.ZYNQ_TEMP] = m.advancedSystemMonitorStatus.zynqTemp
            data[Keys.FE_TEMP] = m.advancedSystemMonitorStatus.feTemp
            AdvancedSystemMonitorData.post_data_update(data)
        elif m.which == Message.Union.AdvancedMagnetometerStatus:
            data = advanced_magnetometer_tab_update()
            data[Keys.YMAX] = m.advancedMagnetometerStatus.ymax
            data[Keys.YMIN] = m.advancedMagnetometerStatus.ymin
            data[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.advancedMagnetometerStatus.data[idx]]
                for idx in range(len(m.advancedMagnetometerStatus.data))
            ]
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
            data[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.trackingSignalsStatus.data[idx]]
                for idx in range(len(m.trackingSignalsStatus.data))
            ]
            data[Keys.XMIN_OFFSET] = m.trackingSignalsStatus.xminOffset
            TrackingSignalsPoints.post_data_update(data)
        elif m.which == Message.Union.TrackingSkyPlotStatus:
            TRACKING_SKY_PLOT_TAB[Keys.SATS][:] = [
                [QPointF(point.az, point.el) for point in m.trackingSkyPlotStatus.sats[idx]]
                for idx in range(len(m.trackingSkyPlotStatus.sats))
            ]
            TRACKING_SKY_PLOT_TAB[Keys.LABELS][:] = [
                list(m.trackingSkyPlotStatus.labels[idx]) for idx in range(len(m.trackingSkyPlotStatus.labels))
            ]
        elif m.which == Message.Union.ObservationStatus:
            data = observation_update()
            data[Keys.TOW] = m.observationStatus.tow
            data[Keys.WEEK] = m.observationStatus.week
            data[Keys.ROWS][:] = obs_rows_to_json(m.observationStatus.rows)
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
                [entry.device, entry.baudrate, entry.flowControl] for entry in m.connectionStatus.previousSerialConfigs
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
        elif m.which == Message.Union.LoggingBarRecordingStatus:
            data = logging_bar_recording_update()
            data[Keys.RECORDING_DURATION_SEC] = m.loggingBarRecordingStatus.recordingDurationSec
            data[Keys.RECORDING_SIZE] = m.loggingBarRecordingStatus.recordingSize
            data[Keys.RECORDING_FILENAME] = (
                m.loggingBarRecordingStatus.recordingFilename.filename
                if m.loggingBarRecordingStatus.recordingFilename.which() == "filename"
                else ""
            )
            LoggingBarData.post_recording_data_update(data)
        elif m.which == Message.Union.UpdateTabStatus:
            UPDATE_TAB[Keys.HARDWARE_REVISION] = m.updateTabStatus.hardwareRevision
            UPDATE_TAB[Keys.FW_VERSION_CURRENT] = m.updateTabStatus.fwVersionCurrent
            UPDATE_TAB[Keys.FW_VERSION_LATEST] = m.updateTabStatus.fwVersionLatest
            UPDATE_TAB[Keys.FW_LOCAL_FILENAME] = m.updateTabStatus.fwLocalFilename
            UPDATE_TAB[Keys.DIRECTORY] = m.updateTabStatus.directory
            UPDATE_TAB[Keys.DOWNLOADING] = m.updateTabStatus.downloading
            UPDATE_TAB[Keys.UPGRADING] = m.updateTabStatus.upgrading
            UPDATE_TAB[Keys.FW_TEXT] = m.updateTabStatus.fwText
            UPDATE_TAB[Keys.FILEIO_LOCAL_FILEPATH] = m.updateTabStatus.fileioLocalFilepath
            UPDATE_TAB[Keys.FILEIO_DESTINATION_FILEPATH] = m.updateTabStatus.fileioDestinationFilepath
            UPDATE_TAB[Keys.FW_OUTDATED] = m.updateTabStatus.fwOutdated
            UPDATE_TAB[Keys.FW_V2_OUTDATED] = m.updateTabStatus.fwV2Outdated
            UPDATE_TAB[Keys.SERIAL_PROMPT] = m.updateTabStatus.serialPrompt
            UPDATE_TAB[Keys.CONSOLE_OUTDATED] = m.updateTabStatus.consoleOutdated
            UPDATE_TAB[Keys.CONSOLE_VERSION_CURRENT] = m.updateTabStatus.consoleVersionCurrent
            UPDATE_TAB[Keys.CONSOLE_VERSION_LATEST] = m.updateTabStatus.consoleVersionLatest
        elif m.which == Message.Union.LogAppend:
            log_panel_lock.lock()
            data = log_panel_update()
            data[Keys.ENTRIES] += [entry.line for entry in m.logAppend.entries]
            log_panel_lock.unlock()
            data[Keys.LOG_LEVEL] = m.logAppend.logLevel
            LogPanelData.post_data_update(data)
        elif m.which == Message.Union.SettingsTableStatus:
            data = settings_table_update()
            data[Keys.ENTRIES][:] = settings_rows_to_json(m.settingsTableStatus.data)
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
    me = os.path.dirname(sys.executable)
    var_frozen = os.environ.get("SWIFTNAV_CONSOLE_FROZEN", "") != ""
    path_frozen = os.path.exists(os.path.join(me, ".frozen"))
    return var_frozen or path_frozen


def get_app_dir() -> str:
    var_frozen = os.environ.get("SWIFTNAV_CONSOLE_FROZEN", "")
    if var_frozen != "":
        return var_frozen
    return os.path.dirname(sys.executable)


def get_capnp_path() -> str:
    """Get the path to the capnp file based on current installer.

    Returns:
        str: The path to the capnp file.
    """
    d = get_app_dir()
    path = ""
    if is_frozen():
        path = os.path.join(d, "resources/base", CONSOLE_BACKEND_CAPNP_PATH)
    else:
        path = os.path.join(
            os.path.dirname(os.path.dirname(__file__)), "src/main/resources/base", CONSOLE_BACKEND_CAPNP_PATH
        )
    return path


def handle_cli_arguments(args: argparse.Namespace, globals_: QObject):
    if args.show_fileio:
        globals_.setProperty("showFileio", True)  # type: ignore
    if args.use_opengl:
        globals_.setProperty("useOpenGL", True)  # type: ignore
    if args.no_antialiasing:
        globals_.setProperty("useAntiAliasing", False)  # type: ignore
    if args.no_high_dpi:
        globals_.setProperty("useHighDpi", False)  # type: ignore
    if args.no_prompts:
        globals_.setProperty("showPrompts", False)  # type: ignore
    if args.refresh_rate is not None:
        globals_.setProperty("currentRefreshRate", args.refresh_rate)  # type: ignore
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


def main(passed_args: Optional[Tuple[str, ...]] = None) -> int:
    parser = argparse.ArgumentParser(add_help=False, usage=argparse.SUPPRESS)
    parser.add_argument("--show-fileio", action="store_true")
    parser.add_argument("--show-file-connection", action="store_true")
    parser.add_argument("--no-prompts", action="store_true")
    parser.add_argument("--use-opengl", action="store_true")
    parser.add_argument("--no-high-dpi", action="store_true")
    parser.add_argument("--no-antialiasing", action="store_true")
    parser.add_argument("--refresh-rate", type=int)
    parser.add_argument("--tab")
    parser.add_argument("--show-csv-log", action="store_true")
    parser.add_argument("--height", type=int)
    parser.add_argument("--width", type=int)

    args_main, unknown_args = parser.parse_known_args()
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
        QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_Use96Dpi)
    else:
        QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
        QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)
    app = QApplication(sys.argv)
    app.setWindowIcon(QIcon(":/images/icon.ico"))
    app.setOrganizationName(ApplicationMetadata.ORGANIZATION_NAME)
    app.setOrganizationDomain(ApplicationMetadata.ORGANIZATION_DOMAIN)
    app.setApplicationName(ApplicationMetadata.APPLICATION_NAME)
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Regular.ttf")
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Bold.ttf")
    QFontDatabase.addApplicationFont(":/fonts/RobotoCondensed-Regular.ttf")
    # We specifically *don't* want the RobotoCondensed-Bold.ttf font so we get the right look when bolded.

    qmlRegisterType(LogPanelData, "SwiftConsole", 1, 0, "LogPanelData")  # type: ignore
    qmlRegisterType(ConnectionData, "SwiftConsole", 1, 0, "ConnectionData")  # type: ignore
    qmlRegisterType(LoggingBarData, "SwiftConsole", 1, 0, "LoggingBarData")  # type: ignore
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
    qmlRegisterType(SolutionPositionPoints, "SwiftConsole", 1, 0, "SolutionPositionPoints")  # type: ignore
    qmlRegisterType(SolutionTableEntries, "SwiftConsole", 1, 0, "SolutionTableEntries")  # type: ignore
    qmlRegisterType(SolutionVelocityPoints, "SwiftConsole", 1, 0, "SolutionVelocityPoints")  # type: ignore
    qmlRegisterType(StatusBarData, "SwiftConsole", 1, 0, "StatusBarData")  # type: ignore
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

    engine.objectCreated.connect(handle_qml_load_errors)  # pylint: disable=no-member

    capnp_path = get_capnp_path()
    backend_main = console_backend.server.Server()  # pylint: disable=no-member,c-extension-no-member
    endpoint_main = backend_main.start()
    if found_help_arg:
        return 0
    # Unfortunately it is not possible to access singletons directly using the PySide2 API.
    # This approach stores the globals somwhere that can be grabbed and manipulated.
    component = QQmlComponent(engine)
    component.setData(
        b'import QtQuick 2.0\nimport "Constants"\nItem{ property var globals: Globals }',  # type: ignore
        QUrl("qrc:/grabGlobals.qml"),
    )
    globals_main = component.create()
    globals_main = globals_main.property("globals")  # type: ignore

    handle_cli_arguments(args_main, globals_main)

    engine.addImportPath("PySide2")
    engine.addImportPath(":/")
    engine.load(QUrl("qrc:/view.qml"))
    if not qml_object_created[0]:
        return 1
    messages_main = capnp.load(capnp_path)  # pylint: disable=no-member
    data_model = DataModel(endpoint_main, messages_main)
    log_panel_model = LogPanelModel()
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
    root_context.setContextProperty("log_panel_model", log_panel_model)
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
    root_context.setContextProperty("data_model", data_model)

    backend_msg_receiver = BackendMessageReceiver(app, backend_main, messages_main)
    backend_msg_receiver.start()

    app.exec_()

    endpoint_main.shutdown()

    backend_msg_receiver.join()

    return 0


if __name__ == "__main__":
    sys.exit(main())
