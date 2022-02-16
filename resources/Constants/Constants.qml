import QtQml 2.15
import QtQuick 2.6
import QtQuick.Controls.Material 2.12
pragma Singleton

QtObject {
    readonly property real tabBarWidth: 70
    readonly property real tabBarHeight: 45
    readonly property real topLevelSpacing: 0
    property QtObject mainTabs
    property QtObject tabInfoBar
    property QtObject logPanel
    property QtObject statusBar
    property QtObject connection
    property QtObject sideNavBar
    property QtObject loggingBar
    property QtObject commonChart
    property QtObject commonLegend
    property QtObject commonTable
    property QtObject advancedImu
    property QtObject advancedMagnetometer
    property QtObject advancedSpectrumAnalyzer
    property QtObject baselinePlot
    property QtObject baselineTable
    property QtObject settingsTab
    property QtObject settingsTable
    property QtObject insSettingsPopup
    property QtObject solutionPosition
    property QtObject solutionTable
    property QtObject solutionVelocity
    property QtObject trackingSignals
    property QtObject observationTab
    property QtObject systemMonitor
    property QtObject genericTable
    property QtObject updateTab
    property QtObject icons
    property QtObject trackingSkyPlot
    property QtObject networking
    property QtObject fusionStatusFlags
    property QtObject logoPopup
    readonly property int staticTimerIntervalRate: 5 // 5 Hz
    readonly property int staticTableTimerIntervalRate: 10 // 10 Hz
    readonly property int staticTimerSlowIntervalRate: 2 // 2 Hz
    readonly property int staticTimerNotificationIntervalRate: 1 // 1 Hz
    readonly property string monoSpaceFont: "Courier New"
    readonly property string fontFamily: "Roboto Condensed"
    property FontLoader robotoCondensedLightFont
    readonly property string lightFontFamily: robotoCondensedLightFont.name
    readonly property real fontScaleFactor: Qt.platform.os == "osx" ? 1.5 : 1
    readonly property real xSmallPointSize: fontScaleFactor * 6
    readonly property real smallPointSize: fontScaleFactor * 7
    readonly property real mediumPointSize: fontScaleFactor * 8
    readonly property real largePointSize: fontScaleFactor * 9
    readonly property real xlPointSize: fontScaleFactor * 12
    readonly property real xxlPointSize: fontScaleFactor * 14
    readonly property bool debugMode: false
    readonly property color swiftWhite: "#FFFFFF"
    readonly property color swiftGrey: "#323F48"
    readonly property color swiftLightGrey: "#3C464F"
    readonly property color swiftControlBackground: "#E0E0E0"
    readonly property color tabButtonUnselectedTextColor: "#767676"
    readonly property color materialGrey: "dimgrey"
    readonly property color swiftOrange: "#FF8300"
    readonly property color spacerColor: "#C2C2C2"

    robotoCondensedLightFont: FontLoader {
        source: "qrc:/fonts/RobotoCondensed-Light.ttf"
    }

    mainTabs: QtObject {
        readonly property int horizontalMargins: 4
        readonly property int verticalMargins: 4
    }

    tabInfoBar: QtObject {
        readonly property bool autoClose: false
        readonly property color tabLabelColor: swiftOrange
        readonly property font tabLabelFont: Qt.font({
            "family": "Roboto Condensed",
            "pointSize": 20,
            "bold": true,
            "letterSpacing": 1,
            "capitalization": Font.AllUppercase
        })
        readonly property string appName: "Console"
        readonly property color appNameColor: swiftLightGrey
        readonly property font appNameFont: Qt.font({
            "family": robotoCondensedLightFont.name,
            "pointSize": 20,
            "letterSpacing": 2,
            "capitalization": Font.AllUppercase
        })
        readonly property string infoButtonIconPath: "qrc:/images/fontawesome/info-circle-solid.svg"
        readonly property color infoButtonIconColor: swiftLightGrey
        readonly property int height: 50
    }

    logoPopup: QtObject {
        readonly property int heightPadding: 120
        readonly property int buttonWidth: 30
        readonly property int buttonRightMargin: 15
        readonly property int buttonTopMargin: 10
        property QtObject licenses
        property QtObject aboutMe

        licenses: QtObject {
            readonly property int dropdownHeight: 40
            readonly property string fontFamily: "Roboto"
            readonly property string robotoFontTabLabel: "Roboto Font"
            readonly property string fontAwesomeIconsTabLabel: "Font Awesome Icons"
            readonly property string robotoFontLicensePath: ":/fonts/Roboto-LICENSE.txt"
            readonly property string fontAwesomeIconsLicensePath: ":/images/fontawesome/LICENSE.txt"
        }

        aboutMe: QtObject {
            readonly property int logoWidth: 200
            readonly property int bottomPadding: 20
            readonly property string supportWebsite: "https://www.swiftnav.com/support"
            readonly property string website: "https://www.swiftnav.com"
            readonly property string copyrightText: "Copyright © 2011-2022 Swift Navigation Inc."
            readonly property int titlePointSize: 14
            readonly property int secondaryPointSize: 10
        }

    }

    sideNavBar: QtObject {
        readonly property int buttonSvgHeight: 15
        readonly property real tabBarHeight: 48
        readonly property real tabBarWidth: 62
        readonly property int buttonPadding: 0
        readonly property int buttonInset: 0
        readonly property int separatorMargin: 10
        readonly property int separatorHeight: 1
        readonly property color backgroundColor: swiftGrey
        readonly property color statusGoodColor: "#07DD01"
        readonly property color statusOkColor: "yellow"
        readonly property color statusBadColor: "red"
    }

    updateTab: QtObject {
        readonly property int outerMargins: 10
        readonly property int innerMargins: 10
        readonly property int textHeight: 20
        readonly property int labelTextAreaSpacing: 10
        readonly property int hardwareRevisionLabelWidth: 100
        readonly property int hardwareVersionElementsLabelWidth: 50
        readonly property int firmwareVersionElementsLabelRightMargin: 5
        readonly property string hardwareRevisionLabel: "Hardware Revision:"
        readonly property string firmwareVersionCurrentLabel: "Current:"
        readonly property string firmwareVersionLatestLabel: "Latest:"
        readonly property string firmwareDownloadDirectoryLabel: "Directory:"
        readonly property string firmwareVersionLocalFileLabel: "Local File:"
        readonly property string fileioDestinationPathLabel: "Destination Path:"
        readonly property string fileioLocalFileLabel: "Local File:"
        readonly property string firmwareVersionTitle: "Firmware Version"
        readonly property string firmwareDownloadTitle: "Firmware Download"
        readonly property string firmwareUpgradeStatusTitle: "Firmware Upgrade Status"
        readonly property string firmwareVersionLocalFilePlaceholderText: "Enter a local file path"
        readonly property string fileioAndProductFeatureToolTitle: "File IO and product feature unlock tool"
        readonly property string updateFirmwareButtonLabel: "Update Firmware"
        readonly property string downloadLatestFirmwareButtonLabel: "Download Latest Firmware"
        readonly property string fileioSendFileToDeviceButtonLabel: "Send File To Device"
        readonly property color placeholderTextColor: "grey"
        readonly property int borderWidth: 1
        readonly property int firmwareVersionColumnSpacing: 0
        readonly property int buttonInset: 0
        readonly property int firmwareVersionLocalFileButtonSpacing: 5
        readonly property int firmwareVersionLocalFileButtonWidth: 50
        readonly property int fileioDestinationPathButtonWidth: 150
        readonly property string dotDotDotLabel: "..."
        readonly property int popupLargeHeight: 275
        readonly property int popupSmallHeight: 230
        readonly property int consoleVersionDialogWidth: 450
        readonly property int fwVersionDialogWidth: 300
        readonly property int upgradeSerialDialogWidth: 450
        readonly property int v2DownloadDialogWidth: 300
        readonly property int popupDelayMilliseconds: 3000
    }

    systemMonitor: QtObject {
        readonly property var columnHeaders: ["Thread Name", "CPU %", "Stack Free"]
        readonly property var metricColumnHeaders: ["Metric", "Value"]
        readonly property string currLabel: "Curr"
        readonly property string maxLabel: "Max"
        readonly property string minLabel: "Min"
        readonly property string avgLabel: "Avg"
        readonly property string obsUnits: "ms"
        readonly property int rows: 6
        readonly property int columns: 6
        readonly property int columnSpacing: 0
        readonly property int rowSpacing: 10
        readonly property int topRowSpan: 2
        readonly property int bottomRowSpan: 4
        readonly property int deviceMonitorColumnSpan: 1
        readonly property int metricsMonitorColumnSpan: 3
        readonly property int observationConnectionMonitorColumnSpan: 2
        readonly property int resetButtonHeight: 50
        readonly property int resetButtonWidth: 100
        readonly property string resetButtonLabel: "Reset Device"
        readonly property int resetButtonIconSideLength: 10
        readonly property int obsTextMargins: 5
        readonly property int textHeight: 20
        readonly property string obsLatencyLabel: "Latency"
        readonly property string obsPeriodLabel: "Period"
        readonly property string zynqTempLabel: "Zynq CPU Temp"
        readonly property string feTempLabel: "RF Frontend Temp"
        readonly property string tempUnits: "C"
        readonly property var defaultObs: {
            "Curr": 0,
            "Avg": 0,
            "Min": 0,
            "Max": 0
        }
        readonly property var defaultThreadsList: {
            "Thread Name": "",
            "CPU %": "",
            "Stack Free": ""
        }
        readonly property var defaultMetricsList: {
            "Metric": "",
            "Value": ""
        }
    }

    networking: QtObject {
        readonly property var columnHeaders: ["Interface Name", "IPv4 Addr", "Running", "Tx Usage", "Rx Usage"]
        readonly property var defaultList: {
            "Interface Name": "",
            "IPv4 Addr": "",
            "Running": "",
            "Tx Usage": "",
            "Rx Usage": ""
        }
        readonly property int refreshButtonHeight: 50
        readonly property int refreshButtonVerticalOffset: 10
        readonly property int refreshButtonWidth: 200
        readonly property int refreshButtonIconSideLength: 12
        readonly property string refreshButtonLabel: "Refresh Network Status"
        readonly property int messageBroadcasterHeight: 150
        readonly property int layoutSpacing: 0
        readonly property int udpStreamingParagraphPadding: 10
        readonly property int messageBroadcasterMargins: 10
        readonly property int messageBroadcasterGridRows: 4
        readonly property int messageBroadcasterGridColumns: 2
        readonly property int messageBroadcasterGridElementLength: 1
        readonly property int messageBroadcasterStartStopButtonHeight: 20
        readonly property int messageBroadcasterTextInputHeight: 20
        readonly property int messageBroadcasterIntValidatorUInt16Min: 0
        readonly property int messageBroadcasterIntValidatorUInt16Max: 65535
    }

    fusionStatusFlags: QtObject {
        readonly property int spacing: 20
        readonly property int fusionStatusWidth: 80
        readonly property int labelMargin: 5
        readonly property int labelFontSize: 14
        readonly property int titleFontSize: 16
        readonly property string title: "Fusion Status"
    }

    genericTable: QtObject {
        readonly property int headerZOffset: 100
        readonly property int padding: 2
        readonly property int leftPadding: 5
        readonly property int rightPadding: 5
        readonly property int borderWidth: 1
        readonly property int mouseAreaResizeWidth: 10
        readonly property int cellHeight: 25
        readonly property string cellHighlightedColor: swiftOrange
        readonly property color textColor: "black"
        readonly property color cellColor: "white"
        readonly property color gradientColor: "gainsboro"
        readonly property color selectedCellColor: "dark grey"
        readonly property color borderColor: "gainsboro"
        readonly property string fontFamily: "Roboto Condensed"
        property var defaultColumns: ["Item", "Value"]
    }

    statusBar: QtObject {
        readonly property int verticalPadding: 6
        readonly property int leftMargin: 15
        readonly property int spacing: 20
        readonly property color borderColor: "black"
        readonly property int borderWidth: 0
        readonly property color textColor: "black"
        readonly property int textPointSize: largePointSize + 1
        readonly property int keyValueSpacing: 5
        readonly property int valueMinimumWidth: 25
        readonly property string portLabel: "Port:"
        readonly property string posLabel: "Position:"
        readonly property string rtkLabel: "RTK:"
        readonly property string satsLabel: "Sats:"
        readonly property string corrAgeLabel: "Correction Age:"
        readonly property string insLabel: "INS:"
        readonly property string antennaLabel: "Antenna:"
        readonly property string defaultValue: "--"
    }

    connection: QtObject {
        readonly property int connectionDropdownWidth: 120
        readonly property int serialSelectionDropdownWidth: 200
        readonly property int dropdownHeight: 40
        readonly property int buttonHeight: 40
        readonly property int buttonSvgHeight: 15
        readonly property int urlBarHeight: 25
        readonly property int connectionMargin: 10
        readonly property int plotRefreshRateDropdownWidth: 50
        readonly property int serialDeviceBaudRateDropdownWidth: 90
        readonly property int serialDeviceFlowControlDropdownWidth: 130
        readonly property int serialDeviceRefreshWidth: 30
        readonly property int connectButtonWidth: 30
        readonly property int connectionPauseWidth: 30
        readonly property int folderButtonWidth: 30
        readonly property int logLevelButtonWidth: 110
        readonly property color placeholderTextColor: "#CDC9C9"
        readonly property int padding: 0
        readonly property string connected: "connected"
        readonly property string connecting: "connecting"
        readonly property string disconnected: "disconnected"
        readonly property string disconnecting: "disconnecting"
        readonly property int warningTimerLockedInterval: 7000
        readonly property int labelRowSpacing: -5
        readonly property int labelLeftMargin: 4
        readonly property string serialLabel: "Serial Device:"
        readonly property string flowLabel: "Flow Control:"
        readonly property string baudrateLabel: "Baudrate:"
        readonly property string hostLabel: "Host:"
        readonly property string portLabel: "Port:"
        readonly property string fileLabel: "File:"
    }

    loggingBar: QtObject {
        readonly property int preferredHeight: 50
        readonly property int buttonHeight: 40
        readonly property real buttonHeightRatio: 2 / 3
        readonly property int buttonSvgHeight: 15
        readonly property int urlBarHeight: 25
        readonly property int loggingBarMargin: 10
        readonly property int sbpLoggingButtonWidth: 120
        readonly property int csvLoggingButtonWidth: 120
        readonly property int folderButtonWidth: 30
        readonly property int directoryBarBorder: 1
        readonly property int directoryBarTextMargin: 10
        readonly property int folderPathBarHeight: 25
        readonly property color placeholderTextColor: "#CDC9C9"
        readonly property string folderButtonPath: "qrc:/images/fontawesome/folder-solid.svg"
        readonly property int recordingLabelWidth: 60
        readonly property int recordingTimeLabelWidth: 60
        readonly property int recordingDividerLabelWidth: 10
        readonly property int recordingSizeLabelWidth: 40
        readonly property font comboBoxFont: Qt.font({
            "family": fontFamily,
            "pointSize": mediumPointSize
        })
    }

    advancedMagnetometer: QtObject {
        readonly property string title: "Raw Magnetometer Data"
        readonly property var legendLabels: ["Mag. X (uT)", "Mag. Y (uT)", "Mag. Z (uT)"]
        readonly property var lineColors: ["#66c2a5", "#fc8d62", "#8da0cb"]
        readonly property int xAxisMax: 200
        readonly property int xAxisMin: 0
        readonly property int xAxisTickCount: 25
        readonly property int yAxisTickCount: 20
        readonly property int legendBottomMargin: 60
        readonly property int legendLeftMargin: 50
        readonly property string suggestionText: "Enable in Settings tab under the \"imu\" group."
        readonly property int suggestionTextRowHeight: 20
        readonly property int yAxisPadding: 10
    }

    advancedSpectrumAnalyzer: QtObject {
        readonly property string title: "Spectrum Analyzer"
        readonly property var lineColors: ["#000000"]
        readonly property int xAxisTickCount: 10
        readonly property real yAxisTickCount: 2.5
        readonly property string yAxisTitleText: "Amplitude (dB)"
        readonly property string xAxisTitleText: "Frequency (MHz)"
        readonly property int dropdownRowHeight: 20
        readonly property int dropdownHeight: 20
        readonly property int dropdownWidth: 100
        readonly property var dropdownModel: ["Channel 1", "Channel 2", "Channel 3", "Channel 4"]
        readonly property string dropdownLabel: "Channel Selection:"
        readonly property string dropdownRowSuggestionText: "Enable in Settings tab under the \"system_monitor\" group."
        readonly property int rowTextHeight: 30
        readonly property int rowTextMargins: 5
    }

    advancedImu: QtObject {
        readonly property string title: "Raw IMU Data"
        readonly property var textDataLabels: ["Imu temp:", "Imu conf:", "Rms acc x:", "Rms acc y:", "Rms acc z:"]
        readonly property var insStatusLabels: ["GNSS Pos:", "GNSS Vel:", "Wheelticks:", "Wheelspeed:", "nhc:", "Static Detection:"]
        readonly property var legendLabels: ["Accn. X", "Accn. Y", "Accn. Z", "Gyro X", "Gyro Y", "Gyro Z"]
        readonly property var lineColors: ["#e41a1c", "#377eb8", "#4daf4a", "#984ea3", "#ff7f00", "#ffff33"]
        readonly property int legendBottomMargin: 120
        readonly property int legendLeftMargin: 80
        readonly property int yAxisTickCount: 10000
        readonly property int xAxisTickCount: 25
        readonly property int xAxisMax: 200
        readonly property int xAxisMin: 0
        readonly property int yAxisMax: 32768
        readonly property int yAxisMin: -32768
        readonly property int textDataLabelWidth: 50
        readonly property int textDataRowHeight: 25
        readonly property int textDataBarHeight: 20
        readonly property int textDataBarMargin: 2
        readonly property int textDataBarBorderWidth: 1
        readonly property string unknownStatusPath: "qrc:/images/fontawesome/square-solid.svg"
        readonly property color unknownStatusColor: "dimgrey"
        readonly property string warningStatusPath: "qrc:/images/fontawesome/exclamation-triangle-solid.svg"
        readonly property color warningStatusColor: "goldenrod"
        readonly property string okStatusPath: "qrc:/images/fontawesome/circle-solid.svg"
        readonly property color okStatusColor: "green"
        readonly property int insStatusImageWidth: 15
        readonly property int urlBarHeight: 25
    }

    baselinePlot: QtObject {
        readonly property int minimumWidth: 410
        readonly property int buttonSvgHeight: 15
        readonly property int navBarMargin: 10
        readonly property int navBarSpacing: 0
        readonly property real navBarButtonWidth: 40
        readonly property real resetFiltersButtonWidth: 100
        readonly property int axesDefaultMin: 0
        readonly property int axesDefaultMax: 1
        readonly property string yAxisTitleText: "N (meters)"
        readonly property string xAxisTitleText: "E (meters)"
        readonly property var legendLabels: ["Base Position", "DGPS", "RTK Float", "RTK Fixed"]
        readonly property var colors: ["#FF0000", "#00B3FF", "#BF00BF", "#FFA500"]
    }

    baselineTable: QtObject {
        readonly property int minimumWidth: 240
        readonly property int defaultColumnWidth: 100
        readonly property color tableBorderColor: "#000000"
        readonly property int tableBorderWidth: 1
        readonly property int tableHeaderTableDataTableSpacing: 0
        readonly property int tableCellHeight: 20
        readonly property int tableCellSpacing: 0
        readonly property int tableSurroundingMargin: 2
        readonly property int tableInnerMargin: 0
        readonly property int tableLeftPadding: 2
        readonly property string tableLeftColumnHeader: "Item"
        readonly property string tableRightColumnHeader: "Value"
    }

    solutionPosition: QtObject {
        readonly property int minimumWidth: 410
        readonly property int buttonSvgHeight: 15
        readonly property int navBarMargin: 10
        readonly property int navBarSpacing: 0
        readonly property int axesDefaultMin: 0
        readonly property int axesDefaultMax: 1
        readonly property real navBarButtonProportionOfParent: 0.11
        readonly property string yAxisTitleText: "Latitude"
        readonly property string xAxisTitleText: "Longitude"
        readonly property var legendLabels: ["SPP", "SBAS", "DGPS", "RTK Float", "RTK Fixed", "DR"]
        readonly property int legendLabelSpacing: 4
        readonly property var colors: ["#FF0000", "#FF00FF", "#00FFFF", "#0000FF", "#00FF00", "#000000"]
    }

    logPanel: QtObject {
        readonly property int preferredHeight: 100
        readonly property int width: 220
        readonly property variant defaultColumnWidthRatios: [0.15, 0.1, 0.75]
        readonly property int maxRows: 200
        readonly property int cellHeight: 20
        readonly property string timestampHeader: "Host Timestamp"
        readonly property string levelHeader: "Log Level"
        readonly property string msgHeader: "Message"
        readonly property int zAboveTable: 100
        readonly property int pauseButtonRightMargin: 25
        readonly property int pauseButtonWidth: 30
        readonly property int pauseButtonPadding: 0
        readonly property string pauseButtonTooltip: "Pause Log Panel"
        readonly property string playButtonTooltip: "Resume Log Panel"
        readonly property string clearButtonTooltip: "Clear Log Panel"
        readonly property int logLevelMenuHeight: 100
        readonly property int logLevelRightMargin: 10
        readonly property int dropdownButtonPadding: 0
        readonly property int dropdownButtonWidth: 20
        readonly property int delegateBorderWidth: 0
    }

    settingsTab: QtObject {
        readonly property int textSettingWidth: 550
        readonly property int buttonIconWidth: 20
        readonly property int buttonIconHeight: 20
        readonly property int paneSmallRowHeight: 15
        readonly property int paneScrollBufferHeight: 100
        readonly property string defaultImportExportRelativePathFromHome: "SwiftNav"
        readonly property string defaultExportFileName: "config.ini"
    }

    settingsTable: QtObject {
        readonly property string tableLeftColumnHeader: "Name"
        readonly property string tableRightColumnHeader: "Value"
        readonly property int minimumWidth: 320
        readonly property int layoutSpacing: 0
        readonly property int settingEditTimeoutMilliseconds: 1000
    }

    solutionTable: QtObject {
        readonly property int minimumWidth: 240
        readonly property int defaultColumnWidth: 100
        readonly property color tableBorderColor: "#000000"
        readonly property int tableBorderWidth: 1
        readonly property int tableHeaderTableDataTableSpacing: 0
        readonly property int tableCellHeight: 20
        readonly property int tableCellSpacing: 0
        readonly property int tableSurroundingMargin: 2
        readonly property int tableInnerMargin: 0
        readonly property int tableLeftPadding: 2
        readonly property string tableLeftColumnHeader: "Item"
        readonly property string tableRightColumnHeader: "Value"
        readonly property int rtkNoteHeight: 65
        readonly property int rtkNoteMargins: 2
        readonly property int rtkNoteBorderWidth: 1
        readonly property string rtkNoteText: "It is necessary to enter the \"Surveyed Position\" settings for the base station in order to view the RTK Positions in this tab."
    }

    solutionVelocity: QtObject {
        readonly property var labels: ["Horizontal", "Vertical"]
        readonly property string xAxisTitleText: "GPS Time of Week"
        readonly property int xAxisMinOffsetFromMaxSeconds: 20
        readonly property int unitDropdownWidth: 75
        readonly property int chartHeightOffset: 0
        readonly property int legendBottomMargin: 120
        readonly property int legendLeftMargin: 80
        readonly property int legendLabelSpacing: 4
    }

    commonLegend: QtObject {
        readonly property int bottomMargin: 120
        readonly property int leftMargin: 80
        readonly property int markerHeight: 3
        readonly property int markerWidth: 20
        readonly property int topMargin: 85
        readonly property int rightMargin: 60
        readonly property real markerPointSizeOffset: 4
        readonly property int labelPointSize: 10
        readonly property int padding: 10
        readonly property int spacing: 5
        readonly property int verticalCenterOffset: -1
        readonly property color borderColor: "#7F7F7F"
        readonly property int borderWidth: 1
        readonly property int borderRadius: 5
        readonly property int shadeHeight: 10
        readonly property color shadeColor: Qt.lighter(swiftOrange, 1.5)
        readonly property int shadeSpeed: 350
    }

    commonChart: QtObject {
        readonly property int zAboveCharts: 100
        readonly property int lineWidth: 1
        readonly property int heightOffset: 50
        readonly property real currentSolutionMarkerSize: 12
        readonly property real solutionMarkerSize: 5
        readonly property real solutionLineWidth: 0.5
        readonly property color backgroundColor: "#CDC9C9"
        readonly property color areaColor: "#F0F0F0"
        readonly property color minorGridLineColor: "#CDC9C9"
        readonly property color gridLineColor: "#CDC9C9"
        readonly property color labelsColor: "#000000"
        readonly property font titleFont: Qt.font({
            "family": fontFamily,
            "pointSize": xlPointSize,
            "bold": true
        })
        readonly property color titleColor: swiftGrey
        readonly property font axisTitleFont: Qt.font({
            "family": fontFamily,
            "pointSize": mediumPointSize,
            "bold": true,
            "letterSpacing": 2,
            "capitalization": Font.AllUppercase
        })
        readonly property font axisLabelsFont: Qt.font({
            "family": fontFamily,
            "pointSize": smallPointSize,
            "bold": true
        })
        readonly property int tickPointSize: 10
        readonly property int buttonHeight: 40
        readonly property int unitDropdownWidth: 90
        readonly property real zoomInMult: 1.1
        readonly property real zoomOutMult: 0.9
    }

    trackingSkyPlot: QtObject {
        readonly property int markerSize: 10
        readonly property var scatterLabels: ["GPS", "GLONASS", "GALILEO", "BEIDOU", "QZSS", "SBAS"]
        readonly property var colors: ["green", "red", "blue", "gold", "pink", "purple"]
        readonly property int axisAngularMax: 360
        readonly property int axisAngularMin: 0
        readonly property int axisAngularTickCount: 13
        readonly property int axisRadialMax: 90
        readonly property int axisRadialMin: 0
        readonly property int axisRadialTickCount: 5
        readonly property int checkboxSpacing: 5
        readonly property int legendTopMargin: 50
        readonly property int legendRightMargin: 200
        readonly property int directionLabelOffset: 30
        readonly property int directionLabelFontSize: 14
    }

    trackingSignals: QtObject {
        readonly property string title: "Tracking C/N0"
        readonly property int legendTopMargin: 12
        readonly property int legendBottomMargin: 72
        readonly property int legendLeftMargin: 18
        readonly property string legendCellTextSample: "XXX XXXX X+NN XNN"
        readonly property string yAxisTitleText: "dB-Hz"
        readonly property string xAxisTitleText: "seconds"
        readonly property int snrThreshold: 15
        readonly property int yAxisMax: 60
        readonly property int yAxisTickInterval: 10
        readonly property int xAxisTickInterval: 10
        readonly property font yAxisTitleFont: Qt.font({
            "family": fontFamily,
            "pointSize": mediumPointSize,
            "bold": true,
            "letterSpacing": 2,
            "capitalization": Font.MixedCase
        })
    }

    observationTab: QtObject {
        readonly property int titleAreaHight: 25
    }

    icons: QtObject {
        readonly property string savePath: "qrc:/images/fontawesome/floppy-o.svg"
        readonly property string refreshPath: "qrc:/images/fontawesome/refresh.svg"
        readonly property string exportPath: "qrc:/images/fontawesome/file-export.svg"
        readonly property string importPath: "qrc:/images/fontawesome/file-import.svg"
        readonly property string warningPath: "qrc:/images/fontawesome/exclamation-triangle.svg"
        readonly property string connectButtonPath: "qrc:/images/fontawesome/power-off-solid.svg"
        readonly property string pauseButtonUrl: "qrc:/pause.svg"
        readonly property string centerOnButtonUrl: "qrc:/center-on.svg"
        readonly property string clearButtonUrl: "qrc:/clear.svg"
        readonly property string zoomAllButtonUrl: "qrc:/zoom-all.svg"
        readonly property string splashScreenPath: "qrc:/images/LogoBackground.jpg"
        readonly property string lightningBoltPath: "qrc:/images/ConnectionIcon.svg"
        readonly property string dropIndicatorPath: "qrc:/qt-project.org/imports/QtQuick/Controls.2/Material/images/drop-indicator.png"
        readonly property string playPath: "qrc:/images/iconic/play.svg"
        readonly property string solidCirclePath: "qrc:/images/fontawesome/circle-solid.svg"
        readonly property string squareSolidPath: "qrc:/images/fontawesome/square-solid.svg"
        readonly property string swiftLogoPath: "qrc:/images/icon.png"
        readonly property string swiftLogoWidePath: "qrc:/images/swiftLogoWide.svg"
        readonly property string folderPath: "qrc:/images/fontawesome/folder-regular.svg"
        readonly property string xPath: "qrc:/images/iconic/x.svg"
        readonly property string advancedPath: "qrc:/images/swift-nav/Advanced.svg"
        readonly property string baselinePath: "qrc:/images/swift-nav/Baseline.svg"
        readonly property string connectionPath: "qrc:/images/swift-nav/Connection.svg"
        readonly property string observationsPath: "qrc:/images/swift-nav/Observations.svg"
        readonly property string trackingPath: "qrc:/images/swift-nav/Tracking.svg"
        readonly property string solutionPath: "qrc:/images/swift-nav/Solution.svg"
        readonly property string settingsPath: "qrc:/images/swift-nav/Settings.svg"
        readonly property string updatePath: "qrc:/images/swift-nav/Update.svg"
    }

    insSettingsPopup: QtObject {
        readonly property var columnHeaders: ["Name", "Current Value", "Recommended Value"]
        readonly property int dialogWidth: 550
        readonly property int columnSpacing: 10
        readonly property int tableHeight: 150
    }

}
