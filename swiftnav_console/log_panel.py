"""Log Panel QObjects.
"""

import json

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, LogLevel, QTKeys


def log_panel_update() -> Dict[str, Any]:
    return {
        Keys.ENTRIES: [],
        Keys.LOG_LEVEL_LABELS: [LogLevel.ERROR, LogLevel.WARNING, LogLevel.NOTICE, LogLevel.INFO, LogLevel.DEBUG],
        Keys.LOG_LEVEL: LogLevel.WARNING,
    }


LOG_PANEL: List[Dict[str, Any]] = [log_panel_update()]


class LogPanelData(QObject):
    _entries: List[Dict[str, str]] = []
    _log_level_labels: List[str] = []
    _log_level: str
    _data_updated = Signal()
    log_panel: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.log_panel = LOG_PANEL[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        LOG_PANEL[0] = update_data
        cls._instance._data_updated.emit()

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.log_panel = LOG_PANEL[0]

    def get_log_level_labels(self) -> List[str]:
        return self._log_level_labels

    def set_log_level_labels(self, log_level_labels: List[str]) -> None:
        self._log_level_labels = log_level_labels

    log_level_labels = Property(QTKeys.QVARIANTLIST, get_log_level_labels, set_log_level_labels)  # type: ignore

    def get_log_level(self) -> str:
        return self._log_level

    def set_log_level(self, log_level: str) -> None:
        self._log_level = log_level

    log_level = Property(str, get_log_level, set_log_level)

    def get_entries(self) -> List[Dict[str, str]]:
        """Getter for _entries."""
        return self._entries

    def set_entries(self, entries: List[Dict[str, str]]) -> None:
        """Setter for _entries."""
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore

    def append_entries(self, entries: List[Dict[str, str]]) -> None:
        self._entries += entries


class LogPanelModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(LogPanelData)  # type: ignore
    def fill_data(self, cp: LogPanelData) -> LogPanelData:  # pylint:disable=no-self-use
        cp.set_log_level_labels(cp.log_panel[Keys.LOG_LEVEL_LABELS])
        cp.set_log_level(cp.log_panel[Keys.LOG_LEVEL])
        # Avoid locking so that message processor has priority to lock
        if cp.log_panel[Keys.ENTRIES]:
            entries = []
            for entry in cp.log_panel[Keys.ENTRIES]:
                entries.append(json.loads(entry))
            cp.append_entries(entries)
            cp.log_panel[Keys.ENTRIES][:] = []
        return cp
