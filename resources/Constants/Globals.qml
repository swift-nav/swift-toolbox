import QtQuick 2.6
pragma Singleton

QtObject {
    property string consoleVersion: "0.0.0"
    property int currentRefreshRate: 5 // 5 Hz
    property bool useOpenGL: true
    property int initialMainTabIndex: 0 // Tracking
    property int initialSubTabIndex: 0 // Signals
    property bool showCsvLog: false
    property bool showFileio: false
    property int height: 600
    property int minimumHeight: 600
    property int width: 1050
    property int minimumWidth: 1050
    property string conn_state: Constants.connection.disconnected
    property bool connected_at_least_once: false
    property string copyClipboard: ""
    property var tablesWithHighlights: []
    property var currentSelectedTable: null

    function clearHighlightedRows() {
        for (var i in tablesWithHighlights) {
            tablesWithHighlights[i].selectedRow = -1;
        }
    }

}
