# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

"""Solution Velocity Tab QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, QPointF, Signal, Slot

from .constants import Keys, QTKeys


def solution_velocity_update() -> Dict[str, Any]:
    return {
        Keys.AVAILABLE_UNITS: [],
        Keys.POINTS: [],
        Keys.COLORS: [],
        Keys.MAX: 0,
        Keys.MIN: 0,
    }


SOLUTION_VELOCITY_TAB: List[Dict[str, Any]] = [solution_velocity_update()]


class SolutionVelocityPoints(QObject):
    _instance: "SolutionVelocityPoints"
    _colors: List[str] = []
    _points: List[List[QPointF]] = [[]]
    _valid: bool = False
    _min: float = 0.0
    _max: float = 0.0
    _available_units: List[str] = []
    _data_updated = Signal()
    solution_velocity: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.solution_velocity = SOLUTION_VELOCITY_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        SOLUTION_VELOCITY_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.solution_velocity = SOLUTION_VELOCITY_TAB[0]
        self.update()  # type: ignore

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
        self._points = [list(map(lambda point: QPointF(point.x, point.y), points[idx])) for idx in range(len(points))]

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])


class SolutionVelocityModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionVelocityPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionVelocityPoints) -> SolutionVelocityPoints:
        cp.set_points(cp.solution_velocity[Keys.POINTS])
        cp.set_colors(cp.solution_velocity[Keys.COLORS])
        cp.set_max(cp.solution_velocity[Keys.MAX])
        cp.set_min(cp.solution_velocity[Keys.MIN])
        cp.set_available_units(cp.solution_velocity[Keys.AVAILABLE_UNITS])
        return cp
