"""Status Bar QObjects.
"""

from typing import Dict, Any

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys

UPDATE_TAB: Dict[str, Any] = {
    Keys.HARDWARE_REVISION: str,
    Keys.FW_VERSION_CURRENT: str,
    Keys.FW_VERSION_LATEST: str,
    Keys.FW_LOCAL_FILENAME: str,
    Keys.DIRECTORY: str,
    Keys.DOWNLOADING: bool,
}

class UpdateTabData(QObject):  # pylint: disable=too-many-instance-attributes

    _hardware_revision: str = ""
    _fw_version_current: str = ""
    _fw_version_latest: str = ""
    _fw_local_filename: str = ""
    _directory: str = ""
    _downloading: bool = False

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


class UpdateTabModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(UpdateTabData)  # type: ignore
    def fill_data(self, cp: UpdateTabData) -> UpdateTabData:  # pylint:disable=no-self-use
        cp.set_hardware_revision(UPDATE_TAB[Keys.HARDWARE_REVISION])
        cp.set_fw_version_current(UPDATE_TAB[Keys.FW_VERSION_CURRENT])
        cp.set_fw_version_latest(UPDATE_TAB[Keys.FW_VERSION_LATEST])
        cp.set_fw_local_filename(UPDATE_TAB[Keys.FW_LOCAL_FILENAME])
        cp.set_directory(UPDATE_TAB[Keys.DIRECTORY])
        cp.set_downloading(UPDATE_TAB[Keys.DOWNLOADING])
        return cp
