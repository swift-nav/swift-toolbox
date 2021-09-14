import QtQml 2.15
import QtQuick 2.6
import QtQuick.Controls.Material 2.12
pragma Singleton

QtObject {
    readonly property int width: 1050
    readonly property int height: 600
    readonly property real margins: 2
    readonly property real tabBarWidth: 70
    readonly property real tabBarHeight: 40
    readonly property real topLevelSpacing: 0
    readonly property real logPanelPreferredHeight: 100
    readonly property real navBarPreferredHeight: 50
    readonly property real statusBarPreferredHeight: 30
    property QtObject logPanel
    property QtObject statusBar
    property QtObject navBar
    property QtObject sideNavBar
    property QtObject loggingBar
    property QtObject licensesPopup
    property QtObject commonChart
    property QtObject commonLegend
    property QtObject commonTable
    property QtObject advancedIns
    property QtObject advancedMagnetometer
    property QtObject advancedSpectrumAnalyzer
    property QtObject baselinePlot
    property QtObject baselineTable
    property QtObject settingsTable
    property QtObject solutionPosition
    property QtObject solutionTable
    property QtObject solutionVelocity
    property QtObject trackingSignals
    property QtObject observationTab
    property QtObject genericTable
    property QtObject updateTab
    property QtObject icons
    readonly property int staticTimerIntervalRate: 5 // 5 Hz
    readonly property int staticTableTimerIntervalRate: 10 // 10 Hz
    readonly property string monoSpaceFont: "Courier New"
    readonly property real smallPointSize: 7
    readonly property real mediumPointSize: 8
    readonly property real largePointSize: 9
    readonly property bool debugMode: false
    readonly property string materialRed: "crimson"
    readonly property string materialGrey: "dimgrey"

    sideNavBar: QtObject {
        readonly property int buttonSvgHeight: 15
        readonly property string hamburgerPath: "images/fontawesome/bars-solid.svg"
        readonly property string trackingPath: "images/fontawesome/satellite-solid.svg"
        readonly property string solutionPath: "images/fontawesome/map-marker-alt-solid.svg"
        readonly property string baselinePath: "images/fontawesome/braille-solid.svg"
        readonly property string observationsPath: "images/fontawesome/table-solid.svg"
        readonly property string settingsPath: "images/fontawesome/cogs-solid.svg"
        readonly property string updatePath: "images/fontawesome/chevron-circle-up-solid.svg"
        readonly property string advancedPath: "images/fontawesome/lock-solid.svg"
        readonly property real tabBarHeight: 45
        readonly property real tabBarWidth: 70
        readonly property int tabBarSpacing: 10
        readonly property int buttonPadding: 0
        readonly property int buttonInset: 0
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
        readonly property string placeholderTextColor: "grey"
        readonly property int borderWidth: 1
        readonly property int firmwareVersionColumnSpacing: 0
        readonly property int buttonInset: 0
        readonly property int firmwareVersionLocalFileButtonSpacing: 5
        readonly property int firmwareVersionLocalFileButtonWidth: 50
        readonly property int fileioDestinationPathButtonWidth: 150
        readonly property string dotDotDotLabel: "..."
    }

    genericTable: QtObject {
        readonly property int headerZOffset: 100
        readonly property int padding: 2
        readonly property int mouseAreaResizeWidth: 10
        readonly property int cellHeight: 25
        readonly property string cellHighlightedColor: "crimson"
        readonly property string cellColor: "white"
        readonly property string gradientColor: "gainsboro"
        readonly property string borderColor: "gainsboro"
        readonly property string fontFamily: "Roboto"
        property var defaultColumns: ["Item", "Value"]
    }

    licensesPopup: QtObject {
        readonly property real tabBarHeight: 40
        readonly property real dialogPopupHeightPadding: 100
        readonly property string robotoFontTabLabel: "Roboto Font"
        readonly property string fontAwesomeIconsTabLabel: "Font Awesome Icons"
        readonly property string robotoFontLicensePath: "../fonts/Roboto-LICENSE.txt"
        readonly property string fontAwesomeIconsLicensePath: "../images/fontawesome/LICENSE.txt"
    }

    statusBar: QtObject {
        readonly property int margin: 10
        readonly property int spacing: 10
        readonly property color borderColor: "#CDC9C9"
        readonly property int borderWidth: 1
        readonly property color keyTextColor: "#00006E"
        readonly property real smallKeyWidthRatio: 0.05
        readonly property int innerKeyValSpacing: 5
        readonly property int arrowsSideLength: 15
        readonly property string arrowsBluePath: "images/iconic/arrows_blue.png"
        readonly property string arrowsGreyPath: "images/iconic/arrows_grey.png"
        readonly property string portLabel: "Port: "
        readonly property string posLabel: "Pos: "
        readonly property string rtkLabel: "RTK: "
        readonly property string satsLabel: "Sats: "
        readonly property string corrAgeLabel: "Corr Age: "
        readonly property string insLabel: "INS: "
        readonly property string defaultValue: "--"
    }

    navBar: QtObject {
        readonly property int connectionDropdownWidth: 100
        readonly property int serialSelectionDropdownWidth: 100
        readonly property int dropdownHeight: 40
        readonly property int buttonHeight: 40
        readonly property int buttonSvgHeight: 15
        readonly property int urlBarHeight: 25
        readonly property int navBarMargin: 10
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
        readonly property string connectButtonPath: "images/fontawesome/power-off-solid.svg"
        readonly property string pauseButtonPath: "images/fontawesome/pause-solid.svg"
        readonly property string folderButtonPath: "images/fontawesome/folder-open-regular.svg"
    }

    loggingBar: QtObject {
        readonly property int buttonHeight: 40
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
        readonly property string folderButtonPath: "images/fontawesome/folder-solid.svg"
    }

    advancedMagnetometer: QtObject {
        readonly property string title: "Raw Magnetometer Data"
        readonly property color titleColor: "#00006E"
        readonly property int titlePointSize: 14
        readonly property var legendLabels: ["Mag. X (uT)", "Mag. Y (uT)", "Mag. Z (uT)"]
        readonly property var lineColors: ["#66c2a5", "#fc8d62", "#8da0cb"]
        readonly property int xAxisMax: 200
        readonly property int xAxisMin: 0
        readonly property int xAxisTickCount: 25
        readonly property int yAxisTickCount: 20
        readonly property int legendBottomMargin: 60
        readonly property int legendLeftMargin: 50
    }

    advancedSpectrumAnalyzer: QtObject {
        readonly property string title: "Spectrum Analyzer"
        readonly property color titleColor: "#00006E"
        readonly property int titlePointSize: 14
        readonly property var lineColors: ["#000000"]
        readonly property int xAxisTickCount: 10
        readonly property real yAxisTickCount: 2.5
        readonly property string yAxisTitleText: "Amplitude (dB)"
        readonly property string xAxisTitleText: "Frequency (MHz)"
        readonly property int dropdownRowHeight: 35
        readonly property int dropdownHeight: 35
        readonly property int dropdownWidth: 100
        readonly property var dropdownModel: ["Channel 1", "Channel 2", "Channel 3", "Channel 4"]
        readonly property string dropdownLabel: "Channel Selection:"
        readonly property string dropdownRowSuggestionText: "Enable in Settings Tab under the \"System Monitor\" group."
        readonly property int rowTextHeight: 30
        readonly property int rowTextMargins: 5
    }

    advancedIns: QtObject {
        readonly property string title: "Raw IMU Data"
        readonly property color titleColor: "#00006E"
        readonly property int titlePointSize: 14
        readonly property var textDataLabels: ["Imu temp:", "Imu conf:", "Rms acc x:", "Rms acc y:", "Rms acc z:"]
        readonly property var insStatusLabels: ["GNSS Pos:", "GNSS Vel:", "Wheelticks:", "Wheelspeed:", "nhc:", "Static Detection:"]
        readonly property var legendLabels: ["Accn. X", "Accn. Y", "Accn. Z", "Gyro X", "Gyro Y", "Gyro Z"]
        readonly property var lineColors: ["#8c510a", "#d8b365", "#f6e8c3", "#c7eae5", "#5ab4ac", "#01665e"]
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
        readonly property string unknownStatusPath: "images/fontawesome/square-solid.svg"
        readonly property string unknownStatusColor: "dimgrey"
        readonly property string warningStatusPath: "images/fontawesome/exclamation-triangle-solid.svg"
        readonly property string warningStatusColor: "goldenrod"
        readonly property string okStatusPath: "images/fontawesome/circle-solid.svg"
        readonly property string okStatusColor: "green"
        readonly property int insStatusImageWidth: 15
    }

    baselinePlot: QtObject {
        readonly property int navBarMargin: 10
        readonly property int navBarSpacing: 0
        readonly property real navBarButtonWidth: 40
        readonly property real resetFiltersButtonWidth: 100
        readonly property string yAxisTitleText: "N (meters)"
        readonly property string xAxisTitleText: "E (meters)"
        readonly property var legendLabels: ["Base Position", "DGPS", "RTK Float", "RTK Fixed"]
        readonly property var colors: ["#FF0000", "#00B3FF", "#BF00BF", "#FFA500"]
    }

    baselineTable: QtObject {
        readonly property int defaultColumnWidth: 80
        readonly property color borderColor: "#000000"
        readonly property int borderWidth: 1
        readonly property int headerTableDataTableSpacing: 0
        readonly property int width: 300
        readonly property int cellHeight: 20
        readonly property int cellSpacing: 0
        readonly property int surroundingMargin: 2
        readonly property int innerMargin: 0
        readonly property int leftPadding: 2
        readonly property string leftColumnHeader: "Item"
        readonly property string rightColumnHeader: "Value"
    }

    solutionPosition: QtObject {
        readonly property int navBarMargin: 10
        readonly property int navBarSpacing: 0
        readonly property real navBarButtonProportionOfParent: 0.11
        readonly property string yAxisTitleText: "Latitude"
        readonly property string xAxisTitleText: "Longitude"
        readonly property var legendLabels: ["SPP", "DGPS", "RTK Float", "RTK Fixed", "DR", "SBAS"]
        readonly property var colors: ["#0000FF", "#00B3FF", "#BF00BF", "#FFA500", "#000000", "#00FF00"]
    }

    logPanel: QtObject {
        readonly property int width: 220
        readonly property variant defaultColumnWidthRatios: [0.15, 0.1, 0.75]
        readonly property int maxRows: 200
        readonly property int cellHeight: 20
        readonly property string timestampHeader: "Host Timestamp"
        readonly property string levelHeader: "Log Level"
        readonly property string msgHeader: "Message"
    }

    settingsTable: QtObject {
        readonly property string tableLeftColumnHeader: "Name"
        readonly property string tableRightColumnHeader: "Value"
    }

    solutionTable: QtObject {
        readonly property int width: 240
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
        readonly property int xAxisLabelsAngle: 45
        readonly property string xAxisTitleText: "GPS Time of Week"
        readonly property int xAxisMinOffsetFromMaxSeconds: 20
        readonly property int unitDropdownWidth: 50
        readonly property int chartHeightOffset: 0
        readonly property int chartBottomMargin: 30
        readonly property int legendBottomMargin: 120
        readonly property int legendLeftMargin: 80
        readonly property int legendLabelPointSize: 9
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
        readonly property color borderColor: "#000000"
        readonly property int borderWidth: 1
    }

    commonChart: QtObject {
        readonly property int zAboveCharts: 100
        readonly property int lineWidth: 1
        readonly property int heightOffset: 50
        readonly property int margin: 20
        readonly property real currentSolutionMarkerSize: 12
        readonly property real solutionMarkerSize: 5
        readonly property real solutionLineWidth: 0.5
        readonly property color backgroundColor: "#CDC9C9"
        readonly property color areaColor: "#FFFFFF"
        readonly property color minorGridLineColor: "#CDC9C9"
        readonly property color gridLineColor: "#CDC9C9"
        readonly property color labelsColor: "#000000"
        readonly property int tickPointSize: 10
        readonly property int buttonHeight: 40
        readonly property int unitDropdownWidth: 90
        readonly property real zoomInMult: 1.1
        readonly property real zoomOutMult: 0.9
    }

    trackingSignals: QtObject {
        readonly property string title: "Tracking C/N0"
        readonly property color titleColor: "#00006E"
        readonly property int titlePointSize: 14
        readonly property int legendBottomMargin: 85
        readonly property int legendLeftMargin: 60
        readonly property int legendLabelPointSize: 6
        readonly property string yAxisTitleText: "dB-Hz"
        readonly property string xAxisTitleText: "seconds"
        readonly property int xAxisMinOffsetFromMaxSeconds: 100
        readonly property int checkBoxVerticalPadding: 0
        readonly property int checkBoxPreferredWidth: 100
        readonly property int snrThreshold: 15
        readonly property int yAxisMax: 60
    }

    observationTab: QtObject {
        readonly property int titlePointSize: 14
        readonly property int titleAreaHight: 25
    }

    icons: QtObject {
        readonly property string savePath: "images/fontawesome/floppy-o.svg"
        readonly property string refreshPath: "images/fontawesome/refresh.svg"
        readonly property string exportPath: "images/fontawesome/file-export.svg"
        readonly property string importPath: "images/fontawesome/file-import.svg"
        readonly property string warningPath: "images/fontawesome/exclamation-triangle.svg"
    }

}
