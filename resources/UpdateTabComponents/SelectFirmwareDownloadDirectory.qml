import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias fwDirectory: directoryInput.text
    property bool fwDirectoryEditing: false

    RowLayout {
        anchors.fill: parent
        spacing: Constants.updateTab.firmwareVersionColumnSpacing

        Rectangle {
            Layout.preferredWidth: Constants.updateTab.hardwareVersionElementsLabelWidth
            Layout.fillHeight: true

            Label {
                text: Constants.updateTab.firmwareDownloadDirectoryLabel
                anchors.fill: parent
                anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                horizontalAlignment: Text.AlignRight
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.width: Constants.advancedImu.textDataBarBorderWidth
            clip: true

            TextInput {
                id: directoryInput

                text: ""
                cursorVisible: true
                selectByMouse: true
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
                anchors.fill: parent
                anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                onTextEdited: {
                    fwDirectoryEditing = true;
                }
                onEditingFinished: {
                    let downloadLatestFirmware = false;
                    let updateFirmware = false;
                    let sendFileToDevice = false;
                    let serialPromptConfirm = false;
                    let updateLocalFilepath = null;
                    let downloadDirectory = text;
                    let fileioLocalFilepath = null;
                    let fileioDestinationFilepath = null;
                    let updateLocalFilename = null;
                    data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                    fwDirectoryEditing = false;
                }
            }

        }

        Item {
            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonSpacing
            Layout.fillHeight: true
        }

        Button {
            id: directorySelectionButton

            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonWidth
            Layout.fillHeight: true
            topInset: Constants.updateTab.buttonInset
            bottomInset: Constants.updateTab.buttonInset
            onClicked: {
                fileDialog.visible = !fileDialog.visible;
            }

            Label {
                text: Constants.updateTab.dotDotDotLabel
                anchors.centerIn: parent
            }

        }

        FileDialog {
            id: fileDialog

            visible: false
            title: "Please choose a folder."
            folder: shortcuts.home
            selectFolder: true
            selectMultiple: false
            onAccepted: {
                var filepath = Utils.fileUrlToString(fileDialog.folder);
                let downloadLatestFirmware = false;
                let updateFirmware = false;
                let sendFileToDevice = false;
                let serialPromptConfirm = false;
                let updateLocalFilepath = null;
                let downloadDirectory = filepath;
                let fileioLocalFilepath = null;
                let fileioDestinationFilepath = null;
                let updateLocalFilename = null;
                data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
            }
            onRejected: {
            }
        }

    }

}
