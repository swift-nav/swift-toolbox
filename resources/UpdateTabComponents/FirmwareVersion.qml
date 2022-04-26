import "../BaseComponents"
import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property string currentVersion: ""
    property string latestVersion: ""
    property alias localFileText: selectLocalFile.localFileText
    property alias localFileTextEditing: selectLocalFile.localFileTextEditing
    property alias upgradeButtonEnable: updateFirmwareButton.enabled
    property bool isSerialConnected: false

    Rectangle {
        width: parent.width
        height: parent.height
        border.width: Constants.updateTab.borderWidth
        border.color: Constants.genericTable.borderColor

        ColumnLayout {
            anchors.fill: parent
            anchors.topMargin: Constants.updateTab.innerMargins
            anchors.leftMargin: Constants.updateTab.innerMargins
            anchors.rightMargin: Constants.updateTab.innerMargins
            width: parent.width
            height: parent.height
            spacing: Constants.updateTab.firmwareVersionColumnSpacing

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins

                SwiftTextbox {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    text: Constants.updateTab.firmwareVersionCurrentLabel
                }

                SwiftTextInput {
                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    placeholderText: currentVersion ? currentVersion : "Waiting for Piksi to send settings..."
                    labelHorizontalAlignment: Text.AlignLeft
                    readOnly: true
                }

            }

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins

                SwiftTextbox {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    text: Constants.updateTab.firmwareVersionLatestLabel
                }

                SwiftTextInput {
                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    placeholderText: latestVersion ? latestVersion : "Loading latest firmware info..."
                    labelHorizontalAlignment: Text.AlignLeft
                    readOnly: true
                }

            }

            SelectLocalFile {
                id: selectLocalFile

                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins
            }

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins
                Layout.bottomMargin: Constants.updateTab.innerMargins
                Layout.alignment: Qt.AlignBottom

                Rectangle {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                }

                SwiftButton {
                    id: updateFirmwareButton

                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    topInset: Constants.updateTab.buttonInset
                    bottomInset: Constants.updateTab.buttonInset
                    onClicked: {
                        let downloadLatestFirmware = false;
                        let updateFirmware = true;
                        let sendFileToDevice = false;
                        let serialPromptConfirm = false;
                        let updateLocalFilepath = null;
                        let downloadDirectory = null;
                        let fileioLocalFilepath = null;
                        let fileioDestinationFilepath = null;
                        let updateLocalFilename = null;
                        backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                        if (isSerialConnected) {
                            if (Globals.showPrompts) {
                                dialog.open();
                            } else {
                                serialPromptConfirm = true;
                                backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                            }
                        }
                    }

                    Label {
                        text: Constants.updateTab.updateFirmwareButtonLabel
                        anchors.centerIn: parent
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                    }

                }

            }

        }

    }

}
