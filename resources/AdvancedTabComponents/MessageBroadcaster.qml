import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property alias ip_address: ipAddressInput.text
    property alias port: portInput.text
    property bool ipAddressEditing: false
    property bool portEditing: false
    property alias startEnabled: startButton.enabled
    property alias stopEnabled: stopButton.enabled
    property alias ipAddressInputEnabled: ipAddressInput.enabled
    property alias portInputEnabled: portInput.enabled
    property bool messageTypeSelectionEnabled: true

    implicitHeight: Constants.networking.messageBroadcasterHeight

    GridLayout {
        anchors.fill: parent
        anchors.margins: Constants.networking.messageBroadcasterMargins
        rows: Constants.networking.messageBroadcasterGridRows
        columns: Constants.networking.messageBroadcasterGridColumns

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            Rectangle {
                anchors.fill: parent

                Label {
                    text: "Messages to broadcast:"
                    anchors.fill: parent
                    anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    horizontalAlignment: Text.AlignRight
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            RowLayout {
                anchors.centerIn: parent

                RadioButton {
                    checked: true
                    enabled: messageTypeSelectionEnabled
                    text: qsTr("Observations")
                    onToggled: {
                        let refresh = false;
                        let start = false;
                        let stop = false;
                        let allMessages = false;
                        let ipv4_address = null;
                        let port = null;
                        backend_request_broker.advanced_networking([refresh, start, stop], allMessages, ipv4_address, port);
                    }
                }

                RadioButton {
                    text: qsTr("All")
                    enabled: messageTypeSelectionEnabled
                    onToggled: {
                        let refresh = false;
                        let start = false;
                        let stop = false;
                        let allMessages = true;
                        let ipv4_address = null;
                        let port = null;
                        backend_request_broker.advanced_networking([refresh, start, stop], allMessages, ipv4_address, port);
                    }
                }
            }
        }

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            Rectangle {
                anchors.fill: parent

                Label {
                    text: "IP Address:"
                    anchors.fill: parent
                    anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    horizontalAlignment: Text.AlignRight
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            Rectangle {
                anchors.right: parent.right
                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter
                height: Constants.networking.messageBroadcasterTextInputHeight
                border.width: Constants.advancedImu.textDataBarBorderWidth
                clip: true

                TextInput {
                    id: ipAddressInput

                    text: ""
                    cursorVisible: true
                    selectByMouse: true
                    font.pixelSize: Constants.largePixelSize
                    font.family: Constants.genericTable.fontFamily
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    onTextEdited: {
                        ipAddressEditing = true;
                    }
                    onEditingFinished: {
                        let refresh = false;
                        let start = false;
                        let stop = false;
                        let allMessages = null;
                        let ipv4_address = text;
                        let port = null;
                        backend_request_broker.advanced_networking([refresh, start, stop], allMessages, ipv4_address, port);
                        ipAddressEditing = false;
                    }
                }
            }
        }

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            Rectangle {
                anchors.fill: parent

                Label {
                    text: "Port:"
                    anchors.fill: parent
                    anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    horizontalAlignment: Text.AlignRight
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            Rectangle {
                anchors.right: parent.right
                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter
                height: Constants.networking.messageBroadcasterTextInputHeight
                border.width: Constants.advancedImu.textDataBarBorderWidth
                clip: true

                TextInput {
                    id: portInput

                    text: ""
                    cursorVisible: true
                    selectByMouse: true
                    font.pixelSize: Constants.largePixelSize
                    font.family: Constants.genericTable.fontFamily
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    onTextEdited: {
                        portEditing = true;
                    }
                    onEditingFinished: {
                        let refresh = false;
                        let start = false;
                        let stop = false;
                        let allMessages = null;
                        let ipv4_address = null;
                        let port = text;
                        backend_request_broker.advanced_networking([refresh, start, stop], allMessages, ipv4_address, port);
                        portEditing = false;
                    }

                    validator: IntValidator {
                        bottom: Constants.networking.messageBroadcasterIntValidatorUInt16Min
                        top: Constants.networking.messageBroadcasterIntValidatorUInt16Max
                    }
                }
            }
        }

        RowLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            Item {
                Layout.fillHeight: true
                Layout.fillWidth: true
            }

            Item {
                Layout.fillHeight: true
                Layout.preferredWidth: parent.width / 2

                SwiftButton {
                    id: startButton

                    width: parent.width
                    height: Constants.networking.messageBroadcasterStartStopButtonHeight
                    topInset: Constants.updateTab.buttonInset
                    bottomInset: Constants.updateTab.buttonInset
                    onClicked: {
                        let refresh = false;
                        let start = true;
                        let stop = false;
                        let allMessages = null;
                        let ipv4_address = null;
                        let port = null;
                        backend_request_broker.advanced_networking([refresh, start, stop], allMessages, ipv4_address, port);
                    }

                    Label {
                        text: "Start"
                        anchors.centerIn: parent
                    }
                }
            }
        }

        RowLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredHeight: Constants.networking.messageBroadcasterGridElementLength
            Layout.columnSpan: Constants.networking.messageBroadcasterGridElementLength
            Layout.preferredWidth: Constants.networking.messageBroadcasterGridElementLength

            Item {
                Layout.fillHeight: true
                Layout.preferredWidth: parent.width / 2

                SwiftButton {
                    id: stopButton

                    enabled: false
                    width: parent.width
                    height: Constants.networking.messageBroadcasterStartStopButtonHeight
                    topInset: Constants.updateTab.buttonInset
                    bottomInset: Constants.updateTab.buttonInset
                    onClicked: {
                        let refresh = false;
                        let start = false;
                        let stop = true;
                        let allMessages = null;
                        let ipv4_address = null;
                        let port = null;
                        backend_request_broker.advanced_networking([refresh, start, stop], allMessages, ipv4_address, port);
                    }

                    Label {
                        text: "Stop"
                        anchors.centerIn: parent
                    }
                }
            }

            Item {
                Layout.fillHeight: true
                Layout.fillWidth: true
            }
        }
    }
}
