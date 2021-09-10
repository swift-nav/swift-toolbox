"""Status Bar QObjects.
"""

from typing import Dict, Any

from PySide2.QtCore import Property, QObject, Slot

from constants import Keys

STATUS_BAR: Dict[str, Any] = {
    Keys.PORT: str,
    Keys.POS: str,
    Keys.RTK: str,
    Keys.SATS: str,
    Keys.CORR_AGE: str,
    Keys.INS: str,
    Keys.DATA_RATE: str,
    Keys.SOLID_CONNECTION: bool,
    Keys.TITLE: str,
}


class StatusBarData(QObject):  # pylint: disable=too-many-instance-attributes

    _port: str = ""
    _pos: str = ""
    _rtk: str = ""
    _sats: str = ""
    _corr_age: str = ""
    _ins: str = ""
    _data_rate: str = ""
    _solid_connection: bool = False
    _title: str = ""

    def get_port(self) -> str:
        return self._port

    def set_port(self, port: str) -> None:
        self._port = port

    port = Property(str, get_port, set_port)

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

    def get_sats(self) -> str:
        return self._sats

    def set_sats(self, sats: str) -> None:
        self._sats = sats

    sats = Property(str, get_sats, set_sats)

    def get_corr_age(self) -> str:
        return self._corr_age

    def set_corr_age(self, corr_age: str) -> None:
        self._corr_age = corr_age

    corr_age = Property(str, get_corr_age, set_corr_age)

    def get_ins(self) -> str:
        return self._ins

    def set_ins(self, ins: str) -> None:
        self._ins = ins

    ins = Property(str, get_ins, set_ins)

    def get_data_rate(self) -> str:
        return self._data_rate

    def set_data_rate(self, data_rate: str) -> None:
        self._data_rate = data_rate

    data_rate = Property(str, get_data_rate, set_data_rate)

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


class StatusBarModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(StatusBarData)  # type: ignore
    def fill_data(self, cp: StatusBarData) -> StatusBarData:  # pylint:disable=no-self-use
        cp.set_port(STATUS_BAR[Keys.PORT])
        cp.set_pos(STATUS_BAR[Keys.POS])
        cp.set_rtk(STATUS_BAR[Keys.RTK])
        cp.set_sats(STATUS_BAR[Keys.SATS])
        cp.set_corr_age(STATUS_BAR[Keys.CORR_AGE])
        cp.set_ins(STATUS_BAR[Keys.INS])
        cp.set_data_rate(STATUS_BAR[Keys.DATA_RATE])
        cp.set_solid_connection(STATUS_BAR[Keys.SOLID_CONNECTION])
        cp.set_title(STATUS_BAR[Keys.TITLE])
        return cp
