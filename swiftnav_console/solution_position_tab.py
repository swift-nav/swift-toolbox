"""Solution Position Tab QObjects.
"""

from typing import Dict, List, Any, Set, Tuple

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

Point = Tuple[float, float]

class SolutionPositionPoints(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods
    _instance: "SolutionPositionPoints"
    _points: List[bool] = [False] * 6
    _cur_points: List[List[QPointF]] = [[]]
    _lat_min: float = 0.0
    _lat_max: float = 0.0
    _lon_min: float = 0.0
    _lon_max: float = 0.0
    _available_units: List[str] = []
    _solution_line: List[QPointF] = []
    _data_updated = Signal()
    solution_position: Dict[str, Any] = {}

    cached_points: List[Set[Point]] = [set(), set(), set(), set(), set(), set()]
    added_points: List[Set[Point]] = [set(), set(), set(), set(), set(), set()]
    deleted_points: List[Set[Point]] = [set(), set(), set(), set(), set(), set()]

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

    def get_points(self) -> List[bool]:
        return self._points

    def set_points(self, points) -> None:
        new_cached_points = [set(), set(), set(), set(), set(), set()]

        for i in range(len(points)):
            for point in points[i]:
                pos = (point.x, point.y)
                new_cached_points[i].add(pos)

        self.added_points = [new_cached_points[i] - self.cached_points[i] for i in range(len(points))]
        self.deleted_points = [self.cached_points[i] - new_cached_points[i] for i in range(len(points))]
        self.cached_points = new_cached_points
        self._points = [bool(self.added_points[i]) for i in range(len(self.added_points))]

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    def get_cur_points(self) -> List[List[QPointF]]:
        return self._cur_points

    def set_cur_points(self, cur_points) -> None:
        self._cur_points = [list(s) for s in self.added_points]

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
        for idx, _ in enumerate(scatters):

            added = self.added_points[idx]
            deleted = self.deleted_points[idx]

            for (add, rem) in zip(added, deleted):
                scatters[idx].replace(add[0], add[1], rem[0], rem[1])

            for (x, y) in added - deleted:
                scatters[idx].append(x, y)

            cur_scatters[idx].replace(self._cur_points[idx])


class SolutionPositionModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionPositionPoints)  # type: ignore
    def fill_console_points(self, cp: SolutionPositionPoints) -> SolutionPositionPoints:  # pylint:disable=no-self-use
        cp.set_points(cp.solution_position[Keys.POINTS])
        cp.set_cur_points(cp.solution_position[Keys.CUR_POINTS])
        cp.set_lat_max(cp.solution_position[Keys.LAT_MAX])
        cp.set_lat_min(cp.solution_position[Keys.LAT_MIN])
        cp.set_lon_max(cp.solution_position[Keys.LON_MAX])
        cp.set_lon_min(cp.solution_position[Keys.LON_MIN])
        cp.set_available_units(cp.solution_position[Keys.AVAILABLE_UNITS])
        cp.set_solution_line(cp.solution_position[Keys.SOLUTION_LINE])
        return cp
