import "AdvancedTabComponents" as AdvancedTabComponents
import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15

Item {
    id: advancedTab

    width: parent.width
    height: parent.height

    TabBar {
        id: advancedBar

        z: Constants.commonChart.zAboveCharts
        currentIndex: Globals.initialMainTabIndex == 6 ? Globals.initialSubTabIndex : 0
        contentHeight: Constants.tabBarHeight

        Repeater {
            model: ["System Monitor", "INS", "Magnetometer", "Networking", "Spectrum Analyzer"]

            TabButton {
                text: modelData
                width: implicitWidth
            }

        }

    }

    Rectangle {
        id: advancedTabBackground

        width: parent.width
        height: parent.height
        anchors.top: advancedBar.bottom
        anchors.bottom: advancedTab.bottom
        Component.onCompleted: {
        }

        StackLayout {
            id: advancedBarLayout

            width: parent.width
            height: parent.height
            currentIndex: advancedBar.currentIndex

            Item {
                id: advancedSystemMonitorTab
            }

            AdvancedTabComponents.AdvancedInsTab {
            }

            Item {
                id: advancedMagnetometerTab
            }

            Item {
                id: advancedNetworkingTab
            }

            Item {
                id: advancedSpectrumAnalyzerTab
            }

        }

    }

}
