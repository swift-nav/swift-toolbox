"""Solution Table QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys, QTKeys


SETTINGS_TABLE: Dict[str, Any] = {
    Keys.ENTRIES: [],
}


class SettingsTableEntries(QObject):

    _entries: List[dict] = []

    def get_entries(self) -> List[dict]:
        """Getter for _entries."""
        return self._entries

    def set_entries(self, entries: List[dict]) -> None:
        """Setter for _entries."""
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore


class SettingsTableModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SettingsTableEntries)  # type: ignore
    def fill_console_points(self, cp: SettingsTableEntries) -> SettingsTableEntries:  # pylint:disable=no-self-use
        cp.set_entries(SETTINGS_TABLE[Keys.ENTRIES])
        return cp


def to_json(entry):
    def handle_null(name):
        e = getattr(entry.setting, name)
        if e.which() == name:
            return getattr(e, name)
        else:
            return False
    if entry.which() == "setting":
        return {
            "name": entry.setting.name,
            "group": entry.setting.group,
            "type": entry.setting.type,
            "expert": entry.setting.expert,
            "readonly": entry.setting.readonly,
            "description": handle_null("description"),
            "defaultValue": handle_null("defaultValue"),
            "notes": handle_null("notes"),
            "units": handle_null("units"),
            "enumeratedPossibleValues": handle_null("enumeratedPossibleValues"),
            "digits": handle_null("digits"),
            "valueOnDevice": handle_null("valueOnDevice"),
        }
    else:
        return { "group": entry.group }


def settings_rows_to_json(rows):
    return [to_json(entry) for entry in rows]
