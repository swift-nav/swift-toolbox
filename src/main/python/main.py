import io
import math
import os
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

from PySide2 import QtQml, QtCore

from PySide2.QtQml import qmlRegisterType, QQmlListReference
from PySide2.QtCore import Property

"""
from sbp.client.drivers.network_drivers import TCPDriver
from sbp.client import Handler, Framer

from sbp.navigation import SBP_MSG_VEL_NED
"""

from typing import List, Optional, Tuple, Any

import console_backend.server

PIKSI_HOST = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com"
PIKSI_PORT = 55555

POINTS_V: List[QPointF] = []

POINTS_H_MINMAX: List[Optional[Tuple[float, float]]] = [None]
POINTS_H: List[QPointF] = []

capnp.remove_import_hook()


def receive_messages(backend, messages):
    while True:
        buffer = backend.fetch_message()
        m = messages.Message.from_bytes(buffer)
        if m.which == "status":
            print(f"status message: {m.status}")
        elif m.which == "velocityStatus":
            POINTS_H_MINMAX[0] = (m.velocityStatus.min, m.velocityStatus.max)
            POINTS_H[:] = [
                QPointF(point.x, point.y) for point in m.velocityStatus.points
            ]
        else:
            print(f"other message: {m}")


"""
def sbp_main():
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
                    POINTS_H_MINMAX[0] = (-abs(v_vel) * 1.5, abs(v_vel) * 1.5)
                else:
                    POINTS_H_MINMAX[0] = (min(X.y() for X in POINTS_H), max(X.y() for X in POINTS_H))
"""


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

    endpoint: console_backend.server.ServerEndpoint
    messages: Any

    def __init__(self, endpoint, messages):
        super(DataModel, self).__init__()
        self.endpoint = endpoint
        self.messages = messages

    @Slot(ConsolePoints) # type: ignore
    def fill_console_points(self, cp: ConsolePoints) -> ConsolePoints:
        if POINTS_H_MINMAX[0] is None:
            cp.setValid(False)
            return cp
        else:
            cp.setValid(True)
            cp.setMin(POINTS_H_MINMAX[0][0])
            cp.setMax(POINTS_H_MINMAX[0][1])
            #print(POINTS_H)
            cp.setPoints(POINTS_H)
            return cp

    @Slot() # type: ignore
    def connect(self) -> None:
        m = self.messages.Message()
        m.connectRequest.host = PIKSI_HOST
        m.connectRequest.port = PIKSI_PORT
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)


if __name__ == '__main__':

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)

    ctx = ApplicationContext()

    qmlRegisterType(ConsolePoints, "SwiftConsole", 1, 0, "ConsolePoints") # type: ignore
    engine = QtQml.QQmlApplicationEngine()

    qml_view = ctx.get_resource('view.qml')
    capnp_path = ctx.get_resource('console_backend.capnp')

    messages = capnp.load(capnp_path)

    backend = console_backend.server.Server()
    endpoint = backend.start()

    data_model = DataModel(endpoint, messages)

    engine.rootContext().setContextProperty("data_model", data_model)
    engine.load(QUrl.fromLocalFile(qml_view))

    threading.Thread(target=receive_messages, args=(backend, messages,), daemon=True).start()
    #threading.Thread(target=sbp_main, daemon=True).start()

    sys.exit(ctx.app.exec_())