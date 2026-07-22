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

from typing import Dict, List, Any

from PySide6.QtCore import Property, Slot, Signal, QAbstractTableModel, Qt, QModelIndex

from .constants import Keys, QTKeys
from .observation_tab import ObservationTableModel, localPadFloat, observation_update


def ssr_stream_update() -> Dict[str, Any]:
    return {Keys.STREAMS: []}


def ssr_sat_correction_update() -> Dict[str, Any]:
    return {Keys.SAT_CORRECTIONS: []}


def ssr_tile_update() -> Dict[str, Any]:
    return {Keys.TILES: []}


def rtcm_status_update() -> Dict[str, Any]:
    return {Keys.RTCM_ROWS: []}


SSR_STREAM_TAB: List[Dict[str, Any]] = [ssr_stream_update()]
SSR_SAT_CORRECTION_TAB: List[Dict[str, Any]] = [ssr_sat_correction_update()]
SSR_TILE_TAB: List[Dict[str, Any]] = [ssr_tile_update()]
RTCM_STATUS_TAB: List[Dict[str, Any]] = [rtcm_status_update()]
OSR_OBSERVATION_TAB: List[Dict[str, Any]] = [observation_update()]


def ssr_stream_rows_to_dicts(rows) -> List[Dict[str, Any]]:
    return [
        {
            "msgType": entry.msgType,
            "lastAgeSec": entry.lastAgeSec,
            "updateIntervalSec": entry.updateIntervalSec,
            "iodSsr": entry.iodSsr,
            "count": entry.count,
        }
        for entry in rows
    ]


def ssr_sat_correction_rows_to_dicts(rows) -> List[Dict[str, Any]]:
    return [
        {
            "sid": entry.sid,
            "radial": entry.radial,
            "along": entry.along,
            "cross": entry.cross,
            "clockC0": entry.clockC0,
            "codeBias": entry.codeBias,
            "phaseBias": entry.phaseBias,
            "ageSec": entry.ageSec,
        }
        for entry in rows
    ]


def ssr_tile_rows_to_dicts(rows) -> List[Dict[str, Any]]:
    return [
        {
            "tileSetId": entry.tileSetId,
            "tileId": entry.tileId,
            "cornerNwLat": entry.cornerNwLat,
            "cornerNwLon": entry.cornerNwLon,
            "rows": entry.rows,
            "cols": entry.cols,
            "nSats": entry.nSats,
        }
        for entry in rows
    ]


def rtcm_rows_to_dicts(rows) -> List[Dict[str, Any]]:
    return [
        {
            "msgId": entry.msgId,
            "rate": entry.rate,
            "ageSec": entry.ageSec,
            "bundle": entry.bundle,
        }
        for entry in rows
    ]


