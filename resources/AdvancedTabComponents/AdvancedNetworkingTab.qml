import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: advancedNetworkingTab

    AdvancedNetworkingData {
        id: advancedNetworkingData
    }

    GridLayout {
        id: gridLayout

        rows: 2
        columns: 5
        rowSpacing: 0
        columnSpacing: 0
        anchors.fill: parent

        ColumnLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 2
            Layout.preferredWidth: 2

            MessageBroadcaster {
                id: messageBroadcaster
                Layout.fillWidth: true
                Layout.preferredHeight: 150
            }

            Item {
                Layout.fillHeight: true
                Layout.fillWidth: true
            }

        }

        Rectangle {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 3
            Layout.preferredWidth: 3

            Text {
                anchors.fill: parent
                anchors.margins: 10
                clip: true
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
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

        // NetworkInfo {
        //     id: networkInfoTable

        //     Layout.fillHeight: true
        //     Layout.fillWidth: true
        //     Layout.rowSpan: 1
        //     Layout.preferredHeight: 1
        //     Layout.columnSpan: 5
        //     Layout.preferredWidth: 5
            
        //     // Layout.fillHeight: true
        //     // Layout.fillWidth: true
        // }

        GroupBox {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 5
            Layout.preferredWidth: 5

            ColumnLayout {
                width: parent.width
                height: parent.width

                NetworkInfo {
                    id: networkInfoTable
                    
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }
                Item {
                    Layout.preferredHeight: 50
                    Layout.fillWidth: true

                    Button {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.verticalCenterOffset: 10
                        width: Constants.networking.refreshButtonWidth
                        height: Constants.networking.refreshButtonHeight
                        ToolTip.visible: hovered
                        ToolTip.text: Constants.networking.refreshButtonLabel
                        text: Constants.networking.refreshButtonLabel
                        icon.source: "../" + Constants.icons.connectButtonPath
                        icon.width: Constants.networking.refreshButtonIconSideLength
                        icon.height: Constants.networking.refreshButtonIconSideLength
                        display: AbstractButton.TextUnderIcon
                        onClicked: {
                            let refresh = false;
                            let start = false;
                            let stop = false;
                            let allMessages = false;
                            let ipv4_address = null;
                            let port = null;
                            data_model.advanced_networking([refresh, start, stop, allMessages], ipv4_address, port);
                        }
                    }

                }

            }

            label: Text {
                text: "Network"
            }

        }

    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            if (!advancedTab.visible)
                return ;
            advanced_networking_model.fill_console_points(advancedNetworkingData);
            messageBroadcaster.ip_address = advancedNetworkingData.ip_address;
            messageBroadcaster.port = advancedNetworkingData.port;
            
            if (!advancedNetworkingData.network_info.length)
                return ;
            print(!advancedNetworkingData.network_info.length)
            print(advancedNetworkingData.network_info)
            networkInfoTable.entries = advancedNetworkingData.network_info;
            
            
        }
    }

}
