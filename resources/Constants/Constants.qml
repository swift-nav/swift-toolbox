import QtQuick 2.6
pragma Singleton

QtObject {
    readonly property int width: 640
    readonly property int height: 480
    property QtObject bottomNavBar
    property int currentRefreshRate: 1000 / 5 // 5 Hz
    property int defaultTimerIntervalRate: 1000 / 5 // 5 Hz

    bottomNavBar: QtObject {
        readonly property int connectionDropdownWidth: 90
        readonly property int serialSelectionDropdownWidth: 90
        readonly property int urlBarHeight: 25
        readonly property int urlBarBorder: 1
        readonly property int urlBarTextMargin: 4
        readonly property int navBarMargin: 10
        readonly property int serialDeviceRefreshWidth: 30
        readonly property int connectionPauseWidth: 30
        readonly property color placeholderTextColor: "#CDC9C9"
        readonly property var available_ref_rates: [5, 10, 25]
    }

    trackingSignals: QtObject {
        readonly property string title: "Tracking C/N0"
        readonly property color titleColor: "#00006E"
        readonly property int titlePointSize: 14
        readonly property color plotBackgroundColor: "#CDC9C9"
        readonly property color plotAreaColor: "#FFFFFF"
        readonly property color legendBorderColor: "#000000"
        readonly property int legendBorderWidth: 1

    }

}
