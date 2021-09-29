import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
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
            }

            TrackingTabComponents.TrackingSkyPlotTab {
            }

        }

    }

}
