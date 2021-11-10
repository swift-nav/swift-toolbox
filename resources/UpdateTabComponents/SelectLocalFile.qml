import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform 1.1 as LP

Item {
    property alias localFileText: localFileTextInput.text
    property bool localFileTextEditing: false

    RowLayout {
        anchors.fill: parent
        spacing: Constants.updateTab.firmwareVersionColumnSpacing

        Rectangle {
            Layout.preferredWidth: Constants.updateTab.hardwareVersionElementsLabelWidth
            Layout.fillHeight: true

            Label {
                text: Constants.updateTab.firmwareVersionLocalFileLabel
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
                id: localFileTextInput

                text: ""
                cursorVisible: true
                selectByMouse: true
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
                anchors.fill: parent
                anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
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
                    data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                    localFileTextEditing = false;
                }

                Label {
                    text: Constants.updateTab.firmwareVersionLocalFilePlaceholderText
                    color: Constants.loggingBar.placeholderTextColor
                    visible: !localFileTextInput.text
                    anchors.fill: parent
                    anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                    anchors.centerIn: parent
                }

            }

        }

        Item {
            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonSpacing
            Layout.fillHeight: true
        }

        Button {
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
            }

        }

        LP.FileDialog {
            id: fileDialog

            visible: false
            title: "Please choose a binary."
            folder: LP.StandardPaths.standardLocations(LP.StandardPaths.HomeLocation)[0]
//            selectFolder: false
//            selectMultiple: false
//            selectExisting: true
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
                data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
            }
            onRejected: {
            }
        }

    }

}
