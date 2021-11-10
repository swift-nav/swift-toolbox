import "Constants"
import QtQuick
import QtQuick.Layouts
import SwiftConsole

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

        SettingsTab {
        }

        UpdateTab {
        }

        AdvancedTab {
        }

    }

}
