/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
import "Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Layouts
import QtQuick.Window
import SwiftConsole

ApplicationWindow {
    id: main

    Material.accent: Constants.swiftOrange
    width: Globals.width
    minimumWidth: Globals.minimumWidth
    height: Globals.height
    minimumHeight: Globals.minimumHeight
    font.pixelSize: Constants.mediumPixelSize
    visible: true
    title: (loggingBar.sbpRecording ? "[L] " : "     ") + statusBar.title
    color: Constants.swiftWhite
    Component.onCompleted: {
        this.x = Screen.width / 2 - width / 2;
        this.y = Screen.height / 2 - height / 2;
    }

    ConnectionData {
        id: connectionData
    }

    TextEdit {
        id: textEdit

        visible: false
        text: Globals.copyClipboard
    }

    Shortcut {
        sequences: [StandardKey.Copy]
        onActivated: {
            textEdit.selectAll();
            textEdit.copy();
            Globals.currentSelectedTable = null;
        }
    }

    MainDialogView {
        id: dialogStack

        anchors.fill: parent
    }

    LogoPopup {
        id: logoPopup

        anchors.fill: parent
    }

    UpdateNotifications {
        anchors.fill: parent
    }

    MouseArea {
        enabled: false
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.rightMargin: parent.width - openRect.x + openRect.anchors.rightMargin
        z: 1
        height: 30
        visible: tabInfoBar.state == "closed"
        hoverEnabled: true
        acceptedButtons: Qt.NoButton
        onPositionChanged: tabInfoBarOpenTimer.restart()
        onExited: tabInfoBarOpenTimer.stop()

        Timer {
            id: tabInfoBarOpenTimer

            interval: 200
            onTriggered: tabInfoBar.open()
        }
    }

    TabInfoBar {
        id: tabInfoBar

        property int openDuration: 1000
        property int closeDuration: 350
        property bool autoClose: Constants.tabInfoBar.autoClose

        function cancelAutoClose() {
            tabInfoBarCloseTimer.stop();
        }

        function closeAfterDelaySubtabless() {
            if (tabName.length > 0 && subTabNames.length == 0)
                tabInfoBarCloseTimer.restart();
            else
                cancelAutoClose();
        }

        // We explicitly do not anchor in the vertical, so the item can
        // be slid up "under" the window.
        anchors.left: parent.left
        anchors.right: parent.right
        height: Constants.tabInfoBar.height
        z: 2
        tabName: sideNavBar.currentTabName
        subTabNames: mainTabs.subTabNames
        onAboutClicked: logoPopup.open()
        // If there is no subtabs, then close it after some time.
        onTabNameChanged: {
            if (autoClose)
                closeAfterDelaySubtabless();
        }
        onEntered: cancelAutoClose()
        onExited: {
            if (autoClose)
                closeAfterDelaySubtabless();
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
                    duration: tabInfoBar.closeDuration
                    easing.type: Easing.OutQuad
                }
            },
            Transition {
                from: "closed"
                to: "opened"

                NumberAnimation {
                    target: tabInfoBar
                    properties: "y"
                    duration: tabInfoBar.openDuration
                    easing.type: Easing.OutQuad
                }
            }
        ]

        Timer {
            id: tabInfoBarCloseTimer

            interval: 3000
            onTriggered: parent.close()
        }
    }

    Rectangle {
        id: openRect

        anchors.right: parent.right
        anchors.rightMargin: 5
        y: -3
        z: 1
        implicitHeight: openArrow.implicitHeight + 9
        implicitWidth: 20
        color: Constants.swiftControlBackground
        radius: 3
        clip: true

        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onClicked: tabInfoBar.open()
            onEntered: openArrowAnimation.start()
            onExited: {
                if (openArrowAnimation.running) {
                    openArrowAnimation.stop();
                    openArrow.y = openArrowAnimation.startingPropertyValue;
                }
            }
        }

        PositionLoopAnimation {
            id: openArrowAnimation

            target: openArrow
            property: "y"
            startingPropertyValue: 0
            totalDuration: 700
            reverse: true
        }

        Text {
            id: openArrow

            anchors.horizontalCenter: parent.horizontalCenter
            y: (parent.height - height) - 3
            text: "▼"
            color: Constants.swiftLightGrey
            onYChanged: {
                if (!openArrowAnimation.running)
                    openArrowAnimation.startingPropertyValue = y;
            }
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
            dataRate: statusBar.dataRate
            solidConnection: statusBar.solidConnection
        }

        StackLayout {
            id: stack

            function connectionScreen() {
                stack.currentIndex = 0;
                Globals.initialMainTabIndex = sideNavBar.currentIndex;
                Globals.initialSubTabIndex = tabInfoBar.curSubTabIndex;
                sideNavBar.currentIndex = -1;
                sideNavBar.checkedButton = null;
            }

            function connectionScreenVisible() {
                return stack.currentIndex == 0;
            }

            function mainView() {
                if (sideNavBar.currentIndex < 0) {
                    sideNavBar.clickButton(Globals.initialMainTabIndex);
                    tabInfoBar.curSubTabIndex = Globals.initialSubTabIndex;
                }
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
                        currentIndex: sideNavBar.currentIndex
                    }

                    ColumnLayout {
                        SplitView.preferredHeight: loggingBar.preferredHeight + logPanel.preferredHeight
                        SplitView.minimumHeight: loggingBar.preferredHeight
                        spacing: Constants.topLevelSpacing

                        LoggingBar {
                            id: loggingBar

                            Layout.fillWidth: true
                            Layout.preferredHeight: preferredHeight
                        }

                        LogPanel {
                            id: logPanel

                            Layout.fillWidth: true
                            Layout.fillHeight: true
                        }
                    }
                }

                StatusBar {
                    id: statusBar

                    Layout.fillWidth: true
                }
            }
        }
    }

    Rectangle {
        z: -1
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.bottomMargin: -1
        height: 2
        color: Constants.swiftGrey
    }
}
