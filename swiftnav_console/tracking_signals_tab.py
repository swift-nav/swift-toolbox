"""Tracking Signals Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot
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

    _check_labels: List[str] = []
    _all_series: List[QtCharts.QXYSeries] = []
    _xaxis_min: float = 0.0
    _xaxis_max: float = 0.0

    # def get_xmin_offset(self) -> float:
    #     """Getter for _xmin_offset."""
    #     return self._xmin_offset

    # def set_xmin_offset(self, xmin_offset_: float) -> None:
    #     """Setter for _xmin_offset."""
    #     self._xmin_offset = xmin_offset_

    # xmin_offset = Property(float, get_xmin_offset, set_xmin_offset)

    def get_num_labels(self) -> int:
        return len(TRACKING_SIGNALS_TAB[Keys.LABELS])

    num_labels = Property(int, get_num_labels)

    def get_xaxis_min(self) -> float:
        """Getter for _xaxis_min."""
        return self._xaxis_min

    xaxis_min = Property(float, get_xaxis_min)

    def get_xaxis_max(self) -> float:
        """Getter for _xaxis_max."""
        return self._xaxis_max

    xaxis_max = Property(float, get_xaxis_max)

    def get_check_labels(self) -> List[str]:
        return self._check_labels

    check_labels = Property(QTKeys.QVARIANTLIST, get_check_labels)  # type: ignore

    def get_all_series(self) -> List[QtCharts.QXYSeries]:
        return self._all_series

    all_series = Property(QTKeys.QVARIANTLIST, get_all_series)  # type: ignore

    @Slot(int)  # type: ignore
    def getLabel(self, index) -> str:
        """Getter for one of the TRACKING_SIGNALS_TAB[Keys.LABELS]."""
        return TRACKING_SIGNALS_TAB[Keys.LABELS][index]

    @Slot(QtCharts.QAbstractSeries)  # type: ignore
    def addSeries(self, series) -> None:
        """Add a QML created series to the all_series list"""
        self._all_series.append(series)

    @Slot(float, bool)  # type: ignore
    def fill_all_series(self, line_width, useOpenGL) -> None:
        points_for_all_series = TRACKING_SIGNALS_TAB[Keys.POINTS]
        # missing_series_indices: List[int] = [] # need to pass up the name too...
        if len(points_for_all_series) == 0:
            return  # missing_series_indices

        labels = TRACKING_SIGNALS_TAB[Keys.LABELS]
        colors = TRACKING_SIGNALS_TAB[Keys.COLORS]
        self._check_labels = TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS]
        self._xaxis_min = points_for_all_series[0][-1].x() + TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET]
        self._xaxis_max = points_for_all_series[0][-1].x()
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
            except IndexError:
                # The current problem is that the series' that are being updated with the points are not the same series that are attached to the chart..
                # Need to get the QML created charts added into the python _all_series list.
                # Probably want to return a sparse array or a dictionary mapping missing series index and series data to create.
                # Though it might be enough to just shoot back a list of indices that need series' created - and a generic series can be created for those,
                # which will be updated with real properties and data on the next timer fire.
                print(f"fill_all_series IndexError for idx {idx}")
                # Need to build up a return value that tells QML which series' to create.
                # missing_series_indices.append(idx)
                # series = QtCharts.QLineSeries()
                # return_series.append(series)
                # series.append(series_points)
                # self._all_series.append(series)
        return  # missing_series_indices

    # @Slot(list)  # type: ignore
    # def fill_series(self, series_list):
    #     for idx, series_and_key in enumerate(series_list):
    #         series, _ = series_and_key
    #         if idx < len(self._points):
    #             series.replace(self._points[idx])


class TrackingSignalsModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(TrackingSignalsPoints)  # type: ignore
    def fill_console_points(self, cp: TrackingSignalsPoints) -> TrackingSignalsPoints:  # pylint:disable=no-self-use
        cp.fill_all_series(TRACKING_SIGNALS_TAB[Keys.POINTS])
        cp.set_labels(TRACKING_SIGNALS_TAB[Keys.LABELS])
        cp.set_check_labels(TRACKING_SIGNALS_TAB[Keys.CHECK_LABELS])
        cp.set_colors(TRACKING_SIGNALS_TAB[Keys.COLORS])
        cp.set_xmin_offset(TRACKING_SIGNALS_TAB[Keys.XMIN_OFFSET])
        return cp
