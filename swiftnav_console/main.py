"""Frontend module for the Swift Console.
"""
import argparse
import os
import sys
import threading

from typing import List, Any, Optional, Tuple

from PySide2.QtWidgets import QApplication  # type: ignore

from PySide2.QtCore import QObject, QUrl, QPointF, Slot
from PySide2.QtCharts import QtCharts  # pylint: disable=unused-import

from PySide2 import QtQml, QtCore

from PySide2.QtGui import QFontDatabase, QIcon

from PySide2.QtQml import QQmlComponent, qmlRegisterType

import swiftnav_console.console_resources  # type: ignore # pylint: disable=unused-import

from .constants import ApplicationMetadata, ConnectionState, ConnectionType, Keys, Tabs, QTKeys

from .tracking_signals_tab import (
    TrackingSignalsPoints,
    TRACKING_SIGNALS_TAB,
)

import time


def receive_messages():
    start = time.time()
    while True:
        t = time.time() - start
        TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS][:] = []
        TRACKING_SIGNALS_TAB[Keys.LABELS][:] = [str(x) for x in range(40)]
        TRACKING_SIGNALS_TAB[Keys.COLORS][:] = ["blue", "red", "green", "orange", "purple"] * 8
        TRACKING_SIGNALS_TAB[Keys.POINTS][:] = [
            [QPointF(t - x / 2, y) for (x, y) in zip(reversed(range(200)), [k] * 200)] for k in range(20, 60)
        ]
        TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET] = -95
        time.sleep(0.2)


def main(passed_args: Optional[Tuple[str, ...]] = None) -> int:
    parser = argparse.ArgumentParser(add_help=False, usage=argparse.SUPPRESS)

    args_main, _ = parser.parse_known_args()
    if passed_args is not None:
        args_main, _ = parser.parse_known_args(passed_args)

    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_EnableHighDpiScaling)
    QtCore.QCoreApplication.setAttribute(QtCore.Qt.AA_UseHighDpiPixmaps)
    app = QApplication(sys.argv)
    app.setWindowIcon(QIcon(":/images/icon.ico"))
    app.setOrganizationName(ApplicationMetadata.ORGANIZATION_NAME)
    app.setOrganizationDomain(ApplicationMetadata.ORGANIZATION_DOMAIN)
    app.setApplicationName(ApplicationMetadata.APPLICATION_NAME)
    qmlRegisterType(TrackingSignalsPoints, "SwiftConsole", 1, 0, "TrackingSignalsPoints")  # type: ignore

    engine = QtQml.QQmlApplicationEngine()
    qml_object_created = [False]

    def handle_qml_load_errors(obj, _url):
        qml_object_created[0] = obj is not None

    engine.objectCreated.connect(handle_qml_load_errors)  # pylint: disable=no-member

    engine.addImportPath("PySide2")
    engine.addImportPath(":/")
    engine.load(QUrl("qrc:/view.qml"))
    if not qml_object_created[0]:
        return 1

    server_thread = threading.Thread(
        target=receive_messages,
        args=(),
        daemon=True,
    )

    server_thread.start()
    app.exec_()

    return 0


if __name__ == "__main__":
    sys.exit(main())
