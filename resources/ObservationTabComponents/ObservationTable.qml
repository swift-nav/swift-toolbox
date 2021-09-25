import "../Constants"
import "../TableComponents"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    id: topLevel

    property alias name: innerText.text
    property variant columnWidths: [1, 1, 1, 1, 1, 1, 1, 1]
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
    }

    Rectangle {
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
                initialWidth: 100 // Math.min(600, observationTableModel.columnWidth(index, tableFont)); height: parent.height
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
        columnWidthProvider: function(column) {
            return columnWidths[column];
        }
        model: observationTableModel

        delegate: Item {
            implicitHeight: cellText.implicitHeight

            Rectangle {
                visible: row === 0
                height: 1
                color: "red"

                anchors {
                    top: parent.top
                    left: parent.left
                    right: parent.right
                    leftMargin: column === 0 ? 0 : -1 * (innerTable.columnSpacing + 1) / 2
                    rightMargin: -1 * (innerTable.columnSpacing + 1) / 2
                }

            }

            Rectangle {
                visible: column === 0
                width: 1
                color: "red"

                anchors {
                    top: parent.top
                    bottom: parent.bottom
                    left: parent.left
                    topMargin: row === 0 ? 0 : -1 * (innerTable.rowSpacing + 1) / 2
                    bottomMargin: -1 * (innerTable.rowSpacing + 1) / 2
                }

            }

            Rectangle {
                height: 1
                color: "red"

                anchors {
                    bottom: parent.bottom
                    left: parent.left
                    right: parent.right
                    bottomMargin: -1 * (innerTable.rowSpacing + 1) / 2
                    leftMargin: column === 0 ? 0 : -1 * (innerTable.columnSpacing + 1) / 2
                    rightMargin: -1 * (innerTable.columnSpacing + 1) / 2
                }

            }

            Rectangle {
                width: 1
                color: "red"

                anchors {
                    top: parent.top
                    bottom: parent.bottom
                    right: parent.right
                    topMargin: row === 0 ? 0 : -1 * (innerTable.rowSpacing + 1) / 2
                    bottomMargin: -1 * (innerTable.rowSpacing + 1) / 2
                    rightMargin: -1 * (innerTable.columnSpacing + 1) / 2
                }

            }

            Text {
                id: cellText

                text: display
                font: topLevel.tableFont
                anchors.centerIn: parent
                padding: 3
            }

        }

    }

    Timer {
        interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
        running: true
        repeat: true
        onTriggered: {
            if (!header.visible)
                return ;

            var columnCount = observationTableModel.columnCount();
            var equalWidth = parent.width / columnCount;
            var newColumnWidths = [];
            for (var i = 0; i < columnCount; i++) {
                newColumnWidths.push(equalWidth);
            }
            if (newColumnWidths[0] != columnWidths[0]) {
                columnWidths = newColumnWidths;
                innerTable.forceLayout();
                header.forceLayout();
            }
        }
    }

}
