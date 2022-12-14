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

"""Advanced Magnetometer Tab QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, QPointF, Signal, Slot

from .constants import Keys, QTKeys


def advanced_magnetometer_tab_update() -> Dict[str, Any]:
    return {
        Keys.YMAX: float,
        Keys.YMIN: float,
        Keys.POINTS: [],
    }


ADVANCED_MAGNETOMETER_TAB: List[Dict[str, Any]] = [advanced_magnetometer_tab_update()]


class AdvancedMagnetometerPoints(QObject):
    _instance: "AdvancedMagnetometerPoints"
    _points: List[List[QPointF]] = [[]]
    _ymin: float = 0.0
    _ymax: float = 0.0
    _data_updated = Signal()
    advanced_magnetometer_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.advanced_magnetometer_tab = ADVANCED_MAGNETOMETER_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        ADVANCED_MAGNETOMETER_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.advanced_magnetometer_tab = ADVANCED_MAGNETOMETER_TAB[0]

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
        self._points = [list(map(lambda point: QPointF(point.x, point.y), points[idx])) for idx in range(len(points))]

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
        cp.set_points(cp.advanced_magnetometer_tab[Keys.POINTS])
        cp.set_ymax(cp.advanced_magnetometer_tab[Keys.YMAX])
        cp.set_ymin(cp.advanced_magnetometer_tab[Keys.YMIN])
        return cp
