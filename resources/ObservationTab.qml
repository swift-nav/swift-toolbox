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

    SplitView {
        id: observationView

        anchors.fill: parent
        orientation: Qt.Vertical
        width: parent.width
        height: parent.height
        visible: localTable.populated || remoteTable.populated

        Rectangle {
            SplitView.minimumHeight: Constants.observationTab.titleAreaHight
            SplitView.preferredHeight: 0.5 * parent.height
            width: parent.width
            color: "lightblue"
            border.color: "#333"
            border.width: 1

            ObservationTabComponents.ObservationTable {
                id: localTable
                anchors.fill: parent
                name: "local"
            }
        }

        Rectangle {
            SplitView.minimumHeight: Constants.observationTab.titleAreaHight
            SplitView.preferredHeight: 0.5 * parent.height
            Layout.fillHeight: true
            width: parent.width
            border.color: "#000000"
            border.width: 1

            ObservationTabComponents.ObservationTable {
                id: remoteTable
                anchors.fill: parent
                name: "remote"
                remote: true
            }
        }

        Timer {
            interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
            running: true
            repeat: true
            onTriggered: {
                if (!observationTab.visible)
                    return ;

                remoteTable.update()
                localTable.update()
            }
        }
    }
}
