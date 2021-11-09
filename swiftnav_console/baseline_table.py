"""Baseline Table QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Slot

from .constants import Keys, QTKeys


BASELINE_TABLE: Dict[str, Any] = {
    Keys.ENTRIES: [],
}


class BaselineTableEntries(QObject):

    _entries: List[List[str]] = []

    def get_entries(self) -> List[List[str]]:
        """Getter for _entries."""
        return self._entries

    def set_entries(self, entries: List[List[str]]) -> None:
        """Setter for _entries."""
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore


class BaselineTableModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(BaselineTableEntries)  # type: ignore
    def fill_console_points(self, cp: BaselineTableEntries) -> BaselineTableEntries:  # pylint:disable=no-self-use
        cp.set_entries(BASELINE_TABLE[Keys.ENTRIES])
        return cp
