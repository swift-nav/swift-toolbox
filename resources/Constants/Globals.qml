/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
pragma Singleton
import QtQuick

QtObject {
    property string consoleVersion: "0.0.0"
    property bool useOpenGL: false
    property bool useAntiAliasing: true
    property bool showPrompts: true
    property bool enableNtrip: false
    property int initialMainTabIndex: 0 // Tracking
    property int initialSubTabIndex: 0 // Signals
    property bool showCsvLog: false
    property bool showFileio: false
    property bool enableMap: false
    property int height: 600
    property int minimumHeight: 600
    property int width: 1000
    property int minimumWidth: 900
    property string conn_state: Constants.connection.disconnected.toUpperCase()
    property string copyClipboard: ""
    property var currentSelectedTable: null
    property bool showFileConnection: false
    property QtObject updateTabData

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
