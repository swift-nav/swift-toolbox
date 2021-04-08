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

from PySide2.QtCore import Property, QUrl, QObject, Slot, QPointF
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

import console_resources  # type: ignore # pylint: disable=unused-import,import-error

import console_backend.server  # type: ignore  # pylint: disable=import-error,no-name-in-module

CONSOLE_BACKEND_CAPNP_PATH = "console_backend.capnp"

PIKSI_HOST = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com"
PIKSI_PORT = 55555

BOTTOM_NAVBAR: Dict[str, Any] = {
    Keys.AVAILABLE_PORTS: [],
    Keys.AVAILABLE_BAUDRATES: [],
    Keys.AVAILABLE_FLOWS: [],
}

SOLUTION_POSITION_TAB: Dict[str, Any] = {
    Keys.AVAILABLE_UNITS: [],
    Keys.CUR_POINTS: [],
    Keys.POINTS: [],
    Keys.LABELS: [],
    Keys.COLORS: [],
    Keys.LAT_MAX: 0,
    Keys.LAT_MIN: 0,
    Keys.LON_MAX: 0,
    Keys.LON_MIN: 0,
}

SOLUTION_TABLE: Dict[str, Any] = {
    Keys.ENTRIES: [],
}

SOLUTION_VELOCITY_TAB: Dict[str, Any] = {
    Keys.AVAILABLE_UNITS: [],
    Keys.POINTS: [],
    Keys.COLORS: [],
    Keys.MAX: 0,
    Keys.MIN: 0,
}

TRACKING_SIGNALS_TAB: Dict[str, Any] = {
    Keys.POINTS: [],
    Keys.CHECK_LABELS: [],
    Keys.LABELS: [],
    Keys.COLORS: [],
    Keys.MAX: 0,
    Keys.MIN: 0,
}


capnp.remove_import_hook()  # pylint: disable=no-member


def receive_messages(app_, backend, messages):
    while True:
        buffer = backend.fetch_message()
        m = messages.Message.from_bytes(buffer)
        if m.which == MessageKeys.STATUS:
            if m.status.text == ApplicationStates.CLOSE:
                return app_.quit()
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
        else:
            pass


class DataModel(QObject):

    endpoint: console_backend.server.ServerEndpoint  # pylint: disable=no-member
    messages: Any

    def __init__(self, endpoint, messages, file_in, connect=False):
        super().__init__()
        self.endpoint = endpoint
        self.messages = messages
        if connect and file_in is not None:
            self.connect_file(file_in)
        elif connect:
            self.connect_tcp(PIKSI_HOST, PIKSI_PORT)

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


class BottomNavbarData(QObject):

    _available_ports: List[str] = []
    _available_baudrates: List[str] = []
    _available_flows: List[str] = []

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


class BottomNavbarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(BottomNavbarData)  # type: ignore
    def fill_data(self, cp: BottomNavbarData) -> BottomNavbarData:  # pylint:disable=no-self-use
        cp.set_available_ports(BOTTOM_NAVBAR[Keys.AVAILABLE_PORTS])
        cp.set_available_baudrates(BOTTOM_NAVBAR[Keys.AVAILABLE_BAUDRATES])
        cp.set_available_flows(BOTTOM_NAVBAR[Keys.AVAILABLE_FLOWS])
        return cp


