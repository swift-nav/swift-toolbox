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

"""Advanced System Monitor QObjects.
"""

from typing import Dict, List, Any

from PySide6.QtCore import Property, QObject, Signal, Slot

from .constants import Keys, QTKeys


def advanced_system_monitor_tab_update() -> Dict[str, Any]:
    return {
        Keys.OBS_PERIOD: [],
        Keys.OBS_LATENCY: [],
        Keys.THREADS_TABLE: [],
        Keys.ZYNQ_TEMP: 0.0,
        Keys.FE_TEMP: 0.0,
    }


ADVANCED_SYSTEM_MONITOR_TAB: List[Dict[str, Any]] = [advanced_system_monitor_tab_update()]


class AdvancedSystemMonitorData(QObject):  # pylint: disable=too-many-instance-attributes
    _instance: "AdvancedSystemMonitorData"
    _obs_period: List[List[Any]] = []
    _obs_latency: List[List[Any]] = []
    _threads_table: List[List[Any]] = []
    _zynq_temp: float = 0.0
    _fe_temp: float = 0.0
    _data_updated = Signal()
    advanced_system_monitor_tab: Dict[str, Any] = {}

    def __init__(self):
        super().__init__()
        assert getattr(self.__class__, "_instance", None) is None
        self.__class__._instance = self
        self.advanced_system_monitor_tab = ADVANCED_SYSTEM_MONITOR_TAB[0]
        self._data_updated.connect(self.handle_data_updated)

    @classmethod
    def post_data_update(cls, update_data: Dict[str, Any]) -> None:
        ADVANCED_SYSTEM_MONITOR_TAB[0] = update_data
        cls._instance._data_updated.emit()  # pylint: disable=protected-access

    @Slot()  # type: ignore
    def handle_data_updated(self) -> None:
        self.advanced_system_monitor_tab = ADVANCED_SYSTEM_MONITOR_TAB[0]

    def get_threads_table(self) -> List[List[str]]:
        """Getter for _threads_table."""
        return self._threads_table

    def set_threads_table(self, threads_table: List[List[str]]) -> None:
        """Setter for _threads_table."""
        self._threads_table = threads_table

    threads_table = Property(QTKeys.QVARIANTLIST, get_threads_table, set_threads_table)  # type: ignore

    def get_obs_latency(self) -> List[List[str]]:
        """Getter for _obs_latency."""
        return self._obs_latency

    def set_obs_latency(self, obs_latency: List[List[str]]) -> None:
        """Setter for _obs_latency."""
        self._obs_latency = obs_latency

    obs_latency = Property(QTKeys.QVARIANTLIST, get_obs_latency, set_obs_latency)  # type: ignore

    def get_obs_period(self) -> List[List[str]]:
        """Getter for _obs_period."""
        return self._obs_period

    def set_obs_period(self, obs_period: List[List[str]]) -> None:
        """Setter for _obs_period."""
        self._obs_period = obs_period

    obs_period = Property(QTKeys.QVARIANTLIST, get_obs_period, set_obs_period)  # type: ignore

    def get_zynq_temp(self) -> float:
        """Getter for _zynq_temp."""
        return self._zynq_temp

    def set_zynq_temp(self, zynq_temp_: float) -> None:
        """Setter for _zynq_temp."""
        self._zynq_temp = zynq_temp_

    zynq_temp = Property(float, get_zynq_temp, set_zynq_temp)

    def get_fe_temp(self) -> float:
        """Getter for _fe_temp."""
        return self._fe_temp

    def set_fe_temp(self, fe_temp_: float) -> None:
        """Setter for _fe_temp."""
        self._fe_temp = fe_temp_

    fe_temp = Property(float, get_fe_temp, set_fe_temp)


class AdvancedSystemMonitorModel(QObject):  # pylint: disable=too-few-public-methods
    @Slot(AdvancedSystemMonitorData)  # type: ignore
    def fill_console_points(self, cp: AdvancedSystemMonitorData) -> AdvancedSystemMonitorData:
        cp.set_obs_latency(cp.advanced_system_monitor_tab[Keys.OBS_LATENCY])
        cp.set_obs_period(cp.advanced_system_monitor_tab[Keys.OBS_PERIOD])
        cp.set_threads_table(cp.advanced_system_monitor_tab[Keys.THREADS_TABLE])
        cp.set_fe_temp(cp.advanced_system_monitor_tab[Keys.FE_TEMP])
        cp.set_zynq_temp(cp.advanced_system_monitor_tab[Keys.ZYNQ_TEMP])
        return cp
