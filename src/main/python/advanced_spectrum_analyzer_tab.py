"""Solution Velocity Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys

ADVANCED_SPECTRUM_ANALYZER_TAB: Dict[str, Any] = {
    Keys.CHANNEL: 0,
    Keys.POINTS: [],
    Keys.YMAX: 0,
    Keys.YMIN: 0,
    Keys.XMAX: 0,
    Keys.XMIN: 0,
}


class AdvancedSpectrumAnalyzerPoints(QObject):

    _points: List[QPointF] = []
    _ymin: float = 0.0
    _ymax: float = 0.0
    _xmin: float = 0.0
    _xmax: float = 0.0
    _channel: int = 0

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
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series):
        for line in series:
            line.replace(self._points)


class AdvancedSpectrumAnalyzerModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedSpectrumAnalyzerPoints)  # type: ignore
    def fill_console_points(  # pylint:disable=no-self-use
        self, cp: AdvancedSpectrumAnalyzerPoints
    ) -> AdvancedSpectrumAnalyzerPoints:
        cp.set_points(ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.POINTS])
        cp.set_ymax(ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.YMAX])
        cp.set_ymin(ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.YMIN])
        cp.set_xmax(ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.XMAX])
        cp.set_xmin(ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.XMIN])
        cp.set_channel(ADVANCED_SPECTRUM_ANALYZER_TAB[Keys.CHANNEL])
        return cp
