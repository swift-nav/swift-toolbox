import QtQuick 2.6
pragma Singleton

QtObject {
    property int currentRefreshRate: 5 // 5 Hz
    property bool useOpenGL: true
    property int initialMainTabIndex: 0 // Tracking
    property int initialSubTabIndex: 0 // Signals
}
