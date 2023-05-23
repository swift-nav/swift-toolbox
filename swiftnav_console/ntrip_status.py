"""Ntrip Status QObjects.
"""

from PySide2.QtCore import QObject, Signal

class NtripStatusData(QObject):  # pylint: disable=too-many-instance-attributes
    _instance: "NtripStatusData"
    ntrip_connected: "Signal"

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self

    @classmethod
    def post_connected(cls, connected: bool) -> None:
        cls._instance.ntrip_connected.emit(connected)
