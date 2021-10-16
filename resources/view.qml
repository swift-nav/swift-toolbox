import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ApplicationWindow {
    id: main

    Material.accent: Constants.swiftOrange
    width: Globals.width
    height: Globals.height
    font.pointSize: Constants.mediumPointSize
    Component.onCompleted: {
        visible = true;
    }

    MainDialogView {
        id: dialogStack

        anchors.fill: parent
    }

    RowLayout {
        property alias drawer: mainDrawer.drawer
        property alias stackView: dialogStack.dialogStack

        anchors.fill: parent

        MainDrawer {
            id: mainDrawer
        }

        SideNavBar {
            id: sideNavBar

            Layout.fillHeight: true
            Layout.minimumWidth: Constants.sideNavBar.tabBarWidth
        }

        StackLayout {
            id: stack

            property bool connected_at_least_once: false

            function connectionScreen() {
                stack.currentIndex = 0;
            }

            function connectionScreenVisible() {
                return stack.currentIndex == 0;
            }

            function mainView() {
                stack.currentIndex = 1;
            }

            function mainViewVisible() {
                return stack.currentIndex == 1;
            }

            currentIndex: 0
            Layout.fillHeight: true
            Layout.fillWidth: true

            ConnectionScreen {
            }

            ColumnLayout {
                id: mainView

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

                    ColumnLayout {
                        SplitView.fillWidth: true
                        SplitView.preferredHeight: Constants.logPanelPreferredHeight + Constants.loggingBarPreferredHeight
                        SplitView.minimumHeight: Constants.loggingBarPreferredHeight
                        spacing: Constants.topLevelSpacing

                        LoggingBar {
                            id: loggingBar

                            Layout.fillWidth: true
                            Layout.preferredHeight: Constants.loggingBarPreferredHeight
                        }

                        LogPanel {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                        }

                    }

                }

                Rectangle {
                    id: statusBar

                    Layout.fillWidth: true
                    Layout.preferredHeight: Constants.statusBarPreferredHeight
                    z: Constants.commonChart.zAboveCharts

                    StatusBar {
                        property alias sbpRecording: loggingBar.sbpRecording
                        property alias title: main.title
                    }

                }

            }

        }

    }

}
