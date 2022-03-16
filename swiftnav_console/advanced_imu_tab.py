"""Advanced Imu Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Signal, Slot

from .constants import Keys, QTKeys


def advanced_imu_tab_update() -> Dict[str, Any]:
    return {
        Keys.FIELDS_DATA: [],
        Keys.POINTS: [],
    }


class AdvancedImuPoints(QObject):

    _points: List[List[QPointF]] = [[]]
    _fields_data: List[float] = []
    _data_updated = Signal(dict)
    advanced_imu_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.advanced_imu_tab = advanced_imu_tab_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.advanced_imu_tab = update_data

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
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])


class AdvancedImuModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedImuPoints)  # type: ignore
    def fill_console_points(self, cp: AdvancedImuPoints) -> AdvancedImuPoints:  # pylint:disable=no-self-use
        cp.set_points(cp.advanced_imu_tab[Keys.POINTS])
        cp.set_fields_data(cp.advanced_imu_tab[Keys.FIELDS_DATA])
        return cp
