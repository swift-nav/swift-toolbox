import "./Constants"
import "BaseComponents"
import "ObservationTabComponents" as ObservationTabComponents
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

MainTab {
    id: observationTab

    ObservationRemoteTableModel {
        id: observationRemoteTableModel
    }

    ObservationLocalTableModel {
        id: observationLocalTableModel
    }

    SplitView {
        id: observationView

        anchors.fill: parent
        orientation: Qt.Vertical
        width: parent.width
        height: parent.height
        visible: true

        Rectangle {
            SplitView.minimumHeight: Constants.observationTab.titleAreaHight
            SplitView.preferredHeight: 0.5 * parent.height
            width: parent.width

            SwiftGroupBox {
                anchors.fill: parent
                anchors.topMargin: 4
                title: "Local"

                ObservationTabComponents.ObservationTable {
                    id: localTable

                    anchors.fill: parent
                    observationTableModel: observationLocalTableModel
                }

            }

        }

        Rectangle {
            SplitView.minimumHeight: Constants.observationTab.titleAreaHight
            Layout.fillHeight: true
            width: parent.width

            SwiftGroupBox {
                anchors.fill: parent
                anchors.topMargin: 4
                title: "Remote"

                ObservationTabComponents.ObservationTable {
                    id: remoteTable

                    anchors.fill: parent
                    observationTableModel: observationRemoteTableModel
                }

            }

        }

        Timer {
            interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
            running: true
            repeat: true
            onTriggered: {
                if (!observationTab.visible)
                    return ;

                remoteTable.update();
                localTable.update();
            }
        }

    }

}
