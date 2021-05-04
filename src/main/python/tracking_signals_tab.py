"""Tracking Signals Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys


TRACKING_SIGNALS_TAB: Dict[str, Any] = {
    Keys.POINTS: [],
    Keys.CHECK_LABELS: [],
    Keys.LABELS: [],
    Keys.COLORS: [],
    Keys.MAX: 0,
    Keys.MIN: 0,
}


class TrackingSignalsPoints(QObject):

    _colors: List[str] = []
    _check_labels: List[str] = []
    _labels: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _valid: bool = False
    _min: float = 0.0
    _max: float = 0.0

    def get_valid(self) -> bool:
        """Getter for _valid.

        Returns:
            bool: Whether it is valid or not.
        """
        return self._valid

    def set_valid(self, valid: bool) -> None:
        """Setter for _valid.
        """
        self._valid = valid

    valid = Property(bool, get_valid, set_valid)

    def get_min(self) -> float:
        """Getter for _min.
        """
        return self._min

    def set_min(self, min_: float) -> None:
        """Setter for _min.
        """
        self._min = min_

    min_ = Property(float, get_min, set_min)

    def get_max(self) -> float:
        """Getter for _max.
        """
        return self._max

    def set_max(self, max_: float) -> None:
        """Setter for _max.
        """
        self._max = max_

    max_ = Property(float, get_max, set_max)

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
        cp.set_max(TRACKING_SIGNALS_TAB[Keys.MAX])
        cp.set_min(TRACKING_SIGNALS_TAB[Keys.MIN])
        return cp
