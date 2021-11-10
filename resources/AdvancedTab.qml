import "AdvancedTabComponents" as AdvancedTabComponents
import "Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

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
            model: ["System Monitor", "IMU", "Magnetometer", "Networking", "Spectrum Analyzer", "INS"]

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

            AdvancedTabComponents.AdvancedSystemMonitorTab {
            }

            AdvancedTabComponents.AdvancedImuTab {
            }

            AdvancedTabComponents.AdvancedMagnetometerTab {
            }

            AdvancedTabComponents.AdvancedNetworkingTab {
            }

            AdvancedTabComponents.AdvancedSpectrumAnalyzerTab {
            }

            AdvancedTabComponents.AdvancedInsTab {
            }

        }

    }

}