class SsrTableModel(QAbstractTableModel):  # pylint: disable=too-few-public-methods
    """Base for the read-only SSR tables on the Corrections tab.

    Rows are refreshed wholesale on each backend update rather than diffed
    cell-by-cell, since SSR messages update at most a few times a second
    (per RTCM DF391), unlike per-epoch observation data.
    """

    row_count_changed = Signal(int, arguments="row_count")
    _data_updated = Signal()
    column_metadata: List[Any] = []
    _backing_store: List[Dict[str, Any]] = [{}]
    _rows_key: str = ""

    def __init__(self, parent=None):
        super().__init__(parent)
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self._rows: List[Dict[str, Any]] = []
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._backing_store[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.update()

    def rowCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(self._rows)

    def columnCount(self, parent=QModelIndex()):  # pylint: disable=unused-argument
        return len(self.column_metadata)

    def data(self, index, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return self.column_metadata[index.column()][1](self._rows[index.row()])

    def headerData(self, section, orientation, role=Qt.DisplayRole):  # pylint: disable=unused-argument
        return self.column_metadata[section][0] if orientation == Qt.Horizontal else section

    @Slot()  # type: ignore
    def update(self) -> None:
        new_rows = self._backing_store[0][self._rows_key]
        if new_rows == self._rows:
            return
        self.beginResetModel()
        self._rows = new_rows
        self.endResetModel()
        self.row_count_changed.emit(self.rowCount())  # type: ignore

    def total_rows(self) -> int:
        return len(self._rows)

    row_count = Property(int, total_rows, notify=row_count_changed)  # type: ignore


class SsrStreamTableModel(SsrTableModel):
    _instance: "SsrStreamTableModel"
    _backing_store = SSR_STREAM_TAB
    _rows_key = Keys.STREAMS
    column_metadata = [
        ("Message", lambda r: r["msgType"]),
        ("Age (s)", lambda r: localPadFloat(r["lastAgeSec"], 3)),
        ("Rate (s)", lambda r: localPadFloat(r["updateIntervalSec"], 3)),
        ("IOD", lambda r: r["iodSsr"]),
        ("Count", lambda r: r["count"]),
    ]


class SsrSatCorrectionTableModel(SsrTableModel):
    _instance: "SsrSatCorrectionTableModel"
    _backing_store = SSR_SAT_CORRECTION_TAB
    _rows_key = Keys.SAT_CORRECTIONS
    column_metadata = [
        ("Signal", lambda r: r["sid"]),
        ("Radial (mm)", lambda r: localPadFloat(r["radial"], 5)),
        ("Along (mm)", lambda r: localPadFloat(r["along"], 5)),
        ("Cross (mm)", lambda r: localPadFloat(r["cross"], 5)),
        ("Clock C0 (mm)", lambda r: localPadFloat(r["clockC0"], 5)),
        ("Code Bias (cm)", lambda r: localPadFloat(r["codeBias"], 4)),
        ("Phase Bias", lambda r: localPadFloat(r["phaseBias"], 5)),
        ("Age (s)", lambda r: localPadFloat(r["ageSec"], 3)),
    ]


class SsrTileTableModel(SsrTableModel):
    _instance: "SsrTileTableModel"
    _backing_store = SSR_TILE_TAB
    _rows_key = Keys.TILES
    column_metadata = [
        ("Tile Set", lambda r: r["tileSetId"]),
        ("Tile ID", lambda r: r["tileId"]),
        ("NW Corner", lambda r: f"{r['cornerNwLat']:.3f}, {r['cornerNwLon']:.3f}"),
        ("Grid Size", lambda r: f"{r['rows']}x{r['cols']}"),
        ("Sats", lambda r: r["nSats"]),
    ]


STALE_BUNDLE_AGE_SEC = 10


class RtcmMessageTableModel(SsrTableModel):
    """Every RTCM3 message ID seen on the raw corrections byte stream, with
    its empirical rate and detected bundle. This is the ground truth the
    Corrections tab QML uses to decide which bundle-specific panels
    (Decoded Observations / SSR Streams / etc.) to show.
    """

    _instance: "RtcmMessageTableModel"
    _backing_store = RTCM_STATUS_TAB
    _rows_key = Keys.RTCM_ROWS
    column_metadata = [
        ("Message ID", lambda r: r["msgId"]),
        ("Bundle", lambda r: r["bundle"]),
        ("Rate (Hz)", lambda r: localPadFloat(r["rate"], 3)),
        ("Age (s)", lambda r: localPadFloat(r["ageSec"], 3)),
    ]

    detected_bundles_changed = Signal()

    @Slot()  # type: ignore
    def update(self) -> None:
        super().update()
        self.detected_bundles_changed.emit()  # type: ignore

    def get_detected_bundles(self) -> List[str]:
        return sorted({r["bundle"] for r in self._rows if r["ageSec"] < STALE_BUNDLE_AGE_SEC})

    # Bundles seen recently enough to be considered "currently active" -
    # used by CorrectionsTab.qml to decide which panels to show in Auto mode.
    detected_bundles = Property(QTKeys.QVARIANTLIST, get_detected_bundles, notify=detected_bundles_changed)  # type: ignore


class OsrObservationTableModel(ObservationTableModel):
    """Decoded per-satellite content of the OSR/NXRTK-MSM5 correction
    stream (what used to be the "Remote" section of the Observations tab).
    """

    _instance: "OsrObservationTableModel"
    _backing_store = OSR_OBSERVATION_TAB

    def __init__(self, parent=None):
        super().__init__(parent)
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
