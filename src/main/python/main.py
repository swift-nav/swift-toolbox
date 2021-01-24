import io
import math
import socket
import sys
import threading
import time

import capnp

from fbs_runtime.application_context.PySide2 import ApplicationContext

from PySide2.QtQuick import QQuickView
from PySide2.QtCore import (Qt, QUrl, QObject, Slot, QPointF)
from PySide2.QtGui import QGuiApplication
from PySide2.QtCharts import QtCharts

from PySide2 import QtQml
from PySide2.QtQml import qmlRegisterType, QQmlListReference
from PySide2.QtCore import Property

from sbp.client.drivers.network_drivers import TCPDriver
from sbp.client import Handler, Framer

from sbp.navigation import SBP_MSG_VEL_NED

from typing import List, Optional, Tuple

from console_backend.server import Server


POINTS_V: List[QPointF] = []

POINTS_H_MINMAX: Optional[Tuple[float, float]] = None
POINTS_H: List[QPointF] = []

capnp.remove_import_hook()


def receive_messages(backend_messages):
    backend_server = Server()
    backend_server.start()
    while True:
        buffer = backend_server.fetch_message()
        m = backend_messages.Message.from_bytes(buffer)
        print(m)


def sbp_main():
    global POINTS_H_MINMAX
    host, port = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com", 55555
    with TCPDriver(host, port) as driver:
        with Handler(Framer(driver.read, None)) as source:
            for msg, _ in source.filter(SBP_MSG_VEL_NED):
                h_vel = math.sqrt(msg.n**2 + msg.e**2) / 1000.0
                v_vel = -msg.d / 1000.0
                if len(POINTS_H) == 200:
                    POINTS_H.pop(0)
                POINTS_H.append(QPointF(msg.tow / 1000.0, h_vel))
                if POINTS_H_MINMAX is None:
                    POINTS_H_MINMAX = (-abs(v_vel) * 1.5, abs(v_vel) * 1.5)
                else:
                    POINTS_H_MINMAX = (min(X.y() for X in POINTS_H), max(X.y() for X in POINTS_H))


class ConsolePoints(QObject):

    _valid: bool = False
    _points: List[QPointF] = []
    _min: float = 0.0
    _max: float = 0.0

    def getValid(self) -> bool:
        return self._valid

    def setValid(self, valid) -> None:
        self._valid = valid

    valid = Property(bool, getValid, setValid)

    def getPoints(self) -> List[QPointF]:
        return self._points

    def setPoints(self, points) -> None:
        self._points = points

    points = Property('QVariantList', getPoints, setPoints) # type: ignore

    def getMin(self) -> float:
        return self._min

    def setMin(self, min: float) -> None:
        self._min = min

    min = Property(float, getMin, setMin)

    def getMax(self) -> float:
        return self._max

    def setMax(self, max: float) -> None:
        self._max = max

    max = Property(float, getMax, setMax)

    @Slot(QtCharts.QXYSeries) # type: ignore
    def fill_series(self, series):
        series.replace(self._points)


class DataModel(QObject):

    points: List[QPointF]

    @Slot(ConsolePoints) # type: ignore
    def fill_console_points(self, cp):
        if POINTS_H_MINMAX is None:
            cp.setValid(False)
            return cp
        else:
            cp.setValid(True)
            cp.setMin(POINTS_H_MINMAX[0])
            cp.setMax(POINTS_H_MINMAX[1])
            cp.setPoints(POINTS_H)
            return cp
        

if __name__ == '__main__':

    ctx = ApplicationContext()

    qmlRegisterType(ConsolePoints, "SwiftConsole", 1, 0, "ConsolePoints") # type: ignore
    engine = QtQml.QQmlApplicationEngine()

    qml_view = ctx.get_resource('view.qml')
    capnp_path = ctx.get_resource('console_backend.capnp')

    backend_messages = capnp.load(capnp_path)

    data_model = DataModel()

    engine.rootContext().setContextProperty("data_model", data_model)
    engine.load(QUrl.fromLocalFile(qml_view))

    threading.Thread(target=receive_messages, args=(backend_messages,), daemon=True).start()
    threading.Thread(target=sbp_main, daemon=True).start()

    sys.exit(ctx.app.exec_())