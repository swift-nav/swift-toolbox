"""Frontend module for the Swift Console.
"""
import sys
import threading

from typing import List, Optional, Tuple, Any

import capnp  # type: ignore

from fbs_runtime.application_context.PySide2 import ApplicationContext  # type: ignore

from PySide2.QtCore import QUrl, QObject, Slot, QPointF  # pylint:disable=no-name-in-module
from PySide2.QtCharts import QtCharts  # pylint:disable=no-name-in-module

from PySide2 import QtQml, QtCore  # pylint:disable=no-name-in-module

from PySide2.QtQml import qmlRegisterType  # pylint:disable=no-name-in-module
from PySide2.QtCore import Property  # pylint:disable=no-name-in-module

import console_resources  # type: ignore # pylint: disable=unused-import,import-error

import console_backend.server  # type: ignore  # pylint: disable=import-error

PIKSI_HOST = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com"
PIKSI_PORT = 55555

POINTS_V: List[QPointF] = []

POINTS_H_MINMAX: List[Optional[Tuple[float, float]]] = [None]
POINTS_H: List[QPointF] = []

capnp.remove_import_hook()  # pylint: disable=no-member


def receive_messages(backend, messages):
    while True:
        buffer = backend.fetch_message()
        m = messages.Message.from_bytes(buffer)
        if m.which == "status":
            print(f"status message: {m.status}")
        elif m.which == "velocityStatus":
            POINTS_H_MINMAX[0] = (m.velocityStatus.min, m.velocityStatus.max)
            POINTS_H[:] = [QPointF(point.x, point.y) for point in m.velocityStatus.points]
        else:
            print(f"other message: {m}")


class ConsolePoints(QObject):

    _valid: bool = False
    _points: List[QPointF] = []
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

    def get_points(self) -> List[QPointF]:
        """Getter for _points.
        """
        return self._points

    def set_points(self, points: List[QPointF]) -> None:
        """Setter for _points.
        """
        self._points = points

    points = Property("QVariantList", get_points, set_points)  # type: ignore

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

    @Slot(QtCharts.QXYSeries)  # type: ignore
    def fill_series(self, series):
        series.replace(self._points)


class DataModel(QObject):

    endpoint: console_backend.server.ServerEndpoint
    messages: Any

    def __init__(self, endpoint, messages):
        super().__init__()
        self.endpoint = endpoint
        self.messages = messages

    @Slot(ConsolePoints)  # type: ignore
    def fill_console_points(self, cp: ConsolePoints) -> ConsolePoints:  # pylint: disable=no-self-use
        if POINTS_H_MINMAX[0] is None:
            cp.set_valid(False)
            return cp
        cp.set_valid(True)
        cp.set_min(POINTS_H_MINMAX[0][0])  # pylint: disable=unsubscriptable-object
        cp.set_max(POINTS_H_MINMAX[0][1])  # pylint: disable=unsubscriptable-object
        cp.set_points(POINTS_H)
        return cp

    @Slot()  # type: ignore
    def connect(self) -> None:
        msg = self.messages.Message()
        msg.connectRequest.host = PIKSI_HOST
        msg.connectRequest.port = PIKSI_PORT
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)


if __name__ == "__main__":

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)

    ctx = ApplicationContext()

    qmlRegisterType(ConsolePoints, "SwiftConsole", 1, 0, "ConsolePoints")  # type: ignore
    engine = QtQml.QQmlApplicationEngine()

    qml_view = ctx.get_resource("view.qml")
    capnp_path = ctx.get_resource("console_backend.capnp")

    messages_main = capnp.load(capnp_path)  # pylint: disable=no-member

    backend_main = console_backend.server.Server()
    endpoint_main = backend_main.start()

    data_model = DataModel(endpoint_main, messages_main)

    engine.rootContext().setContextProperty("data_model", data_model)
    engine.load(QUrl.fromLocalFile(qml_view))

    threading.Thread(target=receive_messages, args=(backend_main, messages_main,), daemon=True).start()

    sys.exit(ctx.app.exec_())
