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

"""Settings Tab QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, QTKeys


def settings_ins_update() -> Dict[str, Any]:
    return {
        Keys.RECOMMENDED_INS_SETTINGS: [],
        Keys.NEW_INS_CONFIRMATON: False,
    }


def settings_table_update() -> Dict[str, Any]:
    return {
        Keys.ENTRIES: [],
    }


SETTINGS_IMPORT_STATUS: List[str] = [""]
SETTINGS_INS: List[Dict[str, Any]] = [settings_ins_update()]
SETTINGS_NOTIFICATION: List[str] = [""]
SETTINGS_TABLE: List[Dict[str, Any]] = [settings_table_update()]


class SettingsTabData(QObject):
    _instance: "SettingsTabData"
    _import_status: str = ""
    _recommended_ins_settings: List[List[Any]] = []
    _new_ins_confirmation: bool = False
    _notification: str = ""
    _data_updated = Signal()
    settings_import_status: str = ""
    settings_ins: Dict[str, Any] = {}
    settings_notification: str = ""

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.settings_import_status = SETTINGS_IMPORT_STATUS[0]
        self.settings_ins = SETTINGS_INS[0]
        self.settings_notification = SETTINGS_NOTIFICATION[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_import_status_update(cls, update_data: str) -> None:
        SETTINGS_IMPORT_STATUS[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @classmethod
    def post_ins_update(cls, update_data: Dict[str, Any]) -> None:
        SETTINGS_INS[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @classmethod
    def post_notification_update(cls, update_data: str) -> None:
        SETTINGS_NOTIFICATION[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.settings_import_status = SETTINGS_IMPORT_STATUS[0]
        self.settings_ins = SETTINGS_INS[0]
        self.settings_notification = SETTINGS_NOTIFICATION[0]
        self.update()

    def get_import_status(self) -> str:
        return self._import_status

    def set_import_status(self, import_status: str) -> None:
        self._import_status = import_status

    import_status = Property(str, get_import_status, set_import_status)

    def get_recommended_ins_settings(self) -> List[List[str]]:
        return self._recommended_ins_settings

    def set_recommended_ins_settings(self, recommended_ins_settings: List[List[str]]) -> None:
        self._recommended_ins_settings = recommended_ins_settings

    recommended_ins_settings = Property(
        QTKeys.QVARIANTLIST, get_recommended_ins_settings, set_recommended_ins_settings  # type: ignore
    )

    def set_new_ins_confirmation(self, new_ins_confirmation: bool) -> None:
        self._new_ins_confirmation = new_ins_confirmation

    def get_new_ins_confirmation(self) -> bool:
        return self._new_ins_confirmation

    new_ins_confirmation = Property(bool, get_new_ins_confirmation, set_new_ins_confirmation)

    def get_notification(self) -> str:
        return self._notification

    def set_notification(self, notification: str) -> None:
        self._notification = notification

    notification = Property(str, get_notification, set_notification)


class SettingsTabModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SettingsTabData)  # type: ignore
    def fill_data(self, cp: SettingsTabData) -> SettingsTabData:
        cp.set_import_status(cp.settings_import_status)
        cp.set_recommended_ins_settings(cp.settings_ins[Keys.RECOMMENDED_INS_SETTINGS])
        cp.set_new_ins_confirmation(cp.settings_ins[Keys.NEW_INS_CONFIRMATON])
        cp.set_notification(cp.settings_notification)
        cp.settings_notification = ""
        return cp

    @Slot(SettingsTabData)  # type: ignore
    def clear_import_status(self, cp: SettingsTabData) -> SettingsTabData:
        cp.settings_import_status = ""
        self.fill_data(cp)
        return cp

    @Slot(SettingsTabData)  # type: ignore
    def clear_new_ins_confirmation(self, cp: SettingsTabData) -> SettingsTabData:
        cp.settings_ins[Keys.NEW_INS_CONFIRMATON] = False
        self.fill_data(cp)
        return cp


class SettingsTableEntries(QObject):
    _instance: "SettingsTableEntries"
    _entries: List[dict] = []
    _data_updated = Signal()
    settings_table: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.settings_table = SETTINGS_TABLE[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        SETTINGS_TABLE[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.settings_table = SETTINGS_TABLE[0]
        self.update()  # type: ignore

    def get_entries(self) -> List[dict]:
        return self._entries

    def set_entries(self, entries: List[dict]) -> None:
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore


class SettingsTableModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SettingsTableEntries)  # type: ignore
    def fill_console_points(self, cp: SettingsTableEntries) -> SettingsTableEntries:
        cp.set_entries(cp.settings_table[Keys.ENTRIES])
        return cp


def to_json(entry):
    def handle_null(name):
        e = getattr(entry.setting, name)
        if e.which() == name:
            return getattr(e, name)
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
    return {"group": entry.group}


def settings_rows_to_json(rows):
    return [to_json(entry) for entry in rows]
