import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0
import "TrackingTabComponents" as TrackingTabComponents

ApplicationWindow {
    id: main

    Material.accent: Constants.swiftOrange
    width: Globals.width
    minimumWidth: Globals.minimumWidth
    height: Globals.height
    minimumHeight: Globals.minimumHeight
    font.pointSize: Constants.mediumPointSize
    visible: true
    // title: (loggingBar.sbpRecording ? "[L] " : "     ") + statusBar.title
    color: Constants.swiftWhite

    TrackingTabComponents.TrackingSignalsTab {
        anchors.fill: parent
    }

    // TextEdit {
    //     id: textEdit

    //     visible: false
    //     text: Globals.copyClipboard
    // }

    // Shortcut {
    //     sequences: [StandardKey.Copy]
    //     onActivated: {
    //         textEdit.selectAll();
    //         textEdit.copy();
    //         Globals.currentSelectedTable = null;
    //     }
    // }

    // MainDialogView {
    //     id: dialogStack

    //     anchors.fill: parent
    // }

    // LogoPopup {
    //     id: logoPopup

    //     anchors.fill: parent
    // }

    // UpdateNotifications {
    //     anchors.fill: parent
    // }

    // MouseArea {
    //     enabled: false
    //     anchors.top: parent.top
    //     anchors.left: parent.left
    //     anchors.right: parent.right
    //     anchors.rightMargin: parent.width - openRect.x + openRect.anchors.rightMargin
    //     z: 1
    //     height: 30
    //     visible: tabInfoBar.state == "closed"
    //     hoverEnabled: true
    //     acceptedButtons: Qt.NoButton
    //     onPositionChanged: tabInfoBarOpenTimer.restart()
    //     onExited: tabInfoBarOpenTimer.stop()

    //     Timer {
    //         id: tabInfoBarOpenTimer

    //         interval: 200
    //         onTriggered: tabInfoBar.open()
    //     }

    // }

    // TabInfoBar {
    //     id: tabInfoBar

    //     property int openDuration: 1000
    //     property int closeDuration: 350
    //     property bool autoClose: Constants.tabInfoBar.autoClose

    //     function cancelAutoClose() {
    //         tabInfoBarCloseTimer.stop();
    //     }

    //     function closeAfterDelaySubtabless() {
    //         if (tabName.length > 0 && subTabNames.length == 0)
    //             tabInfoBarCloseTimer.restart();
    //         else
    //             cancelAutoClose();
    //     }

    //     // We explicitly do not anchor in the vertical, so the item can
    //     // be slid up "under" the window.
    //     anchors.left: parent.left
    //     anchors.right: parent.right
    //     height: Constants.tabInfoBar.height
    //     z: 2
    //     tabName: sideNavBar.currentTabName
    //     subTabNames: mainTabs.subTabNames
    //     onAboutClicked: logoPopup.open()
    //     // If there is no subtabs, then close it after some time.
    //     onTabNameChanged: {
    //         if (autoClose)
    //             closeAfterDelaySubtabless();

    //     }
    //     onEntered: cancelAutoClose()
    //     onExited: {
    //         if (autoClose)
    //             closeAfterDelaySubtabless();

    //     }
    //     states: [
    //         // The opened state sets the y position so the item is
    //         // positioned so it's top is right at the top of the parent
    //         // item.
    //         State {
    //             name: "opened"

    //             PropertyChanges {
    //                 target: tabInfoBar
    //                 y: 0
    //             }

    //         },
    //         // The closed state sets the y position so the item is
    //         // positioned so it's bottom is right at the top of the
    //         // parent item, and all but one pixel height of the item is
    //         // hidden. One pixel is still shown so there is a border
    //         // line at the top of the view.
    //         State {
    //             name: "closed"

    //             PropertyChanges {
    //                 target: tabInfoBar
    //                 y: -height + 1
    //             }

    //         }
    //     ]
    //     // Make the opened/closed state transitions smooth.
    //     transitions: [
    //         Transition {
    //             from: "opened"
    //             to: "closed"

    //             NumberAnimation {
    //                 target: tabInfoBar
    //                 properties: "y"
    //                 duration: tabInfoBar.closeDuration
    //                 easing.type: Easing.OutQuad
    //             }

    //         },
    //         Transition {
    //             from: "closed"
    //             to: "opened"

    //             NumberAnimation {
    //                 target: tabInfoBar
    //                 properties: "y"
    //                 duration: tabInfoBar.openDuration
    //                 easing.type: Easing.OutQuad
    //             }

    //         }
    //     ]

    //     Timer {
    //         id: tabInfoBarCloseTimer

    //         interval: 3000
    //         onTriggered: parent.close()
    //     }

    // }

    // Rectangle {
    //     id: openRect

    //     anchors.right: parent.right
    //     anchors.rightMargin: 5
    //     y: -3
    //     z: 1
    //     implicitHeight: openArrow.implicitHeight + 9
    //     implicitWidth: 20
    //     color: Constants.swiftControlBackground
    //     radius: 3
    //     clip: true

    //     MouseArea {
    //         anchors.fill: parent
    //         hoverEnabled: true
    //         onClicked: tabInfoBar.open()
    //         onEntered: openArrowAnimation.start()
    //         onExited: {
    //             if (openArrowAnimation.running) {
    //                 openArrowAnimation.stop();
    //                 openArrow.y = openArrowAnimation.startingPropertyValue;
    //             }
    //         }
    //     }

    //     PositionLoopAnimation {
    //         id: openArrowAnimation

    //         target: openArrow
    //         property: "y"
    //         startingPropertyValue: 0
    //         totalDuration: 700
    //         reverse: true
    //     }

    //     Text {
    //         id: openArrow

    //         anchors.horizontalCenter: parent.horizontalCenter
    //         y: (parent.height - height) - 3
    //         text: "▼"
    //         color: Constants.swiftLightGrey
    //         onYChanged: {
    //             if (!openArrowAnimation.running)
    //                 openArrowAnimation.startingPropertyValue = y;

    //         }
    //     }

    // }

    // RowLayout {
    //     property alias stackView: dialogStack.dialogStack

    //     anchors.left: parent.left
    //     anchors.right: parent.right
    //     anchors.top: tabInfoBar.bottom
    //     anchors.bottom: parent.bottom
    //     spacing: 0

    //     SideNavBar {
    //         id: sideNavBar

    //         Layout.fillHeight: true
    //         Layout.minimumWidth: Constants.sideNavBar.tabBarWidth
    //         enabled: stack.currentIndex != 0
    //         dataRate: statusBar.dataRate
    //         solidConnection: statusBar.solidConnection
    //     }

    //     StackLayout {
    //         id: stack

    //         function connectionScreen() {
    //             stack.currentIndex = 0;
    //             sideNavBar.currentIndex = -1;
    //             sideNavBar.checkedButton = null;
    //         }

    //         function connectionScreenVisible() {
    //             return stack.currentIndex == 0;
    //         }

    //         function mainView() {
    //             if (sideNavBar.currentIndex < 0)
    //                 sideNavBar.clickButton(Globals.initialMainTabIndex);

    //             stack.currentIndex = 1;
    //         }

    //         function mainViewVisible() {
    //             return stack.currentIndex == 1;
    //         }

    //         currentIndex: 0
    //         Layout.fillHeight: true
    //         Layout.fillWidth: true

    //         ConnectionScreen {
    //         }

    //         ColumnLayout {
    //             id: mainView

    //             spacing: Constants.topLevelSpacing

    //             SplitView {
    //                 orientation: Qt.Vertical
    //                 Layout.fillWidth: true
    //                 Layout.fillHeight: true
    //                 Layout.alignment: Qt.AlignTop

    //                 MainTabs {
    //                     id: mainTabs

    //                     curSubTabIndex: tabInfoBar.curSubTabIndex
    //                     SplitView.fillHeight: true
    //                     currentIndex: sideNavBar.currentIndex
    //                 }

    //                 ColumnLayout {
    //                     SplitView.preferredHeight: loggingBar.preferredHeight + logPanel.preferredHeight
    //                     SplitView.minimumHeight: loggingBar.preferredHeight
    //                     spacing: Constants.topLevelSpacing

    //                     LoggingBar {
    //                         id: loggingBar

    //                         Layout.fillWidth: true
    //                         Layout.preferredHeight: preferredHeight
    //                     }

    //                     LogPanel {
    //                         id: logPanel

    //                         Layout.fillWidth: true
    //                         Layout.fillHeight: true
    //                     }

    //                 }

    //             }

    //             StatusBar {
    //                 id: statusBar

    //                 Layout.fillWidth: true
    //             }

    //         }

    //     }

    // }

    // Rectangle {
    //     z: -1
    //     anchors.left: parent.left
    //     anchors.right: parent.right
    //     anchors.bottom: parent.bottom
    //     anchors.bottomMargin: -1
    //     height: 2
    //     color: Constants.swiftGrey
    // }

}
