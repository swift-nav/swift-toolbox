"""Frontend module for the Swift Console.
"""
import argparse
import os
import sys
import threading

from typing import Dict, List, Optional, Tuple, Any

import capnp  # type: ignore

from PySide2.QtWidgets import QApplication  # type: ignore  # pylint: disable=no-name-in-module

from fbs_runtime.application_context.PySide2 import ApplicationContext  # type: ignore  # pylint: disable=unused-import

from PySide2.QtCore import QUrl, QObject, Slot, QPointF  # pylint:disable=no-name-in-module
from PySide2.QtCharts import QtCharts  # pylint:disable=no-name-in-module

from PySide2 import QtQml, QtCore  # pylint:disable=no-name-in-module

from PySide2.QtQml import qmlRegisterType  # pylint:disable=no-name-in-module
from PySide2.QtCore import Property  # pylint:disable=no-name-in-module

import console_resources  # type: ignore # pylint: disable=unused-import,import-error

import console_backend.server  # type: ignore  # pylint: disable=import-error,no-name-in-module

CLOSE = "CLOSE"
DARWIN = "darwin"

CONSOLE_BACKEND_CAPNP_PATH = "console_backend.capnp"

PIKSI_HOST = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com"
PIKSI_PORT = 55555

POINTS_H_MINMAX: List[Optional[Tuple[float, float]]] = [None]  # pylint: disable=unsubscriptable-object
POINTS_H: List[QPointF] = []
POINTS_V: List[QPointF] = []
TRACKING_POINTS: List[List[QPointF]] = []
TRACKING_HEADERS: List[int] = []

POINTS = "POINTS"
LABELS = "LABELS"
COLORS = "COLORS"
MAX = "MAX"
MIN = "MIN"

TRACKING_SIGNALS_TAB: Dict[str, Any] = {
    POINTS: [],
    LABELS: [],
    COLORS: [],
    MAX: 0,
    MIN: 0,
}

capnp.remove_import_hook()  # pylint: disable=no-member


def receive_messages(app_, backend, messages):
    while True:
        buffer = backend.fetch_message()
        m = messages.Message.from_bytes(buffer)
        if m.which == "status":
            if m.status.text == CLOSE:
                return app_.quit()
        elif m.which == "velocityStatus":
            POINTS_H_MINMAX[0] = (m.velocityStatus.min, m.velocityStatus.max)
            POINTS_H[:] = [QPointF(point.x, point.y) for point in m.velocityStatus.hpoints]
            POINTS_V[:] = [QPointF(point.x, point.y) for point in m.velocityStatus.vpoints]
        elif m.which == "trackingStatus":
            TRACKING_SIGNALS_TAB[LABELS][:] = m.trackingStatus.labels
            TRACKING_SIGNALS_TAB[COLORS][:] = m.trackingStatus.colors
            TRACKING_SIGNALS_TAB[POINTS][:] = [
                [QPointF(point.x, point.y) for point in m.trackingStatus.data[idx]]
                for idx in range(len(m.trackingStatus.data))
            ]
            TRACKING_SIGNALS_TAB[MAX] = m.trackingStatus.max
            TRACKING_SIGNALS_TAB[MIN] = m.trackingStatus.min
        else:
            pass
            # print(f"other message: {m}")



class ConsolePoints(QObject):

    _hpoints: List[QPointF] = []
    _vpoints: List[QPointF] = []
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

    def get_hpoints(self) -> List[QPointF]:
        return self._hpoints

    def set_hpoints(self, hpoints) -> None:
        self._hpoints = hpoints

    def get_vpoints(self) -> List[QPointF]:
        return self._vpoints

    def set_vpoints(self, vpoints) -> None:
        self._vpoints = vpoints

    hpoints = Property("QVariantList", get_hpoints, set_hpoints)  # type: ignore
    vpoints = Property("QVariantList", get_vpoints, set_vpoints)  # type: ignore

    @Slot(QtCharts.QXYSeries)  # type: ignore
    def fill_hseries(self, hseries):
        hseries.replace(self._hpoints)

    @Slot(QtCharts.QXYSeries)  # type: ignore
    def fill_vseries(self, vseries):
        vseries.replace(self._vpoints)


