import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Dialogs
import QtQuick.Layouts
import Qt.labs.platform as LP

Item {
    property alias localFileText: localFileTextInput.text
    property bool localFileTextEditing: false

    RowLayout {
        anchors.fill: parent
        spacing: Constants.updateTab.firmwareVersionColumnSpacing

        SwiftTextbox {
            Layout.preferredWidth: Constants.updateTab.hardwareVersionElementsLabelWidth
            Layout.fillHeight: true
            rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
            text: Constants.updateTab.firmwareVersionLocalFileLabel
        }

        SwiftTextInput {
            id: localFileTextInput

            Layout.fillWidth: true
            Layout.fillHeight: true
            onTextEdited: {
                localFileTextEditing = true;
            }
            onEditingFinished: {
                let downloadLatestFirmware = false;
                let updateFirmware = false;
                let sendFileToDevice = false;
                let serialPromptConfirm = false;
                let updateLocalFilepath = null;
                let downloadDirectory = null;
                let fileioLocalFilepath = null;
                let fileioDestinationFilepath = null;
                let updateLocalFilename = text;
                backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                localFileTextEditing = false;
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
            title: "Please choose a binary."
            currentFolder: LP.StandardPaths.standardLocations(LP.StandardPaths.HomeLocation)[0]
            fileMode: FileDialog.OpenFile
            nameFilters: ["Binary Image Set (*.bin)"]
            onAccepted: {
                var filepath = Utils.fileUrlToString(fileDialog.fileUrl);
                let downloadLatestFirmware = false;
                let updateFirmware = false;
                let sendFileToDevice = false;
                let serialPromptConfirm = false;
                let updateLocalFilepath = filepath;
                let downloadDirectory = null;
                let fileioLocalFilepath = null;
                let fileioDestinationFilepath = null;
                let updateLocalFilename = null;
                backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
            }
            onRejected: {
            }
        }

    }

}
