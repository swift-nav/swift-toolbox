"""Tracking Signals Tab QObjects.
"""

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


class TrackingSignalsPoints(QObject):

    _num_labels: int = 0
    _xaxis_min: float = 0.0
    _xaxis_max: float = 0.0
    _check_labels: List[str] = []
    _all_series: List[QtCharts.QXYSeries] = []
    num_labels_changed = Signal(int, arguments="num_labels")
    xaxis_min_changed = Signal()
    xaxis_max_changed = Signal()
    check_labels_changed = Signal()
    all_series_changed = Signal()

    def get_num_labels(self) -> int:  # pylint:disable=no-self-use
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

    @Slot(int)  # type: ignore
    def getLabel(self, index) -> str:  # pylint:disable=no-self-use
        """Getter for one of the TRACKING_SIGNALS_TAB[Keys.LABELS]."""
        return TRACKING_SIGNALS_TAB[Keys.LABELS][index]

    @Slot(QtCharts.QAbstractSeries)  # type: ignore
    def addSeries(self, series) -> None:
        """Add a QML created series to the all_series list"""
        self._all_series.append(series)
        self.all_series_changed.emit()  # type: ignore

    @Slot(float, bool)  # type: ignore
    def fill_all_series(self, line_width, useOpenGL) -> None:
        cur_num_labels = len(TRACKING_SIGNALS_TAB[Keys.LABELS])
        if self._num_labels != cur_num_labels:
            self._num_labels = cur_num_labels
            self.num_labels_changed.emit(cur_num_labels)  # type: ignore
        points_for_all_series = TRACKING_SIGNALS_TAB[Keys.POINTS]
        if len(points_for_all_series) == 0:
            return

        labels = TRACKING_SIGNALS_TAB[Keys.LABELS]
        colors = TRACKING_SIGNALS_TAB[Keys.COLORS]
        self._check_labels = TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS]
        self.check_labels_changed.emit()  # type: ignore
        self._xaxis_min = points_for_all_series[0][-1].x() + TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET]
        self.xaxis_min_changed.emit()  # type: ignore
        self._xaxis_max = points_for_all_series[0][-1].x()
        self.xaxis_max_changed.emit()  # type: ignore
        for idx, series_points in enumerate(points_for_all_series):
            series = None
            try:
                series = self._all_series[idx]
                series.replace(series_points)
                series.setName(labels[idx])
                series.setColor(colors[idx])
                pen = series.pen()
                pen.setWidthF(line_width)
                series.setPen(pen)
                series.setUseOpenGL(useOpenGL)
                self.all_series_changed.emit()  # type: ignore
            except IndexError:
                # This is ok - QML will create these series, and call addSeries, and these will be
                # updated in the next timer fire/update.
                pass
        return


class TrackingSignalsModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(TrackingSignalsPoints)  # type: ignore
    def fill_console_points(self, cp: TrackingSignalsPoints) -> TrackingSignalsPoints:  # pylint:disable=no-self-use
        cp.fill_all_series(TRACKING_SIGNALS_TAB[Keys.POINTS])
        cp.set_labels(TRACKING_SIGNALS_TAB[Keys.LABELS])
        cp.set_check_labels(TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS])
        cp.set_colors(TRACKING_SIGNALS_TAB[Keys.COLORS])
        cp.set_xmin_offset(TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET])
        return cp
