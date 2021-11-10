import "Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "TrackingTabComponents" as TrackingTabComponents

Item {
    id: trackingTab

    width: parent.width
    height: parent.height

    TabBar {
        id: trackingBar

        z: Constants.commonChart.zAboveCharts
        currentIndex: Globals.initialMainTabIndex == 0 ? Globals.initialSubTabIndex : 0
        contentHeight: Constants.tabBarHeight

        Repeater {
            model: ["Signals", "Sky Plot"]

            TabButton {
                text: modelData
                width: implicitWidth
            }

        }

    }

    Rectangle {
        id: trackingTabBackground

        width: parent.width
        height: parent.height
        anchors.top: trackingBar.bottom
        anchors.bottom: trackingTab.bottom
        Component.onCompleted: {
        }

        StackLayout {
            id: trackingBarLayout

            width: parent.width
            height: parent.height
            currentIndex: trackingBar.currentIndex

            TrackingTabComponents.TrackingSignalsTab {
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

            TrackingTabComponents.TrackingSkyPlotTab {
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

        }

    }

}