class DataModel(QObject):

    endpoint: console_backend.server.ServerEndpoint  # pylint: disable=no-member
    messages: Any

    def __init__(self, endpoint, messages, file_in, connect=False):
        super().__init__()
        self.endpoint = endpoint
        self.messages = messages
        self.file_in = file_in
        self.is_connected = False
        if connect and file_in is not None:
            self.readfile()
        elif connect:
            self.connect()

    @Slot(ConsolePoints)  # type: ignore
    def fill_console_points(self, cp: ConsolePoints) -> ConsolePoints:  # pylint: disable=no-self-use
        if POINTS_H_MINMAX[0] is None:
            cp.set_valid(False)
            return cp
        cp.set_valid(True)
        cp.set_min(POINTS_H_MINMAX[0][0])  # pylint: disable=unsubscriptable-object
        cp.set_max(POINTS_H_MINMAX[0][1])  # pylint: disable=unsubscriptable-object
        cp.set_hpoints(POINTS_H)
        cp.set_vpoints(POINTS_V)
        return cp

    @Slot()  # type: ignore
    def connect(self) -> None:
        if self.is_connected:
            print("Already connected.")
            return
        msg = self.messages.Message()
        msg.connectRequest.host = PIKSI_HOST
        msg.connectRequest.port = PIKSI_PORT
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)
        self.is_connected = True

    @Slot()  # type: ignore
    def readfile(self) -> None:
        if not self.file_in:
            print("No file passed into application.")
            return
        m = self.messages.Message()
        m.fileinRequest = m.init("fileinRequest")
        m.fileinRequest.filename = self.file_in
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)


class TrackingSignalsPoints(QObject):

    _colors: List[str] = []
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

    def get_labels(self) -> List[str]:
        return self._labels

    def set_labels(self, labels) -> None:
        self._labels = labels

    labels = Property("QVariantList", get_labels, set_labels)  # type: ignore

    def get_colors(self) -> List[str]:
        return self._colors

    def set_colors(self, colors) -> None:
        self._colors = colors

    colors = Property("QVariantList", get_colors, set_colors)  # type: ignore

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = points

    points = Property("QVariantList", get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series_and_key in enumerate(series_list):
            series, _ = series_and_key
            series.replace(self._points[idx])


class TrackingSignalsModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(TrackingSignalsPoints)  # type: ignore
    def fill_console_points(self, cp: TrackingSignalsPoints) -> TrackingSignalsPoints:  # pylint:disable=no-self-use
        cp.set_points(TRACKING_SIGNALS_TAB[POINTS])
        cp.set_labels(TRACKING_SIGNALS_TAB[LABELS])
        cp.set_colors(TRACKING_SIGNALS_TAB[COLORS])
        cp.set_max(TRACKING_SIGNALS_TAB[MAX])
        cp.set_min(TRACKING_SIGNALS_TAB[MIN])
        return cp


def get_capnp_path() -> str:
    """Get the path to the capnp file based on current installer.

    Returns:
        str: The path to the capnp file.
    """

    d = os.path.dirname(sys.executable)
    path = ""
    if getattr(sys, "frozen", False) or sys.platform == DARWIN:
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

    qmlRegisterType(ConsolePoints, "SwiftConsole", 1, 0, "ConsolePoints")  # type: ignore
    qmlRegisterType(TrackingSignalsPoints, "SwiftConsole", 1, 0, "TrackingSignalsPoints")  # type: ignore
    engine = QtQml.QQmlApplicationEngine()

    capnp_path = get_capnp_path()

    engine.addImportPath("PySide2")

    engine.load(QUrl("qrc:/view.qml"))

    messages_main = capnp.load(capnp_path)  # pylint: disable=no-member

    backend_main = console_backend.server.Server()  # pylint: disable=no-member
    endpoint_main = backend_main.start()

    data_model = DataModel(endpoint_main, messages_main, args.file_in, args.connect)
    tracking_signals_model = TrackingSignalsModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("tracking_signals_model", tracking_signals_model)
    root_context.setContextProperty("data_model", data_model)

    threading.Thread(target=receive_messages, args=(app, backend_main, messages_main,), daemon=True).start()
    sys.exit(app.exec_())
