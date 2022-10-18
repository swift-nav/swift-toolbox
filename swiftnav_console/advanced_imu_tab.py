"""Advanced Imu Tab QObjects.
"""

from typing import Dict, List, Any
from time import perf_counter_ns

from PySide6.QtCore import Property, QObject, QPointF, Signal, Slot

from .constants import Keys, QTKeys


def advanced_imu_tab_update() -> Dict[str, Any]:
    return {
        Keys.FIELDS_DATA: [],
        Keys.POINTS: [],
    }


ADVANCED_IMU_TAB: List[Dict[str, Any]] = [advanced_imu_tab_update()]


class AdvancedImuPoints(QObject):
    _instance: "AdvancedImuPoints"
    _points: List[List[QPointF]] = [[]]
    _fields_data: List[float] = []
    _data_updated = Signal()
    advanced_imu_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.advanced_imu_tab = ADVANCED_IMU_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

        self.handle_data_updated_time = None
        self.set_points_time = None
        self.fill_series_time = None

    @classmethod
    def _perf_measure(cls, t1: int, last_t2: int, name: str):
        t2 = perf_counter_ns()
        if (last_t2 is None) or t2 > last_t2 + 1000000000:
            last_t2 = t2
            deltaT = t2 - t1
            print(f"{cls.__name__} {name} tottime: {deltaT / 1000000} ms")
        return last_t2

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        ADVANCED_IMU_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        t1 = perf_counter_ns()
        self.advanced_imu_tab = ADVANCED_IMU_TAB[0]
        self.handle_data_updated_time = self._perf_measure(t1, self.handle_data_updated_time, "handle_data_updated")

    def get_fields_data(self) -> List[float]:
        """Getter for _fields_data."""
        return self._fields_data

    def set_fields_data(self, fields_data: List[float]) -> None:
        """Setter for _fields_data."""
        self._fields_data = fields_data

    fields_data = Property(QTKeys.QVARIANTLIST, get_fields_data, set_fields_data)  # type: ignore

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        t1 = perf_counter_ns()
        self._points = points
        self.set_points_time = self._perf_measure(t1, self.set_points_time, "set_points")

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        t1 = perf_counter_ns()
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])
        self.fill_series_time = self._perf_measure(t1, self.fill_series_time, "fill_series")


class AdvancedImuModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedImuPoints)  # type: ignore
    def fill_console_points(self, cp: AdvancedImuPoints) -> AdvancedImuPoints:  # pylint:disable=no-self-use
        cp.set_points(cp.advanced_imu_tab[Keys.POINTS])
        cp.set_fields_data(cp.advanced_imu_tab[Keys.FIELDS_DATA])
        return cp
