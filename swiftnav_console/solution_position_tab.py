"""Solution Position Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Signal, Slot

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
    }


SOLUTION_POSITION_TAB: List[Dict[str, Any]] = [solution_position_update()]


class SolutionPositionPoints(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods

    _points: List[List[QPointF]] = [[]]
    _cur_points: List[List[QPointF]] = [[]]
    _lat_min: float = 0.0
    _lat_max: float = 0.0
    _lon_min: float = 0.0
    _lon_max: float = 0.0
    _available_units: List[str] = []
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
        cls._instance._data_updated.emit()

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
        cp.set_points(cp.solution_position[Keys.POINTS])
        cp.set_cur_points(cp.solution_position[Keys.CUR_POINTS])
        cp.set_lat_max(cp.solution_position[Keys.LAT_MAX])
        cp.set_lat_min(cp.solution_position[Keys.LAT_MIN])
        cp.set_lon_max(cp.solution_position[Keys.LON_MAX])
        cp.set_lon_min(cp.solution_position[Keys.LON_MIN])
        cp.set_available_units(cp.solution_position[Keys.AVAILABLE_UNITS])
        return cp
