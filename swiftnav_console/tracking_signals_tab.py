"""Tracking Signals Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from .constants import Keys, QTKeys


TRACKING_SIGNALS_TAB: Dict[str, Any] = {
    Keys.POINTS: [],
    Keys.CHECK_LABELS: [],
    Keys.LABELS: [],
    Keys.COLORS: [],
    Keys.XMIN_OFFSET: 0,
}


class TrackingSignalsPoints(QObject):

    _colors: List[str] = []
    _check_labels: List[str] = []
    _labels: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _xmin_offset: float = 0.0

    def get_xmin_offset(self) -> float:
        """Getter for _xmin_offset."""
        return self._xmin_offset

    def set_xmin_offset(self, xmin_offset_: float) -> None:
        """Setter for _xmin_offset."""
        self._xmin_offset = xmin_offset_

    xmin_offset = Property(float, get_xmin_offset, set_xmin_offset)

    def get_check_labels(self) -> List[str]:
        return self._check_labels

    def set_check_labels(self, check_labels) -> None:
        self._check_labels = check_labels

    check_labels = Property(QTKeys.QVARIANTLIST, get_check_labels, set_check_labels)  # type: ignore

    def get_labels(self) -> List[str]:
        return self._labels

    def set_labels(self, labels) -> None:
        self._labels = labels

    labels = Property(QTKeys.QVARIANTLIST, get_labels, set_labels)  # type: ignore

    def get_colors(self) -> List[str]:
        return self._colors

    def set_colors(self, colors) -> None:
        self._colors = colors

    colors = Property(QTKeys.QVARIANTLIST, get_colors, set_colors)  # type: ignore

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series_and_key in enumerate(series_list):
            series, _ = series_and_key
            if idx < len(self._points):
                series.replace(self._points[idx])


class TrackingSignalsModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(TrackingSignalsPoints)  # type: ignore
    def fill_console_points(self, cp: TrackingSignalsPoints) -> TrackingSignalsPoints:  # pylint:disable=no-self-use
        cp.set_points(TRACKING_SIGNALS_TAB[Keys.POINTS])
        cp.set_labels(TRACKING_SIGNALS_TAB[Keys.LABELS])
        cp.set_check_labels(TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS])
        cp.set_colors(TRACKING_SIGNALS_TAB[Keys.COLORS])
        cp.set_xmin_offset(TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET])
        return cp
