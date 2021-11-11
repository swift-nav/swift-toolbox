"""Tracking SkyPlot Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot, Signal
from PySide2.QtCharts import QtCharts

from .constants import Keys, QTKeys


TRACKING_SKY_PLOT_TAB: Dict[str, Any] = {
    Keys.SATS: [],
    Keys.LABELS: [],
}


class TrackingSkyPlotPoints(QObject):

    _labels: List[List[str]] = []
    _all_series: List[QtCharts.QXYSeries] = []
    labels_changed = Signal()
    all_series_changed = Signal()

    def get_labels(self) -> List[List[str]]:  # pylint:disable=no-self-use
        return TRACKING_SKY_PLOT_TAB[Keys.LABELS]

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
        for idx, series_points in enumerate(TRACKING_SKY_PLOT_TAB[Keys.SATS]):
            series = self._all_series[idx]
            if series.isVisible():
                series.clear()
                series.replace(series_points)
                series_changed = True

        if series_changed:
            self.all_series_changed.emit()  # type: ignore
