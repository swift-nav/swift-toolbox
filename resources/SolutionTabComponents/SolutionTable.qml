import "../Constants"
import Qt.labs.qmlmodels 1.0
import QtCharts 2.2
import QtQuick 2.14
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionTable

    property variant table: []
    property variant columnWidths: [50, 50]

    width: parent.width
    height: parent.height

    SolutionTableEntries {
        id: solutionTableEntries
    }

    Rectangle {
        id: solutionTableInner

        border.color: "#000000"
        border.width: 1
        width: parent.width
        height: parent.height

        ColumnLayout {
            id: solutionTableRowLayout

            spacing: 0
            width: parent.width
            height: parent.height

            TableView {
                id: solutionTableElementHeaders

                interactive: false
                Layout.minimumHeight: 20
                Layout.fillWidth: true
                Layout.leftMargin: 2
                Layout.rightMargin: 2
                Layout.bottomMargin: 0
                Layout.topMargin: 2
                columnSpacing: 0
                rowSpacing: 0
                clip: true
                columnWidthProvider: function(column) {
                    return columnWidths[column];
                }

                model: TableModel {
                    rows: [{
                        "Item": "Item",
                        "Value": "Value"
                    }]

                    TableModelColumn {
                        display: "Item"
                    }

                    TableModelColumn {
                        display: "Value"
                    }

                }

                delegate: Rectangle {
                    id: textDelegate

                    implicitHeight: 20
                    border.width: 1

                    Text {
                        id: rowText

                        text: display
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            TableView {
                id: solutionTableElement

                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.leftMargin: 2
                Layout.rightMargin: 2
                Layout.bottomMargin: 2
                Layout.topMargin: 0
                interactive: false
                columnSpacing: 0
                rowSpacing: 0
                clip: true
                columnWidthProvider: function(column) {
                    return columnWidths[column];
                }

                model: TableModel {
                    rows: []

                    TableModelColumn {
                        display: "Item"
                    }

                    TableModelColumn {
                        display: "Value"
                    }

                }

                delegate: Rectangle {
                    id: textDelegate

                    implicitHeight: 20
                    border.width: 1

                    Text {
                        id: rowText

                        text: display
                        leftPadding: 2
                    }

                }

            }

            Rectangle {
                id: solutionRTKNote

                Layout.minimumHeight: 40
                Layout.fillWidth: true
                width: parent.width
                Layout.margins: 2
                Layout.alignment: Qt.AlignLeft | Qt.AlignBottom
                border.width: 1

                Text {
                    wrapMode: Text.Wrap
                    anchors.fill: parent
                    text: "It is necessary to enter the \"Surveyed Position\" settings for the base station in order to view the RTK Positions in this tab."
                }

            }

        }

        Timer {
            interval: Constants.currentRefreshRate
            running: true
            repeat: true
            onTriggered: {
                if (!solutionTab.visible)
                    return ;

                solution_table_model.fill_console_points(solutionTableEntries);
                if (!solutionTableEntries.entries.length)
                    return ;

                var entries = solutionTableEntries.entries;
                var table_update = [];
                for (var idx in entries) {
                    table_update.push({
                        "Item": entries[idx][0],
                        "Value": entries[idx][1]
                    });
                }
                solutionTableElement.model.rows = table_update;
                columnWidths = [120, solutionTableArea.width - 120];
                solutionTableElement.forceLayout();
                solutionTableElementHeaders.forceLayout();
            }
        }

    }

}
