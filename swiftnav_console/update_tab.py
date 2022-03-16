"""Status Bar QObjects.
"""

from typing import Any, Dict

from PySide2.QtCore import Property, QObject, Signal, Slot

from .constants import Keys


def update_tab_update() -> Dict[str, Any]:
    return {
        Keys.HARDWARE_REVISION: str,
        Keys.FW_VERSION_CURRENT: str,
        Keys.FW_VERSION_LATEST: str,
        Keys.FW_LOCAL_FILENAME: str,
        Keys.DIRECTORY: str,
        Keys.DOWNLOADING: bool,
        Keys.UPGRADING: bool,
        Keys.FW_TEXT: str,
        Keys.FILEIO_LOCAL_FILEPATH: str,
        Keys.FILEIO_DESTINATION_FILEPATH: str,
        Keys.FW_OUTDATED: bool,
        Keys.FW_V2_OUTDATED: bool,
        Keys.SERIAL_PROMPT: bool,
        Keys.CONSOLE_OUTDATED: bool,
        Keys.CONSOLE_VERSION_CURRENT: str,
        Keys.CONSOLE_VERSION_LATEST: str,
    }


class UpdateTabData(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods

    _hardware_revision: str = ""
    _fw_version_current: str = ""
    _fw_version_latest: str = ""
    _fw_local_filename: str = ""
    _directory: str = ""
    _downloading: bool = False
    _upgrading: bool = False
    _fw_text: str = ""
    _fileio_local_filepath: str = ""
    _fileio_destination_filepath: str = ""
    _fw_outdated: bool = False
    _fw_v2_outdated: bool = False
    _serial_prompt: bool = False
    _console_outdated: bool = False
    _console_version_current: str = ""
    _console_version_latest: str = ""
    _data_updated: Signal = Signal(dict)
    update_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.update_tab = update_tab_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.update_tab = update_data

    def get_hardware_revision(self) -> str:
        return self._hardware_revision

    def set_hardware_revision(self, hardware_revision: str) -> None:
        self._hardware_revision = hardware_revision

    hardware_revision = Property(str, get_hardware_revision, set_hardware_revision)

    def get_fw_version_current(self) -> str:
        return self._fw_version_current

    def set_fw_version_current(self, fw_version_current: str) -> None:
        self._fw_version_current = fw_version_current

    fw_version_current = Property(str, get_fw_version_current, set_fw_version_current)

    def get_fw_version_latest(self) -> str:
        return self._fw_version_latest

    def set_fw_version_latest(self, fw_version_latest: str) -> None:
        self._fw_version_latest = fw_version_latest

    fw_version_latest = Property(str, get_fw_version_latest, set_fw_version_latest)

    def get_fw_local_filename(self) -> str:
        return self._fw_local_filename

    def set_fw_local_filename(self, fw_local_filename: str) -> None:
        self._fw_local_filename = fw_local_filename

    fw_local_filename = Property(str, get_fw_local_filename, set_fw_local_filename)

    def get_directory(self) -> str:
        return self._directory

    def set_directory(self, directory: str) -> None:
        self._directory = directory

    directory = Property(str, get_directory, set_directory)

    def get_downloading(self) -> bool:
        return self._downloading

    def set_downloading(self, downloading: bool) -> None:
        self._downloading = downloading

    downloading = Property(bool, get_downloading, set_downloading)

    def get_upgrading(self) -> bool:
        return self._upgrading

    def set_upgrading(self, upgrading: bool) -> None:
        self._upgrading = upgrading

    upgrading = Property(bool, get_upgrading, set_upgrading)

    def get_fw_text(self) -> str:
        return self._fw_text

    def set_fw_text(self, fw_text: str) -> None:
        self._fw_text = fw_text

    fw_text = Property(str, get_fw_text, set_fw_text)

    def get_fileio_local_filepath(self) -> str:
        return self._fileio_local_filepath

    def set_fileio_local_filepath(self, fileio_local_filepath: str) -> None:
        self._fileio_local_filepath = fileio_local_filepath

    fileio_local_filepath = Property(str, get_fileio_local_filepath, set_fileio_local_filepath)

    def get_fileio_destination_filepath(self) -> str:
        return self._fileio_destination_filepath

    def set_fileio_destination_filepath(self, fileio_destination_filepath: str) -> None:
        self._fileio_destination_filepath = fileio_destination_filepath

    fileio_destination_filepath = Property(str, get_fileio_destination_filepath, set_fileio_destination_filepath)

    def get_fw_outdated(self) -> bool:
        return self._fw_outdated

    def set_fw_outdated(self, fw_outdated: bool) -> None:
        self._fw_outdated = fw_outdated

    fw_outdated = Property(bool, get_fw_outdated, set_fw_outdated)

    def get_fw_v2_outdated(self) -> bool:
        return self._fw_v2_outdated

    def set_fw_v2_outdated(self, fw_v2_outdated: bool) -> None:
        self._fw_v2_outdated = fw_v2_outdated

    fw_v2_outdated = Property(bool, get_fw_v2_outdated, set_fw_v2_outdated)

    def get_serial_prompt(self) -> bool:
        return self._serial_prompt

    def set_serial_prompt(self, serial_prompt: bool) -> None:
        self._serial_prompt = serial_prompt

    serial_prompt = Property(bool, get_serial_prompt, set_serial_prompt)

    def get_console_outdated(self) -> bool:
        return self._console_outdated

    def set_console_outdated(self, console_outdated: bool) -> None:
        self._console_outdated = console_outdated

    console_outdated = Property(bool, get_console_outdated, set_console_outdated)

    def get_console_version_current(self) -> str:
        return self._console_version_current

    def set_console_version_current(self, console_version_current: str) -> None:
        self._console_version_current = console_version_current

    console_version_current = Property(str, get_console_version_current, set_console_version_current)

    def get_console_version_latest(self) -> str:
        return self._console_version_latest

    def set_console_version_latest(self, console_version_latest: str) -> None:
        self._console_version_latest = console_version_latest

    console_version_latest = Property(str, get_console_version_latest, set_console_version_latest)


class UpdateTabModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(UpdateTabData)  # type: ignore
    def fill_data(self, cp: UpdateTabData) -> UpdateTabData:  # pylint:disable=no-self-use
        cp.set_hardware_revision(cp.update_tab[Keys.HARDWARE_REVISION])
        cp.set_fw_version_current(cp.update_tab[Keys.FW_VERSION_CURRENT])
        cp.set_fw_version_latest(cp.update_tab[Keys.FW_VERSION_LATEST])
        cp.set_fw_local_filename(cp.update_tab[Keys.FW_LOCAL_FILENAME])
        cp.set_directory(cp.update_tab[Keys.DIRECTORY])
        cp.set_downloading(cp.update_tab[Keys.DOWNLOADING])
        cp.set_upgrading(cp.update_tab[Keys.UPGRADING])
        cp.set_fw_text(cp.update_tab[Keys.FW_TEXT])
        cp.set_fileio_local_filepath(cp.update_tab[Keys.FILEIO_LOCAL_FILEPATH])
        cp.set_fileio_destination_filepath(cp.update_tab[Keys.FILEIO_DESTINATION_FILEPATH])
        cp.set_fw_outdated(cp.update_tab[Keys.FW_OUTDATED])
        cp.set_fw_v2_outdated(cp.update_tab[Keys.FW_V2_OUTDATED])
        cp.set_serial_prompt(cp.update_tab[Keys.SERIAL_PROMPT])
        cp.set_console_outdated(cp.update_tab[Keys.CONSOLE_OUTDATED])
        cp.set_console_version_current(cp.update_tab[Keys.CONSOLE_VERSION_CURRENT])
        cp.set_console_version_latest(cp.update_tab[Keys.CONSOLE_VERSION_LATEST])
        return cp
