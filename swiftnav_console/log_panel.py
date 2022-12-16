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

"""Log Panel QObjects.
"""

import json

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Signal, Slot
from PySide6.QtQml import QmlElement

from .constants import Keys, LogLevel, QTKeys

QML_IMPORT_NAME = "SwiftConsole"
QML_IMPORT_MAJOR_VERSION = 1


def log_panel_update() -> Dict[str, Any]:
    return {
        Keys.ENTRIES: [],
        Keys.LOG_LEVEL_LABELS: [LogLevel.ERROR, LogLevel.WARNING, LogLevel.NOTICE, LogLevel.INFO, LogLevel.DEBUG],
        Keys.LOG_LEVEL: LogLevel.WARNING,
    }


LOG_PANEL: List[Dict[str, Any]] = [log_panel_update()]


@QmlElement
class LogPanelData(QObject):
    _instance: "LogPanelData"
    _entries: List[Dict[str, str]] = []
    _log_level_labels: List[str] = []
    _log_level: str
    data_updated = Signal()
    log_panel: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.log_panel = LOG_PANEL[0]
        self.data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        LOG_PANEL[0] = update_data
        cls._instance.data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.log_panel = LOG_PANEL[0]

    def get_log_level_labels(self) -> List[str]:
        return self._log_level_labels

    def set_log_level_labels(self, log_level_labels: List[str]) -> None:
        self._log_level_labels = log_level_labels

    log_level_labels = Property(QTKeys.QVARIANTLIST, get_log_level_labels, set_log_level_labels)  # type: ignore

    def get_log_level(self) -> str:
        return self._log_level

    def set_log_level(self, log_level: str) -> None:
        self._log_level = log_level

    log_level = Property(str, get_log_level, set_log_level)

    def get_entries(self) -> List[Dict[str, str]]:
        """Getter for _entries."""
        return self._entries

    def set_entries(self, entries: List[Dict[str, str]]) -> None:
        """Setter for _entries."""
        self._entries = entries

    entries = Property(QTKeys.QVARIANTLIST, get_entries, set_entries)  # type: ignore

    def append_entries(self, entries: List[Dict[str, str]]) -> None:
        self._entries += entries


@QmlElement
class LogPanelModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(LogPanelData)  # type: ignore
    def fill_data(self, cp: LogPanelData) -> LogPanelData:  # pylint:disable=no-self-use
        cp.set_log_level_labels(cp.log_panel[Keys.LOG_LEVEL_LABELS])
        cp.set_log_level(cp.log_panel[Keys.LOG_LEVEL])
        # Avoid locking so that message processor has priority to lock
        if cp.log_panel[Keys.ENTRIES]:
            entries = []
            for entry in cp.log_panel[Keys.ENTRIES]:
                entries.append(json.loads(entry))
            cp.append_entries(entries)
            cp.log_panel[Keys.ENTRIES][:] = []
        return cp
