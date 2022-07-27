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
    property int selectedRow: -1
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

        policy: ScrollBar.AlwaysOn
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
