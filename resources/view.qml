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
    onWidthChanged: {
        if (width < Constants.minimumWidth)
            width = Constants.minimumWidth;

    }
    onHeightChanged: {
        if (height < Constants.minimumHeight)
            height = Constants.minimumHeight;

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

                LogPanel {
                    SplitView.fillWidth: true
                    SplitView.preferredHeight: Constants.logPanelPreferredHeight
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
                    property alias title: main.title
                }

            }

        }

    }

}
