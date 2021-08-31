import "Constants"
import QtQuick 2.5
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

        UpdateTab {
        }

        AdvancedTab {
        }

    }

}
