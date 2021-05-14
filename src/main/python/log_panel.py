"""Nav Bar QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QMutex, QObject, Slot

from constants import Keys, QTKeys


LOG_PANEL: Dict[str, Any] = {
    Keys.ENTRIES: [],
}
log_panel_lock = QMutex()


class LogPanelData(QObject):
    _entries: List[str] = []

    def get_entries(self) -> List[str]:
        """Getter for _entries."""
        return self._entries

    def set_entries(self, entries: List[str]) -> None:
        """Setter for _entries."""
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore

    def append_entries(self, entries: List[str]) -> None:
        self._entries += entries


class LogPanelModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(LogPanelData)  # type: ignore
    def fill_data(self, cp: LogPanelData) -> LogPanelData:  # pylint:disable=no-self-use
        # Avoid locking so that message processor has priority to lock
        if LOG_PANEL[Keys.ENTRIES]:
            if log_panel_lock.try_lock():
                cp.append_entries(LOG_PANEL[Keys.ENTRIES])
                LOG_PANEL[Keys.ENTRIES][:] = []
                log_panel_lock.unlock()
        return cp
