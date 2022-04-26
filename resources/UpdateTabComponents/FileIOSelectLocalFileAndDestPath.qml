import "../BaseComponents"
import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias destinationText: destinationPathTextInput.text
    property bool destinationTextEditing: false
    property alias localText: localFileTextInput.text
    property bool localTextEditing: false

    RowLayout {
        anchors.fill: parent

        SwiftTextbox {
            Layout.preferredWidth: Constants.updateTab.hardwareVersionElementsLabelWidth
            Layout.fillHeight: true
            text: Constants.updateTab.fileioLocalFileLabel
        }

        SwiftTextInput {
            id: localFileTextInput

            Layout.fillWidth: true
            Layout.fillHeight: true
            onTextEdited: {
                localTextEditing = true;
            }
            onEditingFinished: {
                let downloadLatestFirmware = false;
                let updateFirmware = false;
                let sendFileToDevice = false;
                let serialPromptConfirm = false;
                let updateLocalFilepath = null;
                let downloadDirectory = null;
                let fileioLocalFilepath = text;
                let fileioDestinationFilepath = null;
                let updateLocalFilename = null;
                backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                localTextEditing = false;
            }
        }

        Item {
            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonSpacing
            Layout.fillHeight: true
        }

        SwiftButton {
            id: localFileSelectionButton

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
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
            }

        }

        FileDialog {
            id: fileDialog

            visible: false
            title: "Please choose a file."
            currentFolder: StandardPaths.standardLocations(StandardPaths.HomeLocation)[0]
            fileMode: FileDialog.OpenFile
            nameFilters: ["All Files (*)"]
            onAccepted: {
                var filepath = Utils.fileUrlToString(fileDialog.fileUrl);
                let downloadLatestFirmware = false;
                let updateFirmware = false;
                let sendFileToDevice = false;
                let serialPromptConfirm = false;
                let updateLocalFilepath = null;
                let downloadDirectory = null;
                let fileioLocalFilepath = filepath;
                let fileioDestinationFilepath = null;
                let updateLocalFilename = null;
                backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
            }
            onRejected: {
            }
        }

        SwiftTextbox {
            Layout.preferredWidth: Constants.updateTab.hardwareVersionElementsLabelWidth * 2
            Layout.fillHeight: true
            text: Constants.updateTab.fileioDestinationPathLabel
        }

        SwiftTextInput {
            id: destinationPathTextInput

            Layout.fillWidth: true
            Layout.fillHeight: true
            onTextEdited: {
                destinationTextEditing = true;
            }
            onEditingFinished: {
                let downloadLatestFirmware = false;
                let updateFirmware = false;
                let sendFileToDevice = false;
                let serialPromptConfirm = false;
                let updateLocalFilepath = null;
                let downloadDirectory = null;
                let fileioLocalFilepath = null;
                let fileioDestinationFilepath = text;
                let updateLocalFilename = null;
                backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                destinationTextEditing = false;
            }
        }

        Item {
            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonSpacing
            Layout.fillHeight: true
        }

        SwiftButton {
            id: sendFileToDeviceButton

            Layout.preferredWidth: Constants.updateTab.fileioDestinationPathButtonWidth
            Layout.fillHeight: true
            topInset: Constants.updateTab.buttonInset
            bottomInset: Constants.updateTab.buttonInset
            onClicked: {
                let downloadLatestFirmware = false;
                let updateFirmware = false;
                let sendFileToDevice = true;
                let serialPromptConfirm = false;
                let updateLocalFilepath = null;
                let downloadDirectory = null;
                let fileioLocalFilepath = null;
                let fileioDestinationFilepath = null;
                let updateLocalFilename = null;
                backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
            }

            Label {
                text: Constants.updateTab.fileioSendFileToDeviceButtonLabel
                anchors.centerIn: parent
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
            }

        }

    }

}
