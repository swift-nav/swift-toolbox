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

    RowLayout {
        anchors.fill: parent

        SideNavBar {
            id: sideNavBar

            Layout.fillHeight: true
            Layout.minimumWidth: Constants.sideNavBar.tabBarWidth
        }

        ColumnLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true
            spacing: Constants.topLevelSpacing

            SplitView {
                orientation: Qt.Vertical
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.leftMargin: Constants.margins
                Layout.rightMargin: Constants.margins
                Layout.alignment: Qt.AlignTop

                MainTabs {
                    id: mainTabs

                    property alias curIndex: sideNavBar.curIndex

                    SplitView.fillHeight: true
                    SplitView.fillWidth: true
                    Layout.leftMargin: Constants.margins
                    Layout.rightMargin: Constants.margins
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
                id: loggingBar

                Layout.fillWidth: true
                Layout.preferredHeight: Constants.navBarPreferredHeight
                z: Constants.commonChart.zAboveCharts
                visible: false

                LoggingBar {
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

}
