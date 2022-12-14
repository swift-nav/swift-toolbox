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
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

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
