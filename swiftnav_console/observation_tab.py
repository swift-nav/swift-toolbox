from typing import Dict, List, Any
from copy import deepcopy
from collections import namedtuple

from PySide2.QtCore import Property, Slot, Signal, QAbstractTableModel, Qt, QModelIndex

from .constants import Keys, QTKeys

PrnEntry = namedtuple("PrnEntry", ["sat", "code"])


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


def format_prn_string(sat, code):
    return "{} ({})".format(sat, code)


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


class ObservationTableModel(QAbstractTableModel):  # pylint: disable=too-many-public-methods
    # pylint: disable=too-many-instance-attributes
    # Might want to move the column_widths logic into QML and use QML's
    # FontMetrics, but for now this is ok.

    tow_changed = Signal(float, arguments="tow")
    week_changed = Signal(int, arguments="week")
    row_count_changed = Signal(int, arguments="row_count")
    remote_changed = Signal(bool, arguments="remote")
    show_gps_only_changed = Signal(bool, arguments="show_gps_only")
    codes_changed = Signal()
    dataPopulated = Signal()

    column_metadata = [
        (
            "PRN",
            lambda obsData: format_prn_string(obsData["prn"].sat, obsData["prn"].code),
        ),
        ("Pseudorange (m)", lambda obsData: localPadFloat(obsData["pseudoRange"], 1)),
        ("Carrier Phase (cycles)", lambda obsData: localPadFloat(obsData["carrierPhase"], 1)),
        ("C/N0 (dB-Hz)", lambda obsData: localPadFloat(obsData["cn0"], 1)),
        ("Meas. Doppler (Hz)", lambda obsData: localPadFloat(obsData["measuredDoppler"], 1)),
        ("Comp. Doppler (Hz)", lambda obsData: localPadFloat(obsData["computedDoppler"], 1)),
        ("Lock", lambda obsData: obsData["lock"]),
        ("Flags", lambda obsData: showFlags(obsData["lock"])),
    ]

    def __init__(self, parent=None):
        super().__init__(parent)
        self._tow = 0
        self._week = 0
        self._rows = []
        self._remote = False
        self._column_widths = [None] * len(ObservationTableModel.column_metadata)
        self._columnWidth_calls = [0] * len(self._column_widths)
        self.json_col_names = None
        self._total_rows = 0
        self._code_filters = set()
        self._codes = set()

    def get_codes(self) -> List[List[str]]:
        observation_tab = REMOTE_OBSERVATION_TAB if self._remote else LOCAL_OBSERVATION_TAB
        return [entry["prn"].code for entry in observation_tab[Keys.ROWS]]

    def get_codes_by_prefix(self, prefix) -> List[List[str]]:
        return sorted([code for code in self._codes if code.startswith(prefix)])

    def get_gps_codes(self) -> List[List[str]]:
        return self.get_codes_by_prefix("GPS")

    def get_glo_codes(self) -> List[List[str]]:
        return self.get_codes_by_prefix("GLO")

    def get_bds_codes(self) -> List[List[str]]:
        return self.get_codes_by_prefix("BDS")

    def get_gal_codes(self) -> List[List[str]]:
        return self.get_codes_by_prefix("GAL")

    def get_qzs_codes(self) -> List[List[str]]:
        return self.get_codes_by_prefix("QZS")

    def get_sbas_codes(self) -> List[List[str]]:
        return self.get_codes_by_prefix("SBAS")

    def set_codes(self, codes) -> None:
        self._codes = codes
        self.codes_changed.emit()  # type: ignore

    @Slot(str, bool)  # type: ignore
    def filter_prn(self, prn, val) -> None:
        if val:
            self._code_filters.add(prn)
        else:
            self._code_filters.discard(prn)

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

    def total_rows(self) -> int:
        observation_tab = REMOTE_OBSERVATION_TAB if self._remote else LOCAL_OBSERVATION_TAB
        return len(observation_tab[Keys.ROWS])

    def rowCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(self._rows)

    @Slot(int, result=QTKeys.QVARIANT)  # type: ignore
    def getRow(self, index) -> QTKeys.QVARIANT:  # type: ignore
        return self._rows[index]

    def columnCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(ObservationTableModel.column_metadata)

    def data(self, index, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return ObservationTableModel.column_metadata[index.column()][1](self._rows[index.row()])

    def headerData(self, section, orientation, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return ObservationTableModel.column_metadata[section][0] if orientation == Qt.Horizontal else section

    @Slot()  # type: ignore
    def update(self) -> None:
        observation_tab = REMOTE_OBSERVATION_TAB if self._remote else LOCAL_OBSERVATION_TAB
        if observation_tab[Keys.TOW] != self._tow:
            self.set_tow(observation_tab[Keys.TOW])
        if observation_tab[Keys.WEEK] != self._week:
            self.set_week(observation_tab[Keys.WEEK])
        codes = list(set(entry["prn"].code for entry in observation_tab[Keys.ROWS]))
        if codes != self._codes:
            self.set_codes(codes)

        # dicts are guaranteed to be in insertion order as of Python 3.7, so
        # no need to do key lookup
        # https://stackoverflow.com/questions/39980323/are-dictionaries-ordered-in-python-3-6
        rowsToInsert = []
        rowIdx = 0
        for row in observation_tab[Keys.ROWS]:
            if row["prn"].code in self._code_filters:
                continue

            if rowIdx + 1 > len(self._rows):
                rowsToInsert.append(deepcopy(row))
                continue

            current_row = self._rows[rowIdx]

            for colIdx, obsKey in enumerate(row):
                if row[obsKey] != current_row[obsKey]:
                    current_row[obsKey] = row[obsKey]
                    modelIdx = self.index(rowIdx, colIdx)
                    self.dataChanged.emit(modelIdx, modelIdx)  # pylint: disable=no-member

            rowIdx += 1

        num_rows_removed = len(self._rows) - rowIdx

        # Remove old rows, if necessary
        if num_rows_removed > 0:
            self.beginRemoveRows(QModelIndex(), rowIdx, num_rows_removed)
            self._rows = self._rows[:rowIdx]
            self.endRemoveRows()
            self.row_count_changed.emit(self.rowCount())  # type: ignore

        if len(rowsToInsert) > 0:
            self.beginInsertRows(QModelIndex(), len(self._rows), len(self._rows) + len(rowsToInsert) - 1)
            self._rows.extend(rowsToInsert)
            self.endInsertRows()
            self.row_count_changed.emit(self.rowCount())  # type: ignore

        if len(self._rows) > 0 and len(self._rows[-1]) == self.columnCount():
            self.dataPopulated.emit()  # type: ignore

    @Slot(float, int, result=str)  # type: ignore
    @Slot(float, int, int, result=str)  # type: ignore
    @Slot(float, int, int, bool, result=str)  # type: ignore
    def padFloat(self, num, length, digits=2, allowNegative=True):  # pylint: disable=no-self-use
        return localPadFloat(num, length, digits, allowNegative)

    # Intentionally do not provide a setter in the property - no setting from QML.
    week = Property(float, get_week, notify=week_changed)  # type: ignore
    tow = Property(float, get_tow, notify=tow_changed)  # type: ignore
    row_count = Property(int, total_rows, notify=row_count_changed)  # type: ignore
    # Except this one - QML needs to specify if the model should be returning local data or remote data.
    remote = Property(bool, get_remote, set_remote, notify=remote_changed)  # type: ignore
    gps_codes = Property(QTKeys.QVARIANTLIST, get_gps_codes, notify=codes_changed)  # type: ignore
    glo_codes = Property(QTKeys.QVARIANTLIST, get_glo_codes, notify=codes_changed)  # type: ignore
    bds_codes = Property(QTKeys.QVARIANTLIST, get_bds_codes, notify=codes_changed)  # type: ignore
    gal_codes = Property(QTKeys.QVARIANTLIST, get_gal_codes, notify=codes_changed)  # type: ignore
    qzs_codes = Property(QTKeys.QVARIANTLIST, get_qzs_codes, notify=codes_changed)  # type: ignore
    sbas_codes = Property(QTKeys.QVARIANTLIST, get_sbas_codes, notify=codes_changed)  # type: ignore
    # Confusingly, codes depends on self._rows not self._codes
    codes = Property(QTKeys.QVARIANTLIST, get_codes, notify=row_count_changed)  # type: ignore


def obs_rows_to_json(rows):
    return [
        {
            "prn": PrnEntry(entry.sat, entry.code),
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
