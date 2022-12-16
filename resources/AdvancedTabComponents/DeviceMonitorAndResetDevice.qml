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
import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property alias deviceMonitor: deviceMonitor

    RowLayout {
        anchors.fill: parent

        DeviceMonitor {
            id: deviceMonitor

            Layout.preferredWidth: parent.width * 0.5
            Layout.fillHeight: true
        }

        Button {
            id: resetButton

            Layout.preferredWidth: parent.width * 0.5
            Layout.alignment: Qt.AlignHCenter
            ToolTip.visible: hovered
            ToolTip.text: Constants.systemMonitor.resetButtonLabel
            text: Constants.systemMonitor.resetButtonLabel
            icon.source: Constants.icons.connectButtonPath
            icon.width: Constants.systemMonitor.resetButtonIconSideLength
            icon.height: Constants.systemMonitor.resetButtonIconSideLength
            display: AbstractButton.TextUnderIcon
            flat: true
            onClicked: {
                backend_request_broker.reset_device();
            }
        }
    }
}
