/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
import "../BaseComponents"
import "../Constants"
import Qt.labs.platform as LP
import QtQuick
import QtQuick.Controls
import QtQuick.Dialogs
import QtQuick.Layouts
import SwiftConsole

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
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
            }
        }

        FileDialog {
            id: fileDialog

            visible: false
            title: "Please choose a file."
            currentFolder: LP.StandardPaths.standardLocations(LP.StandardPaths.HomeLocation)[0]
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

        Button {
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
