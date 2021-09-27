import "../Constants"
import "../TableComponents"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    id: topLevel

    property alias name: innerText.text
    property alias remote: observationTableModel.remote
    property bool populated: observationTableModel ? observationTableModel.row_count > 0 : false
    property font tableFont: Qt.font({
        "family": Constants.monoSpaceFont,
        "pointSize": Constants.mediumPointSize
    })

    function update() {
        observationTableModel.update();
    }

    border.color: "#000000"
    border.width: 1

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

    Item {
        id: innerTextArea

        height: Constants.observationTab.titleAreaHight

        Text {
            id: innerText

            padding: 5
            font.pointSize: Constants.observationTab.titlePointSize
        }

    }

    Rectangle {
        id: innerStats

        anchors.top: innerTextArea.bottom
        border.width: 5
        height: 25
        color: "transparent"

        RowLayout {
            Text {
                id: weekLabel

                text: "Week:"
                ToolTip.text: "GPS Week Number (since 1980)"
            }

            Text {
                id: weekValue

                text: observationTableModel ? observationTableModel.week : ""
                font: Constants.monoSpaceFont
            }

            Text {
                id: towLabel

                text: "TOW:"
                ToolTip.text: "GPS milliseconds in week"
            }

            Text {
                id: towValue

                text: observationTableModel ? observationTableModel.padFloat(observationTableModel.tow, 2) : ""
                font: Constants.monoSpaceFont
            }

            Text {
                id: totalLabel

                text: "Total:"
                ToolTip.text: "Total observation count"
            }

            Text {
                id: totalValue

                text: observationTableModel ? observationTableModel.row_count : ""
                font: Constants.monoSpaceFont
            }

        }

    }

    Row {
        id: header

        anchors.top: innerStats.bottom
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

    TableView {
        id: innerTable

        height: parent.height - innerStats.height - innerText.height - header.height - 6
        anchors.top: header.bottom
        columnSpacing: 1
        rowSpacing: 1
        clip: true
        width: parent.width
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
