import "../Constants"
import "./observation_tab.js" as ObsTabJS
import Qt.labs.qmlmodels 1.0
import QtQuick 2.14
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    property alias name: innerText.text
    property alias model: innerTable.model
    property variant week: 0
    property variant tow: 0
    property variant columnWidths: [1, 1, 1, 1, 1, 1, 1, 1]

    border.color: "#000000"
    border.width: 1

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

                text: week
                font: Constants.monoSpaceFont
            }

            Text {
                id: towLabel

                text: "TOW:"
                ToolTip.text: "GPS milliseconds in week"
            }

            Text {
                id: towValue

                text: ObsTabJS.padFloat(tow, 2)
                font: Constants.monoSpaceFont
            }

            Text {
                id: totalLabel

                text: "Total:"
                ToolTip.text: "Total observation count"
            }

            Text {
                id: totalValue

                text: innerTable.model.rows.length
                font: Constants.monoSpaceFont
            }

        }

    }

    TableView {
        // force comment inside

        id: columnHeaderTable

        interactive: false
        anchors.top: innerStats.bottom
        height: 20
        width: parent.width
        columnSpacing: 1
        rowSpacing: 1
        clip: true
        columnWidthProvider: function(column) {
            return columnWidths[column];
        }

        model: TableModel {
            rows: [{
            }] // empty row triggers the display methods

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.obsColNames[modelIndex.column];
                }
            }

        }

        delegate: Rectangle {
            implicitHeight: 20
            border.width: 1

            Text {
                text: display
                anchors.centerIn: parent
                leftPadding: 2
            }

        }

    }

    TableView {
        id: innerTable

        height: parent.height - innerStats.height - innerText.height - columnHeaderTable.height - 6
        anchors.top: columnHeaderTable.bottom
        columnSpacing: columnHeaderTable.columnSpacing
        rowSpacing: columnHeaderTable.columnSpacing
        clip: true
        width: parent.width
        columnWidthProvider: function(column) {
            return columnWidths[column];
        }

        model: TableModel {
            rows: ObsTabJS.getObsSampleData(Constants.debugMode)

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

            TableModelColumn {
                display: function(modelIndex) {
                    return ObsTabJS.getObsCell(innerTable.model, modelIndex);
                }
            }

        }

        delegate: DelegateChooser {
            DelegateChoice {
                column: 0 // prn

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: display
                        font: Constants.monoSpaceFont
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            DelegateChoice {
                column: 1 // pseudoRange

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: ObsTabJS.padFloat(display, 11)
                        font: Constants.monoSpaceFont
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            DelegateChoice {
                column: 2 // carrierPhase

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: ObsTabJS.padFloat(display, 13)
                        font: Constants.monoSpaceFont
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            DelegateChoice {
                column: 3 // cn0

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: ObsTabJS.padFloat(display, 9)
                        font: Constants.monoSpaceFont
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            DelegateChoice {
                column: 4 // measuredDoppler

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: ObsTabJS.padFloat(display, 9)
                        font: Constants.monoSpaceFont
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            DelegateChoice {
                column: 5 // computedDoppler

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: ObsTabJS.padFloat(display, 9)
                        font: Constants.monoSpaceFont
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            DelegateChoice {
                column: 6 // lock

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: display
                        font: Constants.monoSpaceFont
                        anchors.centerIn: parent
                        leftPadding: 2
                    }

                }

            }

            DelegateChoice {
                column: 7 // flags

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: display
                        font: Constants.monoSpaceFont
                        leftPadding: 2
                    }

                }

            }

        }

    }

    Timer {
        interval: Constants.currentRefreshRate
        running: true
        repeat: true
        onTriggered: {
            if (!columnHeaderTable.visible)
                return ;

            var columnCount = ObsTabJS.obsColNames.length;
            var equalWidth = parent.width / columnCount;
            var newColumnWidths = [];
            for (var i = 0; i < columnCount; i++) {
                newColumnWidths.push(equalWidth);
            }
            if (newColumnWidths[0] != columnWidths[0]) {
                columnWidths = newColumnWidths;
                innerTable.forceLayout();
                columnHeaderTable.forceLayout();
            }
        }
    }

}
