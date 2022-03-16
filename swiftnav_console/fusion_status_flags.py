"""Fusion Status Bar QObjects.
"""

from typing import Dict, Any

from PySide2.QtCore import Property, QObject, Signal, Slot

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


class FusionStatusFlagsData(QObject):

    _gnsspos: str = FusionStatus.UNKNOWN
    _gnssvel: str = FusionStatus.UNKNOWN
    _wheelticks: str = FusionStatus.UNKNOWN
    _speed: str = FusionStatus.UNKNOWN
    _nhc: str = FusionStatus.UNKNOWN
    _zerovel: str = FusionStatus.UNKNOWN
    _data_updated = Signal(dict)
    fusion_status_flags: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.fusion_status_flags = fusion_status_flags_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.fusion_status_flags = update_data

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
        cp.set_gnsspos(cp.fusion_status_flags[Keys.GNSSPOS])
        cp.set_gnssvel(cp.fusion_status_flags[Keys.GNSSVEL])
        cp.set_wheelticks(cp.fusion_status_flags[Keys.WHEELTICKS])
        cp.set_speed(cp.fusion_status_flags[Keys.SPEED])
        cp.set_nhc(cp.fusion_status_flags[Keys.NHC])
        cp.set_zerovel(cp.fusion_status_flags[Keys.ZEROVEL])
        return cp
