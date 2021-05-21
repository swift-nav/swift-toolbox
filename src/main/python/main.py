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

from PySide2.QtQml import QQmlComponent, qmlRegisterType

from constants import ApplicationStates, MessageKeys, Keys, Tabs

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


from observation_tab import (
    ObservationData,
    ObservationModel,
    REMOTE_OBSERVATION_TAB,
    LOCAL_OBSERVATION_TAB,
    obs_rows_to_json,
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
    Tabs.ADVANCED: {
        MAIN_INDEX: 6,
        SUB_INDEX: 0,
    },
}


capnp.remove_import_hook()  # pylint: disable=no-member


def receive_messages(app_, backend, messages):
    while True:
        buffer = backend.fetch_message()
        m = messages.Message.from_bytes(buffer)
        if m.which == MessageKeys.STATUS:
            if m.status.text == ApplicationStates.CLOSE:
                print("hi")
                return app_.quit()
            if m.status.text == ApplicationStates.CONNECTED:
                NAV_BAR[Keys.CONNECTED] = True
            elif m.status.text == ApplicationStates.DISCONNECTED:
                NAV_BAR[Keys.CONNECTED] = False

        elif m.which == MessageKeys.SOLUTION_POSITION_STATUS:
            SOLUTION_POSITION_TAB[Keys.LABELS][:] = m.solutionPositionStatus.labels
            SOLUTION_POSITION_TAB[Keys.COLORS][:] = m.solutionPositionStatus.colors
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
        elif m.which == MessageKeys.SOLUTION_TABLE_STATUS:
            SOLUTION_TABLE[Keys.ENTRIES][:] = [[entry.key, entry.val] for entry in m.solutionTableStatus.data]
        elif m.which == MessageKeys.SOLUTION_VELOCITY_STATUS:
            SOLUTION_VELOCITY_TAB[Keys.COLORS][:] = m.solutionVelocityStatus.colors
            SOLUTION_VELOCITY_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.solutionVelocityStatus.data[idx]]
                for idx in range(len(m.solutionVelocityStatus.data))
            ]
            SOLUTION_VELOCITY_TAB[Keys.MAX] = m.solutionVelocityStatus.max
            SOLUTION_VELOCITY_TAB[Keys.MIN] = m.solutionVelocityStatus.min
            SOLUTION_VELOCITY_TAB[Keys.AVAILABLE_UNITS][:] = m.solutionVelocityStatus.availableUnits
        elif m.which == MessageKeys.TRACKING_SIGNALS_STATUS:
            TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS][:] = m.trackingSignalsStatus.checkLabels
            TRACKING_SIGNALS_TAB[Keys.LABELS][:] = m.trackingSignalsStatus.labels
            TRACKING_SIGNALS_TAB[Keys.COLORS][:] = m.trackingSignalsStatus.colors
            TRACKING_SIGNALS_TAB[Keys.POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.trackingSignalsStatus.data[idx]]
                for idx in range(len(m.trackingSignalsStatus.data))
            ]
            TRACKING_SIGNALS_TAB[Keys.MAX] = m.trackingSignalsStatus.max
            TRACKING_SIGNALS_TAB[Keys.MIN] = m.trackingSignalsStatus.min
        elif m.which == MessageKeys.OBSERVATION_STATUS:
            if m.observationStatus.isRemote:
                REMOTE_OBSERVATION_TAB[Keys.TOW] = m.observationStatus.tow
                REMOTE_OBSERVATION_TAB[Keys.WEEK] = m.observationStatus.week
                REMOTE_OBSERVATION_TAB[Keys.ROWS][:] = obs_rows_to_json(m.observationStatus.rows)
            else:
                LOCAL_OBSERVATION_TAB[Keys.TOW] = m.observationStatus.tow
                LOCAL_OBSERVATION_TAB[Keys.WEEK] = m.observationStatus.week
                LOCAL_OBSERVATION_TAB[Keys.ROWS][:] = obs_rows_to_json(m.observationStatus.rows)
        elif m.which == MessageKeys.STATUS_BAR_STATUS:
            STATUS_BAR[Keys.PORT] = m.statusBarStatus.port
            STATUS_BAR[Keys.POS] = m.statusBarStatus.pos
            STATUS_BAR[Keys.RTK] = m.statusBarStatus.rtk
            STATUS_BAR[Keys.SATS] = m.statusBarStatus.sats
            STATUS_BAR[Keys.CORR_AGE] = m.statusBarStatus.corrAge
            STATUS_BAR[Keys.INS] = m.statusBarStatus.ins
            STATUS_BAR[Keys.DATA_RATE] = m.statusBarStatus.dataRate
            STATUS_BAR[Keys.SOLID_CONNECTION] = m.statusBarStatus.solidConnection
        elif m.which == MessageKeys.NAV_BAR_STATUS:
            NAV_BAR[Keys.AVAILABLE_PORTS][:] = m.navBarStatus.availablePorts
            NAV_BAR[Keys.AVAILABLE_BAUDRATES][:] = m.navBarStatus.availableBaudrates
            NAV_BAR[Keys.AVAILABLE_FLOWS][:] = m.navBarStatus.availableFlows
            NAV_BAR[Keys.AVAILABLE_REFRESH_RATES][:] = m.navBarStatus.availableRefreshRates
            NAV_BAR[Keys.PREVIOUS_HOSTS][:] = m.navBarStatus.previousHosts
            NAV_BAR[Keys.PREVIOUS_PORTS][:] = m.navBarStatus.previousPorts
            NAV_BAR[Keys.PREVIOUS_FILES][:] = m.navBarStatus.previousFiles
        elif m.which == MessageKeys.LOGGING_BAR_STATUS:
            LOGGING_BAR[Keys.FOLDER] = m.loggingBarStatus.folder
            LOGGING_BAR[Keys.PREVIOUS_FOLDERS][:] = m.loggingBarStatus.previousFolders
            print(LOGGING_BAR)
        elif m.which == MessageKeys.LOG_APPEND:
            log_panel_lock.lock()
            LOG_PANEL[Keys.ENTRIES] += [entry.line for entry in m.logAppend.entries]
            log_panel_lock.unlock()
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
        msg = self.messages.Message()
        msg.connectRequest = msg.init(MessageKeys.CONNECT_REQUEST)
        req = self.messages.Message()
        req.fileRequest = req.init(MessageKeys.FILE_REQUEST)
        req.fileRequest.filename = str(filename)
        msg.connectRequest.request = req
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int)  # type: ignore
    def connect_tcp(self, host: str, port: int) -> None:
        msg = self.messages.Message()
        msg.connectRequest = msg.init(MessageKeys.CONNECT_REQUEST)
        req = self.messages.Message()
        req.tcpRequest = req.init(MessageKeys.TCP_REQUEST)
        req.tcpRequest.host = str(host)
        req.tcpRequest.port = int(port)
        msg.connectRequest.request = req
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int, str)  # type: ignore
    def connect_serial(self, device: str, baudrate: int, flow_control: str) -> None:
        msg = self.messages.Message()
        msg.connectRequest = msg.init(MessageKeys.CONNECT_REQUEST)
        req = self.messages.Message()
        req.serialRequest = req.init(MessageKeys.SERIAL_REQUEST)
        req.serialRequest.device = str(device)
        req.serialRequest.baudrate = int(baudrate)
        req.serialRequest.flowControl = str(flow_control)
        msg.connectRequest.request = req
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def disconnect(self) -> None:
        msg = self.messages.Message()
        msg.connectRequest = msg.init(MessageKeys.CONNECT_REQUEST)
        req = self.messages.Message()
        req.disconnectRequest = req.init(MessageKeys.DISCONNECT_REQUEST)
        msg.connectRequest.request = req
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def serial_refresh(self) -> None:
        msg = self.messages.Message()
        msg.connectRequest = msg.init(MessageKeys.CONNECT_REQUEST)
        req = self.messages.Message()
        req.serialRefreshRequest = req.init(MessageKeys.SERIAL_REFRESH_REQUEST)
        msg.connectRequest.request = req
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(bool)  # type: ignore
    def pause(self, pause_: bool) -> None:
        msg = self.messages.Message()
        msg.connectRequest = msg.init(MessageKeys.CONNECT_REQUEST)
        req = self.messages.Message()
        req.pauseRequest = req.init(MessageKeys.PAUSE_REQUEST)
        req.pauseRequest.pause = pause_
        msg.connectRequest.request = req
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def tracking_signals_check_visibility(self, checks: List[str]) -> None:
        m = self.messages.Message()
        m.trackingSignalsStatusFront = m.init(MessageKeys.TRACKING_SIGNALS_STATUS_FRONT)
        m.trackingSignalsStatusFront.trackingSignalsCheckVisibility = checks
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_velocity_unit(self, unit: str) -> None:
        m = self.messages.Message()
        m.solutionVelocityStatusFront = m.init(MessageKeys.SOLUTION_VELOCITY_STATUS_FRONT)
        m.solutionVelocityStatusFront.solutionVelocityUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_position_unit(self, unit: str) -> None:
        m = self.messages.Message()
        m.solutionPositionStatusUnitFront = m.init(MessageKeys.SOLUTION_POSITION_STATUS_UNIT_FRONT)
        m.solutionPositionStatusUnitFront.solutionPositionUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def solution_position(self, buttons: list) -> None:
        m = self.messages.Message()
        m.solutionPositionStatusButtonFront = m.init(MessageKeys.SOLUTION_POSITION_STATUS_BUTTON_FRONT)
        m.solutionPositionStatusButtonFront.solutionPositionPause = buttons[0]
        m.solutionPositionStatusButtonFront.solutionPositionClear = buttons[1]
        m.solutionPositionStatusButtonFront.solutionPositionZoom = buttons[2]
        m.solutionPositionStatusButtonFront.solutionPositionCenter = buttons[3]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, str)  # type: ignore
    def logging_bar(self, buttons, directory) -> None:
        m = self.messages.Message()
        m.loggingBarFront = m.init(MessageKeys.LOGGING_BAR_FRONT)
        m.loggingBarFront.solutionLogging = buttons[0]
        m.loggingBarFront.sbpLogging = buttons[1]
        m.loggingBarFront.sbpFileFormat = buttons[2]
        m.loggingBarFront.directory = directory
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


if __name__ == "__main__":
    parser = argparse.ArgumentParser(add_help=False, usage=argparse.SUPPRESS)
    parser.add_argument("--no-opengl", action="store_false")
    parser.add_argument("--refresh-rate")
    parser.add_argument("--tab")

    args_main, _ = parser.parse_known_args()

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)
    app = QApplication()

    qmlRegisterType(LogPanelData, "SwiftConsole", 1, 0, "LogPanelData")  # type: ignore
    qmlRegisterType(NavBarData, "SwiftConsole", 1, 0, "NavBarData")  # type: ignore
    qmlRegisterType(LoggingBarData, "SwiftConsole", 1, 0, "LoggingBarData")  # type: ignore
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

    threading.Thread(
        target=receive_messages,
        args=(
            app,
            backend_main,
            messages_main,
        ),
        daemon=True,
    ).start()
    sys.exit(app.exec_())
