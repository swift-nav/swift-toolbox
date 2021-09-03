"""Frontend module for the Swift Console.
"""
import argparse
import os
import sys
import threading

from typing import List, Any

import capnp  # type: ignore

from PySide2.QtWidgets import QApplication  # type: ignore

from fbs_runtime.application_context.PySide2 import ApplicationContext  # type: ignore  # pylint: disable=unused-import

from PySide2.QtCore import QObject, QUrl, QPointF, Slot
from PySide2.QtCharts import QtCharts  # pylint: disable=unused-import

from PySide2 import QtQml, QtCore

from PySide2.QtGui import QFontDatabase

from PySide2.QtQml import QQmlComponent, qmlRegisterType

from constants import ApplicationStates, Keys, Tabs

from log_panel import (
    LOG_PANEL,
    log_panel_lock,
    LogPanelData,
    LogPanelModel,
)

from nav_bar import (
    NAV_BAR,
    NavBarData,
    NavBarModel,
)

from logging_bar import (
    LOGGING_BAR,
    LoggingBarData,
    LoggingBarModel,
)

from advanced_ins_tab import (
    AdvancedInsModel,
    AdvancedInsPoints,
    ADVANCED_INS_TAB,
)

from advanced_magnetometer_tab import (
    AdvancedMagnetometerModel,
    AdvancedMagnetometerPoints,
    ADVANCED_MAGNETOMETER_TAB,
)

from advanced_spectrum_analyzer_tab import (
    AdvancedSpectrumAnalyzerModel,
    AdvancedSpectrumAnalyzerPoints,
    ADVANCED_SPECTRUM_ANALYZER_TAB,
)

from fusion_status_flags import (
    FusionStatusFlagsModel,
    FusionStatusFlagsData,
    FUSION_STATUS_FLAGS,
)

from baseline_plot import (
    BaselinePlotModel,
    BaselinePlotPoints,
    BASELINE_PLOT,
)

from baseline_table import (
    BaselineTableEntries,
    BaselineTableModel,
    BASELINE_TABLE,
)

from observation_tab import (
    ObservationData,
    ObservationModel,
    REMOTE_OBSERVATION_TAB,
    LOCAL_OBSERVATION_TAB,
    obs_rows_to_json,
)

from settings_table import (
    SettingsTableEntries,
    SettingsTableModel,
    SETTINGS_TABLE,
    settings_rows_to_json,
)

from solution_position_tab import (
    SolutionPositionModel,
    SolutionPositionPoints,
    SOLUTION_POSITION_TAB,
)

from solution_table import (
    SolutionTableEntries,
    SolutionTableModel,
    SOLUTION_TABLE,
)

from solution_velocity_tab import (
    SolutionVelocityModel,
    SolutionVelocityPoints,
    SOLUTION_VELOCITY_TAB,
)

from status_bar import (
    STATUS_BAR,
    StatusBarData,
    StatusBarModel,
)

from tracking_signals_tab import (
    TrackingSignalsModel,
    TrackingSignalsPoints,
    TRACKING_SIGNALS_TAB,
)

import console_resources  # type: ignore # pylint: disable=unused-import,import-error

import console_backend.server  # type: ignore  # pylint: disable=import-error,no-name-in-module

CONSOLE_BACKEND_CAPNP_PATH = "console_backend.capnp"

PIKSI_HOST = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com"
PIKSI_PORT = 55555


MAIN_INDEX = "MAIN_INDEX"
SUB_INDEX = "SUB_INDEX"

