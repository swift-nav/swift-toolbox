"""Baseline Table QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, QTKeys


def baseline_table_update() -> Dict[str, Any]:
    return {
        Keys.ENTRIES: [],
    }


class BaselineTableEntries(QObject):

    _entries: List[List[str]] = []
    _data_updated = Signal(dict)
    baseline_table: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.baseline_table = baseline_table_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.baseline_table = update_data

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
        cp.set_entries(cp.baseline_table[Keys.ENTRIES])
        return cp
