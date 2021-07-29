import "Constants"
// import QtCharts 2.2
import QtQuick 2.5
// import QtGraphicalEffects 1.15
// import QtLocation 5.15
// import QtQuick.Controls 2.5
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    StackLayout {
        id: stackLayout

        anchors.fill: parent
        currentIndex: parent.curIndex

        Item {
        }

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
