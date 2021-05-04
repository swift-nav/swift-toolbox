"""Frontend module for the Swift Console.
"""
import argparse
import os
import sys
import threading

from typing import Dict, List, Any

import capnp  # type: ignore

from PySide2.QtWidgets import QApplication  # type: ignore

from fbs_runtime.application_context.PySide2 import ApplicationContext  # type: ignore  # pylint: disable=unused-import

from PySide2.QtCore import Property, QMutex, QObject, QUrl, QPointF, Slot
from PySide2.QtCharts import QtCharts  # pylint: disable=unused-import

from PySide2 import QtQml, QtCore

from PySide2.QtQml import qmlRegisterType

from constants import ApplicationStates, MessageKeys, Keys, QTKeys
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

LOG_PANEL: Dict[str, Any] = {
    Keys.ENTRIES: [],
}
log_panel_lock = QMutex()

BOTTOM_NAVBAR: Dict[str, Any] = {
    Keys.AVAILABLE_PORTS: [],
    Keys.AVAILABLE_BAUDRATES: [],
    Keys.AVAILABLE_FLOWS: [],
    Keys.CONNECTED: False,
}


capnp.remove_import_hook()  # pylint: disable=no-member


def receive_messages(app_, backend, messages):
    while True:
        buffer = backend.fetch_message()
        m = messages.Message.from_bytes(buffer)
        if m.which == MessageKeys.STATUS:
            if m.status.text == ApplicationStates.CLOSE:
                return app_.quit()
            if m.status.text == ApplicationStates.CONNECTED:
                BOTTOM_NAVBAR[Keys.CONNECTED] = True
            elif m.status.text == ApplicationStates.DISCONNECTED:
                BOTTOM_NAVBAR[Keys.CONNECTED] = False

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
        elif m.which == MessageKeys.BOTTOM_NAVBAR_STATUS:
            BOTTOM_NAVBAR[Keys.AVAILABLE_PORTS][:] = m.bottomNavbarStatus.availablePorts
            BOTTOM_NAVBAR[Keys.AVAILABLE_BAUDRATES][:] = m.bottomNavbarStatus.availableBaudrates
            BOTTOM_NAVBAR[Keys.AVAILABLE_FLOWS][:] = m.bottomNavbarStatus.availableFlows
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


class LogPanelData(QObject):
    _entries: List[str] = []

    def get_entries(self) -> List[str]:
        """Getter for _entries."""
        return self._entries

    def set_entries(self, entries: List[str]) -> None:
        """Setter for _entries."""
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore

    def append_entries(self, entries: List[str]) -> None:
        self._entries += entries


class LogPanelModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(LogPanelData)  # type: ignore
    def fill_data(self, cp: LogPanelData) -> LogPanelData:  # pylint:disable=no-self-use
        # Avoid locking so that message processor has priority to lock
        if LOG_PANEL[Keys.ENTRIES]:
            if log_panel_lock.try_lock():
                cp.append_entries(LOG_PANEL[Keys.ENTRIES])
                LOG_PANEL[Keys.ENTRIES][:] = []
                log_panel_lock.unlock()
        return cp


class BottomNavbarData(QObject):

    _available_ports: List[str] = []
    _available_baudrates: List[str] = []
    _available_flows: List[str] = []
    _connected: bool = False

    def get_available_ports(self) -> List[str]:
        return self._available_ports

    def set_available_ports(self, available_ports: List[str]) -> None:
        self._available_ports = available_ports

    available_ports = Property(QTKeys.QVARIANTLIST, get_available_ports, set_available_ports)  # type: ignore

    def get_available_baudrates(self) -> List[str]:
        return self._available_baudrates

    def set_available_baudrates(self, available_baudrates: List[str]) -> None:
        self._available_baudrates = available_baudrates

    available_baudrates = Property(
        QTKeys.QVARIANTLIST, get_available_baudrates, set_available_baudrates  # type: ignore
    )

    def get_available_flows(self) -> List[str]:
        return self._available_flows

    def set_available_flows(self, available_flows: List[str]) -> None:
        self._available_flows = available_flows

    available_flows = Property(QTKeys.QVARIANTLIST, get_available_flows, set_available_flows)  # type: ignore

    def get_connected(self) -> bool:
        """Getter for _connected.

        Returns:
            bool: Whether a connection is live or not.
        """
        return self._connected

    def set_connected(self, connected: bool) -> None:
        """Setter for _connected.
        """
        self._connected = connected

    connected = Property(bool, get_connected, set_connected)


class BottomNavbarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(BottomNavbarData)  # type: ignore
    def fill_data(self, cp: BottomNavbarData) -> BottomNavbarData:  # pylint:disable=no-self-use
        cp.set_available_ports(BOTTOM_NAVBAR[Keys.AVAILABLE_PORTS])
        cp.set_available_baudrates(BOTTOM_NAVBAR[Keys.AVAILABLE_BAUDRATES])
        cp.set_available_flows(BOTTOM_NAVBAR[Keys.AVAILABLE_FLOWS])
        return cp


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


if __name__ == "__main__":
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--no-opengl")
    parser.add_argument("--refresh-rate")
    parser.add_argument("--tab")

    args, _ = parser.parse_known_args()

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)
    app = QApplication()

    qmlRegisterType(LogPanelData, "SwiftConsole", 1, 0, "LogPanelData")  # type: ignore
    qmlRegisterType(BottomNavbarData, "SwiftConsole", 1, 0, "BottomNavbarData")  # type: ignore
    qmlRegisterType(SolutionPositionPoints, "SwiftConsole", 1, 0, "SolutionPositionPoints")  # type: ignore
    qmlRegisterType(SolutionTableEntries, "SwiftConsole", 1, 0, "SolutionTableEntries")  # type: ignore
    qmlRegisterType(SolutionVelocityPoints, "SwiftConsole", 1, 0, "SolutionVelocityPoints")  # type: ignore
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
    bottom_navbar_model = BottomNavbarModel()
    solution_position_model = SolutionPositionModel()
    solution_table_model = SolutionTableModel()
    solution_velocity_model = SolutionVelocityModel()
    tracking_signals_model = TrackingSignalsModel()
    remote_observation_model = ObservationModel()
    local_observation_model = ObservationModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("log_panel_model", log_panel_model)
    root_context.setContextProperty("bottom_navbar_model", bottom_navbar_model)
    root_context.setContextProperty("solution_position_model", solution_position_model)
    root_context.setContextProperty("solution_table_model", solution_table_model)
    root_context.setContextProperty("solution_velocity_model", solution_velocity_model)
    root_context.setContextProperty("tracking_signals_model", tracking_signals_model)
    root_context.setContextProperty("remote_observation_model", remote_observation_model)
    root_context.setContextProperty("local_observation_model", local_observation_model)
    root_context.setContextProperty("data_model", data_model)

    threading.Thread(target=receive_messages, args=(app, backend_main, messages_main), daemon=True).start()
    sys.exit(app.exec_())
