import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

RowLayout {
    id: mainTabs

    spacing: Constants.topLevelSpacing
    

    TabBar {
        id: tab

        z: Constants.commonChart.zAboveCharts
        currentIndex: Globals.initialMainTabIndex
        Layout.fillHeight: true
        contentHeight: Constants.tabBarHeight
        contentWidth: Constants.tabBarWidth
        // elideMode: Text.ElideNone
        Component.onCompleted: {
            tab.contentItem.orientation = ListView.Vertical;
            // tabBar.elideMode = Text.ElideNone
        }

        Repeater {
            id: repeater

            model: ["Tracking", "Solution", "Baseline", "Observations", "Settings", "Update", "Advanced"]

            TabButton {
                text: modelData
                width: Constants.tabBarWidth
                anchors.horizontalCenter: parent.horizontalCenter
                Component.onCompleted: {
                    contentItem.children[0].elide = Text.ElideNone;
                }
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

        BaselineTab {
        }

        ObservationTab {
        }

        Item {
            id: settingsTab
        }

        Item {
            id: updateTab
        }

        AdvancedTab {
        }

    }

}
