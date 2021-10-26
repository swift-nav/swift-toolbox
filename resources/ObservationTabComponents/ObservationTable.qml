import "../Constants"
import "../TableComponents"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    id: observationTable

    property alias remote: observationTableModel.remote
    property bool populated: observationTableModel ? observationTableModel.row_count > 0 : false
    property font tableFont: Qt.font({
        "family": Constants.genericTable.fontFamily,
        "pointSize": Constants.largePointSize
    })

    function update() {
        observationTableModel.update();
    }

    spacing: 0

    ObservationTableModel {
        id: observationTableModel

        onDataPopulated: {
            var widthLeft = observationTable.width;
            var idealColumnWidths = [];
            for (var col = 0; col < headerRepeater.count; col++) {
                var idealColumnWidth = Math.min(500, observationTableModel.columnWidth(col, tableFont, headerRepeater.itemAt(col).font));
                idealColumnWidths.push(idealColumnWidth);
                widthLeft -= idealColumnWidths[col];
            }
            var extraWidth = widthLeft / headerRepeater.count;
            for (var col = 0; col < headerRepeater.count; col++) {
                headerRepeater.itemAt(col).initialWidth = idealColumnWidths[col] + extraWidth;
            }
            innerTable.forceLayout();
        }
    }

    RowLayout {
        id: innerStats

        property int textPadding: 3

        spacing: 3

        Label {
            id: weekLabel

            text: "Week:"
            padding: parent.textPadding
            ToolTip.text: "GPS Week Number (since 1980)"
        }

        Label {
            id: weekValue

            text: observationTableModel ? observationTableModel.week : ""
            padding: parent.textPadding
        }

        Label {
            id: towLabel

            text: "TOW:"
            padding: parent.textPadding
            ToolTip.text: "GPS milliseconds in week"
        }

        Label {
            id: towValue

            text: observationTableModel ? observationTableModel.padFloat(observationTableModel.tow, 2) : ""
            padding: parent.textPadding
        }

        Label {
            id: totalLabel

            text: "Total:"
            padding: parent.textPadding
            ToolTip.text: "Total observation count"
        }

        Label {
            id: totalValue

            text: observationTableModel ? observationTableModel.row_count : ""
            padding: parent.textPadding
        }

    }

    Item {
        Layout.fillWidth: true
        implicitHeight: header.implicitHeight
        clip: true

        Row {
            id: header

            width: innerTable.contentWidth
            x: -innerTable.contentX
            z: 1
            spacing: innerTable.columnSpacing

            Repeater {
                id: headerRepeater

                model: observationTableModel ? observationTableModel.columnCount() : 0

                SortableColumnHeading {
                    initialWidth: Math.min(500, observationTableModel.columnWidth(index, tableFont, font))
                    height: Constants.genericTable.cellHeight
                    table: innerTable
                }

            }

        }

    }

    TableView {
        id: innerTable

        width: Math.min(header.width + 1, parent.width)
        Layout.fillHeight: true
        columnSpacing: -1
        rowSpacing: -1
        clip: true
        onWidthChanged: {
            // Don't ask why this is needed. It's a hack.
            // If you want to find out, just comment out this code.
            if (width === 0) {
                width = Qt.binding(function() {
                    return Math.min(header.width + 1, observationTable.width);
                });
                forceLayout();
            }
        }
        boundsBehavior: Flickable.StopAtBounds
        columnWidthProvider: function(column) {
            return headerRepeater.itemAt(column).width;
        }
        model: observationTableModel

        delegate: TableCellDelegate {
            implicitHeight: Constants.genericTable.cellHeight
            font: tableFont
        }

        ScrollBar.horizontal: ScrollBar {
        }

        ScrollBar.vertical: ScrollBar {
        }

    }

    Rectangle {
        height: 1
        width: innerTable.width - 1
        color: "transparent"
        border.color: Constants.genericTable.borderColor
    }

}
