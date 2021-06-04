"""Advanced Ins Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, QPointF, Slot

from constants import Keys, QTKeys

ADVANCED_INS_TAB: Dict[str, Any] = {
    Keys.TEXT_DATA: [],
    # Keys.TEXT_DATA_LABELS: [],
    Keys.POINTS: [],
    # Keys.LABELS: [],
    Keys.COLORS: [],
}


class AdvancedInsPoints(QObject):

    _points: List[List[QPointF]] = [[]]
    # _labels: List[str] = []
    _text_data: List[str] = []
    # _text_data_labels: List[str] = []

    def get_text_data(self) -> List[str]:
        """Getter for _text_data."""
        return self._text_data

    def set_text_data(self, text_data: List[str]) -> None:
        """Setter for _text_data."""
        self._text_data = text_data

    text_data = Property(QTKeys.QVARIANTLIST, get_text_data, set_text_data)  # type: ignore

    # def get_text_data_labels(self) -> List[str]:
    #     return self._text_data_labels

    # def set_text_data_labels(self, text_data_labels: List[str]) -> None:
    #     self._text_data_labels = text_data_labels

    # text_data_labels = Property(QTKeys.QVARIANTLIST, get_text_data_labels, set_text_data_labels)  # type: ignore

    # def get_labels(self) -> List[str]:
    #     return self._labels

    # def set_labels(self, labels: List[str]) -> None:
    #     self._labels = labels

    # labels = Property(QTKeys.QVARIANTLIST, get_labels, set_labels)  # type: ignore

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
        # cp.set_labels(ADVANCED_INS_TAB[Keys.LABELS])
        cp.set_text_data(ADVANCED_INS_TAB[Keys.TEXT_DATA])
        # cp.set_text_data_labels(ADVANCED_INS_TAB[Keys.TEXT_DATA_LABELS])
        return cp
