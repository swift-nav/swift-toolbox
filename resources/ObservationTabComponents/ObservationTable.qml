import "../Constants"
import "../TableComponents"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ColumnLayout {
    id: observationTable

    property alias name: innerText.text
    property alias remote: observationTableModel.remote
    property bool populated: observationTableModel ? observationTableModel.row_count > 0 : false
    property font tableFont: Qt.font({
        "family": Constants.monoSpaceFont,
        "pointSize": Constants.mediumPointSize
    })
    spacing: 0

    function update() {
        observationTableModel.update();
    }

    ObservationTableModel {
        id: observationTableModel

        onDataPopulated: {
            for(var col = 0; col < headerRepeater.count; col++) {
                var initWidth = Math.min(500, observationTableModel.columnWidth(col, tableFont));
                headerRepeater.itemAt(col).initialWidth = initWidth;
            }
            innerTable.forceLayout()
        }
    }

    Text {
        id: innerText
        Layout.leftMargin: 10
        Layout.bottomMargin: 5

        height: Constants.observationTab.titleAreaHight
        padding: 3
        font.pointSize: Constants.observationTab.titlePointSize
    }

    RowLayout {
        id: innerStats
        Layout.leftMargin: 10

        property int textPadding: 3
        spacing: 3
        Text {
            id: weekLabel

            text: "Week:"
            padding: parent.textPadding
            ToolTip.text: "GPS Week Number (since 1980)"
        }

        Text {
            id: weekValue

            text: observationTableModel ? observationTableModel.week : ""
            padding: parent.textPadding
            font: Constants.monoSpaceFont
        }

        Text {
            id: towLabel

            text: "TOW:"
            padding: parent.textPadding
            ToolTip.text: "GPS milliseconds in week"
        }

        Text {
            id: towValue

            text: observationTableModel ? observationTableModel.padFloat(observationTableModel.tow, 2) : ""
            padding: parent.textPadding
            font: Constants.monoSpaceFont
        }

        Text {
            id: totalLabel

            text: "Total:"
            padding: parent.textPadding
            ToolTip.text: "Total observation count"
        }

        Text {
            id: totalValue

            text: observationTableModel ? observationTableModel.row_count : ""
            padding: parent.textPadding
            font: Constants.monoSpaceFont
        }

    }

    Item {
        // Layout.alignment: Qt.AlignHCenter
        // Layout.fillWidth: true
        Layout.leftMargin: 10
        implicitHeight: header.implicitHeight
        implicitWidth: header.implicitWidth
        clip: true

        Row {
            id: header

            //Layout.leftMargin: 3
            //Layout.rightMargin: 3
            // Layout.alignment: Qt.AlignHCenter
            width: innerTable.contentWidth
            x: -innerTable.contentX
            z: 1
            spacing: innerTable.columnSpacing

            Repeater {
                id: headerRepeater

                model: observationTableModel ? observationTableModel.columnCount() : 0

                SortableColumnHeading {
                    initialWidth: Math.min(500, observationTableModel.columnWidth(index, tableFont))
                    table: innerTable
                }

            }

        }

    }

    TableView {
        id: innerTable

        width: Math.min(header.width + 1, parent.width)
        // Layout.fillWidth: true
        // Layout.maximumWidth: header.width + 1
        Layout.leftMargin: 10
        Layout.bottomMargin: 10
        Layout.fillHeight: true
        // Layout.alignment: Qt.AlignHCenter // x: header.x
        columnSpacing: 1
        rowSpacing: 1
        clip: true
        onWidthChanged: {
            // Don't ask why this is needed. It's a hack.
            // If you want to find out, just comment out this code.
            if (width === 0) {
                width = Qt.binding(function() { return Math.min(header.width + 1, observationTable.width); })
                forceLayout()
            }
        }
        boundsBehavior: Flickable.StopAtBounds
        columnWidthProvider: function(column) {
            return headerRepeater.itemAt(column).width
        }
        model: observationTableModel

        delegate: TableCellDelegate {
            font: tableFont
            rowSpacing: innerTable.rowSpacing
            columnSpacing: innerTable.columnSpacing
        }

        ScrollBar.horizontal: ScrollBar {
        }

        ScrollBar.vertical: ScrollBar {
        }

    }

}
