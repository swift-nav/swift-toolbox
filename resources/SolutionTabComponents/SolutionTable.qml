// import "ContactModel" as ContactModel

import QtCharts 2.2
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionTable

    property variant keys: []
    property variant vals: []

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

        RowLayout {
            id: solutionTableRowLayout

            width: parent.width
            height: parent.height

            ListView {
                id: solutionTableKeys

                width: parent.width / 2
                Layout.fillHeight: true
                // Layout.fillWidth:
                spacing: 5
                model: keys
                delegate: keysDelegate
                focus: true

                Component {
                    id: keysDelegate

                    Rectangle {
                        id: key

                        border.color: "#000000"
                        border.width: 1
                        width: implicitWidth
                        height: keyText.height

                        Text {
                            id: keyText

                            text: modelData
                        }

                    }

                }

                header: Rectangle {
                    id: kheader

                    border.color: "#FFFFFF"
                    border.width: 1
                    width: implicitWidth
                    height: kheaderText.height

                    Text {
                        id: kheaderText

                        text: "Item"
                    }

                }

            }

            ListView {
                id: solutionTableVals

                Layout.fillHeight: true
                Layout.fillWidth: true
                model: keys
                delegate: valsDelegate
                focus: true

                Component {
                    id: valsDelegate

                    Rectangle {
                        id: val

                        z: 100
                        width: implicitWidth
                        height: valText.height
                        border.color: "#000000"
                        border.width: 4

                        Text {
                            id: valText

                            text: modelData
                        }

                    }

                }

                // width: parent.width/2
                // height: parent.height
                header: Rectangle {
                    id: vheader

                    width: implicitWidth
                    height: vheaderText.height
                    border.color: "#000000"
                    border.width: 4

                    Text {
                        id: vheaderText

                        text: "Value"
                    }

                }

            }

        }

        Timer {
            interval: 1000 / 5 // 5 Hz refresh
            running: true
            repeat: true
            onTriggered: {
                if (!solutionTab.visible)
                    return ;

                solution_table_model.fill_console_points(solutionTableEntries);
                if (!solutionTableEntries.entries.length)
                    return ;

                var entries = solutionTableEntries.entries;
                for (var idx in entries) {
                    if (keys.length != entries.length) {
                        keys.push(entries[idx][0]);
                        vals.push(entries[idx][1]);
                        solutionTableKeys.model = keys;
                    } else {
                        vals[idx] = entries[idx][1];
                    }
                }
                solutionTableVals.model = vals;
            }
        }

    }

}
