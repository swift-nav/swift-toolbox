import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ApplicationWindow {
    width: Constants.width
    height: Constants.height
    font.pointSize: 8
    Component.onCompleted: {
        visible = true;
    }

    Rectangle {
        height: parent.height
        width: parent.width

        SplitView {
            orientation: Qt.Vertical
            spacing: 2
            width: parent.width
            height: parent.height - bottomNavBar.height

            Rectangle {
                id: mainTabs

                Layout.alignment: Qt.AlignTop
                Layout.preferredWidth: parent.width
                SplitView.fillHeight: true
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

                    ObservationTab {
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
                SplitView.preferredHeight: 100
                Layout.alignment: Qt.AlignBottom

                LogPanel {
                }

            }

        }

        Rectangle {
            id: bottomNavBar

            width: parent.width
            anchors.bottom: parent.bottom
            height: 50
            Layout.alignment: Qt.AlignBottom

            BottomNavBar {
            }

        }

    }

}
