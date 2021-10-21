import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15

Item {
    property alias localFileText: localFileTextInput.text
    property bool localFileTextEditing: false

    RowLayout {
        anchors.fill: parent
        spacing: Constants.updateTab.firmwareVersionColumnSpacing

        Rectangle {
            Layout.preferredWidth: Constants.updateTab.hardwareVersionElementsLabelWidth
            Layout.fillHeight: true

            Text {
                text: Constants.updateTab.firmwareVersionLocalFileLabel
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
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

                Text {
                    text: Constants.updateTab.firmwareVersionLocalFilePlaceholderText
                    color: Constants.loggingBar.placeholderTextColor
                    font.pointSize: Constants.largePointSize
                    font.family: Constants.genericTable.fontFamily
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

            Text {
                text: Constants.updateTab.dotDotDotLabel
                anchors.centerIn: parent
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
            }

        }

        FileDialog {
            id: fileDialog

            visible: false
            title: "Please choose a binary."
            folder: shortcuts.home
            selectFolder: false
            selectMultiple: false
            selectExisting: true
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
