"""Fusion Status Bar QObjects.
"""

from typing import Dict, Any

from PySide6.QtCore import Property, QObject, Slot

from .constants import Keys, FusionStatus

FUSION_STATUS_FLAGS: Dict[str, Any] = {
    Keys.GNSSPOS: FusionStatus.UNKNOWN,
    Keys.GNSSVEL: FusionStatus.UNKNOWN,
    Keys.WHEELTICKS: FusionStatus.UNKNOWN,
    Keys.SPEED: FusionStatus.UNKNOWN,
    Keys.NHC: FusionStatus.UNKNOWN,
    Keys.ZEROVEL: FusionStatus.UNKNOWN,
}


class FusionStatusFlagsData(QObject):

    _gnsspos: str = FusionStatus.UNKNOWN
    _gnssvel: str = FusionStatus.UNKNOWN
    _wheelticks: str = FusionStatus.UNKNOWN
    _speed: str = FusionStatus.UNKNOWN
    _nhc: str = FusionStatus.UNKNOWN
    _zerovel: str = FusionStatus.UNKNOWN

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
    def fill_console_points(self, cp: FusionStatusFlagsData) -> FusionStatusFlagsData:  # pylint:disable=no-self-use
        cp.set_gnsspos(FUSION_STATUS_FLAGS[Keys.GNSSPOS])
        cp.set_gnssvel(FUSION_STATUS_FLAGS[Keys.GNSSVEL])
        cp.set_wheelticks(FUSION_STATUS_FLAGS[Keys.WHEELTICKS])
        cp.set_speed(FUSION_STATUS_FLAGS[Keys.SPEED])
        cp.set_nhc(FUSION_STATUS_FLAGS[Keys.NHC])
        cp.set_zerovel(FUSION_STATUS_FLAGS[Keys.ZEROVEL])
        return cp
