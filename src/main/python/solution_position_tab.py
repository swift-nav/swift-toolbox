"""Solution Position Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys

SOLUTION_POSITION_TAB: Dict[str, Any] = {
    Keys.AVAILABLE_UNITS: [],
    Keys.CUR_POINTS: [],
    Keys.POINTS: [],
    Keys.LABELS: [],
    Keys.COLORS: [],
    Keys.LAT_MAX: 0,
    Keys.LAT_MIN: 0,
    Keys.LON_MAX: 0,
    Keys.LON_MIN: 0,
}


class SolutionPositionPoints(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods

    _colors: List[str] = []
    _labels: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _cur_points: List[List[QPointF]] = [[]]
    _valid: bool = False
    _lat_min: float = 0.0
    _lat_max: float = 0.0
    _lon_min: float = 0.0
    _lon_max: float = 0.0
    _available_units: List[str] = []

    def get_valid(self) -> bool:
        """Getter for _valid.

        Returns:
            bool: Whether it is valid or not.
        """
        return self._valid

    def set_valid(self, valid: bool) -> None:
        """Setter for _valid."""
        self._valid = valid

    valid = Property(bool, get_valid, set_valid)

    def get_lat_min(self) -> float:
        """Getter for _lat_min."""
        return self._lat_min

    def set_lat_min(self, lat_min_: float) -> None:
        """Setter for _lat_min."""
        self._lat_min = lat_min_

    lat_min_ = Property(float, get_lat_min, set_lat_min)

    def get_lat_max(self) -> float:
        """Getter for _lat_max."""
        return self._lat_max

    def set_lat_max(self, lat_max_: float) -> None:
        """Setter for _lat_max."""
        self._lat_max = lat_max_

    lat_max_ = Property(float, get_lat_max, set_lat_max)

    def get_lon_min(self) -> float:
        """Getter for _lon_min."""
        return self._lon_min

    def set_lon_min(self, lon_min_: float) -> None:
        """Setter for _lon_min."""
        self._lon_min = lon_min_

    lon_min_ = Property(float, get_lon_min, set_lon_min)

    def get_lon_max(self) -> float:
        """Getter for _lon_max."""
        return self._lon_max

    def set_lon_max(self, lon_max_: float) -> None:
        """Setter for _lon_max."""
        self._lon_max = lon_max_

    lon_max_ = Property(float, get_lon_max, set_lon_max)

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

    def get_cur_points(self) -> List[List[QPointF]]:
        return self._cur_points

    def set_cur_points(self, cur_points) -> None:
        self._cur_points = cur_points

    cur_points = Property(QTKeys.QVARIANTLIST, get_cur_points, set_cur_points)  # type: ignore

    def get_available_units(self) -> List[str]:
        return self._available_units

    def set_available_units(self, available_units: List[str]) -> None:
        self._available_units = available_units

    available_units = Property(QTKeys.QVARIANTLIST, get_available_units, set_available_units)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        lines = series_list[0]
        scatters = series_list[1]
        cur_scatters = series_list[2]
        for idx, _ in enumerate(lines):
            lines[idx].replace(self._points[idx])
            scatters[idx].replace(self._points[idx])
            cur_scatters[idx].replace(self._cur_points[idx])


class SolutionPositionModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionPositionPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionPositionPoints) -> SolutionPositionPoints:  # pylint:disable=no-self-use
        cp.set_points(SOLUTION_POSITION_TAB[Keys.POINTS])
        cp.set_cur_points(SOLUTION_POSITION_TAB[Keys.CUR_POINTS])
        cp.set_labels(SOLUTION_POSITION_TAB[Keys.LABELS])
        cp.set_colors(SOLUTION_POSITION_TAB[Keys.COLORS])
        cp.set_lat_max(SOLUTION_POSITION_TAB[Keys.LAT_MAX])
        cp.set_lat_min(SOLUTION_POSITION_TAB[Keys.LAT_MIN])
        cp.set_lon_max(SOLUTION_POSITION_TAB[Keys.LON_MAX])
        cp.set_lon_min(SOLUTION_POSITION_TAB[Keys.LON_MIN])
        cp.set_available_units(SOLUTION_POSITION_TAB[Keys.AVAILABLE_UNITS])
        return cp
