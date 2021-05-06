import QtQuick 2.6
pragma Singleton

QtObject {
    readonly property int width: 640
    readonly property int height: 480
    readonly property real margins: 2
    readonly property real topLevelSpacing: 0
    readonly property real logPanelPreferredHeight: 100
    readonly property real navBarPreferredHeight: 50
    property QtObject navBar
    property QtObject commonChart
    property QtObject commonLegend
    property QtObject solutionPosition
    property QtObject solutionTable
    property QtObject solutionVelocity
    property QtObject trackingSignals
    property QtObject observationTab
    property int defaultTimerIntervalRate: 1000 / 5 // 5 Hz
    property string monoSpaceFont: "Courier New"
    property int pointSize: 6
    readonly property bool debugMode: false

    navBar: QtObject {
        readonly property int connectionDropdownWidth: 70
        readonly property int serialSelectionDropdownWidth: 90
        readonly property int dropdownHeight: 40
        readonly property int buttonHeight: 40
        readonly property int urlBarHeight: 25
        readonly property int urlBarBorder: 1
        readonly property int urlBarTextMargin: 4
        readonly property int navBarMargin: 10
        readonly property int plotRefreshRateDropdownWidth: 50
        readonly property int serialDeviceBaudRateDropdownWidth: 90
        readonly property int serialDeviceFlowControlDropdownWidth: 100
        readonly property int serialDeviceRefreshWidth: 30
        readonly property int connectButtonWidth: 70
        readonly property int connectionPauseWidth: 30
        readonly property color placeholderTextColor: "#CDC9C9"
        readonly property var all_refresh_rates: [1, 5, 10, 25]
        readonly property var default_refresh_rate_index: 1
    }

    solutionPosition: QtObject {
        readonly property int navBarMargin: 10
        readonly property int navBarSpacing: 0
        readonly property real navBarButtonProportionOfParent: 0.1
        readonly property int chartCurrentSolutionMarkerSize: 15
        readonly property int chartSolutionMarkerSize: 5
        readonly property real chartSolutionLineWidth: 0.1
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
        readonly property int markerPointSize: 14
        readonly property int labelPointSize: 10
        readonly property int padding: 10
        readonly property int verticalCenterOffset: -1
        readonly property color borderColor: "#000000"
        readonly property int borderWidth: 1
    }

    commonChart: QtObject {
        readonly property int zAboveCharts: 100
        readonly property int lineWidth: 1
        readonly property int heightOffset: 50
        readonly property int margin: 20
        readonly property int currentSolutionMarkerSize: 15
        readonly property int solutionMarkerSize: 5
        readonly property real solutionLineWidth: 0.1
        readonly property color backgroundColor: "#CDC9C9"
        readonly property color areaColor: "#FFFFFF"
        readonly property color minorGridLineColor: "#CDC9C9"
        readonly property color gridLineColor: "#CDC9C9"
        readonly property color labelsColor: "#000000"
        readonly property int tickPointSize: 10
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
    }

    observationTab: QtObject {
        readonly property int titlePointSize: 14
        readonly property int titleAreaHight: 25
    }

}
