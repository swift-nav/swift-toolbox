import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import SwiftConsole 1.0

TableView {
    id: tableView

    property variant columnWidths: []
    property int selectedRow: -1
    property int _currentSelectedIndex: -1
    property bool stayFocused: false
    property int currentSelectedIndex: (!stayFocused && Globals.currentSelectedTable == this) ? _currentSelectedIndex : -1
    property int delegateBorderWidth: Constants.genericTable.borderWidth
    property color delegateBorderColor: Constants.genericTable.borderColor

    clip: true
    columnSpacing: -1
    rowSpacing: -1
    columnWidthProvider: function(column) {
        return columnWidths[column];
    }
    reuseItems: true
    boundsBehavior: Flickable.StopAtBounds
    Component.onCompleted: {
        console.assert(columnWidths.length == model.columnCount, "length of columnWidths does not match column count.");
    }
    onWidthChanged: {
        tableView.forceLayout();
    }
    onFocusChanged: {
        if (!stayFocused)
            _currentSelectedIndex = -1;

    }

    ScrollBar.horizontal: ScrollBar {
    }

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Rectangle {
        implicitHeight: Constants.genericTable.cellHeight
        implicitWidth: tableView.columnWidthProvider(column)
        border.color: delegateBorderColor
        border.width: delegateBorderWidth
        color: row == currentSelectedIndex ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

        Label {
            width: parent.width
            horizontalAlignment: Text.AlignLeft
            verticalAlignment: Text.AlignVCenter
            clip: true
            font.family: Constants.genericTable.fontFamily
            font.pointSize: Constants.largePointSize
            text: model.display
            elide: Text.ElideRight
            padding: Constants.genericTable.padding
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
