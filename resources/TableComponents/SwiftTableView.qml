import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import SwiftConsole 1.0

TableView {
    id: tableView

    property variant columnWidths: []
    property int selectedRow: -1

    columnSpacing: -1
    rowSpacing: -1
    columnWidthProvider: function(column) {
        return columnWidths[column];
    }
    reuseItems: true
    boundsBehavior: Flickable.StopAtBounds
    Component.onCompleted: {
        console.assert(columnWidths.length == model.columnCount, "length of columnWidths does not match column count.");
        Globals.tablesWithHighlights.push(this);
    }
    onWidthChanged: {
        tableView.forceLayout();
    }

    ScrollBar.horizontal: ScrollBar {
    }

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Rectangle {
        implicitHeight: Constants.genericTable.cellHeight
        implicitWidth: tableView.columnWidthProvider(column)
        border.color: Constants.genericTable.borderColor
        color: row == tableView.selectedRow ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

        Label {
            width: parent.width
            horizontalAlignment: Text.AlignLeft
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
            onPressed: {
                Globals.clearHighlightedRows();
                tableView.focus = true;
                if (tableView.selectedRow == row) {
                    tableView.selectedRow = -1;
                } else {
                    tableView.selectedRow = row;
                    Globals.copyClipboard = JSON.stringify(tableView.model.getRow(tableView.selectedRow));
                }
            }
        }

    }

}
