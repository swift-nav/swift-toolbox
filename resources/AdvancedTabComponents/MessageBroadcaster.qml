import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {

    property alias ip_address: ipAddressInput.text
    property alias port: portInput.text

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
                    text: qsTr("Observations")
                }

                RadioButton {
                    text: qsTr("All")
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
                        // fwDirectoryEditing = true;
                        // let downloadLatestFirmware = false;
                        // let updateFirmware = false;
                        // let sendFileToDevice = false;
                        // let serialPromptConfirm = false;
                        // let updateLocalFilepath = null;
                        // let downloadDirectory = text;
                        // let fileioLocalFilepath = null;
                        // let fileioDestinationFilepath = null;
                        // let updateLocalFilename = null;
                        // data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                        // fwDirectoryEditing = false;
                    }
                    onEditingFinished: {
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
                    }
                    onEditingFinished: {
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

                    width: parent.width
                    height: 20
                    topInset: Constants.updateTab.buttonInset
                    bottomInset: Constants.updateTab.buttonInset
                    onClicked: {
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
