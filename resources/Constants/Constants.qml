import QtQuick 2.6
pragma Singleton

QtObject {
    readonly property int width: 640
    readonly property int height: 480
    property QtObject bottomNavBar

    property QtObject solutionPosition
    property QtObject solutionTable
    property QtObject solutionVelocity
    property QtObject trackingSignals
    property int currentRefreshRate: 1000 / 5 // 5 Hz
    property int defaultTimerIntervalRate: 1000 / 5 // 5 Hz
    readonly property color plotBackgroundColor: "#CDC9C9"
    readonly property color plotAreaColor: "#FFFFFF"
    readonly property color plotMinorGridLineColor: "#CDC9C9"
    readonly property color plotGridLineColor: "#CDC9C9"
    readonly property color plotLabelsColor: "#000000"
    readonly property int plotTickPointSize: 10
    readonly property color legendBorderColor: "#000000"
    readonly property int legendBorderWidth: 1

    bottomNavBar: QtObject {
        readonly property int connectionDropdownWidth: 90
        readonly property int serialSelectionDropdownWidth: 90
        readonly property int urlBarHeight: 25
        readonly property int urlBarBorder: 1
        readonly property int urlBarTextMargin: 4
        readonly property int navBarMargin: 10
        readonly property int plotRefreshRateDropdownWidth: 60
        readonly property int serialDeviceRefreshWidth: 30
        readonly property int connectionPauseWidth: 30
        readonly property color placeholderTextColor: "#CDC9C9"
        readonly property var available_ref_rates: [5, 10, 25]
    }

    solutionPosition: QtObject {
        readonly property int navBarMargin: 10
        readonly property int navBarSpacing: 0
        readonly property real navBarButtonProportionOfParent: 0.1
        readonly property int chartHeightOffset: 50
        readonly property int chartMargin: 20
        readonly property int chartCurrentSolutionMarkerSize: 15
        readonly property int chartSolutionMarkerSize: 5
        readonly property real chartSolutionLineWidth: 0.1
        readonly property string yAxisTitleText: "Latitude"
        readonly property string xAxisTitleText: "Longitude"
        readonly property int legendTopMargin: 85
        readonly property int legendRightMargin: 60
        readonly property int legendMarkerPointSize: 14
        readonly property int legendLabelPointSize: 10
        readonly property int legendPadding: 10
        readonly property int legendVerticalCenterOffset: -1
        
    }

    solutionTable: QtObject {
        readonly property var defaultColumnWidths: [120, 120]
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
        readonly property var tableHeaderModel: [{
                        "Item": "Item",
                        "Value": "Value"
                    }]
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
        readonly property int unitDropdownWidth: 100
        readonly property int chartLineWidth: 1
        readonly property int chartHeightOffset: 0//100
        readonly property int chartBottomMargin: 20
        readonly property int legendBottomMargin: 120
        readonly property int legendLeftMargin: 80
        readonly property int legendMarkerHeight: 3
        readonly property int legendMarkerWidth: 20
        readonly property int legendLabelPointSize: 9
        readonly property int legendPadding: 10
        readonly property int legendVerticalCenterOffset: -1
        
    }

    trackingSignals: QtObject {
        readonly property string title: "Tracking C/N0"
        readonly property color titleColor: "#00006E"
        readonly property int titlePointSize: 14
        readonly property int legendBottomMargin: 85
        readonly property int legendLeftMargin: 60
    }

}
