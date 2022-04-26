"""Tracking SkyPlot Tab QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Slot, Signal
from PySide6 import QtCharts

from .constants import Keys, QTKeys


def tracking_sky_plot_update() -> Dict[str, Any]:
    return {
        Keys.SATS: [],
        Keys.LABELS: [],
    }


TRACKING_SKY_PLOT_TAB: List[Dict[str, Any]] = [tracking_sky_plot_update()]


class TrackingSkyPlotPoints(QObject):

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
        cls._instance._data_updated.emit()

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self._tracking_sky_plot = TRACKING_SKY_PLOT_TAB[0]

    def get_labels(self) -> List[List[str]]:  # pylint:disable=no-self-use
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
        for idx, series_points in enumerate(self._tracking_sky_plot[Keys.SATS]):
            series = self._all_series[idx]
            if series.isVisible():
                series.clear()
                series.replace(series_points)
                series_changed = True

        if series_changed:
            self.all_series_changed.emit()  # type: ignore
