import argparse
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

POINTS_H_MINMAX: List[Optional[Tuple[float, float]]] = [None]
POINTS_H: List[QPointF] = []
POINTS_V: List[QPointF] = []
TRACKING_POINTS: List[List[QPointF]] = []
TRACKING_HEADERS: List[int] = []

capnp.remove_import_hook()


def receive_messages(backend, messages):
    while True:
        buffer = backend.fetch_message()
        m = messages.Message.from_bytes(buffer)
        if m.which == "status":
            # print(f"status message: {m.status}")
            pass
        elif m.which == "velocityStatus":
            POINTS_H_MINMAX[0] = (m.velocityStatus.min, m.velocityStatus.max)
            POINTS_H[:] = [
                QPointF(point.x, point.y) for point in m.velocityStatus.hpoints
            ]
            POINTS_V[:] = [
                QPointF(point.x, point.y) for point in m.velocityStatus.vpoints
            ]
        elif m.which == "trackingStatus":
            TRACKING_HEADERS[:] = m.trackingStatus.headers
            TRACKING_POINTS[:] = [
                [QPointF(point.x, point.y) for point in m.trackingStatus.data[idx]] for idx in range(len(m.trackingStatus.data))
            ]
        else:
            print(f"other message: {m}")

class ConsolePoints(QObject):

    _valid: bool = False
    _hpoints: List[QPointF] = []
    _vpoints: List[QPointF] = []
    _min: float = 0.0
    _max: float = 0.0

    def getValid(self) -> bool:
        return self._valid

    def setValid(self, valid) -> None:
        self._valid = valid

    valid = Property(bool, getValid, setValid)

    def getHPoints(self) -> List[QPointF]:
        return self._hpoints

    def setHPoints(self, hpoints) -> None:
        self._hpoints = hpoints
    
    def getVPoints(self) -> List[QPointF]:
        return self._vpoints

    def setVPoints(self, vpoints) -> None:
        self._vpoints = vpoints

    hpoints = Property('QVariantList', getHPoints, setHPoints) # type: ignore
    vpoints = Property('QVariantList', getVPoints, setVPoints) # type: ignore

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
    def fill_hseries(self, hseries):
        hseries.replace(self._hpoints)

    @Slot(QtCharts.QXYSeries) # type: ignore
    def fill_vseries(self, vseries):
        vseries.replace(self._vpoints)


class DataModel(QObject):

    endpoint: console_backend.server.ServerEndpoint
    messages: Any
    file_in: Any

    def __init__(self, endpoint, messages, file_in):
        super(DataModel, self).__init__()
        self.endpoint = endpoint
        self.messages = messages
        self.file_in = file_in
        self.is_connected = False

    @Slot(ConsolePoints) # type: ignore
    def fill_console_points(self, cp: ConsolePoints) -> ConsolePoints:
        if POINTS_H_MINMAX[0] is None:
            cp.setValid(False)
            return cp
        else:
            cp.setValid(True)
            cp.setMin(POINTS_H_MINMAX[0][0])
            cp.setMax(POINTS_H_MINMAX[0][1])
            # cp.setMin(min(POINTS_H))
            # cp.setMax(max(POINTS_H))

            #print(POINTS_H)
            cp.setHPoints(POINTS_H)
            cp.setVPoints(POINTS_V)
            return cp

    @Slot() # type: ignore
    def connect(self) -> None:
        if self.is_connected:
            print("One does not simply connect to Piksi twice.")
            return
        m = self.messages.Message()
        m.connectRequest.host = PIKSI_HOST
        m.connectRequest.port = PIKSI_PORT
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)
        self.is_connected = True

    @Slot() # type: ignore
    def readfile(self) -> None:
        print(self.file_in)
        if not self.file_in:
            print("No file passed into application.")
            return
        m = self.messages.Message()
        m.fileinRequest = m.init("fileinRequest")
        m.fileinRequest.filename = self.file_in
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

class TrackingSignalsPoints(QObject):

    _valid: bool = False
    _points: List[List[QPointF]] = [[]]
    _min: float = 0.0
    _max: float = 0.0

    def getValid(self) -> bool:
        return self._valid

    def setValid(self, valid) -> None:
        self._valid = valid

    valid = Property(bool, getValid, setValid)

    def getPoints(self) -> List[List[QPointF]]:
        return self._points

    def setPoints(self, points) -> None:
        self._points = points

    points = Property('QVariantList', getPoints, setPoints) # type: ignore

    @Slot(QtCharts.QXYSeries, int) # type: ignore
    def fill_series(self, series, idx):
        print(series.points)
        series.replace(self._points[idx])


class TrackingSignalsModel(QObject):

    @Slot(TrackingSignalsPoints) # type: ignore
    def fill_console_points(self, cp: TrackingSignalsPoints) -> TrackingSignalsPoints:
        # if POINTS_MINMAX[0] is None:
        #     cp.setValid(False)
        #     return cp
        # else:
        #     cp.setValid(True)
        # cp.setMin(POINTS_H_MINMAX[0][0])
        # cp.setMax(POINTS_H_MINMAX[0][1])
        # cp.setMin(min(POINTS_H))
        # cp.setMax(max(POINTS_H))
        cp.setPoints(TRACKING_POINTS)
        return cp


if __name__ == '__main__':

    parser = argparse.ArgumentParser()

    parser.add_argument('--file-in', help='Input file to parse.')

    args = parser.parse_args()
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)

    ctx = ApplicationContext()

    qmlRegisterType(ConsolePoints, "SwiftConsole", 1, 0, "ConsolePoints") # type: ignore
    qmlRegisterType(TrackingSignalsPoints, "SwiftConsole", 1, 0, "TrackingSignalsPoints") # type: ignore
    engine = QtQml.QQmlApplicationEngine()

    qml_view = ctx.get_resource('view.qml')
    capnp_path = ctx.get_resource('console_backend.capnp')

    messages = capnp.load(capnp_path)

    backend = console_backend.server.Server()
    endpoint = backend.start()
    data_model = DataModel(endpoint, messages, args.file_in)
    tracking_signals_model = TrackingSignalsModel()
    root_context = engine.rootContext()
    root_context.setContextProperty("tracking_signals_model", tracking_signals_model)
    root_context.setContextProperty("data_model", data_model)
    
    
    
    engine.load(QUrl.fromLocalFile(qml_view))

    threading.Thread(target=receive_messages, args=(backend, messages,), daemon=True).start()

    sys.exit(ctx.app.exec_())