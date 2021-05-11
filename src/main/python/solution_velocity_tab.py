"""Solution Velocity Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys

SOLUTION_VELOCITY_TAB: Dict[str, Any] = {
    Keys.AVAILABLE_UNITS: [],
    Keys.POINTS: [],
    Keys.COLORS: [],
    Keys.MAX: 0,
    Keys.MIN: 0,
}


class SolutionVelocityPoints(QObject):

    _colors: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _valid: bool = False
    _min: float = 0.0
    _max: float = 0.0
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

    def get_min(self) -> float:
        """Getter for _min."""
        return self._min

    def set_min(self, min_: float) -> None:
        """Setter for _min."""
        self._min = min_

    min_ = Property(float, get_min, set_min)

    def get_max(self) -> float:
        """Getter for _max."""
        return self._max

    def set_max(self, max_: float) -> None:
        """Setter for _max."""
        self._max = max_

    max_ = Property(float, get_max, set_max)

    def get_available_units(self) -> List[str]:
        """Getter for _available_units."""
        return self._available_units

    def set_available_units(self, available_units: List[str]) -> None:
        """Setter for _available_units."""
        self._available_units = available_units

    available_units = Property(QTKeys.QVARIANTLIST, get_available_units, set_available_units)  # type: ignore

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
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])


class SolutionVelocityModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionVelocityPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionVelocityPoints) -> SolutionVelocityPoints:  # pylint:disable=no-self-use
        cp.set_points(SOLUTION_VELOCITY_TAB[Keys.POINTS])
        cp.set_colors(SOLUTION_VELOCITY_TAB[Keys.COLORS])
        cp.set_max(SOLUTION_VELOCITY_TAB[Keys.MAX])
        cp.set_min(SOLUTION_VELOCITY_TAB[Keys.MIN])
        cp.set_available_units(SOLUTION_VELOCITY_TAB[Keys.AVAILABLE_UNITS])
        return cp
