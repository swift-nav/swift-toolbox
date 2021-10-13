from typing import Dict, Any
from copy import deepcopy

from PySide2.QtCore import Property, Slot, Signal, QAbstractTableModel, Qt, QModelIndex
from PySide2.QtGui import QFont, QFontMetrics, QGuiApplication

from .constants import Keys


def localPadFloat(num, length, digits=2, allowNegative=True):
    if not num:
        return "--"
    s = f"{num:.{digits}f}"
    padLength = length if not allowNegative else length + 1
    return f"{s:>{padLength}}"


def showFlags(flags):
    if not flags:
        return "0x0000"
    # flagStr = '0x' + flags.toString(16).padStart(4, '0') + ' =';
    flagStr = f"0x{flags:0>4x} ="

    # Bit 0 is Pseudorange valid
    if flags & 0x01:
        flagStr += " PR"
    # Bit 1 is Carrier phase valid
    if flags & 0x02:
        flagStr += " CP"
    # Bit 2 is Half-cycle ambiguity
    if flags & 0x04:
        flagStr += " 1/2C"
    # Bit 3 is Measured Doppler Valid
    if flags & 0x08:
        flagStr += " MD"
    return flagStr


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
    # pylint: disable=too-many-instance-attributes
    # Might want to move the column_widths logic into QML and use QML's
    # FontMetrics, but for now this is ok.

    tow_changed = Signal(float, arguments="tow")
    week_changed = Signal(int, arguments="week")
    row_count_changed = Signal(int, arguments="row_count")
    remote_changed = Signal(bool, arguments="remote")
    dataPopulated = Signal()

    column_metadata = [
        ("PRN", lambda columnValue: columnValue),
        ("Pseudorange (m)", lambda columnValue: localPadFloat(columnValue, 1)),
        ("Carrier Phase (cycles)", lambda columnValue: localPadFloat(columnValue, 1)),
        ("C/N0 (dB-Hz)", lambda columnValue: localPadFloat(columnValue, 1)),
        ("Meas. Doppler (Hz)", lambda columnValue: localPadFloat(columnValue, 1)),
        ("Comp. Doppler (Hz)", lambda columnValue: localPadFloat(columnValue, 1)),
        ("Lock", lambda columnValue: columnValue),
        ("Flags", showFlags),
    ]

    def __init__(self, parent=None):
        super().__init__(parent)
        self._tow = 0
        self._week = 0
        self._rows = []
        self._remote = False
        self._column_widths = [None] * len(ObservationTableModel.column_metadata)
        self._columnWidth_calls = [0] * len(self._column_widths)
        self._column_widths_seen_data_all_columns = False
        self.json_col_names = None

    def set_tow(self, tow) -> None:
        """Setter for _tow."""
        self._tow = tow
        self.tow_changed.emit(self._tow)  # type: ignore

    def get_tow(self) -> float:
        return self._tow

    def set_week(self, week) -> None:
        """Setter for _week."""
        self._week = week
        self.week_changed.emit(self._week)  # type: ignore

    def get_week(self) -> int:
        return self._week

    def set_remote(self, remote) -> None:
        """Setter for _remote."""
        self._remote = remote
        self.remote_changed.emit(self._remote)  # type: ignore

    def get_remote(self) -> bool:
        return self._remote

    def rowCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(self._rows)

    def columnCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(ObservationTableModel.column_metadata)

    def data(self, index, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return ObservationTableModel.column_metadata[index.column()][1](
            self._rows[index.row()][self.json_col_names[index.column()]]
        )

    def headerData(self, section, orientation, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return ObservationTableModel.column_metadata[section][0] if orientation == Qt.Horizontal else section

    @Slot()  # type: ignore
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
                    if self.json_col_names is None:
                        self.json_col_names = list(row.keys())
                    rowsToInsert.append(deepcopy(row))

        if len(rowsToInsert) > 0:
            self.beginInsertRows(QModelIndex(), len(self._rows), len(self._rows) + len(rowsToInsert) - 1)
            self._rows.extend(rowsToInsert)
            self.endInsertRows()
            self.row_count_changed.emit(self.rowCount())  # type: ignore

        if (
            len(self._rows) > 0
            and len(self._rows[-1]) == self.columnCount()
            and not self._column_widths_seen_data_all_columns
        ):
            self.dataPopulated.emit()  # type: ignore
            self._column_widths_seen_data_all_columns = True

    @Slot(int, result=int)  # type: ignore
    @Slot(int, QFont, result=int)  # type: ignore
    @Slot(int, QFont, QFont, result=int)  # type: ignore
    def columnWidth(self, column, tableFont=None, headerFont=None):
        # Don't cache until the second call on a column because the first call to this per column
        # is done before any data has come in, and columns are just sized to headers.
        if not self._column_widths[column] or self._columnWidth_calls[column] < 2:
            margin = 8
            defaultFontMetrics = QFontMetrics(QGuiApplication.font())
            tfm = defaultFontMetrics if tableFont is None else QFontMetrics(tableFont)
            hfm = defaultFontMetrics if headerFont is None else QFontMetrics(headerFont)
            ret = hfm.width(str(self.headerData(column, Qt.Horizontal)) + " ^") + margin
            for rowIdx in range(len(self._rows)):
                modelIdx = self.index(rowIdx, column)
                cellData = str(self.data(modelIdx))
                ret = max(ret, tfm.width(cellData) + margin)
            self._column_widths[column] = ret

        self._columnWidth_calls[column] += 1
        return self._column_widths[column]

    @Slot(float, int, result=str)  # type: ignore
    @Slot(float, int, int, result=str)  # type: ignore
    @Slot(float, int, int, bool, result=str)  # type: ignore
    def padFloat(self, num, length, digits=2, allowNegative=True):  # pylint: disable=no-self-use
        return localPadFloat(num, length, digits, allowNegative)

    # Intentionally do not provide a setter in the property - no setting from QML.
    week = Property(float, get_week, notify=week_changed)  # type: ignore
    tow = Property(float, get_tow, notify=tow_changed)  # type: ignore
    row_count = Property(int, rowCount, notify=row_count_changed)  # type: ignore
    # Except this one - QML needs to specify if the model should be returning local data or remote data.
    remote = Property(bool, get_remote, set_remote, notify=remote_changed)  # type: ignore


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
