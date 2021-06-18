"""Advanced Magnetometer Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys

ADVANCED_MAGNETOMETER_TAB: Dict[str, Any] = {
    Keys.YMAX: float,
    Keys.YMIN: float,
    Keys.POINTS: [],
}


class AdvancedMagnetometerPoints(QObject):

    _points: List[List[QPointF]] = [[]]
    _ymin: float = 0.0
    _ymax: float = 0.0

    def get_ymin(self) -> float:
        """Getter for _ymin."""
        return self._ymin

    def set_ymin(self, ymin_: float) -> None:
        """Setter for _ymin."""
        self._ymin = ymin_

    ymin = Property(float, get_ymin, set_ymin)

    def get_ymax(self) -> float:
        """Getter for _ymax."""
        return self._ymax

    def set_ymax(self, ymax_: float) -> None:
        """Setter for _ymax."""
        self._ymax = ymax_

    ymax = Property(float, get_ymax, set_ymax)

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])


class AdvancedMagnetometerModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedMagnetometerPoints)  # type: ignore
    def fill_console_points(  # pylint:disable=no-self-use
        self, cp: AdvancedMagnetometerPoints
    ) -> AdvancedMagnetometerPoints:
        cp.set_points(ADVANCED_MAGNETOMETER_TAB[Keys.POINTS])
        cp.set_ymax(ADVANCED_MAGNETOMETER_TAB[Keys.YMAX])
        cp.set_ymin(ADVANCED_MAGNETOMETER_TAB[Keys.YMIN])
        return cp
