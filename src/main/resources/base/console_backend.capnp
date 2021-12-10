@0xe7871c33e8243ee4;

struct TcpRequest {
    host @0 :Text;
    port @1 :UInt16;
}

struct FileRequest {
    filename @0 :Text;
}

struct SerialRequest {
    device @0 :Text;
    baudrate @1 :UInt32;
    flowControl @2 :Text;
}

struct SerialRefreshRequest {
    refresh @0 :Void = void;
}

struct DisconnectRequest {
    disconnect @0 :Void = void;
}

struct LogLevelFront {
    logLevel @0 :Text;
}

enum LogLevel {
    error @0;
    warn @1;
    info @2;
    debug @3;
    trace @4;
}

struct LogEntry {
    timestamp @0 :Text;
    level @1 :LogLevel;
    line @2 :Text;
}

struct LogAppend {
    entries @0 :List(LogEntry);
    logLevel @1: Text;
}

struct SkyPlotObs {
    az @0 :UInt16;
    el @1 :UInt16;
}

struct TrackingSkyPlotStatus {
    sats @0 :List(List(SkyPlotObs));
    labels @1 :List(List(Text));
}

struct KeyValPair {
    key @0 :Text;
    val @1 :Text;
}

struct SolutionTableStatus {
    data @0 :List(KeyValPair);
}

struct Setting {
    name @0 :Text;
    group @1 :Text;
    type @2 :Text;
    expert @3 :Bool;
    readonly @4 :Bool;
    description :union {
        description @5 :Text;
        noDescription @6 :Void;
    }
    defaultValue :union {
        defaultValue @7 :Text;
        noDefaultValue @8 :Void;
    }
    notes :union {
        notes @9 :Text;
        noNotes @10 :Void;
    }
    units :union {
        units @11 :Text;
        noUnits @12 :Void;
    }
    enumeratedPossibleValues :union {
        enumeratedPossibleValues @13 :Text;
        noEnumeratedPossibleValues @14 :Void;
    }
    digits :union {
        digits @15 :Text;
        noDigits @16 :Void;
    }
    valueOnDevice :union {
        valueOnDevice @17 :Text;
        noValueOnDevice @18 :Void;
    }
}

struct SettingsRow {
    union {
        setting @0 :Setting;
        group @1 :Text;
    }
}

struct SettingsTableStatus {
    data @0 :List(SettingsRow);
}

struct SettingsRefreshRequest {
    refresh @0 :Void = void;
}

struct SettingsSaveRequest {
    save @0 :Void = void;
}

struct SettingsExportRequest {
    path @0 :Text;
}

struct SettingsImportRequest {
    path @0 :Text;
}

struct SettingsImportResponse {
    status @0 :Text;
}

struct SettingsWriteRequest {
    group @0 :Text;
    name @1 :Text;
    value @2 :Text;
}

struct SettingsResetRequest {
    reset @0 :Void = void;
}

struct Point {
    x @0 :Float64;
    y @1 :Float64;
}

struct ConnectionStatus {
    availableBaudrates @0 : List(UInt32);
    availablePorts @1 : List(Text);
    availableFlows @2 : List(Text);
    previousHosts @3: List(Text);
    previousPorts @4: List(UInt16);
    previousFiles @5: List(Text);
    previousSerialConfigs @6: List(SerialRequest);
    lastSerialDevice: union {
        port @7 :Text;
        none @8 :Void = void;
    }
    consoleVersion @9: Text;
}

struct StatusBarStatus {
    antennaStatus @0 :Text;
    pos @1 : Text;
    rtk @2 : Text;
    sats @3: UInt8;
    corrAge @4 : Float64;
    ins @5: Text;
    dataRate @6: Float64;
    solidConnection @7: Bool;
    title @8: Text;
}

struct BaselinePlotStatus {
    data @0 :List(List(Point));
    nMin @1 :Float64;
    nMax @2 :Float64;
    eMin @3 :Float64;
    eMax @4 :Float64;
    curData @5 :List(List(Point));
}

struct BaselineTableStatus {
    data @0 :List(KeyValPair);
}

struct ObservationTableRow {
    code @0 :Text;
    pseudoRange @1 :Float64;
    carrierPhase @2 :Float64;
    cn0 @3 :Float64;
    measuredDoppler @4 :Float64;
    computedDoppler @5 :Float64;
    lock @6 :UInt16;
    flags @7 :UInt8;
    sat @8: Int16;
}

struct ObservationStatus {
    isRemote @0 :Bool;
    tow @1 :Float64;
    week @2 :UInt16;
    rows @3 :List(ObservationTableRow);
}

struct SolutionPositionStatus {
    data @0 :List(List(Point));
    latMin @1 :Float64;
    latMax @2 :Float64;
    lonMin @3 :Float64;
    lonMax @4 :Float64;
    curData @5 :List(List(Point));
    availableUnits @6 : List(Text);
}

