"""Tracking Signals Tab QObjects.
"""

from threading import Lock
from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Signal, Slot
from PySide2.QtCharts import QtCharts

from .constants import Keys, QTKeys


TRACKING_SIGNALS_TAB: Dict[str, Any] = {
    Keys.POINTS: [],
    Keys.CHECK_LABELS: [],
    Keys.LABELS: [],
    Keys.COLORS: [],
    Keys.XMIN_OFFSET: 0,
}
TRACKING_SIGNALS_TAB_LOCK: Lock = Lock()


class TrackingSignalsPoints(QObject):

    _num_labels: int = 0
    _xaxis_min: float = 0.0
    _xaxis_max: float = 0.0
    _check_labels: List[str] = []
    _all_series: List[QtCharts.QXYSeries] = []
    _enabled_series: List[QtCharts.QXYSeries] = []
    num_labels_changed = Signal(int, arguments="num_labels")
    xaxis_min_changed = Signal()
    xaxis_max_changed = Signal()
    check_labels_changed = Signal()
    all_series_changed = Signal()
    enabled_series_changed = Signal()

    def get_num_labels(self) -> int:  # pylint:disable=no-self-use
        with TRACKING_SIGNALS_TAB_LOCK:
            return len(TRACKING_SIGNALS_TAB[Keys.LABELS])

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
    def getLabel(self, index) -> str:  # pylint:disable=no-self-use
        """Getter for one of the TRACKING_SIGNALS_TAB[Keys.LABELS]."""
        return TRACKING_SIGNALS_TAB[Keys.LABELS][index]

    @Slot(QtCharts.QAbstractSeries)  # type: ignore
    def addSeries(self, series) -> None:
        """Add a QML created series to the all_series list"""
        self._all_series.append(series)
        self.all_series_changed.emit()  # type: ignore

    @Slot()  # type: ignore
    def fill_all_series(self) -> None:
        with TRACKING_SIGNALS_TAB_LOCK:
            cur_num_labels = len(TRACKING_SIGNALS_TAB[Keys.LABELS])
            labels = TRACKING_SIGNALS_TAB[Keys.LABELS]
            colors = TRACKING_SIGNALS_TAB[Keys.COLORS]
            points_for_all_series = TRACKING_SIGNALS_TAB[Keys.POINTS]
            check_labels = TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS]
            xmin_offset = TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET]
        if self._num_labels != cur_num_labels:
            self._num_labels = cur_num_labels
            self.num_labels_changed.emit(cur_num_labels)  # type: ignore

        if self._check_labels != check_labels:
            self._check_labels = check_labels
            self.check_labels_changed.emit()  # type: ignore

        if len(points_for_all_series) != 0:
            xaxis_min = points_for_all_series[0][-1].x() + xmin_offset
            if self._xaxis_min != xaxis_min:
                self._xaxis_min = xaxis_min
                self.xaxis_min_changed.emit()  # type: ignore
            xaxis_max = points_for_all_series[0][-1].x()
            if self._xaxis_max != xaxis_max:
                self._xaxis_max = xaxis_max
                self.xaxis_max_changed.emit()  # type: ignore

        series_changed = False
        enabled_series = []
        for idx, series_points in enumerate(points_for_all_series):
            series = None
            try:
                series = self._all_series[idx]
                series.clear()
                series.replace(series_points)
                series.setName(labels[idx])
                series.setColor(colors[idx])
                series_changed = True

                if len(series_points) > 0:
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
