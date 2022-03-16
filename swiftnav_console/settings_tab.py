"""Settings Tab QObjects.
"""

from typing import Dict, List, Any

from PySide2.QtCore import Property, QObject, Signal, Slot

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


class SettingsTabData(QObject):

    _import_status: str = ""
    _recommended_ins_settings: List[List[Any]] = []
    _new_ins_confirmation: bool = False
    _notification: str = ""
    _import_status_updated = Signal(str)
    _ins_updated = Signal(dict)
    _notification_updated = Signal(str)
    settings_import_status: str = ""
    settings_ins: Dict[str, Any] = {}
    settings_notification: str = ""

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.settings_import_status = ""
        self.settings_ins = settings_ins_update()
        self.settings_notification = ""
        self._import_status_updated.connect(self.handle_import_status_updated)
        self._ins_updated.connect(self.handle_ins_updated)
        self._notification_updated.connect(self.handle_notification_updated)

    @classmethod
    def post_import_status_update(cls, update_data: str) -> None:
        cls._instance._import_status_updated.emit(update_data)

    @classmethod
    def post_ins_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._ins_updated.emit(update_data)

    @classmethod
    def post_notification_update(cls, update_data: str) -> None:
        cls._instance._notification_updated.emit(update_data)

    @Slot(str)  # type: ignore
    def handle_import_status_updated(self, data: str) -> None:
        self.settings_import_status = data

    @Slot(dict)  # type: ignore
    def handle_ins_updated(self, update_data: Dict[str, Any]) -> None:
        self.settings_ins = update_data

    @Slot(str)  # type: ignore
    def handle_notification_updated(self, data: str) -> None:
        self.settings_notification = data

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
    def fill_data(self, cp: SettingsTabData) -> SettingsTabData:  # pylint:disable=no-self-use
        cp.set_import_status(cp.settings_import_status)
        cp.set_recommended_ins_settings(cp.settings_ins[Keys.RECOMMENDED_INS_SETTINGS])
        cp.set_new_ins_confirmation(cp.settings_ins[Keys.NEW_INS_CONFIRMATON])
        cp.set_notification(cp.settings_notification)
        cp.settings_notification = ""
        return cp

    @Slot(SettingsTabData)  # type: ignore
    def clear_import_status(self, cp: SettingsTabData) -> SettingsTabData:  # pylint:disable=no-self-use
        cp.settings_import_status = ""
        self.fill_data(cp)
        return cp

    @Slot(SettingsTabData)  # type: ignore
    def clear_new_ins_confirmation(self, cp: SettingsTabData) -> SettingsTabData:  # pylint:disable=no-self-use
        cp.settings_ins[Keys.NEW_INS_CONFIRMATON] = False
        self.fill_data(cp)
        return cp


class SettingsTableEntries(QObject):

    _entries: List[dict] = []
    _data_updated = Signal(dict)
    settings_table: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.settings_table = settings_table_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.settings_table = update_data

    def get_entries(self) -> List[dict]:
        return self._entries

    def set_entries(self, entries: List[dict]) -> None:
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore


class SettingsTableModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(SettingsTableEntries)  # type: ignore
    def fill_console_points(self, cp: SettingsTableEntries) -> SettingsTableEntries:  # pylint:disable=no-self-use
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
