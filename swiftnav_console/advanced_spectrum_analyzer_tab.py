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

from PySide6 import QtCharts
from PySide6.QtCore import Property, QObject, QPointF, Signal, Slot

from .constants import Keys, QTKeys


def advanced_spectrum_analyzer_tab_update() -> Dict[str, Any]:
    return {
        Keys.CHANNEL: 0,
        Keys.POINTS: [],
        Keys.YMAX: 0,
        Keys.YMIN: 0,
        Keys.XMAX: 0,
        Keys.XMIN: 0,
    }


ADVANCED_SPECTRUM_ANALYZER_TAB: List[Dict[str, Any]] = [advanced_spectrum_analyzer_tab_update()]


class AdvancedSpectrumAnalyzerPoints(QObject):
    _instance: "AdvancedSpectrumAnalyzerPoints"
    _points: List[QPointF] = []
    _ymin: float = 0.0
    _ymax: float = 0.0
    _xmin: float = 0.0
    _xmax: float = 0.0
    _channel: int = 0
    _data_updated = Signal()
    advanced_spectrum_analyzer_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.advanced_spectrum_analyzer_tab = ADVANCED_SPECTRUM_ANALYZER_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        ADVANCED_SPECTRUM_ANALYZER_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.advanced_spectrum_analyzer_tab = ADVANCED_SPECTRUM_ANALYZER_TAB[0]
        self.update()  # type: ignore

    def get_ymin(self) -> float:
        """Getter for _ymin."""
        return self._ymin

    def set_ymin(self, ymin: float) -> None:
        """Setter for _ymin."""
        self._ymin = ymin

    ymin = Property(float, get_ymin, set_ymin)

    def get_ymax(self) -> float:
        """Getter for _ymax."""
        return self._ymax

    def set_ymax(self, ymax: float) -> None:
        """Setter for _ymax."""
        self._ymax = ymax

    ymax = Property(float, get_ymax, set_ymax)

    def get_xmin(self) -> float:
        """Getter for _xmin."""
        return self._xmin

    def set_xmin(self, xmin: float) -> None:
        """Setter for _xmin."""
        self._xmin = xmin

    xmin = Property(float, get_xmin, set_xmin)

    def get_xmax(self) -> float:
        """Getter for _xmax."""
        return self._xmax

    def set_xmax(self, xmax: float) -> None:
        """Setter for _xmax."""
        self._xmax = xmax

    xmax = Property(float, get_xmax, set_xmax)

    def get_channel(self) -> int:
        """Getter for _channel."""
        return self._channel

    def set_channel(self, channel: int) -> None:
        """Setter for _channel."""
        self._channel = channel

    channel = Property(int, get_channel, set_channel)

    def get_points(self) -> List[QPointF]:
        return self._points

    def set_points(self, points) -> None:
        self._points = [QPointF(point.x, point.y) for point in points]

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(QtCharts.QXYSeries)  # type: ignore
    def fill_series(self, series):
        series.replace(self._points)


class AdvancedSpectrumAnalyzerModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedSpectrumAnalyzerPoints)  # type: ignore
    def fill_console_points(self, cp: AdvancedSpectrumAnalyzerPoints) -> AdvancedSpectrumAnalyzerPoints:
        cp.set_points(cp.advanced_spectrum_analyzer_tab[Keys.POINTS])
        cp.set_ymax(cp.advanced_spectrum_analyzer_tab[Keys.YMAX])
        cp.set_ymin(cp.advanced_spectrum_analyzer_tab[Keys.YMIN])
        cp.set_xmax(cp.advanced_spectrum_analyzer_tab[Keys.XMAX])
        cp.set_xmin(cp.advanced_spectrum_analyzer_tab[Keys.XMIN])
        cp.set_channel(cp.advanced_spectrum_analyzer_tab[Keys.CHANNEL])
        return cp
