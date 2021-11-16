import "Constants"
import QtQuick 2.5
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias currentIndex: stackLayout.currentIndex
    StackLayout {
        id: stackLayout

        anchors.fill: parent

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
