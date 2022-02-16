import QtQuick 2.6
pragma Singleton

QtObject {
    property string consoleVersion: "0.0.0"
    property int currentRefreshRate: 5 // 5 Hz
    property bool useOpenGL: false
    property bool useAntiAliasing: true
    property bool showPrompts: true
    property int initialMainTabIndex: 0 // Tracking
    property int initialSubTabIndex: -1 // Not triggered unless greater than -1. Defaults to first tab.
    property bool showCsvLog: false
    property bool showFileio: false
    property int height: 600
    property int minimumHeight: 600
    property int width: 1050
    property int minimumWidth: 1050
    property string conn_state: Constants.connection.disconnected.toUpperCase()
    property string copyClipboard: ""
    property var tablesWithHighlights: []
    property var currentSelectedTable: null
    property bool showFileConnection: false
    property QtObject updateTabData

    function clearHighlightedRows() {
        for (var i in tablesWithHighlights) {
            tablesWithHighlights[i].selectedRow = -1;
        }
    }

    updateTabData: QtObject {
        property bool consoleOutdated: false
        property bool fwV2Outdated: false
        property bool fwOutdated: false
        property string fwVersionCurrent: ""
        property string fwVersionLatest: ""
        property string consoleVersionCurrent: ""
        property string consoleVersionLatest: ""
    }

}
