"""Solution Table QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, QTKeys


def solution_table_update() -> Dict[str, Any]:
    return {
        Keys.ENTRIES: [],
    }


SOLUTION_TABLE: List[Dict[str, Any]] = [solution_table_update()]


class SolutionTableEntries(QObject):

    _entries: List[List[str]] = []
    _valid: bool = False
    _data_updated = Signal()
    solution_table: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.solution_table = SOLUTION_TABLE[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        SOLUTION_TABLE[0] = update_data
        cls._instance._data_updated.emit()

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.solution_table = SOLUTION_TABLE[0]

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
        cp.set_entries(cp.solution_table[Keys.ENTRIES])
        return cp
