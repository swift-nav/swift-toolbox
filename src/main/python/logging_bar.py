"""Logging Bar QObjects.
"""

from typing import Dict, Any, List

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys, QTKeys

LOGGING_BAR: Dict[str, Any] = {
    Keys.FOLDER: str,
    Keys.PREVIOUS_FOLDERS: [],
}


class LoggingBarData(QObject):  # pylint: disable=too-many-instance-attributes

    _folder: str = ""
    _previous_folders: List[str] = []

    def get_folder(self) -> str:
        return self._folder

    def set_folder(self, folder: str) -> None:
        self._folder = folder

    folder = Property(str, get_folder, set_folder)

    def get_previous_folders(self) -> List[str]:
        return self._previous_folders

    def set_previous_folders(self, previous_folders: List[str]) -> None:
        self._previous_folders = previous_folders

    previous_folders = Property(QTKeys.QVARIANTLIST, get_previous_folders, set_previous_folders)  # type: ignore


class LoggingBarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(LoggingBarData)  # type: ignore
    def fill_data(self, cp: LoggingBarData) -> LoggingBarData:  # pylint:disable=no-self-use
        cp.set_folder(LOGGING_BAR[Keys.FOLDER])
        cp.set_previous_folders(LOGGING_BAR[Keys.PREVIOUS_FOLDERS])
        return cp