struct SolutionVelocityStatus {
    min @0 :Float64;
    max @1 :Float64;
    data @2 :List(List(Point));
    availableUnits @3 : List(Text);
    colors @4 :List(Text);
}

struct ThreadState {
    name @0 :Text;
    cpu @1 :Float64;
    stackFree @2 :UInt32;
}

struct UartState {
    key @0 :Text;
    val @1 :Int32;
}

struct AdvancedSystemMonitorStatus {
    obsLatency @0 :List(UartState);
    obsPeriod @1 :List(UartState);
    threadsTable @2 :List(ThreadState);
    zynqTemp @3: Float64;
    feTemp @4: Float64;
    csacTelemList @5: List(KeyValPair);
    csacReceived @6: Bool;
}

struct NetworkState {
    interfaceName @0 :Text;
    ipv4Address @1 :Text;
    running @2 :Bool;
    txUsage @3 :Text;
    rxUsage @4 :Text;
}

struct AdvancedNetworkingStatus {
    networkInfo @0 :List(NetworkState);
    running @1 :Bool;
    ipAddress @2 :Text;
    port @3 :UInt16;
}

struct AdvancedNetworkingStatusFront {
    refresh @0 :Bool;
    start @1 :Bool;
    stop @2 :Bool;
    allMessages :union {
        toggle @3 :Bool;
        none @4 :Void = void;
    }
    ipv4Address :union {
        address @5 :Text;
        none @6 :Void = void;
    }
    port :union {
        port @7 :UInt16;
        none @8 :Void = void;
    }
}

struct TrackingSignalsStatus {
    xminOffset @0 :Float64;
    labels @1 :List(Text);
    data @2 :List(List(Point));
    colors @3 :List(Text);
    checkLabels @4 :List(Text);
}

struct AdvancedImuStatus {
    data @0 :List(List(Point));
    fieldsData @1 :List(Float64);
}

struct AdvancedMagnetometerStatus {
    data @0 :List(List(Point));
    ymin @1 :Float64;
    ymax @2 :Float64;
}

struct FusionStatusFlagsStatus {
    gnsspos @0 :Text;
    gnssvel @1 :Text;
    wheelticks @2 :Text;
    speed @3 :Text;
    nhc @4 :Text;
    zerovel @5 :Text;
}

struct AdvancedSpectrumAnalyzerStatus {
    ymin @0 :Float32;
    ymax @1 :Float32;
    xmin @2 :Float32;
    xmax @3 :Float32;
    data @4 :List(Point);
    channel @5 : UInt16;
}

struct LoggingBarFront {
    csvLogging @0 :Bool;
    sbpLogging @1 :Bool;
    sbpLoggingFormat @2 :Text;
    directory @3 :Text;
}

struct LoggingBarStatus {
    previousFolders @0 : List(Text);
    csvLogging @1 :Bool;
    sbpLogging @2 :Bool;
    sbpLoggingFormat @3 :Text;
}
struct LoggingBarRecordingStatus {
    recordingDurationSec @0 : UInt64;
    recordingSize @1 :Text;
    recordingFilename :union {
        filename @2 :Text;
        none @3 :Void = void;
    }
}

struct UpdateTabStatus {
    hardwareRevision @0 : Text;
    fwVersionCurrent @1 : Text;
    fwVersionLatest @2 : Text;
    fwLocalFilename @3: Text;
    directory @4 : Text;
    downloading @5 : Bool;
    upgrading @6 : Bool;
    fwText @7: Text;
    fileioDestinationFilepath @8: Text;
    fileioLocalFilepath @9: Text;
    fwOutdated @10: Bool;
    fwV2Outdated @11: Bool;
    serialPrompt @12: Bool;
    consoleOutdated @13: Bool;
    consoleVersionCurrent @14: Text;
    consoleVersionLatest @15: Text;
}

struct UpdateTabStatusFront {
    updateFirmware @0: Bool;
    downloadLatestFirmware @1 : Bool;
    sendFileToDevice @2: Bool;
    updateLocalFilepath :union {
        filepath @3 :Text;
        none @4 :Void;
    }
    downloadDirectory :union {
        directory @5 :Text;
        none @6 :Void;
    }
    fileioLocalFilepath :union {
        filepath @7 :Text;
        none @8 :Void;
    }
    fileioDestinationFilepath :union {
        filepath @9 :Text;
        none @10 :Void;
    }
    updateLocalFilename :union {
        filepath @11 :Text;
        none @12 :Void;
    }
    serialPromptConfirm @13: Bool;
}

struct TrackingSignalsStatusFront {
    trackingSignalsCheckVisibility @0 :List(Text);
}

