from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys, QTKeys


REMOTE_OBSERVATION_TAB: Dict[str, Any] = {
    Keys.TOW: 0,
    Keys.WEEK: 0,
    Keys.ROWS: [],
}

LOCAL_OBSERVATION_TAB: Dict[str, Any] = {
    Keys.TOW: 0,
    Keys.WEEK: 0,
    Keys.ROWS: [],
}


class ObservationData(QObject):

    _tow: float = 0.0
    _week: int = 0
    _rows: List[Any] = []

    def set_week(self, week) -> None:
        """Setter for _week."""
        self._week = week

    def get_week(self) -> int:
        return self._week

    week = Property(float, get_week, set_week)

    def set_tow(self, tow) -> None:
        """Setter for _tow."""
        self._tow = tow

    def get_tow(self) -> float:
        return self._tow

    tow = Property(float, get_tow, set_tow)

    def get_rows(self) -> List[Any]:
        """Getter for _rows."""
        return self._rows

    def set_rows(self, rows: List[Any]) -> None:
        """Setter for _rows."""
        self._rows = rows

    rows = Property(QTKeys.QVARIANTLIST, get_rows, set_rows)  # type: ignore


class ObservationModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(ObservationData, bool)  # type: ignore
    def fill_data(self, cp: ObservationData, is_remote: bool) -> ObservationData:  # pylint:disable=no-self-use
        if is_remote:
            cp.set_week(REMOTE_OBSERVATION_TAB[Keys.WEEK])
            cp.set_tow(REMOTE_OBSERVATION_TAB[Keys.TOW])
            cp.set_rows(REMOTE_OBSERVATION_TAB[Keys.ROWS])
        else:
            cp.set_week(LOCAL_OBSERVATION_TAB[Keys.WEEK])
            cp.set_tow(LOCAL_OBSERVATION_TAB[Keys.TOW])
            cp.set_rows(LOCAL_OBSERVATION_TAB[Keys.ROWS])
        return cp


def obs_rows_to_json(rows):
    return [
        {
            "prn": entry.prn,
            "pseudoRange": entry.pseudoRange,
            "carrierPhase": entry.carrierPhase,
            "cn0": entry.cn0,
            "measuredDoppler": entry.measuredDoppler,
            "computedDoppler": entry.computedDoppler,
            "lock": entry.lock,
            "flags": entry.flags,
        }
        for entry in rows
    ]
