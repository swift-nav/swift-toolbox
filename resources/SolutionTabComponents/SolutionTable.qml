// import "ContactModel" as ContactModel

import QtCharts 2.2
import QtQuick 2.14
import Qt.labs.qmlmodels 1.0
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionTable

    property variant keys: []
    property variant vals: []
    property variant table: []
    property variant columnWidths: [50, 50]

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    SolutionTableEntries {
        id: solutionTableEntries
    }

    Rectangle {
        id: solutionTableInner

        border.color: "#000000"
        border.width: 1
        // anchors.bottom: trackingSignalsChart.bottom
        // anchors.left: trackingSignalsChart.left
        // anchors.bottomMargin: 85
        // anchors.leftMargin: 60
        
        width: parent.width
        height: parent.height

        ColumnLayout {
            id: solutionTableRowLayout
            spacing: 0
            width: parent.width
            height: parent.height
            TableView {
                id: solutionTableElementHeaders
                // width: solutionTabBackground.width
                interactive: false
                Layout.minimumHeight: 20
                Layout.fillWidth: true
                Layout.leftMargin: 2
                Layout.rightMargin: 2
                Layout.bottomMargin: 0
                Layout.topMargin: 2
                // anchors.fill: parent
                columnSpacing: 0
                rowSpacing: 0
                clip: true
                // onWidthChanged: solutionTableElement.forceLayout()
                columnWidthProvider: function (column) { return columnWidths[column] }
                model: TableModel {
                    TableModelColumn { display: "Item" }
                    TableModelColumn { display: "Value" }

                    rows: [{"Item": "Item", "Value": "Value"}]
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
                // width: solutionTabBackground.width
                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.leftMargin: 2
                Layout.rightMargin: 2
                Layout.bottomMargin: 2
                Layout.topMargin: 0
                // anchors.fill: parent
                interactive: false
                columnSpacing: 0
                rowSpacing: 0
                clip: true
                
                // contentWidth: solutionTab.width
                onWidthChanged: solutionTableElement.forceLayout()
                columnWidthProvider: function (column) { return columnWidths[column] }
                model: TableModel {
                    TableModelColumn { display: "Item" }
                    TableModelColumn { id: hello; display: "Value" }

                    rows: []
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
        }
        Timer {
            interval: 1000 / 10 // 10 Hz refresh
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
                    if (keys.length != entries.length) {
                        table_update.push({"Item": entries[idx][0], "Value": entries[idx][1]});
                    }
                }
                solutionTableElement.model.rows = table_update;
                columnWidths = [120, solutionTableArea.width-120]
                solutionTableElement.forceLayout();
                solutionTableElementHeaders.forceLayout();
                
            }
        }
    }
}
