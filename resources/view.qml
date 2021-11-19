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
    minimumWidth: Globals.minimumWidth
    height: Globals.height
    minimumHeight: Globals.minimumHeight
    font.pointSize: Constants.mediumPointSize
    visible: true

    MainDialogView {
        id: dialogStack

        anchors.fill: parent
    }

    LogoPopup {
        id: logoPopup

        anchors.fill: parent
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        TabInfoBar {
            id: tabInfoBar

            Layout.fillWidth: true
            subTabNames: mainTabs.subTabNames
        }

        RowLayout {
            property alias stackView: dialogStack.dialogStack

            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 0

            SideNavBar {
                id: sideNavBar

                Layout.fillHeight: true
                Layout.minimumWidth: Constants.sideNavBar.tabBarWidth
                enabled: stack.currentIndex != 0
            }

            StackLayout {
                id: stack

                function connectionScreen() {
                    stack.currentIndex = 0;
                }

                function connectionScreenVisible() {
                    return stack.currentIndex == 0;
                }

                function mainView() {
                    if (sideNavBar.currentIndex < 0)
                        sideNavBar.clickButton(Globals.initialMainTabIndex);

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
                        Layout.alignment: Qt.AlignTop

                        MainTabs {
                            id: mainTabs

                            curSubTabIndex: tabInfoBar.curSubTabIndex
                            SplitView.fillHeight: true
                            SplitView.fillWidth: true
                            Layout.leftMargin: Constants.margins
                            Layout.rightMargin: Constants.margins
                            currentIndex: sideNavBar.currentIndex
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

}
