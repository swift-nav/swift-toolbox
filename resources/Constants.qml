import QtQuick 2.6
pragma Singleton

QtObject {
    readonly property int width: 640
    readonly property int height: 480
    property QtObject bottomNavBar

    bottomNavBar: QtObject {
        readonly property int navBarMargin: 10
        readonly property int serialDeviceRefreshWidth: 30
        readonly property color placeholderTextColor: "#CDC9C9"
    }

}
