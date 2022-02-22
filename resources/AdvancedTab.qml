import "AdvancedTabComponents" as AdvancedTabComponents
import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15

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