TAB_LAYOUT = {
    Tabs.TRACKING_SIGNALS: {
        MAIN_INDEX: 0,
        SUB_INDEX: 0,
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
    Tabs.ADVANCED_INS: {
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
}


capnp.remove_import_hook()  # pylint: disable=no-member


def receive_messages(app_, backend, messages):
    while True:
        buffer = backend.fetch_message()
        if not buffer:
            print("terminating GUI loop", file=sys.stderr)
            break
        Message = messages.Message
        m = Message.from_bytes(buffer)
        if m.which == Message.Union.Status:
            if m.status.text == ApplicationStates.CLOSE:
                return app_.quit()
            if m.status.text == ApplicationStates.CONNECTED:
                NAV_BAR[Keys.CONNECTED] = True
            elif m.status.text == ApplicationStates.DISCONNECTED:
                NAV_BAR[Keys.CONNECTED] = False

        elif m.which == Message.Union.SolutionPositionStatus:
            SOLUTION_POSITION_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionPositionStatus.data[idx]]
                for idx in range(len(m.solutionPositionStatus.data))
            ]
            SOLUTION_POSITION_TAB[Keys.CUR_POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionPositionStatus.curData[idx]]
                for idx in range(len(m.solutionPositionStatus.curData))
            ]
            SOLUTION_POSITION_TAB[Keys.LAT_MAX] = m.solutionPositionStatus.latMax
            SOLUTION_POSITION_TAB[Keys.LAT_MIN] = m.solutionPositionStatus.latMin
            SOLUTION_POSITION_TAB[Keys.LON_MAX] = m.solutionPositionStatus.lonMax
            SOLUTION_POSITION_TAB[Keys.LON_MIN] = m.solutionPositionStatus.lonMin
            SOLUTION_POSITION_TAB[Keys.AVAILABLE_UNITS][:] = m.solutionPositionStatus.availableUnits
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
            BASELINE_PLOT[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.baselinePlotStatus.data[idx]]
                for idx in range(len(m.baselinePlotStatus.data))
            ]
            BASELINE_PLOT[Keys.CUR_POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.baselinePlotStatus.curData[idx]]
                for idx in range(len(m.baselinePlotStatus.curData))
            ]
            BASELINE_PLOT[Keys.N_MAX] = m.baselinePlotStatus.nMax
            BASELINE_PLOT[Keys.N_MIN] = m.baselinePlotStatus.nMin
            BASELINE_PLOT[Keys.E_MAX] = m.baselinePlotStatus.eMax
            BASELINE_PLOT[Keys.E_MIN] = m.baselinePlotStatus.eMin
        elif m.which == Message.Union.BaselineTableStatus:
            BASELINE_TABLE[Keys.ENTRIES][:] = [[entry.key, entry.val] for entry in m.baselineTableStatus.data]
        elif m.which == Message.Union.AdvancedInsStatus:
            ADVANCED_INS_TAB[Keys.FIELDS_DATA][:] = m.advancedInsStatus.fieldsData
            ADVANCED_INS_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.advancedInsStatus.data[idx]]
                for idx in range(len(m.advancedInsStatus.data))
            ]
        elif m.which == Message.Union.AdvancedSpectrumAnalyzerStatus:
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.CHANNEL] = m.advancedSpectrumAnalyzerStatus.channel
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.POINTS][:] = [
                QPointF(point.x, point.y) for point in m.advancedSpectrumAnalyzerStatus.data
            ]
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.YMAX] = m.advancedSpectrumAnalyzerStatus.ymax
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.YMIN] = m.advancedSpectrumAnalyzerStatus.ymin
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.XMAX] = m.advancedSpectrumAnalyzerStatus.xmax
            ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.XMIN] = m.advancedSpectrumAnalyzerStatus.xmin
        elif m.which == Message.Union.AdvancedMagnetometerStatus:
            ADVANCED_MAGNETOMETER_TAB[Keys.YMAX] = m.advancedMagnetometerStatus.ymax
            ADVANCED_MAGNETOMETER_TAB[Keys.YMIN] = m.advancedMagnetometerStatus.ymin
            ADVANCED_MAGNETOMETER_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.advancedMagnetometerStatus.data[idx]]
                for idx in range(len(m.advancedMagnetometerStatus.data))
            ]
        elif m.which == Message.Union.FusionStatusFlagsStatus:
            FUSION_STATUS_FLAGS[Keys.GNSSPOS] = m.fusionStatusFlagsStatus.gnsspos
            FUSION_STATUS_FLAGS[Keys.GNSSVEL] = m.fusionStatusFlagsStatus.gnssvel
            FUSION_STATUS_FLAGS[Keys.WHEELTICKS] = m.fusionStatusFlagsStatus.wheelticks
            FUSION_STATUS_FLAGS[Keys.SPEED] = m.fusionStatusFlagsStatus.speed
            FUSION_STATUS_FLAGS[Keys.NHC] = m.fusionStatusFlagsStatus.nhc
            FUSION_STATUS_FLAGS[Keys.ZEROVEL] = m.fusionStatusFlagsStatus.zerovel
        elif m.which == Message.Union.TrackingSignalsStatus:
            TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS][:] = m.trackingSignalsStatus.checkLabels
            TRACKING_SIGNALS_TAB[Keys.LABELS][:] = m.trackingSignalsStatus.labels
            TRACKING_SIGNALS_TAB[Keys.COLORS][:] = m.trackingSignalsStatus.colors
            TRACKING_SIGNALS_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.trackingSignalsStatus.data[idx]]
                for idx in range(len(m.trackingSignalsStatus.data))
            ]
            TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET] = m.trackingSignalsStatus.xminOffset
        elif m.which == Message.Union.ObservationStatus:
            if m.observationStatus.isRemote:
                REMOTE_OBSERVATION_TAB[Keys.TOW] = m.observationStatus.tow
                REMOTE_OBSERVATION_TAB[Keys.WEEK] = m.observationStatus.week
                REMOTE_OBSERVATION_TAB[Keys.ROWS][:] = obs_rows_to_json(m.observationStatus.rows)
            else:
                LOCAL_OBSERVATION_TAB[Keys.TOW] = m.observationStatus.tow
                LOCAL_OBSERVATION_TAB[Keys.WEEK] = m.observationStatus.week
                LOCAL_OBSERVATION_TAB[Keys.ROWS][:] = obs_rows_to_json(m.observationStatus.rows)
        elif m.which == Message.Union.StatusBarStatus:
            STATUS_BAR[Keys.PORT] = m.statusBarStatus.port
            STATUS_BAR[Keys.POS] = m.statusBarStatus.pos
            STATUS_BAR[Keys.RTK] = m.statusBarStatus.rtk
            STATUS_BAR[Keys.SATS] = m.statusBarStatus.sats
            STATUS_BAR[Keys.CORR_AGE] = m.statusBarStatus.corrAge
            STATUS_BAR[Keys.INS] = m.statusBarStatus.ins
            STATUS_BAR[Keys.DATA_RATE] = m.statusBarStatus.dataRate
            STATUS_BAR[Keys.SOLID_CONNECTION] = m.statusBarStatus.solidConnection
        elif m.which == Message.Union.NavBarStatus:
            NAV_BAR[Keys.AVAILABLE_PORTS][:] = m.navBarStatus.availablePorts
            NAV_BAR[Keys.AVAILABLE_BAUDRATES][:] = m.navBarStatus.availableBaudrates
            NAV_BAR[Keys.AVAILABLE_FLOWS][:] = m.navBarStatus.availableFlows
            NAV_BAR[Keys.AVAILABLE_REFRESH_RATES][:] = m.navBarStatus.availableRefreshRates
            NAV_BAR[Keys.PREVIOUS_HOSTS][:] = m.navBarStatus.previousHosts
            NAV_BAR[Keys.PREVIOUS_PORTS][:] = m.navBarStatus.previousPorts
            NAV_BAR[Keys.PREVIOUS_FILES][:] = m.navBarStatus.previousFiles
            NAV_BAR[Keys.LOG_LEVEL] = m.navBarStatus.logLevel
        elif m.which == Message.Union.LoggingBarStatus:
            LOGGING_BAR[Keys.PREVIOUS_FOLDERS][:] = m.loggingBarStatus.previousFolders
            LOGGING_BAR[Keys.CSV_LOGGING] = m.loggingBarStatus.csvLogging
            LOGGING_BAR[Keys.SBP_LOGGING] = m.loggingBarStatus.sbpLogging
        elif m.which == Message.Union.LogAppend:
            log_panel_lock.lock()
            LOG_PANEL[Keys.ENTRIES] += [entry.line for entry in m.logAppend.entries]
            log_panel_lock.unlock()
        elif m.which == Message.Union.SettingsTableStatus:
            SETTINGS_TABLE[Keys.ENTRIES][:] = settings_rows_to_json(m.settingsTableStatus.data)
        else:
            pass


