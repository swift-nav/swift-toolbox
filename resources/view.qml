import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ApplicationWindow {
    width: 640
    height: 480
    font.pointSize: 8
    Component.onCompleted: {
        visible = true;
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 2
        width: parent.width
        height: parent.height

        Rectangle {
            id: mainTabs

            height: parent.height - consoleLog.height
            width: parent.width

            TabBar {
                id: tab

                z: 100
                width: parent.width

                Repeater {
                    model: ["Tracking", "Solution", "Baseline", "Observations", "Settings", "Update", "Advanced"]

                    TabButton {
                        text: modelData
                        width: implicitWidth
                    }

                }

            }

            StackLayout {
                width: parent.width
                height: parent.height - tab.height
                anchors.top: tab.bottom
                currentIndex: tab.currentIndex

                TrackingTab {
                }

                SolutionTab {
                }

                Item {
                    id: baselineTab
                }

                Item {
                    id: observationsTab
                }

                Item {
                    id: settingsTab
                }

                Item {
                    id: updateTab
                }

                Item {
                    id: advancedTab
                }

            }

        }

        Rectangle {
            id: consoleLog

            width: parent.width
            height: 100
            Layout.alignment: Qt.AlignBottom

            RowLayout {
                Button {
                    text: "Connect"
                    onClicked: data_model.connect()
                }

                Button {
                    text: "File In"
                    onClicked: data_model.readfile()
                }

            }

        }

    }

}