struct SolutionVelocityStatusFront {
    solutionVelocityUnit @0 :Text;
}

struct AdvancedSpectrumAnalyzerStatusFront {
    channel @0 :UInt16;
}

struct AdvancedSystemMonitorStatusFront {
    resetDevice @0 :Void = void;
}

struct SolutionPositionStatusUnitFront {
    solutionPositionUnit @0 :Text;
}
struct SolutionPositionStatusButtonFront {
    solutionPositionCenter @0 :Bool;
    solutionPositionZoom @1 :Bool;
    solutionPositionClear @2 :Bool;
    solutionPositionPause @3 :Bool;
}

struct BaselinePlotStatusButtonFront {
    clear @0 :Bool;
    pause @1 :Bool;
    resetFilters @2 :Bool;
}

struct Status {
    text @0 :Text;
}

struct RecommendedInsSettingsRow {
    settingGroup @0 :Text;
    settingName @1 :Text;
    currentValue @2 :Text;
    recommendedValue @3 :Text;
}

struct InsSettingsChangeResponse {
    recommendedSettings @0 :List(RecommendedInsSettingsRow);
}

struct ConfirmInsChange {
    confirm @0 :Void = void;
}

struct AutoSurveyRequest {
    request @0 :Void = void;
}

struct Message {
    union {
        solutionVelocityStatus @0 :SolutionVelocityStatus;
        status @1 :Status;
        trackingSignalsStatus @2 :TrackingSignalsStatus;
        trackingSignalsStatusFront @3 :TrackingSignalsStatusFront;
        solutionVelocityStatusFront @4 :SolutionVelocityStatusFront;
        solutionTableStatus @5 :SolutionTableStatus;
        solutionPositionStatus @6 :SolutionPositionStatus;
        solutionPositionStatusButtonFront @7 :SolutionPositionStatusButtonFront;
        solutionPositionStatusUnitFront @8 :SolutionPositionStatusUnitFront;
        tcpRequest @9 :TcpRequest;
        fileRequest @10 :FileRequest;
        serialRequest @11 :SerialRequest;
        disconnectRequest @12 :DisconnectRequest;
        connectionStatus @13 :ConnectionStatus;
        serialRefreshRequest @14 :SerialRefreshRequest;
        logAppend @15 :LogAppend;
        observationStatus @16 :ObservationStatus;
        statusBarStatus @17 :StatusBarStatus;
        loggingBarFront @18 :LoggingBarFront;
        loggingBarStatus @19 :LoggingBarStatus;
        logLevelFront @20 :LogLevelFront;
        advancedImuStatus @21 :AdvancedImuStatus;
        fusionStatusFlagsStatus @22 :FusionStatusFlagsStatus;
        advancedMagnetometerStatus @23 :AdvancedMagnetometerStatus;
        baselinePlotStatus @24 :BaselinePlotStatus;
        baselineTableStatus @25 :BaselineTableStatus;
        baselinePlotStatusButtonFront @26 :BaselinePlotStatusButtonFront;
        advancedSpectrumAnalyzerStatus @27:AdvancedSpectrumAnalyzerStatus;
        advancedSpectrumAnalyzerStatusFront @28:AdvancedSpectrumAnalyzerStatusFront;
        updateTabStatus @29:UpdateTabStatus;
        updateTabStatusFront @30:UpdateTabStatusFront;
        settingsTableStatus @31 :SettingsTableStatus;
        settingsRefreshRequest @32 :SettingsRefreshRequest;
        settingsExportRequest @33 :SettingsExportRequest;
        settingsImportRequest @34 :SettingsImportRequest;
        settingsImportResponse @35 :SettingsImportResponse;
        settingsWriteRequest @36 :SettingsWriteRequest;
        settingsResetRequest @37 :SettingsResetRequest;
        settingsSaveRequest @38 :SettingsSaveRequest;
        advancedSystemMonitorStatus @39 :AdvancedSystemMonitorStatus;
        threadState @40 :ThreadState;
        uartState @41 :UartState;
        advancedSystemMonitorStatusFront @42 :AdvancedSystemMonitorStatusFront;
        skyPlotObs @43 :SkyPlotObs;
        trackingSkyPlotStatus @44 :TrackingSkyPlotStatus;
        advancedNetworkingStatus @45 :AdvancedNetworkingStatus;
        networkState @46 :NetworkState;
        advancedNetworkingStatusFront @47 :AdvancedNetworkingStatusFront;
        insSettingsChangeResponse @48 : InsSettingsChangeResponse;
        confirmInsChange @49 : ConfirmInsChange;
        autoSurveyRequest @50 : AutoSurveyRequest;
        loggingBarRecordingStatus @51 : LoggingBarRecordingStatus;
    }
}
