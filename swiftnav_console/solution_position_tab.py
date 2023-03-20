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

"""Solution Position Tab QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, QPointF, Signal, Slot

from .constants import Keys, QTKeys


def solution_position_update() -> Dict[str, Any]:
    return {
        Keys.AVAILABLE_UNITS: [],
        Keys.CUR_POINTS: [],
        Keys.POINTS: [],
        Keys.LAT_MAX: 0,
        Keys.LAT_MIN: 0,
        Keys.LON_MAX: 0,
        Keys.LON_MIN: 0,
        Keys.SOLUTION_LINE: [],
    }


SOLUTION_POSITION_TAB: List[Dict[str, Any]] = [solution_position_update()]


class SolutionPositionPoints(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods
    _instance: "SolutionPositionPoints"
    _points: List[List[QPointF]] = [[]]
    _cur_points: List[List[QPointF]] = [[]]
    _lat_min: float = 0.0
    _lat_max: float = 0.0
    _lon_min: float = 0.0
    _lon_max: float = 0.0
    _available_units: List[str] = []
    _solution_line: List[QPointF] = []
    _data_updated = Signal()
    solution_position: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.solution_position = SOLUTION_POSITION_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        SOLUTION_POSITION_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.solution_position = SOLUTION_POSITION_TAB[0]
        self.update()  # type: ignore

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

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = [list(map(lambda point: QPointF(point.x, point.y), points[idx])) for idx in range(len(points))]

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    def get_cur_points(self) -> List[List[QPointF]]:
        return self._cur_points

    def set_cur_points(self, cur_points) -> None:
        self._cur_points = [
            list(map(lambda point: QPointF(point.x, point.y), cur_points[idx])) for idx in range(len(cur_points))
        ]

    cur_points = Property(QTKeys.QVARIANTLIST, get_cur_points, set_cur_points)  # type: ignore

    def get_available_units(self) -> List[str]:
        return self._available_units

    def set_available_units(self, available_units: List[str]) -> None:
        self._available_units = available_units

    available_units = Property(QTKeys.QVARIANTLIST, get_available_units, set_available_units)  # type: ignore

    def get_solution_line(self) -> List[QPointF]:
        return self._solution_line

    def set_solution_line(self, solution_line) -> None:
        self._solution_line = list(map(lambda point: QPointF(point.x, point.y), solution_line))

    solution_line = Property(QTKeys.QVARIANTLIST, get_solution_line, set_solution_line)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        line = series_list[0]
        scatters = series_list[1]
        cur_scatters = series_list[2]
        line.replace(self._solution_line)
        for idx, scatter in enumerate(scatters):
            scatter.replace(self._points[idx])
            cur_scatters[idx].replace(self._cur_points[idx])


class SolutionPositionModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionPositionPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionPositionPoints) -> SolutionPositionPoints:
        cp.set_points(cp.solution_position[Keys.POINTS])
        cp.set_cur_points(cp.solution_position[Keys.CUR_POINTS])
        cp.set_lat_max(cp.solution_position[Keys.LAT_MAX])
        cp.set_lat_min(cp.solution_position[Keys.LAT_MIN])
        cp.set_lon_max(cp.solution_position[Keys.LON_MAX])
        cp.set_lon_min(cp.solution_position[Keys.LON_MIN])
        cp.set_available_units(cp.solution_position[Keys.AVAILABLE_UNITS])
        cp.set_solution_line(cp.solution_position[Keys.SOLUTION_LINE])
        return cp
