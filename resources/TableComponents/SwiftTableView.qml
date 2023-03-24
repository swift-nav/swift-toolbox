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
import QtQuick
import QtQuick.Controls
import SwiftConsole

TableView {
    id: tableView

    property alias horizontalScrollBar: _horizontalScrollBar
    property alias verticalScrollBar: _verticalScrollBar
    property variant columnWidths: []
    property variant columnAlignments: []
    property int _currentSelectedIndex: -1
    property bool stayFocused: false
    property int currentSelectedIndex: (!stayFocused && Globals.currentSelectedTable == this) ? _currentSelectedIndex : -1
    property int delegateBorderWidth: Constants.genericTable.borderWidth
    property color delegateBorderColor: Constants.genericTable.borderColor
    property font tableFont: Qt.font({
            "family": Constants.genericTable.fontFamily,
            "pixelSize": Constants.largePixelSize
        })

    clip: true
    columnSpacing: -1
    rowSpacing: -1
    columnWidthProvider: function (column) {
        return columnWidths[column];
    }
    reuseItems: true
    boundsBehavior: Flickable.StopAtBounds
    Component.onCompleted: {
    }
    onWidthChanged: {
        tableView.forceLayout();
    }
    onFocusChanged: {
        if (!stayFocused)
            _currentSelectedIndex = -1;
    }

    ScrollBar.horizontal: ScrollBar {
        id: _horizontalScrollBar
    }

    ScrollBar.vertical: ScrollBar {
        id: _verticalScrollBar
        width: 20
        onSizeChanged: {
            if (position + size > 1)
                position = 1 - size;
        }
    }

    delegate: Rectangle {
        implicitHeight: Constants.genericTable.cellHeight
        implicitWidth: tableView.columnWidthProvider(column)
        border.color: delegateBorderColor
        border.width: delegateBorderWidth
        color: row == currentSelectedIndex ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

        Label {
            width: parent.width
            height: parent.height
            horizontalAlignment: tableView.columnAlignments[column] || Text.AlignLeft
            verticalAlignment: Text.AlignVCenter
            clip: true
            font: tableFont
            text: model.display
            elide: Text.ElideRight
            leftPadding: Constants.genericTable.leftPadding
            rightPadding: Constants.genericTable.rightPadding
        }

        MouseArea {
            width: parent.width
            height: parent.height
            anchors.centerIn: parent
            onClicked: {
                tableView.focus = true;
                if (_currentSelectedIndex == row) {
                    Globals.currentSelectedTable = null;
                    _currentSelectedIndex = -1;
                } else {
                    Globals.currentSelectedTable = tableView;
                    _currentSelectedIndex = row;
                    Globals.copyClipboard = JSON.stringify(tableView.model.getRow(_currentSelectedIndex));
                }
            }
        }
    }
}
