import "./Constants"
import "ObservationTabComponents" as ObservationTabComponents
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: observationTab

    width: parent.width
    height: parent.height

    ObservationData {
        id: observationData
    }

    SplitView {
        id: observationView

        anchors.fill: parent
        orientation: Qt.Vertical
        width: parent.width
        height: parent.height

        Rectangle {
            SplitView.minimumHeight: 100
            SplitView.preferredHeight: 300
            width: parent.width
            color: "lightblue"
            border.color: "#333"
            border.width: 1

            ObservationTabComponents.ObservationTable {
                id: localTable

                name: "local"
                width: parent.width
                height: parent.height
            }

        }

        Rectangle {
            SplitView.minimumHeight: 100
            SplitView.fillHeight: true
            width: parent.width
            border.color: "#000000"
            border.width: 1

            ObservationTabComponents.ObservationTable {
                id: remoteTable

                name: "remote"
                width: parent.width
                height: parent.height
            }

        }

        Timer {
            interval: Constants.currentRefreshRate
            running: true
            repeat: true
            onTriggered: {
                if (!observationTab.visible)
                    return ;

                remote_observation_model.fill_data(observationData, true);
                if (observationData.rows.length) {
                    remoteTable.tow = observationData.tow;
                    remoteTable.week = observationData.week;
                    remoteTable.model.rows = observationData.rows;
                }
                local_observation_model.fill_data(observationData, false);
                if (observationData.rows.length) {
                    localTable.tow = observationData.tow;
                    localTable.week = observationData.week;
                    localTable.model.rows = observationData.rows;
                }
            }
        }

    }

}
