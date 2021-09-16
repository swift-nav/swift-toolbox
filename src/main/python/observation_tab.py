from typing import Dict, Any
from copy import deepcopy

from PySide2.QtCore import Property, Slot, Signal, QAbstractTableModel, Qt, QModelIndex

from constants import Keys


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


class ObservationTableModel(QAbstractTableModel):
    tow_changed = Signal(float, arguments=["tow"])
    week_changed = Signal(int, arguments=["week"])
    row_count_changed = Signal(int, arguments=["row_count"])
    remote_changed = Signal(bool, arguments=["remote"])

    column_names = [
        "PRN",
        "Pseudorange (m)",
        "Carrier Phase (cycles)",
        "C/N0 (dB-Hz)",
        "Meas. Doppler (Hz)",
        "Comp. Doppler (Hz)",
        "Lock",
        "Flags",
    ]

    def __init__(self, parent=None):
        super().__init__(parent)
        self._tow = 0
        self._week = 0
        self._rows = []
        self._remote = False

        self.col_names = None

    def set_tow(self, tow) -> None:
        """Setter for _tow."""
        self._tow = tow
        self.tow_changed.emit(self._tow)

    def get_tow(self) -> float:
        return self._tow

    def set_week(self, week) -> None:
        """Setter for _week."""
        self._week = week
        self.week_changed.emit(self._week)

    def get_week(self) -> int:
        return self._week

    def set_remote(self, remote) -> None:
        """Setter for _remote."""
        self._remote = remote
        self.remote_changed.emit(self._remote)

    def get_remote(self) -> bool:
        return self._remote

    def rowCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(self._rows)

    def columnCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(ObservationTableModel.column_names)

    def data(self, index, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return self._rows[index.row()][self.col_names[index.column()]]

    @Slot(int, result=str)
    def rowData(self, rowIdx):
        return str(self._rows[rowIdx].values())

    def headerData(self, section, orientation, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return ObservationTableModel.column_names if orientation == Qt.Horizontal else section

    @Slot()
    def update(self) -> None:
        observation_tab = REMOTE_OBSERVATION_TAB if self._remote else LOCAL_OBSERVATION_TAB
        if observation_tab[Keys.TOW] != self._tow:
            self.set_tow(observation_tab[Keys.TOW])
        if observation_tab[Keys.WEEK] != self._week:
            self.set_week(observation_tab[Keys.WEEK])
        # dicts are guaranteed to be in insertion order as of Python 3.7, so
        # no need to do key lookup
        # https://stackoverflow.com/questions/39980323/are-dictionaries-ordered-in-python-3-6
        rowsToInsert = []
        for rowIdx in range(len(observation_tab[Keys.ROWS])):
            row = observation_tab[Keys.ROWS][rowIdx]
            for colIdx in range(len(row)):
                column = list(row)[colIdx]
                try:
                    modelRow = self._rows[rowIdx]
                    if row[column] != modelRow[column]:
                        modelRow[column] = row[column]
                        modelIdx = self.index(rowIdx, colIdx)
                        self.dataChanged.emit(modelIdx, modelIdx)  # pylint: disable=no-member
                except IndexError:
                    if self.col_names is None:
                        self.col_names = list(row.keys())
                    rowsToInsert.append(deepcopy(row))

        if len(rowsToInsert) > 0:
            self.beginInsertRows(QModelIndex(), len(self._rows), len(self._rows) + len(rowsToInsert) - 1)
            self._rows.extend(rowsToInsert)
            self.endInsertRows()
            self.row_count_changed.emit(self.rowCount())

    # Intentionally do not provide a setter in the property - no setting from QML.
    week = Property(float, get_week, notify=week_changed)
    tow = Property(float, get_tow, notify=tow_changed)
    row_count = Property(int, rowCount, notify=row_count_changed)
    # Except this one - QML needs to specify if the model should be returning local data or remote data.
    remote = Property(bool, get_remote, set_remote, notify=remote_changed)


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