class DataModel(QObject):

    endpoint: console_backend.server.ServerEndpoint  # pylint: disable=no-member
    messages: Any

    def __init__(self, endpoint, messages):
        super().__init__()
        self.endpoint = endpoint
        self.messages = messages

    @Slot()  # type: ignore
    def connect(self) -> None:
        self.connect_tcp(PIKSI_HOST, PIKSI_PORT)

    @Slot(str)  # type: ignore
    def connect_file(self, filename: str) -> None:
        Message = self.messages.Message
        msg = Message()
        msg.fileRequest = msg.init(Message.Union.FileRequest)
        msg.fileRequest.filename = str(filename)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int)  # type: ignore
    def connect_tcp(self, host: str, port: int) -> None:
        Message = self.messages.Message
        msg = Message()
        msg.tcpRequest = msg.init(Message.Union.TcpRequest)
        msg.tcpRequest.host = str(host)
        msg.tcpRequest.port = int(port)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int, str)  # type: ignore
    def connect_serial(self, device: str, baudrate: int, flow_control: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.serialRequest = msg.init(Message.Union.SerialRequest)
        msg.serialRequest.device = str(device)
        msg.serialRequest.baudrate = int(baudrate)
        msg.serialRequest.flowControl = str(flow_control)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def disconnect(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.disconnectRequest = msg.init(Message.Union.DisconnectRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def serial_refresh(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.serialRefreshRequest = msg.init(Message.Union.SerialRefreshRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def settings_refresh(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsRefreshRequest = msg.init(Message.Union.SettingsRefreshRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def settings_export_request(self, path: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsExportRequest = msg.init(Message.Union.SettingsExportRequest)
        msg.settingsExportRequest.path = path
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def settings_import_request(self, path: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsImportRequest = msg.init(Message.Union.SettingsImportRequest)
        msg.settingsImportRequest.path = path
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, str, str)  # type: ignore
    def settings_save_request(self, group: str, name: str, value: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsSaveRequest = msg.init(Message.Union.SettingsSaveRequest)
        msg.settingsSaveRequest.group = group
        msg.settingsSaveRequest.name = name
        msg.settingsSaveRequest.value = value
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(bool)  # type: ignore
    def pause(self, pause_: bool) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.pauseRequest = msg.init(Message.Union.PauseRequest)
        msg.pauseRequest.pause = pause_
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def tracking_signals_check_visibility(self, checks: List[str]) -> None:
        Message = self.messages.Message
        m = Message()
        m.trackingSignalsStatusFront = m.init(Message.Union.TrackingSignalsStatusFront)
        m.trackingSignalsStatusFront.trackingSignalsCheckVisibility = checks
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_velocity_unit(self, unit: str) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionVelocityStatusFront = m.init(Message.Union.SolutionVelocityStatusFront)
        m.solutionVelocityStatusFront.solutionVelocityUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(int)  # type: ignore
    def advanced_spectrum_analyzer_channel(self, channel: int) -> None:
        Message = self.messages.Message
        m = Message()
        m.advancedSpectrumAnalyzerStatusFront = m.init(Message.Union.AdvancedSpectrumAnalyzerStatusFront)
        m.advancedSpectrumAnalyzerStatusFront.channel = channel
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_position_unit(self, unit: str) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionPositionStatusUnitFront = m.init(Message.Union.SolutionPositionStatusUnitFront)
        m.solutionPositionStatusUnitFront.solutionPositionUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def solution_position(self, buttons: list) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionPositionStatusButtonFront = m.init(Message.Union.SolutionPositionStatusButtonFront)
        m.solutionPositionStatusButtonFront.solutionPositionPause = buttons[0]
        m.solutionPositionStatusButtonFront.solutionPositionClear = buttons[1]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def baseline_plot(self, buttons: list) -> None:
        Message = self.messages.Message
        m = Message()
        m.baselinePlotStatusButtonFront = m.init(Message.Union.BaselinePlotStatusButtonFront)
        m.baselinePlotStatusButtonFront.pause = buttons[0]
        m.baselinePlotStatusButtonFront.clear = buttons[1]
        m.baselinePlotStatusButtonFront.resetFilters = buttons[2]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, str)  # type: ignore
    def logging_bar(self, buttons, directory) -> None:
        Message = self.messages.Message
        m = Message()
        m.loggingBarFront = m.init(Message.Union.LoggingBarFront)
        m.loggingBarFront.csvLogging = buttons[0]
        m.loggingBarFront.sbpLogging = buttons[1]
        m.loggingBarFront.directory = directory
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def log_level(self, log_level) -> None:
        Message = self.messages.Message
        m = Message()
        m.logLevelFront = m.init(Message.Union.LogLevelFront)
        m.logLevelFront.logLevel = str(log_level)
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)


def is_frozen() -> bool:
    """Check whether the application is frozen.

    FBS and nuitka agnostic.

    Returns:
        bool: Whether the application is frozen.
    """
    return getattr(sys, "frozen", False) or "__compiled__" in globals()


def get_capnp_path() -> str:
    """Get the path to the capnp file based on current installer.

    Returns:
        str: The path to the capnp file.
    """

    d = os.path.dirname(sys.executable)
    path = ""
    if is_frozen():
        path = os.path.join(d, CONSOLE_BACKEND_CAPNP_PATH)
    else:
        path = os.path.join(os.path.dirname(os.path.dirname(__file__)), "resources/base", CONSOLE_BACKEND_CAPNP_PATH)
    return path


def handle_cli_arguments(args: argparse.Namespace, globals_: QObject):
    if args.no_opengl:
        globals_.setProperty("useOpenGL", False)  # type: ignore
    if args.refresh_rate is not None:
        globals_.setProperty("currentRefreshRate", args.refresh_rate)  # type: ignore
    if args.tab is not None:
        layout_idxs = TAB_LAYOUT[args.tab]
        globals_.setProperty("initialMainTabIndex", layout_idxs[MAIN_INDEX])  # type: ignore
        globals_.setProperty("initialSubTabIndex", layout_idxs[SUB_INDEX])  # type: ignore
    if args.show_csv_log:
        globals_.setProperty("showCsvLog", True)  # type: ignore


if __name__ == "__main__":
    parser = argparse.ArgumentParser(add_help=False, usage=argparse.SUPPRESS)
    parser.add_argument("--no-opengl", action="store_false")
    parser.add_argument("--refresh-rate")
    parser.add_argument("--tab")
    parser.add_argument("--show-csv-log", action="store_true")

    args_main, _ = parser.parse_known_args()

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)
    app = QApplication()
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Regular.ttf")
    QFontDatabase.addApplicationFont(":/fonts/Roboto-Bold.ttf")

    qmlRegisterType(LogPanelData, "SwiftConsole", 1, 0, "LogPanelData")  # type: ignore
    qmlRegisterType(NavBarData, "SwiftConsole", 1, 0, "NavBarData")  # type: ignore
    qmlRegisterType(LoggingBarData, "SwiftConsole", 1, 0, "LoggingBarData")  # type: ignore
    qmlRegisterType(AdvancedInsPoints, "SwiftConsole", 1, 0, "AdvancedInsPoints")  # type: ignore
    qmlRegisterType(AdvancedMagnetometerPoints, "SwiftConsole", 1, 0, "AdvancedMagnetometerPoints")  # type: ignore
    qmlRegisterType(
        AdvancedSpectrumAnalyzerPoints, "SwiftConsole", 1, 0, "AdvancedSpectrumAnalyzerPoints"  # type: ignore
    )
    qmlRegisterType(FusionStatusFlagsData, "SwiftConsole", 1, 0, "FusionStatusFlagsData")  # type: ignore
    qmlRegisterType(BaselinePlotPoints, "SwiftConsole", 1, 0, "BaselinePlotPoints")  # type: ignore
    qmlRegisterType(BaselineTableEntries, "SwiftConsole", 1, 0, "BaselineTableEntries")  # type: ignore
    qmlRegisterType(SettingsTableEntries, "SwiftConsole", 1, 0, "SettingsTableEntries")  # type: ignore
    qmlRegisterType(SolutionPositionPoints, "SwiftConsole", 1, 0, "SolutionPositionPoints")  # type: ignore
    qmlRegisterType(SolutionTableEntries, "SwiftConsole", 1, 0, "SolutionTableEntries")  # type: ignore
    qmlRegisterType(SolutionVelocityPoints, "SwiftConsole", 1, 0, "SolutionVelocityPoints")  # type: ignore
    qmlRegisterType(StatusBarData, "SwiftConsole", 1, 0, "StatusBarData")  # type: ignore
    qmlRegisterType(TrackingSignalsPoints, "SwiftConsole", 1, 0, "TrackingSignalsPoints")  # type: ignore
    qmlRegisterType(ObservationData, "SwiftConsole", 1, 0, "ObservationData")  # type: ignore

    engine = QtQml.QQmlApplicationEngine()

    capnp_path = get_capnp_path()

    engine.addImportPath("PySide2")

    engine.load(QUrl("qrc:/view.qml"))

    messages_main = capnp.load(capnp_path)  # pylint: disable=no-member

    backend_main = console_backend.server.Server()  # pylint: disable=no-member
    endpoint_main = backend_main.start()

    data_model = DataModel(endpoint_main, messages_main)
    log_panel_model = LogPanelModel()
    nav_bar_model = NavBarModel()
    advanced_ins_model = AdvancedInsModel()
    advanced_magnetometer_model = AdvancedMagnetometerModel()
    advanced_spectrum_analyzer_model = AdvancedSpectrumAnalyzerModel()
    fusion_engine_flags_model = FusionStatusFlagsModel()
    baseline_plot_model = BaselinePlotModel()
    baseline_table_model = BaselineTableModel()
    settings_table_model = SettingsTableModel()
    solution_position_model = SolutionPositionModel()
    solution_table_model = SolutionTableModel()
    solution_velocity_model = SolutionVelocityModel()
    status_bar_model = StatusBarModel()
    logging_bar_model = LoggingBarModel()
    tracking_signals_model = TrackingSignalsModel()
    remote_observation_model = ObservationModel()
    local_observation_model = ObservationModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("log_panel_model", log_panel_model)
    root_context.setContextProperty("nav_bar_model", nav_bar_model)
    root_context.setContextProperty("advanced_ins_model", advanced_ins_model)
    root_context.setContextProperty("advanced_magnetometer_model", advanced_magnetometer_model)
    root_context.setContextProperty("advanced_spectrum_analyzer_model", advanced_spectrum_analyzer_model)
    root_context.setContextProperty("fusion_engine_flags_model", fusion_engine_flags_model)
    root_context.setContextProperty("baseline_plot_model", baseline_plot_model)
    root_context.setContextProperty("baseline_table_model", baseline_table_model)
    root_context.setContextProperty("settings_table_model", settings_table_model)
    root_context.setContextProperty("solution_position_model", solution_position_model)
    root_context.setContextProperty("solution_table_model", solution_table_model)
    root_context.setContextProperty("solution_velocity_model", solution_velocity_model)
    root_context.setContextProperty("status_bar_model", status_bar_model)
    root_context.setContextProperty("logging_bar_model", logging_bar_model)
    root_context.setContextProperty("tracking_signals_model", tracking_signals_model)
    root_context.setContextProperty("remote_observation_model", remote_observation_model)
    root_context.setContextProperty("local_observation_model", local_observation_model)
    root_context.setContextProperty("data_model", data_model)

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

    server_thread = threading.Thread(
        target=receive_messages,
        args=(
            app,
            backend_main,
            messages_main,
        ),
        daemon=True,
    )

    server_thread.start()
    app.exec_()

    endpoint_main.shutdown()
    server_thread.join()

    sys.exit()
