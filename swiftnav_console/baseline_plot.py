"""Baseline Plot QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Signal, Slot

from .constants import Keys, QTKeys


def baseline_tab_update() -> Dict[str, Any]:
    return {
        Keys.CUR_POINTS: [],
        Keys.POINTS: [],
        Keys.N_MAX: 0,
        Keys.N_MIN: 0,
        Keys.E_MAX: 0,
        Keys.E_MIN: 0,
    }


BASELINE_PLOT: List[Dict[str, Any]] = [baseline_tab_update()]


class BaselinePlotPoints(QObject):

    _points: List[List[QPointF]] = [[]]
    _cur_points: List[List[QPointF]] = [[]]
    _n_min: float = 0.0
    _n_max: float = 0.0
    _e_min: float = 0.0
    _e_max: float = 0.0
    _data_updated = Signal()

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.baseline_plot = BASELINE_PLOT[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        BASELINE_PLOT[0] = update_data
        cls._instance._data_updated.emit()

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.baseline_plot = BASELINE_PLOT[0]

    def get_n_min(self) -> float:
        """Getter for _n_min."""
        return self._n_min

    def set_n_min(self, n_min_: float) -> None:
        """Setter for _n_min."""
        self._n_min = n_min_

    n_min = Property(float, get_n_min, set_n_min)

    def get_n_max(self) -> float:
        """Getter for _n_max."""
        return self._n_max

    def set_n_max(self, n_max_: float) -> None:
        """Setter for _n_max."""
        self._n_max = n_max_

    n_max = Property(float, get_n_max, set_n_max)

    def get_e_min(self) -> float:
        """Getter for _e_min."""
        return self._e_min

    def set_e_min(self, e_min_: float) -> None:
        """Setter for _e_min."""
        self._e_min = e_min_

    e_min = Property(float, get_e_min, set_e_min)

    def get_e_max(self) -> float:
        """Getter for _e_max."""
        return self._e_max

    def set_e_max(self, e_max_: float) -> None:
        """Setter for _e_max."""
        self._e_max = e_max_

    e_max = Property(float, get_e_max, set_e_max)

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

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        scatters = series_list[0]
        cur_scatters = series_list[1]
        for idx, _ in enumerate(scatters):
            scatters[idx].replace(self._points[idx])
            cur_scatters[idx].replace(self._cur_points[idx])


class BaselinePlotModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(BaselinePlotPoints)  # type: ignore
    def fill_console_points(self, cp: BaselinePlotPoints) -> BaselinePlotPoints:  # pylint:disable=no-self-use
        cp.set_points(cp.baseline_plot[Keys.POINTS])
        cp.set_cur_points(cp.baseline_plot[Keys.CUR_POINTS])
        cp.set_n_max(cp.baseline_plot[Keys.N_MAX])
        cp.set_n_min(cp.baseline_plot[Keys.N_MIN])
        cp.set_e_max(cp.baseline_plot[Keys.E_MAX])
        cp.set_e_min(cp.baseline_plot[Keys.E_MIN])
        return cp
