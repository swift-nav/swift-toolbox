import "Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "TrackingTabComponents" as TrackingTabComponents

MainTab {
    id: trackingTab

    subTabNames: ["Signals", "Sky Plot"]
    curSubTabIndex: 0

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
