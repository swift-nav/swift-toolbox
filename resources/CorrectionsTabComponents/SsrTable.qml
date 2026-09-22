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
import "../Constants"
import "../TableComponents"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

// Generic read-only table used by the three SSR panels on the Corrections
// tab. Column layout is supplied by the caller; row data comes from `model`
// (an SsrStreamTableModel / SsrSatCorrectionTableModel / SsrTileTableModel).
ColumnLayout {
    id: ssrTable

    property alias tableModel: innerTable.model
    property variant columnWidths: []
    property variant columnNames: []
    property real mouse_x: 0

    function _modelValid() {
        return tableModel != 0;
    }

    spacing: 0
    onWidthChanged: {
        innerTable.forceLayout();
    }
    onHeightChanged: {
        innerTable.forceLayout();
    }

    HorizontalHeaderView {
        id: horizontalHeader

        interactive: false
        syncView: innerTable
        z: Constants.genericTable.headerZOffset

        delegate: Rectangle {
            implicitWidth: columnWidths[index]
            implicitHeight: Constants.genericTable.cellHeight
            border.color: Constants.genericTable.borderColor

            Label {
                width: parent.width
                anchors.centerIn: parent
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                text: columnNames[index]
                elide: Text.ElideRight
                clip: true
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
            }

            MouseArea {
                width: Constants.genericTable.mouseAreaResizeWidth
                height: parent.height
                anchors.right: parent.right
                cursorShape: Qt.SizeHorCursor
                enabled: index !== (columnNames.length - 1)
                visible: index !== (columnNames.length - 1)
                onPressed: {
                    mouse_x = mouseX;
                }
                onPositionChanged: {
                    if (pressed) {
                        var delta_x = (mouseX - mouse_x);
                        var next_idx = (index + 1) % columnNames.length;
                        var min_width = ssrTable.width / (2 * columnNames.length);
                        if (columnWidths[index] + delta_x > min_width && columnWidths[next_idx] - delta_x > min_width) {
                            columnWidths[index] += delta_x;
                            columnWidths[next_idx] -= delta_x;
                        }
                        innerTable.forceLayout();
                    }
                }
            }

            gradient: Gradient {
                GradientStop {
                    position: 0
                    color: Constants.genericTable.cellColor
                }

                GradientStop {
                    position: 1
                    color: Constants.genericTable.gradientColor
                }
            }
        }
    }

    SwiftTableView {
        id: innerTable

        Layout.fillHeight: true
        Layout.fillWidth: true
        columnWidths: parent.columnWidths
    }
}
