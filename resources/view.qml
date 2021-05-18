import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ApplicationWindow {
    width: Constants.width
    height: Constants.height
    font.pointSize: Constants.mediumPointSize
    Component.onCompleted: {
        visible = true;
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: Constants.topLevelSpacing

        SplitView {
            orientation: Qt.Vertical
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.leftMargin: Constants.margins
            Layout.rightMargin: Constants.margins
            Layout.alignment: Qt.AlignTop

            ColumnLayout {
                id: mainTabs

                spacing: Constants.topLevelSpacing
                SplitView.fillHeight: true
                SplitView.fillWidth: true
                Layout.leftMargin: Constants.margins
                Layout.rightMargin: Constants.margins

                TabBar {
                    id: tab

                    Layout.fillWidth: true
                    z: Constants.commonChart.zAboveCharts
                    currentIndex: Globals.initialMainTabIndex

                    Repeater {
                        model: ["Tracking", "Solution", "Baseline", "Observations", "Settings", "Update", "Advanced"]

                        TabButton {
                            text: modelData
                            width: implicitWidth
                        }

                    }

                }

                StackLayout {
                    Layout.fillHeight: true
                    Layout.fillWidth: true
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

                SplitView.fillWidth: true
                SplitView.preferredHeight: Constants.logPanelPreferredHeight

                LogPanel {
                }

            }

        }

        Rectangle {
            id: navBar

            Layout.fillWidth: true
            Layout.preferredHeight: Constants.navBarPreferredHeight
            z: Constants.commonChart.zAboveCharts

            NavBar {
            }

        }

        Rectangle {
            id: statusBar

            Layout.fillWidth: true
            Layout.preferredHeight: Constants.statusBarPreferredHeight
            z: Constants.commonChart.zAboveCharts

            StatusBar {
            }

        }

    }

}
