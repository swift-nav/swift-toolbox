import "../Constants"
import "../TableComponents"
import "./observation_tab.js" as ObsTabJS
import Qt.labs.qmlmodels 1.0
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
        family: Constants.monoSpaceFont,
        pointSize: Constants.mediumPointSize
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

                text: observationTableModel ? ObsTabJS.padFloat(observationTableModel.tow, 2) : ""
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
        // height: 20
        x: -innerTable.contentX
        z: 1
        spacing: 4
        function relayout() {
            headerRepeater.model = 0
            headerRepeater.model = innerTable.model.columnCount()
        }

        Repeater {
            id: headerRepeater
            model: innerTable.model ? innerTable.model.columnCount() : 0
            // Rectangle {
            //     color: "Dark Grey"
            //     width: myText.implicitWidth
            //     height: myText.implicitHeight
            //     Text {
            //         id: myText
            //         text: innerTable.model.headerData(index, Qt.Horizontal)
            //     }
            // }
            SortableColumnHeading {
                // height: 20
                initialWidth: 100  // Math.min(600, innerTable.model.columnWidth(index, tableFont)); height: parent.height
                reorderable: true
                sortable: true
                headerRelayoutProvider: header.relayout
                table: innerTable
                onSorting: {
                    for (var i = 0; i < headerRepeater.model; ++i)
                        if (i != index)
                            headerRepeater.itemAt(i).clearSorting()
                }
            }
        }
    }

    TableView {
        id: innerTable

        height: parent.height - innerStats.height - innerText.height - header.height - 6
        anchors.top: header.bottom
        columnSpacing: 0
        rowSpacing: 0
        clip: true
        width: parent.width
        columnWidthProvider: function(column) {
            return columnWidths[column];
        }
        // onHeightChanged: console.log("innerTable.height: " + height)
        // onWidthChanged: console.log("innerTable.width: " + width)
        model: observationTableModel

        delegate: DelegateChooser {
            DelegateChoice {
                column: 0 // prn

                delegate: Rectangle {
                    implicitHeight: 20
                    border.width: 1

                    Text {
                        text: display
                        font: topLevel.tableFont
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
                        font: topLevel.tableFont
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
                        font: topLevel.tableFont
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
                        font: topLevel.tableFont
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
                        font: topLevel.tableFont
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
                        font: topLevel.tableFont
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
                        font: topLevel.tableFont
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
                        text: ObsTabJS.showFlags(display)
                        font: topLevel.tableFont
                        leftPadding: 2
                    }

                }

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

            var columnCount = ObsTabJS.obsColNames.length;
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
