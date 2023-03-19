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
import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: advancedNetworkingTab

    AdvancedNetworkingData {
        id: advancedNetworkingData

        function update() {
            if (!advancedTab.visible)
                return;
            advanced_networking_model.fill_console_points(advancedNetworkingData);
            if (advancedNetworkingData.running) {
                messageBroadcaster.messageTypeSelectionEnabled = false;
                messageBroadcaster.ipAddressInputEnabled = false;
                messageBroadcaster.portInputEnabled = false;
                messageBroadcaster.startEnabled = false;
                messageBroadcaster.stopEnabled = true;
            } else {
                messageBroadcaster.messageTypeSelectionEnabled = true;
                messageBroadcaster.ipAddressInputEnabled = true;
                messageBroadcaster.portInputEnabled = true;
                messageBroadcaster.startEnabled = true;
                messageBroadcaster.stopEnabled = false;
            }
            if (!messageBroadcaster.ipAddressEditing)
                messageBroadcaster.ip_address = advancedNetworkingData.ip_address;
            if (!messageBroadcaster.portEditing)
                messageBroadcaster.port = advancedNetworkingData.port;
            if (!advancedNetworkingData.network_info.length)
                return;
            networkInfoTable.entries = advancedNetworkingData.network_info;
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: Constants.networking.layoutSpacing

        RowLayout {
            Layout.fillWidth: true
            spacing: Constants.networking.layoutSpacing

            MessageBroadcaster {
                id: messageBroadcaster

                Layout.alignment: Qt.AlignTop
                Layout.preferredWidth: parent.width * 2 / 5
            }

            Rectangle {
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true
                implicitHeight: udpStreamingLabel.implicitHeight

                Label {
                    id: udpStreamingLabel

                    anchors.fill: parent
                    padding: Constants.networking.udpStreamingParagraphPadding
                    clip: true
                    wrapMode: Text.Wrap
                    text: {
                        let text = "";
                        text += "UDP Streaming";
                        text += "\n\nBroadcast SBP information received by ";
                        text += "the console to other machines or processes over UDP. With the \'Observations\' ";
                        text += "radio button selected, the console will broadcast the necessary information ";
                        text += "for a rover Piksi to acheive an RTK solution. ";
                        text += "\n\nThis can be used to stream observations to a remote Piksi through ";
                        text += "aircraft telemetry via ground control software such as MAVProxy or ";
                        text += "Mission Planner.";
                        return text;
                    }
                }
            }
        }

        GroupBox {
            Layout.fillHeight: true
            Layout.fillWidth: true
            title: "Network"

            ColumnLayout {
                anchors.fill: parent

                Item {
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    NetworkInfo {
                        id: networkInfoTable

                        width: parent.width
                        height: parent.height
                    }
                }

                Item {
                    Layout.preferredHeight: Constants.networking.refreshButtonHeight
                    Layout.fillWidth: true

                    Button {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.verticalCenterOffset: Constants.networking.refreshButtonVerticalOffset
                        ToolTip.visible: hovered
                        ToolTip.text: Constants.networking.refreshButtonLabel
                        text: Constants.networking.refreshButtonLabel
                        icon.source: Constants.icons.refreshPath
                        icon.width: Constants.networking.refreshButtonIconSideLength
                        icon.height: Constants.networking.refreshButtonIconSideLength
                        display: AbstractButton.TextUnderIcon
                        flat: true
                        onClicked: {
                            let refresh = true;
                            let start = false;
                            let stop = false;
                            let allMessages = null;
                            let ipv4_address = null;
                            let port = null;
                            backend_request_broker.advanced_networking([refresh, start, stop], allMessages, ipv4_address, port);
                        }
                    }
                }
            }
        }
    }
}
