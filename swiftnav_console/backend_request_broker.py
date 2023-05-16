from typing import Any, List, Optional
from PySide2.QtCore import QObject, Slot

from .constants import QTKeys

PIKSI_HOST = "192.168.0.222"
PIKSI_PORT = 55555


class BackendRequestBroker(QObject):  # pylint: disable=too-many-instance-attributes,too-many-public-methods

    endpoint: Any
    messages: Any

    def __init__(self, endpoint, messages):
        super().__init__()
        self.endpoint = endpoint
        self.messages = messages

    @Slot()  # type: ignore
    def connect(self) -> None:
        self.connect_tcp(PIKSI_HOST, PIKSI_PORT)

    @Slot(str)  # type: ignore
    def connect_file(self, filename: str) -> None:
        Message = self.messages.Message
        msg = Message()
        msg.fileRequest = msg.init(Message.Union.FileRequest)
        msg.fileRequest.filename = str(filename)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int)  # type: ignore
    def connect_tcp(self, host: str, port: int) -> None:
        Message = self.messages.Message
        msg = Message()
        msg.tcpRequest = msg.init(Message.Union.TcpRequest)
        msg.tcpRequest.host = str(host)
        msg.tcpRequest.port = int(port)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, int, str)  # type: ignore
    def connect_serial(self, device: str, baudrate: int, flow_control: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.serialRequest = msg.init(Message.Union.SerialRequest)
        msg.serialRequest.device = str(device)
        msg.serialRequest.baudrate = int(baudrate)
        msg.serialRequest.flowControl = str(flow_control)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def disconnect(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.disconnectRequest = msg.init(Message.Union.DisconnectRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def serial_refresh(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.serialRefreshRequest = msg.init(Message.Union.SerialRefreshRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def settings_refresh(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsRefreshRequest = msg.init(Message.Union.SettingsRefreshRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def settings_reset_request(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsResetRequest = msg.init(Message.Union.SettingsResetRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def settings_save_request(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsSaveRequest = msg.init(Message.Union.SettingsSaveRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def settings_export_request(self, path: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsExportRequest = msg.init(Message.Union.SettingsExportRequest)
        msg.settingsExportRequest.path = path
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def settings_import_request(self, path: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsImportRequest = msg.init(Message.Union.SettingsImportRequest)
        msg.settingsImportRequest.path = path
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, str, str)  # type: ignore
    def settings_write_request(self, group: str, name: str, value: str) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.settingsWriteRequest = msg.init(Message.Union.SettingsWriteRequest)
        msg.settingsWriteRequest.group = group
        msg.settingsWriteRequest.name = name
        msg.settingsWriteRequest.value = value
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(bool)  # type: ignore
    def connection_dialog_status(self, visible: bool) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.connectionDialogStatus = msg.init(Message.Union.ConnectionDialogStatus)
        msg.connectionDialogStatus.visible = visible
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def reset_device(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.advancedSystemMonitorStatusFront = msg.init(Message.Union.AdvancedSystemMonitorStatusFront)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def confirm_ins_change(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.confirmInsChange = msg.init(Message.Union.ConfirmInsChange)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def tracking_signals_check_visibility(self, checks: List[str]) -> None:
        Message = self.messages.Message
        m = Message()
        m.trackingSignalsStatusFront = m.init(Message.Union.TrackingSignalsStatusFront)
        m.trackingSignalsStatusFront.trackingSignalsCheckVisibility = checks
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_velocity_unit(self, unit: str) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionVelocityStatusFront = m.init(Message.Union.SolutionVelocityStatusFront)
        m.solutionVelocityStatusFront.solutionVelocityUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(int)  # type: ignore
    def advanced_spectrum_analyzer_channel(self, channel: int) -> None:
        Message = self.messages.Message
        m = Message()
        m.advancedSpectrumAnalyzerStatusFront = m.init(Message.Union.AdvancedSpectrumAnalyzerStatusFront)
        m.advancedSpectrumAnalyzerStatusFront.channel = channel
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT)  # type: ignore
    def advanced_networking(
        self, buttons: list, all_messages_toggle: Optional[bool], ipv4_address: Optional[str], port: Optional[int]
    ) -> None:
        Message = self.messages.Message
        m = Message()
        m.advancedNetworkingStatusFront = m.init(Message.Union.AdvancedNetworkingStatusFront)
        m.advancedNetworkingStatusFront.refresh = buttons[0]
        m.advancedNetworkingStatusFront.start = buttons[1]
        m.advancedNetworkingStatusFront.stop = buttons[2]
        if all_messages_toggle is not None:
            m.advancedNetworkingStatusFront.allMessages.toggle = all_messages_toggle
        else:
            m.advancedNetworkingStatusFront.allMessages.none = None
        if ipv4_address is not None:
            m.advancedNetworkingStatusFront.ipv4Address.address = ipv4_address
        else:
            m.advancedNetworkingStatusFront.ipv4Address.none = None
        if port is not None:
            m.advancedNetworkingStatusFront.port.port = int(port)
        else:
            m.advancedNetworkingStatusFront.port.none = None
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def solution_position_unit(self, unit: str) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionPositionStatusUnitFront = m.init(Message.Union.SolutionPositionStatusUnitFront)
        m.solutionPositionStatusUnitFront.solutionPositionUnit = unit
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def solution_position(self, buttons: list) -> None:
        Message = self.messages.Message
        m = Message()
        m.solutionPositionStatusButtonFront = m.init(Message.Union.SolutionPositionStatusButtonFront)
        m.solutionPositionStatusButtonFront.solutionPositionPause = buttons[0]
        m.solutionPositionStatusButtonFront.solutionPositionClear = buttons[1]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list)  # type: ignore
    def baseline_plot(self, buttons: list) -> None:
        Message = self.messages.Message
        m = Message()
        m.baselinePlotStatusButtonFront = m.init(Message.Union.BaselinePlotStatusButtonFront)
        m.baselinePlotStatusButtonFront.pause = buttons[0]
        m.baselinePlotStatusButtonFront.clear = buttons[1]
        m.baselinePlotStatusButtonFront.resetFilters = buttons[2]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT)  # type: ignore
    def update_tab(
        self,
        buttons: list,
        update_local_filepath: Optional[str],
        download_directory: Optional[str],
        fileio_local_filepath: Optional[str],
        fileio_destination_filepath: Optional[str],
        update_local_filename: Optional[str],
    ) -> None:
        Message = self.messages.Message
        m = Message()
        m.updateTabStatusFront = m.init(Message.Union.UpdateTabStatusFront)
        if update_local_filepath is not None:
            m.updateTabStatusFront.updateLocalFilepath.filepath = str(update_local_filepath)
        else:
            m.updateTabStatusFront.updateLocalFilepath.none = None

        if download_directory is not None:
            m.updateTabStatusFront.downloadDirectory.directory = str(download_directory)
        else:
            m.updateTabStatusFront.downloadDirectory.none = None
        if fileio_local_filepath is not None:
            m.updateTabStatusFront.fileioLocalFilepath.filepath = str(fileio_local_filepath)
        else:
            m.updateTabStatusFront.fileioLocalFilepath.none = None

        if fileio_destination_filepath is not None:
            m.updateTabStatusFront.fileioDestinationFilepath.filepath = str(fileio_destination_filepath)
        else:
            m.updateTabStatusFront.fileioDestinationFilepath.none = None

        if update_local_filename is not None:
            m.updateTabStatusFront.updateLocalFilename.filepath = str(update_local_filename)
        else:
            m.updateTabStatusFront.updateLocalFilename.none = None

        m.updateTabStatusFront.downloadLatestFirmware = buttons[0]
        m.updateTabStatusFront.updateFirmware = buttons[1]
        m.updateTabStatusFront.sendFileToDevice = buttons[2]
        m.updateTabStatusFront.serialPromptConfirm = buttons[3]
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(list, str)  # type: ignore
    def logging_bar(self, buttons, directory) -> None:
        Message = self.messages.Message
        m = Message()
        m.loggingBarFront = m.init(Message.Union.LoggingBarFront)
        m.loggingBarFront.csvLogging = buttons[0]
        m.loggingBarFront.sbpLogging = buttons[1]
        m.loggingBarFront.sbpLoggingFormat = buttons[2]
        m.loggingBarFront.directory = directory
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str)  # type: ignore
    def log_level(self, log_level) -> None:
        Message = self.messages.Message
        m = Message()
        m.logLevelFront = m.init(Message.Union.LogLevelFront)
        m.logLevelFront.logLevel = str(log_level)
        buffer = m.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()  # type: ignore
    def auto_survey_request(self) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.autoSurveyRequest = msg.init(Message.Union.AutoSurveyRequest)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot(str, str, str, int, QTKeys.QVARIANT, QTKeys.QVARIANT, QTKeys.QVARIANT)  # type: ignore
    def ntrip_connect(
        self,
        url: str,
        username: str,
        password: str,
        gga_period: int,
        lat: Optional[float],
        lon: Optional[float],
        alt: Optional[float],
    ) -> None:
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.ntripConnect = msg.init(Message.Union.NtripConnect)
        msg.ntripConnect.url = url
        msg.ntripConnect.username = username
        msg.ntripConnect.password = password
        msg.ntripConnect.ggaPeriod = gga_period
        if lat is not None and lon is not None and alt is not None:
            msg.ntripConnect.position.pos.lat = lat
            msg.ntripConnect.position.pos.lon = lon
            msg.ntripConnect.position.pos.alt = alt
        else:
            msg.ntripConnect.position.none = None
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)

    @Slot()
    def ntrip_disconnect(self):
        Message = self.messages.Message
        msg = self.messages.Message()
        msg.ntripDisconnect = msg.init(Message.Union.NtripDisconnect)
        buffer = msg.to_bytes()
        self.endpoint.send_message(buffer)
