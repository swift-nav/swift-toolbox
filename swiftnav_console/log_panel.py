"""Log Panel QObjects.
"""

import json

from typing import Dict, List, Any

from PySide6.QtCore import Property, QMutex, QObject, Slot

from .constants import Keys, LogLevel, QTKeys


LOG_PANEL: Dict[str, Any] = {
    Keys.ENTRIES: [],
    Keys.LOG_LEVEL_LABELS: [LogLevel.ERROR, LogLevel.WARNING, LogLevel.NOTICE, LogLevel.INFO, LogLevel.DEBUG],
    Keys.LOG_LEVEL: LogLevel.INFO,
}
log_panel_lock = QMutex()


class LogPanelData(QObject):
    _entries: List[Dict[str, str]] = []
    _log_level_labels: List[str] = []
    _log_level: str

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
        cp.set_log_level_labels(LOG_PANEL[Keys.LOG_LEVEL_LABELS])
        cp.set_log_level(LOG_PANEL[Keys.LOG_LEVEL])
        # Avoid locking so that message processor has priority to lock
        if LOG_PANEL[Keys.ENTRIES]:
            if log_panel_lock.try_lock():
                entries = []
                for entry in LOG_PANEL[Keys.ENTRIES]:
                    entries.append(json.loads(entry))
                cp.append_entries(entries)
                LOG_PANEL[Keys.ENTRIES][:] = []
                log_panel_lock.unlock()
        return cp
