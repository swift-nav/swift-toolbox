import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property alias currentVersion: currentVersionText.text
    property alias latestVersion: latestVersionText.text
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
            width: parent.width
            height: parent.height
            spacing: Constants.updateTab.firmwareVersionColumnSpacing

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins

                Rectangle {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height

                    Label {
                        text: Constants.updateTab.firmwareVersionCurrentLabel
                        anchors.fill: parent
                        anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                        horizontalAlignment: Text.AlignRight
                    }

                }

                Rectangle {
                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    border.width: Constants.advancedImu.textDataBarBorderWidth

                    Label {
                        id: currentVersionText

                        text: ""
                        clip: true
                        color: Constants.updateTab.placeholderTextColor
                        anchors.centerIn: parent
                    }

                }

            }

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins

                Rectangle {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height

                    Label {
                        text: Constants.updateTab.firmwareVersionLatestLabel
                        anchors.fill: parent
                        anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                        horizontalAlignment: Text.AlignRight
                    }

                }

                Rectangle {
                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    border.width: Constants.advancedImu.textDataBarBorderWidth

                    Label {
                        id: latestVersionText

                        text: ""
                        clip: true
                        color: Constants.updateTab.placeholderTextColor
                        anchors.centerIn: parent
                    }

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

                Button {
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
                        data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                        if (isSerialConnected)
                            dialog.open();

                    }

                    Label {
                        text: Constants.updateTab.updateFirmwareButtonLabel
                        anchors.centerIn: parent
                    }

                }

            }

        }

    }

}
