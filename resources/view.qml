import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
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

    MouseArea {
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 10
        visible: tabInfoBar.state == "closed"
        hoverEnabled: true
        z: 1
        onEntered: {
            console.log("entered mousearea");
            tabInfoBar.state = "opened";
        }
    }

    TabInfoBar {
        id: tabInfoBar

        // We explicitly do not anchor in the vertical, so the item can
        // be slid up "under" the window.
        anchors.left: parent.left
        anchors.right: parent.right
        tabName: sideNavBar.currentTabName
        subTabNames: mainTabs.subTabNames
        state: "opened"
        // When the tab name changes, make sure this item is shown.
        // If there is no subtabs, then close it after some time.
        onTabNameChanged: {
            state = "opened";
            if (tabName.length > 0 && subTabNames.length == 0)
                tabInfoBarCloseTimer.restart();

        }
        states: [
            // The opened state sets the y position so the item is
            // positioned so it's top is right at the top of the parent
            // item.
            State {
                name: "opened"

                PropertyChanges {
                    target: tabInfoBar
                    y: 0
                }

            },
            // The closed state sets the y position so the item is
            // positioned so it's bottom is right at the top of the
            // parent item, and all but one pixel height of the item is
            // hidden. One pixel is still shown so there is a border
            // line at the top of the view.
            State {
                name: "closed"

                PropertyChanges {
                    target: tabInfoBar
                    y: -height + 1
                }

            }
        ]
        // Make the opened/closed state transitions smooth.
        transitions: [
            Transition {
                from: "opened"
                to: "closed"

                NumberAnimation {
                    target: tabInfoBar
                    properties: "y"
                    duration: 1000
                    easing.type: Easing.OutQuad
                }

            },
            Transition {
                from: "closed"
                to: "opened"

                NumberAnimation {
                    target: tabInfoBar
                    properties: "y"
                    duration: 350
                    easing.type: Easing.OutQuad
                }

            }
        ]

        Timer {
            id: tabInfoBarCloseTimer

            interval: 3000
            running: false
            onTriggered: parent.state = "closed"
        }

        // This captures any clicks outside of the buttons, and toggles
        // the state from opened to closed or vice versa.
        MouseArea {
            z: -1
            anchors.fill: parent
            onClicked: parent.state = parent.state == "opened" ? "closed" : "opened"
        }

    }

    RowLayout {
        property alias stackView: dialogStack.dialogStack

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: tabInfoBar.bottom
        anchors.bottom: parent.bottom
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
