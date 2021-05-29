"""Logging Bar QObjects.
"""

from typing import Dict, Any, List

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys, LogLevel, QTKeys, SbpLogging

LOGGING_BAR: Dict[str, Any] = {
    Keys.PREVIOUS_FOLDERS: [],
    Keys.CSV_LOGGING: False,
    Keys.SBP_LOGGING: SbpLogging.OFF,
    Keys.SBP_LOGGING_LABELS: [SbpLogging.OFF, SbpLogging.SBP_JSON, SbpLogging.SBP],
    Keys.LOG_LEVEL_LABELS: [LogLevel.ERROR, LogLevel.WARNING, LogLevel.NOTICE, LogLevel.INFO, LogLevel.DEBUG],
}


class LoggingBarData(QObject):  # pylint: disable=too-many-instance-attributes

    _csv_logging: bool = False
    _sbp_logging: str = SbpLogging.OFF
    _sbp_logging_labels: List[str] = []
    _log_level_labels: List[str] = []
    _previous_folders: List[str] = []

    def get_csv_logging(self) -> bool:
        return self._csv_logging

    def set_csv_logging(self, csv_logging: bool) -> None:
        self._csv_logging = csv_logging

    csv_logging = Property(bool, get_csv_logging, set_csv_logging)

    def get_sbp_logging(self) -> str:
        return self._sbp_logging

    def set_sbp_logging(self, sbp_logging: str) -> None:
        self._sbp_logging = sbp_logging

    sbp_logging = Property(str, get_sbp_logging, set_sbp_logging)

    def get_sbp_logging_labels(self) -> List[str]:
        return self._sbp_logging_labels

    def set_sbp_logging_labels(self, sbp_logging_labels: List[str]) -> None:
        self._sbp_logging_labels = sbp_logging_labels

    sbp_logging_labels = Property(QTKeys.QVARIANTLIST, get_sbp_logging_labels, set_sbp_logging_labels)  # type: ignore

    def get_log_level_labels(self) -> List[str]:
        return self._log_level_labels

    def set_log_level_labels(self, log_level_labels: List[str]) -> None:
        self._log_level_labels = log_level_labels

    log_level_labels = Property(QTKeys.QVARIANTLIST, get_log_level_labels, set_log_level_labels)  # type: ignore

    def get_previous_folders(self) -> List[str]:
        return self._previous_folders

    def set_previous_folders(self, previous_folders: List[str]) -> None:
        self._previous_folders = previous_folders

    previous_folders = Property(QTKeys.QVARIANTLIST, get_previous_folders, set_previous_folders)  # type: ignore


class LoggingBarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(LoggingBarData)  # type: ignore
    def fill_data(self, cp: LoggingBarData) -> LoggingBarData:  # pylint:disable=no-self-use
        cp.set_csv_logging(LOGGING_BAR[Keys.CSV_LOGGING])
        cp.set_sbp_logging(LOGGING_BAR[Keys.SBP_LOGGING])
        cp.set_log_level_labels(LOGGING_BAR[Keys.LOG_LEVEL_LABELS])
        cp.set_sbp_logging_labels(LOGGING_BAR[Keys.SBP_LOGGING_LABELS])
        cp.set_previous_folders(LOGGING_BAR[Keys.PREVIOUS_FOLDERS])
        return cp
