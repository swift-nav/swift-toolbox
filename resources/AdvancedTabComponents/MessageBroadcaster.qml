import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

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

    GridLayout {
        anchors.fill: parent
        anchors.margins: 10
        rows: 4
        columns: 2

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

            Rectangle {
                anchors.fill: parent

                Text {
                    text: "Messages to broadcast:"
                    font.family: Constants.genericTable.fontFamily
                    font.pointSize: Constants.largePointSize
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
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

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
                        data_model.advanced_networking([refresh, start, stop, allMessages], ipv4_address, port);
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
                        data_model.advanced_networking([refresh, start, stop, allMessages], ipv4_address, port);
                    }
                }

            }

        }

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

            Rectangle {
                anchors.fill: parent

                Text {
                    text: "IP Address:"
                    font.family: Constants.genericTable.fontFamily
                    font.pointSize: Constants.largePointSize
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
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

            Rectangle {
                anchors.right: parent.right
                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter
                height: 20
                border.width: Constants.advancedIns.textDataBarBorderWidth
                clip: true

                TextInput {
                    id: ipAddressInput

                    text: ""
                    cursorVisible: true
                    selectByMouse: true
                    font.pointSize: Constants.largePointSize
                    font.family: Constants.genericTable.fontFamily
                    anchors.fill: parent
                    anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    onTextEdited: {
                        ipAddressEditing = true;
                    }
                    onEditingFinished: {
                        let refresh = false;
                        let start = false;
                        let stop = false;
                        let allMessages = false;
                        let ipv4_address = text;
                        let port = null;
                        data_model.advanced_networking([refresh, start, stop, allMessages], ipv4_address, port);
                        ipAddressEditing = false;
                    }
                }

            }

        }

        Item {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

            Rectangle {
                anchors.fill: parent

                Text {
                    text: "Port:"
                    font.family: Constants.genericTable.fontFamily
                    font.pointSize: Constants.largePointSize
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
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

            Rectangle {
                anchors.right: parent.right
                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter
                height: 20
                border.width: Constants.advancedIns.textDataBarBorderWidth
                clip: true

                TextInput {
                    id: portInput

                    text: ""
                    cursorVisible: true
                    selectByMouse: true
                    font.pointSize: Constants.largePointSize
                    font.family: Constants.genericTable.fontFamily
                    anchors.fill: parent
                    anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    onTextEdited: {
                        portEditing = true;
                    }
                    onEditingFinished: {
                        let refresh = false;
                        let start = false;
                        let stop = false;
                        let allMessages = false;
                        let ipv4_address = null;
                        let port = text;
                        data_model.advanced_networking([refresh, start, stop, allMessages], ipv4_address, port);
                        portEditing = false;
                    }

                    validator: IntValidator {
                        bottom: 0
                        top: 65535
                    }

                }

            }

        }

        RowLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

            Item {
                Layout.fillHeight: true
                Layout.fillWidth: true
            }

            Item {
                Layout.fillHeight: true
                Layout.preferredWidth: parent.width / 2

                Button {
                    id: startButton

                    width: parent.width
                    height: 20
                    topInset: Constants.updateTab.buttonInset
                    bottomInset: Constants.updateTab.buttonInset
                    onClicked: {
                        let refresh = false;
                        let start = true;
                        let stop = false;
                        let allMessages = false;
                        let ipv4_address = null;
                        let port = null;
                        data_model.advanced_networking([refresh, start, stop, allMessages], ipv4_address, port);
                    }

                    Text {
                        text: "Start"
                        anchors.centerIn: parent
                        font.pointSize: Constants.largePointSize
                        font.family: Constants.genericTable.fontFamily
                    }

                }

            }

        }

        RowLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.rowSpan: 1
            Layout.preferredHeight: 1
            Layout.columnSpan: 1
            Layout.preferredWidth: 1

            Item {
                Layout.fillHeight: true
                Layout.preferredWidth: parent.width / 2

                Button {
                    id: stopButton

                    enabled: false
                    width: parent.width
                    height: 20
                    topInset: Constants.updateTab.buttonInset
                    bottomInset: Constants.updateTab.buttonInset
                    onClicked: {
                        let refresh = false;
                        let start = false;
                        let stop = true;
                        let allMessages = false;
                        let ipv4_address = null;
                        let port = null;
                        data_model.advanced_networking([refresh, start, stop, allMessages], ipv4_address, port);
                    }

                    Text {
                        text: "Stop"
                        anchors.centerIn: parent
                        font.pointSize: Constants.largePointSize
                        font.family: Constants.genericTable.fontFamily
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
