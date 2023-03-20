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

"""Fusion Status Bar QObjects.
"""

from typing import Dict, Any, List

from PySide6.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, FusionStatus


def fusion_status_flags_update() -> Dict[str, Any]:
    return {
        Keys.GNSSPOS: FusionStatus.UNKNOWN,
        Keys.GNSSVEL: FusionStatus.UNKNOWN,
        Keys.WHEELTICKS: FusionStatus.UNKNOWN,
        Keys.SPEED: FusionStatus.UNKNOWN,
        Keys.NHC: FusionStatus.UNKNOWN,
        Keys.ZEROVEL: FusionStatus.UNKNOWN,
    }


FUSION_STATUS_FLAGS: List[Dict[str, Any]] = [fusion_status_flags_update()]


class FusionStatusFlagsData(QObject):
    _instance: "FusionStatusFlagsData"
    _gnsspos: str = FusionStatus.UNKNOWN
    _gnssvel: str = FusionStatus.UNKNOWN
    _wheelticks: str = FusionStatus.UNKNOWN
    _speed: str = FusionStatus.UNKNOWN
    _nhc: str = FusionStatus.UNKNOWN
    _zerovel: str = FusionStatus.UNKNOWN
    _data_updated = Signal()
    fusion_status_flags: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.fusion_status_flags = fusion_status_flags_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        FUSION_STATUS_FLAGS[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.fusion_status_flags = FUSION_STATUS_FLAGS[0]
        self.update()  # type: ignore

    def get_gnsspos(self) -> str:
        return self._gnsspos

    def set_gnsspos(self, gnsspos: str) -> None:
        self._gnsspos = gnsspos

    gnsspos = Property(str, get_gnsspos, set_gnsspos)

    def get_gnssvel(self) -> str:
        return self._gnssvel

    def set_gnssvel(self, gnssvel: str) -> None:
        self._gnssvel = gnssvel

    gnssvel = Property(str, get_gnssvel, set_gnssvel)

    def get_wheelticks(self) -> str:
        return self._wheelticks

    def set_wheelticks(self, wheelticks: str) -> None:
        self._wheelticks = wheelticks

    wheelticks = Property(str, get_wheelticks, set_wheelticks)

    def get_speed(self) -> str:
        return self._speed

    def set_speed(self, speed: str) -> None:
        self._speed = speed

    speed = Property(str, get_speed, set_speed)

    def get_nhc(self) -> str:
        return self._nhc

    def set_nhc(self, nhc: str) -> None:
        self._nhc = nhc

    nhc = Property(str, get_nhc, set_nhc)

    def get_zerovel(self) -> str:
        return self._zerovel

    def set_zerovel(self, zerovel: str) -> None:
        self._zerovel = zerovel

    zerovel = Property(str, get_zerovel, set_zerovel)


class FusionStatusFlagsModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(FusionStatusFlagsData)  # type: ignore
    def fill_console_points(self, cp: FusionStatusFlagsData) -> FusionStatusFlagsData:
        cp.set_gnsspos(cp.fusion_status_flags[Keys.GNSSPOS])
        cp.set_gnssvel(cp.fusion_status_flags[Keys.GNSSVEL])
        cp.set_wheelticks(cp.fusion_status_flags[Keys.WHEELTICKS])
        cp.set_speed(cp.fusion_status_flags[Keys.SPEED])
        cp.set_nhc(cp.fusion_status_flags[Keys.NHC])
        cp.set_zerovel(cp.fusion_status_flags[Keys.ZEROVEL])
        return cp
