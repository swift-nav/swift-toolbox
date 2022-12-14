/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
import "Constants"
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
                    return;
                remoteTable.update();
                localTable.update();
            }
        }
    }
}
