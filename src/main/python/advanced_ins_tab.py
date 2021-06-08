"""Advanced Ins Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys

ADVANCED_INS_TAB: Dict[str, Any] = {
    Keys.TEXT_DATA: [],
    Keys.POINTS: [],
}


class AdvancedInsPoints(QObject):

    _points: List[List[QPointF]] = [[]]
    _text_data: List[float] = []

    def get_text_data(self) -> List[float]:
        """Getter for _text_data."""
        return self._text_data

    def set_text_data(self, text_data: List[float]) -> None:
        """Setter for _text_data."""
        self._text_data = text_data

    text_data = Property(QTKeys.QVARIANTLIST, get_text_data, set_text_data)  # type: ignore

    def get_points(self) -> List[List[QPointF]]:
        return self._points

    def set_points(self, points) -> None:
        self._points = points

    points = Property(QTKeys.QVARIANTLIST, get_points, set_points)  # type: ignore

    @Slot(list)  # type: ignore
    def fill_series(self, series_list):
        for idx, series in enumerate(series_list):
            series.replace(self._points[idx])


class AdvancedInsModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedInsPoints)  # type: ignore
    def fill_console_points(self, cp: AdvancedInsPoints) -> AdvancedInsPoints:  # pylint:disable=no-self-use
        cp.set_points(ADVANCED_INS_TAB[Keys.POINTS])
        cp.set_text_data(ADVANCED_INS_TAB[Keys.TEXT_DATA])
        return cp
