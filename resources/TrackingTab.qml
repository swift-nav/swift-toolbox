import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "TrackingTabComponents" as TrackingTabComponents

MainTab {
    id: trackingTab

    subTabNames: ["Signals", "Sky Plot"]
    curSubTabIndex: Globals.initialMainTabIndex == 0 ? Globals.initialSubTabIndex : 0

    StackLayout {
        id: trackingBarLayout

        anchors.fill: parent
        currentIndex: curSubTabIndex

        TrackingTabComponents.TrackingSignalsTab {
        }

        TrackingTabComponents.TrackingSkyPlotTab {
        }

    }

}
