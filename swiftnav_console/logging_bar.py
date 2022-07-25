"""Logging Bar QObjects.
"""

from typing import Dict, Any, List

from PySide6.QtCore import Property, QObject, QStringListModel, Signal, Slot, Qt
from PySide6.QtQml import QmlElement

from .constants import Keys, SbpLogging

QML_IMPORT_NAME = "SwiftConsole"
QML_IMPORT_MAJOR_VERSION = 1
# QML_IMPORT_MINOR_VERSION is optional


def logging_bar_update() -> Dict[str, Any]:
    return {
        Keys.PREVIOUS_FOLDERS: [],
        Keys.CSV_LOGGING: False,
        Keys.SBP_LOGGING: False,
        Keys.SBP_LOGGING_FORMAT: SbpLogging.SBP_JSON,
        Keys.SBP_LOGGING_LABELS: [SbpLogging.SBP_JSON, SbpLogging.SBP],
    }


def logging_bar_recording_update() -> Dict[str, Any]:
    return {
        Keys.RECORDING_DURATION_SEC: int,
        Keys.RECORDING_SIZE: float,
        Keys.RECORDING_FILENAME: str,
    }


LOGGING_BAR: List[Dict[str, Any]] = [logging_bar_update()]
LOGGING_BAR_RECORDING: List[Dict[str, Any]] = [logging_bar_recording_update()]


@QmlElement
class SwiftStringListModel(QStringListModel):
    @Slot(str, result=int)
    def indexOf(self, string: str) -> int:
        matchedIndices = self.match(self.index(0), Qt.EditRole, string)
        index = matchedIndices[0].row() if len(matchedIndices) > 0 else -1
        return index

    def __len__(self) -> int:
        return len(self.stringList())


@QmlElement
class LoggingBarData(QObject):  # pylint: disable=too-many-instance-attributes

    _csv_logging: bool = False
    _sbp_logging: bool = False
    _sbp_logging_format: str = SbpLogging.SBP_JSON
    _sbp_logging_labels: QStringListModel = SwiftStringListModel()
    _previous_folders: QStringListModel = SwiftStringListModel()
    _recording_duration_sec: int = 0
    _recording_size: float = 0
    _recording_filename: str = ""
    _data_updated = Signal()
    logging_bar: Dict[str, Any] = {}
    logging_bar_recording: Dict[str, Any] = {}

    sbp_logging_labels_changed = Signal(QObject)
    previous_folders_changed = Signal(QObject)

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.logging_bar = logging_bar_update()
        self.logging_bar_recording = logging_bar_recording_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        LOGGING_BAR[0] = update_data
        cls._instance._data_updated.emit()

    @classmethod
    def post_recording_data_update(cls, update_data: Dict[str, Any]) -> None:
        LOGGING_BAR_RECORDING[0] = update_data
        cls._instance._data_updated.emit()

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.logging_bar = LOGGING_BAR[0]
        self.logging_bar_recording = LOGGING_BAR_RECORDING[0]

    def get_csv_logging(self) -> bool:
        return self._csv_logging

    def set_csv_logging(self, csv_logging: bool) -> None:
        self._csv_logging = csv_logging

    csv_logging = Property(bool, get_csv_logging, set_csv_logging)

    def get_sbp_logging(self) -> bool:
        return self._sbp_logging

    def set_sbp_logging(self, sbp_logging: bool) -> None:
        self._sbp_logging = sbp_logging

    sbp_logging = Property(bool, get_sbp_logging, set_sbp_logging)

    def get_sbp_logging_format(self) -> str:
        return self._sbp_logging_format

    def set_sbp_logging_format(self, sbp_logging_format: str) -> None:
        self._sbp_logging_format = sbp_logging_format

    sbp_logging_format = Property(str, get_sbp_logging_format, set_sbp_logging_format)

    # sbp_logging_labels property
    def get_sbp_logging_labels(self) -> QObject:
        return self._sbp_logging_labels

    def set_sbp_logging_labels(self, sbp_logging_labels: List[str]) -> None:
        if len(self._sbp_logging_labels) == 0:
            self._sbp_logging_labels.setStringList(sbp_logging_labels)
            self.sbp_logging_labels_changed.emit(self._sbp_logging_labels)

    sbp_logging_labels = Property(QObject, get_sbp_logging_labels, notify=sbp_logging_labels_changed)  # type: ignore

    # previous_folders property
    def get_previous_folders(self) -> QObject:
        return self._previous_folders

    def set_previous_folders(self, previous_folders: List[str]) -> None:
        if previous_folders != self._previous_folders.stringList():
            self._previous_folders.setStringList(previous_folders)
            self.previous_folders_changed.emit(self._previous_folders)

    previous_folders = Property(QObject, get_previous_folders, notify=previous_folders_changed)  # type: ignore

    def get_recording_size(self) -> float:
        return self._recording_size

    def set_recording_size(self, recording_size: float) -> None:
        self._recording_size = recording_size

    # Using float type here to avoid overflow issues when converting to int, https://bugreports.qt.io/browse/PYSIDE-648.
    recording_size = Property(float, get_recording_size, set_recording_size)

    def get_recording_duration_sec(self) -> int:
        return self._recording_duration_sec

    def set_recording_duration_sec(self, recording_duration_sec: int) -> None:
        self._recording_duration_sec = recording_duration_sec

    recording_duration_sec = Property(int, get_recording_duration_sec, set_recording_duration_sec)  # type: ignore

    def get_recording_filename(self) -> str:
        return self._recording_filename

    def set_recording_filename(self, recording_filename: str) -> None:
        self._recording_filename = recording_filename

    recording_filename = Property(str, get_recording_filename, set_recording_filename)  # type: ignore


class LoggingBarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(LoggingBarData)  # type: ignore
    def fill_data(self, cp: LoggingBarData) -> LoggingBarData:  # pylint:disable=no-self-use
        cp.set_csv_logging(cp.logging_bar[Keys.CSV_LOGGING])
        cp.set_sbp_logging(cp.logging_bar[Keys.SBP_LOGGING])
        cp.set_sbp_logging_format(cp.logging_bar[Keys.SBP_LOGGING_FORMAT])
        cp.set_sbp_logging_labels(cp.logging_bar[Keys.SBP_LOGGING_LABELS])
        cp.set_previous_folders(cp.logging_bar[Keys.PREVIOUS_FOLDERS])
        cp.set_recording_size(cp.logging_bar_recording[Keys.RECORDING_SIZE])
        cp.set_recording_duration_sec(cp.logging_bar_recording[Keys.RECORDING_DURATION_SEC])
        cp.set_recording_filename(cp.logging_bar_recording[Keys.RECORDING_FILENAME])
        return cp
