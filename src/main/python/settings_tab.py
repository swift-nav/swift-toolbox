"""Settings Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys, QTKeys


SETTINGS_TAB: Dict[str, Any] = {
    Keys.IMPORT_STATUS: None,
}


SETTINGS_TABLE: Dict[str, Any] = {
    Keys.ENTRIES: [],
}


class SettingsTabData(QObject):

    _import_status: str = ""

    def get_import_status(self) -> str:
        return self._import_status

    def set_import_status(self, import_status: str) -> None:
        self._import_status = import_status

    import_status = Property(str, get_import_status, set_import_status)


class SettingsTabModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SettingsTabData)  # type: ignore
    def fill_data(self, cp: SettingsTabData) -> SettingsTabData:  # pylint:disable=no-self-use
        cp.set_import_status(SETTINGS_TAB[Keys.IMPORT_STATUS])
        return cp

    @Slot(SettingsTabData)  # type: ignore
    def clear_import_status(self, cp: SettingsTabData) -> SettingsTabData:  # pylint:disable=no-self-use
        SETTINGS_TAB[Keys.IMPORT_STATUS] = ""
        self.fill_data(cp)
        return cp


class SettingsTableEntries(QObject):

    _entries: List[dict] = []

    def get_entries(self) -> List[dict]:
        return self._entries

    def set_entries(self, entries: List[dict]) -> None:
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
        return {"group": entry.group}


def settings_rows_to_json(rows):
    return [to_json(entry) for entry in rows]
