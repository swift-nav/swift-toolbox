"""Solution Table QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Slot

from .constants import Keys, QTKeys


SOLUTION_TABLE: Dict[str, Any] = {
    Keys.ENTRIES: [],
}


class SolutionTableEntries(QObject):

    _entries: List[List[str]] = []
    _valid: bool = False

    def get_valid(self) -> bool:
        """Getter for _valid.

        Returns:
            bool: Whether it is valid or not.
        """
        return self._valid

    def set_valid(self, valid: bool) -> None:
        """Setter for _valid."""
        self._valid = valid

    valid = Property(bool, get_valid, set_valid)

    def get_entries(self) -> List[List[str]]:
        """Getter for _entries."""
        return self._entries

    def set_entries(self, entries: List[List[str]]) -> None:
        """Setter for _entries."""
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore


class SolutionTableModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SolutionTableEntries)  # type: ignore
    def fill_console_points(self, cp: SolutionTableEntries) -> SolutionTableEntries:  # pylint:disable=no-self-use
        cp.set_entries(SOLUTION_TABLE[Keys.ENTRIES])
        return cp
