import "AdvancedTabComponents" as AdvancedTabComponents
import "Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

MainTab {
    id: advancedTab

    subTabNames: ["System Monitor", "IMU", "Magnetometer", "Networking", "Spectrum Analyzer", "INS"]
    curSubTabIndex: 0

    StackLayout {
        id: advancedBarLayout

        anchors.fill: parent
        currentIndex: curSubTabIndex

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
