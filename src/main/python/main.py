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

from PySide2.QtCore import QUrl, QObject, Slot, QPointF
from PySide2.QtCharts import QtCharts  # pylint: disable=unused-import

from PySide2 import QtQml, QtCore

from PySide2.QtQml import qmlRegisterType
from PySide2.QtCore import Property

from constants import ApplicationStates, MessageKeys, Keys, QTKeys

import console_resources  # type: ignore # pylint: disable=unused-import,import-error

import console_backend.server  # type: ignore  # pylint: disable=import-error,no-name-in-module

CONSOLE_BACKEND_CAPNP_PATH = "console_backend.capnp"

PIKSI_HOST = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com"
PIKSI_PORT = 55555

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
        elif m.which == MessageKeys.SOLUTION_TABLE_STATUS:
            SOLUTION_TABLE[Keys.ENTRIES][:] = [
                (entry.key, entry.val) for entry in m.solutionTableStatus.data
            ]
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
        else:
            pass


class DataModel(QObject):

    endpoint: console_backend.server.ServerEndpoint  # pylint: disable=no-member
    messages: Any

    def __init__(self, endpoint, messages, file_in, connect=False):
        super().__init__()
        self.endpoint = endpoint
        self.messages = messages
        self.file_in = file_in
        if connect and file_in is not None:
            self.readfile()
        elif connect:
            self.connect()

    @Slot()  # type: ignore
    def connect(self) -> None:
        msg = self.messages.Message()
        msg.connectRequest.host = PIKSI_HOST
        msg.connectRequest.port = PIKSI_PORT
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def readfile(self) -> None:
        m = self.messages.Message()
        m.fileinRequest = m.init("fileinRequest")
        m.fileinRequest.filename = str(self.file_in)
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def tracking_signals_check_visibility(self, checks: List[str]) -> None:
        m = self.messages.Message()
        m.trackingSignalsStatusFront = m.init("trackingSignalsStatusFront")
        m.trackingSignalsStatusFront.trackingSignalsCheckVisibility = checks
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_velocity_unit(self, unit: str) -> None:
        m = self.messages.Message()
        m.solutionVelocityStatusFront = m.init("solutionVelocityStatusFront")
        m.solutionVelocityStatusFront.solutionVelocityUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)


<<<<<<< Updated upstream
=======
class SolutionTableEntries(QObject):

    _entries: List[Tuple[str, str]] = []
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

    def get_entries(self) -> List[Tuple[str, str]]:
        """Getter for _entries.
        """
        return self._entries

    def set_entries(self, entries: List[Tuple[str, str]]) -> None:
        """Setter for _entries.
        """
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])

class SolutionTableModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionVelocityPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionVelocityPoints) -> SolutionVelocityPoints:  # pylint:disable=no-self-use
        cp.set_points(SOLUTION_VELOCITY_TAB[Keys.POINTS])
        cp.set_colors(SOLUTION_VELOCITY_TAB[Keys.COLORS])
        cp.set_max(SOLUTION_VELOCITY_TAB[Keys.MAX])
        cp.set_min(SOLUTION_VELOCITY_TAB[Keys.MIN])
        cp.set_available_units(SOLUTION_VELOCITY_TAB[Keys.AVAILABLE_UNITS])
        return cp


>>>>>>> Stashed changes
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

    qmlRegisterType(SolutionVelocityPoints, "SwiftConsole", 1, 0, "SolutionVelocityPoints")  # type: ignore
    qmlRegisterType(TrackingSignalsPoints, "SwiftConsole", 1, 0, "TrackingSignalsPoints")  # type: ignore
    engine = QtQml.QQmlApplicationEngine()

    capnp_path = get_capnp_path()

    engine.addImportPath("PySide2")

    engine.load(QUrl("qrc:/view.qml"))

    messages_main = capnp.load(capnp_path)  # pylint: disable=no-member

    backend_main = console_backend.server.Server()  # pylint: disable=no-member
    endpoint_main = backend_main.start()

    data_model = DataModel(endpoint_main, messages_main, args.file_in, args.connect)
    solution_velocity_model = SolutionVelocityModel()
    tracking_signals_model = TrackingSignalsModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("solution_velocity_model", solution_velocity_model)
    root_context.setContextProperty("tracking_signals_model", tracking_signals_model)
    root_context.setContextProperty("data_model", data_model)

    threading.Thread(target=receive_messages, args=(app, backend_main, messages_main,), daemon=True).start()
    sys.exit(app.exec_())
