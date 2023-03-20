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

"""Tracking SkyPlot Tab QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Slot, Signal, QPointF
from PySide6 import QtCharts

from .constants import Keys, QTKeys


def tracking_sky_plot_update() -> Dict[str, Any]:
    return {
        Keys.SATS: [],
        Keys.LABELS: [],
    }


TRACKING_SKY_PLOT_TAB: List[Dict[str, Any]] = [tracking_sky_plot_update()]


class TrackingSkyPlotPoints(QObject):
    _instance: "TrackingSkyPlotPoints"
    _labels: List[List[str]] = []
    _all_series: List[QtCharts.QXYSeries] = []
    _data_updated = Signal()
    labels_changed = Signal()
    all_series_changed = Signal()
    _tracking_sky_plot: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self._tracking_sky_plot = TRACKING_SKY_PLOT_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        TRACKING_SKY_PLOT_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self._tracking_sky_plot = TRACKING_SKY_PLOT_TAB[0]
        self.update()  # type: ignore

    def get_labels(self) -> List[List[str]]:
        return self._tracking_sky_plot[Keys.LABELS]

    labels = Property(list, get_labels, notify=labels_changed)  # type: ignore

    def get_all_series(self) -> List[QtCharts.QXYSeries]:
        return self._all_series

    all_series = Property(QTKeys.QVARIANTLIST, get_all_series, notify=all_series_changed)  # type: ignore

    @Slot(QtCharts.QAbstractSeries)  # type: ignore
    def addSeries(self, series) -> None:
        """Add a QML created series to the all_series list"""
        self._all_series.append(series)
        self.all_series_changed.emit()  # type: ignore

    @Slot()  # type: ignore
    def fill_all_series(self) -> None:
        series_changed = False
        for idx, sats in enumerate(self._tracking_sky_plot[Keys.SATS]):
            series = self._all_series[idx]
            if series.isVisible():
                series.clear()
                series.replace(list(map(lambda point: QPointF(point.az, point.el), sats)))
                series_changed = True

        if series_changed:
            self.all_series_changed.emit()  # type: ignore
