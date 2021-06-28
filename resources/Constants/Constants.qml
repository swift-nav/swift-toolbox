import QtQuick 2.6
import QtQuick.Controls.Material 2.12
pragma Singleton

QtObject {
    readonly property int width: 1050
    readonly property int height: 600
    readonly property real margins: 2
    readonly property real tabBarHeight: 30
    readonly property real topLevelSpacing: 0
    readonly property real logPanelPreferredHeight: 100
    readonly property real navBarPreferredHeight: 50
    readonly property real statusBarPreferredHeight: 30
    property QtObject statusBar
    property QtObject navBar
    property QtObject loggingBar
    property QtObject commonChart
    property QtObject commonLegend
    property QtObject commonTable
    property QtObject advancedIns
    property QtObject advancedMagnetometer
    property QtObject baselinePlot
    property QtObject baselineTable
    property QtObject solutionPosition
    property QtObject solutionTable
    property QtObject solutionVelocity
    property QtObject trackingSignals
    property QtObject observationTab
    readonly property int staticTimerIntervalRate: 5 // 5 Hz
    readonly property int staticTableTimerIntervalRate: 10 // 10 Hz
    readonly property string monoSpaceFont: "Courier New"
    readonly property real smallPointSize: 7
    readonly property real mediumPointSize: 8
    readonly property real largePointSize: 9
    readonly property bool debugMode: false
    readonly property string materialRed: "crimson"
    readonly property string materialGrey: "dimgrey"

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
    }

    solutionTable: QtObject {
        readonly property int defaultColumnWidth: 80
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
        readonly property int rtkNoteHeight: 40
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

}
