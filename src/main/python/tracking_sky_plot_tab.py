"""Tracking SkyPlot Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys


TRACKING_SKY_PLOT_TAB: Dict[str, Any] = {
    Keys.SATS: [],
    Keys.LABELS: [],
}


class TrackingSkyPlotPoints(QObject):

    _sats: List[List[QPointF]] = []
    _labels: List[List[str]] = []

    def get_sats(self) -> List[List[QPointF]]:
        return self._sats

    def set_sats(self, sats) -> None:
        self._sats = sats

    sats = Property(QTKeys.QVARIANTLIST, get_sats, set_sats)  # type: ignore

    def get_labels(self) -> List[List[str]]:
        return self._labels

    def set_labels(self, labels) -> None:
        self._labels = labels

    labels = Property(list, get_labels, set_labels)

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._sats[idx])


class TrackingSkyPlotModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(TrackingSkyPlotPoints)  # type: ignore
    def fill_console_points(self, cp: TrackingSkyPlotPoints) -> TrackingSkyPlotPoints:  # pylint:disable=no-self-use
        cp.set_sats(TRACKING_SKY_PLOT_TAB[Keys.SATS])
        cp.set_labels(TRACKING_SKY_PLOT_TAB[Keys.LABELS])
        return cp
