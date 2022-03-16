"""Status Bar QObjects.
"""

from typing import Dict, Any

from PySide2.QtCore import Property, QObject, Signal, Slot

from .constants import Keys


def status_bar_update() -> Dict[str, Any]:
    return {
        Keys.POS: str,
        Keys.RTK: str,
        Keys.SATS: int,
        Keys.CORR_AGE: float,
        Keys.INS: str,
        Keys.DATA_RATE: float,
        Keys.SOLID_CONNECTION: bool,
        Keys.TITLE: str,
        Keys.ANTENNA_STATUS: str,
    }


class StatusBarData(QObject):  # pylint: disable=too-many-instance-attributes

    _pos: str = ""
    _rtk: str = ""
    _sats: int = 0
    _corr_age: float = 0.0
    _ins: str = ""
    _data_rate: float = 0.0
    _solid_connection: bool = False
    _title: str = ""
    _antenna_status: str = ""
    _data_updated = Signal(dict)
    status_bar: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.status_bar = status_bar_update()
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        cls._instance._data_updated.emit(update_data)

    @Slot(dict)  # type: ignore
    def handle_data_updated(self, update_data: Dict[str, Any]) -> None:
        self.status_bar = update_data

    def get_pos(self) -> str:
        return self._pos

    def set_pos(self, pos: str) -> None:
        self._pos = pos

    pos = Property(str, get_pos, set_pos)

    def get_rtk(self) -> str:
        return self._rtk

    def set_rtk(self, rtk: str) -> None:
        self._rtk = rtk

    rtk = Property(str, get_rtk, set_rtk)

    def get_sats(self) -> int:
        return self._sats

    def set_sats(self, sats: int) -> None:
        self._sats = sats

    sats = Property(int, get_sats, set_sats)

    def get_corr_age(self) -> float:
        return self._corr_age

    def set_corr_age(self, corr_age: float) -> None:
        self._corr_age = corr_age

    corr_age = Property(float, get_corr_age, set_corr_age)

    def get_ins(self) -> str:
        return self._ins

    def set_ins(self, ins: str) -> None:
        self._ins = ins

    ins = Property(str, get_ins, set_ins)

    def get_data_rate(self) -> float:
        return self._data_rate

    def set_data_rate(self, data_rate: float) -> None:
        self._data_rate = data_rate

    data_rate = Property(float, get_data_rate, set_data_rate)

    def get_solid_connection(self) -> bool:
        return self._solid_connection

    def set_solid_connection(self, solid_connection: bool) -> None:
        self._solid_connection = solid_connection

    solid_connection = Property(bool, get_solid_connection, set_solid_connection)

    def get_title(self) -> str:
        return self._title

    def set_title(self, title: str) -> None:
        self._title = title

    title = Property(str, get_title, set_title)

    def get_antenna_status(self) -> str:
        return self._antenna_status

    def set_antenna_status(self, antenna_status: str) -> None:
        self._antenna_status = antenna_status

    antenna_status = Property(str, get_antenna_status, set_antenna_status)


class StatusBarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(StatusBarData)  # type: ignore
    def fill_data(self, cp: StatusBarData) -> StatusBarData:  # pylint:disable=no-self-use
        cp.set_pos(cp.status_bar[Keys.POS])
        cp.set_rtk(cp.status_bar[Keys.RTK])
        cp.set_sats(cp.status_bar[Keys.SATS])
        cp.set_corr_age(cp.status_bar[Keys.CORR_AGE])
        cp.set_ins(cp.status_bar[Keys.INS])
        cp.set_data_rate(cp.status_bar[Keys.DATA_RATE])
        cp.set_solid_connection(cp.status_bar[Keys.SOLID_CONNECTION])
        cp.set_title(cp.status_bar[Keys.TITLE])
        cp.set_antenna_status(cp.status_bar[Keys.ANTENNA_STATUS])
        return cp
