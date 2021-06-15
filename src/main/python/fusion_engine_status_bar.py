"""Fusion Engine Status Bar QObjects.
"""

from typing import Dict, Any

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys, FusionEngineStatus

FUSION_ENGINE_STATUS_BAR: Dict[str, Any] = {
    Keys.GNSSPOS: FusionEngineStatus.UNKNOWN,
    Keys.GNSSVEL: FusionEngineStatus.UNKNOWN,
    Keys.WHEELTICKS: FusionEngineStatus.UNKNOWN,
    Keys.SPEED: FusionEngineStatus.UNKNOWN,
    Keys.NHC: FusionEngineStatus.UNKNOWN,
    Keys.ZEROVEL: FusionEngineStatus.UNKNOWN,
}


class FusionEngineStatusBarData(QObject):

    _gnsspos: str = FusionEngineStatus.UNKNOWN
    _gnssvel: str = FusionEngineStatus.UNKNOWN
    _wheelticks: str = FusionEngineStatus.UNKNOWN
    _speed: str = FusionEngineStatus.UNKNOWN
    _nhc: str = FusionEngineStatus.UNKNOWN
    _zerovel: str = FusionEngineStatus.UNKNOWN

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


class FusionEngineStatusBarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(FusionEngineStatusBarData)  # type: ignore
    def fill_console_points(  # pylint:disable=no-self-use
        self, cp: FusionEngineStatusBarData
    ) -> FusionEngineStatusBarData:
        cp.set_gnsspos(FUSION_ENGINE_STATUS_BAR[Keys.GNSSPOS])
        cp.set_gnssvel(FUSION_ENGINE_STATUS_BAR[Keys.GNSSVEL])
        cp.set_wheelticks(FUSION_ENGINE_STATUS_BAR[Keys.WHEELTICKS])
        cp.set_speed(FUSION_ENGINE_STATUS_BAR[Keys.SPEED])
        cp.set_nhc(FUSION_ENGINE_STATUS_BAR[Keys.NHC])
        cp.set_zerovel(FUSION_ENGINE_STATUS_BAR[Keys.ZEROVEL])
        return cp
