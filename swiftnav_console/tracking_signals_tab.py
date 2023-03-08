# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

"""Tracking Signals Tab QObjects.
"""

from typing import Dict, List, Any, Optional

from PySide6.QtCore import Property, QObject, QPointF, Signal, Slot
from PySide6 import QtCharts
from PySide6.QtQml import QmlElement

from .constants import Keys, QTKeys

QML_IMPORT_NAME = "SwiftConsole"
QML_IMPORT_MAJOR_VERSION = 1


def tracking_signals_tab_update() -> Dict[str, Any]:
    return {
        Keys.POINTS: [],
        Keys.CHECK_LABELS: [],
        Keys.LABELS: [],
        Keys.COLORS: [],
        Keys.XMIN_OFFSET: 0,
    }


TRACKING_SIGNALS_TAB: List[Dict[str, Any]] = [tracking_signals_tab_update()]


# pylint:disable=too-many-instance-attributes
@QmlElement
class TrackingSignalsPoints(QObject):
    _instance: "TrackingSignalsPoints"
    _num_labels: int = 0
    _xaxis_min: float = 0.0
    _xaxis_max: float = 0.0
    _check_labels: List[str] = []
    _empty_series: Optional[QtCharts.QXYSeries] = None
    _all_series: List[QtCharts.QXYSeries] = []
    _enabled_series: List[QtCharts.QXYSeries] = []
    num_labels_changed = Signal(int, arguments="num_labels")
    xaxis_min_changed = Signal()
    xaxis_max_changed = Signal()
    check_labels_changed = Signal()
    all_series_changed = Signal()
    enabled_series_changed = Signal()
    data_updated = Signal()
    _tracking_signals_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self._tracking_signals_tab = TRACKING_SIGNALS_TAB[0]
        self.data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        TRACKING_SIGNALS_TAB[0] = update_data
        cls._instance.data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self._tracking_signals_tab = TRACKING_SIGNALS_TAB[0]

    def get_num_labels(self) -> int:
        return len(self._tracking_signals_tab[Keys.LABELS])

    num_labels = Property(int, get_num_labels, notify=num_labels_changed)  # type: ignore

    def get_xaxis_min(self) -> float:
        """Getter for _xaxis_min."""
        return self._xaxis_min

    xaxis_min = Property(float, get_xaxis_min, notify=xaxis_min_changed)  # type: ignore

    def get_xaxis_max(self) -> float:
        """Getter for _xaxis_max."""
        return self._xaxis_max

    xaxis_max = Property(float, get_xaxis_max, notify=xaxis_max_changed)  # type: ignore

    def get_check_labels(self) -> List[str]:
        return self._check_labels

    check_labels = Property(QTKeys.QVARIANTLIST, get_check_labels, notify=check_labels_changed)  # type: ignore

    def get_all_series(self) -> List[QtCharts.QXYSeries]:
        return self._all_series

    all_series = Property(QTKeys.QVARIANTLIST, get_all_series, notify=all_series_changed)  # type: ignore

    def get_enabled_series(self) -> List[QtCharts.QXYSeries]:
        return self._enabled_series

    enabled_series = Property(QTKeys.QVARIANTLIST, get_enabled_series, notify=enabled_series_changed)  # type: ignore

    @Slot(int)  # type: ignore
    def getLabel(self, index) -> str:
        """Getter for one of the TRACKING_SIGNALS_TAB[Keys.LABELS]."""
        return self._tracking_signals_tab[Keys.LABELS][index]

    @Slot(QtCharts.QAbstractSeries)  # type: ignore
    def addSeries(self, series) -> None:
        """Add a QML created series to the all_series list"""
        self._all_series.append(series)
        self.all_series_changed.emit()  # type: ignore

    @Slot()  # type: ignore
    def fill_all_series(self) -> None:
        cur_num_labels = len(self._tracking_signals_tab[Keys.LABELS])
        if self._num_labels != cur_num_labels:
            self._num_labels = cur_num_labels
            self.num_labels_changed.emit(cur_num_labels)  # type: ignore

        all_points = [
            [QPointF(point.x, point.y) for point in points] for points in self._tracking_signals_tab[Keys.POINTS]
        ]

        labels = self._tracking_signals_tab[Keys.LABELS]
        colors = self._tracking_signals_tab[Keys.COLORS]
        if self._check_labels != self._tracking_signals_tab[Keys.CHECK_LABELS]:
            self._check_labels = self._tracking_signals_tab[Keys.CHECK_LABELS]
            self.check_labels_changed.emit()  # type: ignore

        if len(all_points) != 0:
            xaxis_min = all_points[0][-1].x() + self._tracking_signals_tab[Keys.XMIN_OFFSET]
            if self._xaxis_min != xaxis_min:
                self._xaxis_min = xaxis_min
                self.xaxis_min_changed.emit()  # type: ignore
            xaxis_max = all_points[0][-1].x()
            if self._xaxis_max != xaxis_max:
                self._xaxis_max = xaxis_max
                self.xaxis_max_changed.emit()  # type: ignore

        series_changed = False
        enabled_series = []

        for idx in range(len(all_points) - 1):
            series_points = all_points[idx]
            try:
                if len(series_points) > 0:
                    series = self._all_series[idx]
                    series.clear()
                    series.replace(series_points)
                    series.setName(labels[idx])
                    series.setColor(colors[idx])
                    series_changed = True
                    enabled_series.append(series)
            except IndexError:
                # This is ok - QML will create these series, and call addSeries, and these will be
                # updated in the next timer fire/update.
                pass

        disabled_seriess = set(self._all_series) - set(enabled_series)
        for disabled_series in disabled_seriess:
            if disabled_series.count() > 0:
                series_changed = True
                disabled_series.clear()

        if series_changed:
            self.all_series_changed.emit()  # type: ignore

        if enabled_series != self._enabled_series:
            self._enabled_series = enabled_series
            self.enabled_series_changed.emit()  # type: ignore