class SolutionPositionPoints(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods

    _colors: List[str] = []
    _labels: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _cur_points: List[List[QPointF]] = [[]]
    _valid: bool = False
    _lat_min: float = 0.0
    _lat_max: float = 0.0
    _lon_min: float = 0.0
    _lon_max: float = 0.0
    _available_units: List[str] = []

    def get_valid(self) -> bool:
        """Getter for _valid.

        Returns:
            bool: Whether it is valid or not.
        """
        return self._valid

    def set_valid(self, valid: bool) -> None:
        """Setter for _valid.
        """
        self._valid = valid

    valid = Property(bool, get_valid, set_valid)

    def get_lat_min(self) -> float:
        """Getter for _lat_min.
        """
        return self._lat_min

    def set_lat_min(self, lat_min_: float) -> None:
        """Setter for _lat_min.
        """
        self._lat_min = lat_min_

    lat_min_ = Property(float, get_lat_min, set_lat_min)

    def get_lat_max(self) -> float:
        """Getter for _lat_max.
        """
        return self._lat_max

    def set_lat_max(self, lat_max_: float) -> None:
        """Setter for _lat_max.
        """
        self._lat_max = lat_max_

    lat_max_ = Property(float, get_lat_max, set_lat_max)

    def get_lon_min(self) -> float:
        """Getter for _lon_min.
        """
        return self._lon_min

    def set_lon_min(self, lon_min_: float) -> None:
        """Setter for _lon_min.
        """
        self._lon_min = lon_min_

    lon_min_ = Property(float, get_lon_min, set_lon_min)

    def get_lon_max(self) -> float:
        """Getter for _lon_max.
        """
        return self._lon_max

    def set_lon_max(self, lon_max_: float) -> None:
        """Setter for _lon_max.
        """
        self._lon_max = lon_max_

    lon_max_ = Property(float, get_lon_max, set_lon_max)

    def get_labels(self) -> List[str]:
        return self._labels

    def set_labels(self, labels) -> None:
        self._labels = labels

    labels = Property(QTKeys.QVARIANTLIST, get_labels, set_labels)  # type: ignore

    def get_colors(self) -> List[str]:
        return self._colors

    def set_colors(self, colors) -> None:
        self._colors = colors

    colors = Property(QTKeys.QVARIANTLIST, get_colors, set_colors)  # type: ignore

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    def get_cur_points(self) -> List[List[QPointF]]:
        return self._cur_points

    def set_cur_points(self, cur_points) -> None:
        self._cur_points = cur_points

    cur_points = Property(QTKeys.QVARIANTLIST, get_cur_points, set_cur_points)  # type: ignore

    def get_available_units(self) -> List[str]:
        return self._available_units

    def set_available_units(self, available_units: List[str]) -> None:
        self._available_units = available_units

    available_units = Property(QTKeys.QVARIANTLIST, get_available_units, set_available_units)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        lines = series_list[0]
        scatters = series_list[1]
        cur_scatters = series_list[2]
        for idx, _ in enumerate(lines):
            lines[idx].replace(self._points[idx])
            scatters[idx].replace(self._points[idx])
            cur_scatters[idx].replace(self._cur_points[idx])


class SolutionPositionModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionPositionPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionPositionPoints) -> SolutionPositionPoints:  # pylint:disable=no-self-use
        cp.set_points(SOLUTION_POSITION_TAB[Keys.POINTS])
        cp.set_cur_points(SOLUTION_POSITION_TAB[Keys.CUR_POINTS])
        cp.set_labels(SOLUTION_POSITION_TAB[Keys.LABELS])
        cp.set_colors(SOLUTION_POSITION_TAB[Keys.COLORS])
        cp.set_lat_max(SOLUTION_POSITION_TAB[Keys.LAT_MAX])
        cp.set_lat_min(SOLUTION_POSITION_TAB[Keys.LAT_MIN])
        cp.set_lon_max(SOLUTION_POSITION_TAB[Keys.LON_MAX])
        cp.set_lon_min(SOLUTION_POSITION_TAB[Keys.LON_MIN])
        cp.set_available_units(SOLUTION_POSITION_TAB[Keys.AVAILABLE_UNITS])
        return cp


class SolutionTableEntries(QObject):

    _entries: List[List[str]] = []
    _valid: bool = False

    def get_valid(self) -> bool:
        """Getter for _valid.

        Returns:
            bool: Whether it is valid or not.
        """
        return self._valid

    def set_valid(self, valid: bool) -> None:
        """Setter for _valid.
        """
        self._valid = valid

    valid = Property(bool, get_valid, set_valid)

    def get_entries(self) -> List[List[str]]:
        """Getter for _entries.
        """
        return self._entries

    def set_entries(self, entries: List[List[str]]) -> None:
        """Setter for _entries.
        """
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])


class SolutionTableModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionTableEntries)  # type: ignore
    def fill_console_points(self, cp: SolutionTableEntries) -> SolutionTableEntries:  # pylint:disable=no-self-use
        cp.set_entries(SOLUTION_TABLE[Keys.ENTRIES])
        return cp


class SolutionVelocityPoints(QObject):

    _colors: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _valid: bool = False
    _min: float = 0.0
    _max: float = 0.0
    _available_units: List[str] = []

    def get_valid(self) -> bool:
        """Getter for _valid.

        Returns:
            bool: Whether it is valid or not.
        """
        return self._valid

    def set_valid(self, valid: bool) -> None:
        """Setter for _valid.
        """
        self._valid = valid

    valid = Property(bool, get_valid, set_valid)

    def get_min(self) -> float:
        """Getter for _min.
        """
        return self._min

    def set_min(self, min_: float) -> None:
        """Setter for _min.
        """
        self._min = min_

    min_ = Property(float, get_min, set_min)

    def get_max(self) -> float:
        """Getter for _max.
        """
        return self._max

    def set_max(self, max_: float) -> None:
        """Setter for _max.
        """
        self._max = max_

    max_ = Property(float, get_max, set_max)

    def get_available_units(self) -> List[str]:
        """Getter for _available_units.
        """
        return self._available_units

    def set_available_units(self, available_units: List[str]) -> None:
        """Setter for _available_units.
        """
        self._available_units = available_units

    available_units = Property(QTKeys.QVARIANTLIST, get_available_units, set_available_units)  # type: ignore

    def get_colors(self) -> List[str]:
        return self._colors

    def set_colors(self, colors) -> None:
        self._colors = colors

    colors = Property(QTKeys.QVARIANTLIST, get_colors, set_colors)  # type: ignore

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])


class SolutionVelocityModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionVelocityPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionVelocityPoints) -> SolutionVelocityPoints:  # pylint:disable=no-self-use
        cp.set_points(SOLUTION_VELOCITY_TAB[Keys.POINTS])
        cp.set_colors(SOLUTION_VELOCITY_TAB[Keys.COLORS])
        cp.set_max(SOLUTION_VELOCITY_TAB[Keys.MAX])
        cp.set_min(SOLUTION_VELOCITY_TAB[Keys.MIN])
        cp.set_available_units(SOLUTION_VELOCITY_TAB[Keys.AVAILABLE_UNITS])
        return cp


class TrackingSignalsPoints(QObject):

    _colors: List[str] = []
    _check_labels: List[str] = []
    _labels: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _valid: bool = False
    _min: float = 0.0
    _max: float = 0.0

    def get_valid(self) -> bool:
        """Getter for _valid.

        Returns:
            bool: Whether it is valid or not.
        """
        return self._valid

    def set_valid(self, valid: bool) -> None:
        """Setter for _valid.
        """
        self._valid = valid

    valid = Property(bool, get_valid, set_valid)

    def get_min(self) -> float:
        """Getter for _min.
        """
        return self._min

    def set_min(self, min_: float) -> None:
        """Setter for _min.
        """
        self._min = min_

    min_ = Property(float, get_min, set_min)

    def get_max(self) -> float:
        """Getter for _max.
        """
        return self._max

    def set_max(self, max_: float) -> None:
        """Setter for _max.
        """
        self._max = max_

    max_ = Property(float, get_max, set_max)

    def get_check_labels(self) -> List[str]:
        return self._check_labels

    def set_check_labels(self, check_labels) -> None:
        self._check_labels = check_labels

    check_labels = Property(QTKeys.QVARIANTLIST, get_check_labels, set_check_labels)  # type: ignore

    def get_labels(self) -> List[str]:
        return self._labels

    def set_labels(self, labels) -> None:
        self._labels = labels

    labels = Property(QTKeys.QVARIANTLIST, get_labels, set_labels)  # type: ignore

    def get_colors(self) -> List[str]:
        return self._colors

    def set_colors(self, colors) -> None:
        self._colors = colors

    colors = Property(QTKeys.QVARIANTLIST, get_colors, set_colors)  # type: ignore

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series_and_key in enumerate(series_list):
            series, _ = series_and_key
            if idx < len(self._points):
                series.replace(self._points[idx])


class TrackingSignalsModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(TrackingSignalsPoints)  # type: ignore
    def fill_console_points(self, cp: TrackingSignalsPoints) -> TrackingSignalsPoints:  # pylint:disable=no-self-use
        cp.set_points(TRACKING_SIGNALS_TAB[Keys.POINTS])
        cp.set_labels(TRACKING_SIGNALS_TAB[Keys.LABELS])
        cp.set_check_labels(TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS])
        cp.set_colors(TRACKING_SIGNALS_TAB[Keys.COLORS])
        cp.set_max(TRACKING_SIGNALS_TAB[Keys.MAX])
        cp.set_min(TRACKING_SIGNALS_TAB[Keys.MIN])
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
    parser = argparse.ArgumentParser()
    parser.add_argument("--file-in", help="Input file to parse.")
    parser.add_argument("--connect", help="Connect automatically.", action="store_true")
    args = parser.parse_args()

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)
    app = QApplication()

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

    data_model = DataModel(endpoint_main, messages_main, args.file_in, args.connect)
    bottom_navbar_model = BottomNavbarModel()
    solution_position_model = SolutionPositionModel()
    solution_table_model = SolutionTableModel()
    solution_velocity_model = SolutionVelocityModel()
    tracking_signals_model = TrackingSignalsModel()
    remote_observation_model = ObservationModel()
    local_observation_model = ObservationModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("bottom_navbar_model", bottom_navbar_model)
    root_context.setContextProperty("solution_position_model", solution_position_model)
    root_context.setContextProperty("solution_table_model", solution_table_model)
    root_context.setContextProperty("solution_velocity_model", solution_velocity_model)
    root_context.setContextProperty("tracking_signals_model", tracking_signals_model)
    root_context.setContextProperty("remote_observation_model", remote_observation_model)
    root_context.setContextProperty("local_observation_model", local_observation_model)
    root_context.setContextProperty("data_model", data_model)

    threading.Thread(target=receive_messages, args=(app, backend_main, messages_main,), daemon=True).start()
    sys.exit(app.exec_())
