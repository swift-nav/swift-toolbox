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
import "Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole
import "UpdateTabComponents" as UpdateTabComponents

MainTab {
    id: updateTab

    function upgradeSerialConfirmDialogText() {
        let text = "";
        text += "Upgrading your device via UART / RS232 may take up to 30 minutes.\n\n";
        text += "If the device you are upgrading has an accessible USB host port, it is recommended to instead follow the ";
        text += "'USB Flashdrive Upgrade Procedure\' that now appears in the Firmware upgrade status box.\n\n";
        text += "Are you sure you want to continue upgrading over serial?\n";
        return text;
    }

    UpdateTabData {
        id: updateTabData

        function update() {
            update_tab_model.fill_data(updateTabData);
            Globals.updateTabData.consoleOutdated = updateTabData.console_outdated;
            Globals.updateTabData.fwV2Outdated = updateTabData.fw_v2_outdated;
            Globals.updateTabData.fwOutdated = updateTabData.fw_outdated;
            Globals.updateTabData.fwVersionCurrent = updateTabData.fw_version_current;
            Globals.updateTabData.fwVersionLatest = updateTabData.fw_version_latest;
            Globals.updateTabData.consoleVersionCurrent = updateTabData.console_version_current;
            Globals.updateTabData.consoleVersionLatest = updateTabData.console_version_latest;
            if (updateTabData.fw_version_latest) {
                firmwareRevision.revision = updateTabData.hardware_revision;
                firmwareVersion.currentVersion = updateTabData.fw_version_current;
                firmwareVersion.latestVersion = updateTabData.fw_version_latest;
            }
            if (updateTabData.fw_version_current)
                firmwareVersion.isSerialConnected = updateTabData.serial_prompt;
            else
                firmwareVersion.isSerialConnected = false;
            if (!updateTab.visible)
                return;
            if (!firmwareDownload.fwDirectoryEditing)
                firmwareDownload.fwDirectory = updateTabData.directory;
            if (fwLogTextArea.text != updateTabData.fw_text)
                fwLogTextArea.text = updateTabData.fw_text;
            firmwareDownload.downloadButtonEnable = !updateTabData.downloading && !updateTabData.upgrading;
            firmwareVersion.upgradeButtonEnable = updateTabData.fw_version_current && !updateTabData.upgrading && !updateTabData.downloading;
            if (!firmwareVersion.localFileTextEditing)
                firmwareVersion.localFileText = updateTabData.fw_local_filename;
            if (!fileioSelect.destinationTextEditing)
                fileioSelect.destinationText = updateTabData.fileio_destination_filepath;
            if (!fileioSelect.localTextEditing)
                fileioSelect.localText = updateTabData.fileio_local_filepath;
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Constants.updateTab.outerMargins

        Rectangle {
            Layout.alignment: Qt.AlignTop
            Layout.preferredHeight: Constants.updateTab.textHeight
            Layout.fillWidth: true

            UpdateTabComponents.FirmwareRevision {
                id: firmwareRevision

                anchors.fill: parent
            }
        }

        Rectangle {
            Layout.alignment: Qt.AlignTop
            Layout.preferredHeight: Constants.updateTab.textHeight
            Layout.fillWidth: true

            UpdateTabComponents.FirmwareVersionAndDownloadLabels {
                anchors.fill: parent
            }
        }

        Rectangle {
            Layout.alignment: Qt.AlignTop
            Layout.preferredHeight: parent.height / 3
            Layout.fillWidth: true

            RowLayout {
                anchors.fill: parent
                width: parent.width
                height: parent.height

                UpdateTabComponents.FirmwareVersion {
                    id: firmwareVersion

                    property alias dialog: upgradeSerialDialog

                    Layout.preferredWidth: parent.width / 2
                    Layout.fillHeight: true
                }

                UpdateTabComponents.FirmwareDownload {
                    id: firmwareDownload

                    Layout.preferredWidth: parent.width / 2
                    Layout.fillHeight: true
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.updateTab.textHeight

            Label {
                text: Constants.updateTab.firmwareUpgradeStatusTitle
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.width: Constants.updateTab.borderWidth
            border.color: Constants.genericTable.borderColor
            clip: true

            ScrollView {
                id: control

                anchors.fill: parent
                anchors.margins: Constants.updateTab.innerMargins

                TextEdit {
                    id: fwLogTextArea

                    onTextChanged: {
                        scrollBarVertical.position = 1 - scrollBarVertical.size;
                    }
                    readOnly: true
                    selectByMouse: true
                    selectByKeyboard: true
                    cursorVisible: true
                    activeFocusOnPress: false
                    font.family: Constants.genericTable.fontFamily
                    font.pixelSize: Constants.largePixelSize
                }

                ScrollBar.vertical: ScrollBar {
                    id: scrollBarVertical

                    parent: control
                    x: control.mirrored ? 0 : control.width - width
                    y: control.topPadding
                    height: control.availableHeight
                    active: control.ScrollBar.horizontal.active
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.updateTab.textHeight
            visible: Globals.showFileio

            Label {
                text: Constants.updateTab.fileioAndProductFeatureToolTitle
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
            }
        }

        Rectangle {
            Layout.alignment: Qt.AlignBottom
            Layout.preferredHeight: Constants.updateTab.textHeight
            Layout.fillWidth: true
            Layout.leftMargin: Constants.updateTab.innerMargins
            Layout.rightMargin: Constants.updateTab.innerMargins
            visible: Globals.showFileio

            UpdateTabComponents.FileIOSelectLocalFileAndDestPath {
                id: fileioSelect

                anchors.fill: parent
            }
        }
    }

    Dialog {
        id: upgradeSerialDialog

        x: (parent.width - Constants.sideNavBar.tabBarWidth - Constants.updateTab.upgradeSerialDialogWidth) / 2
        y: parent.height / 2
        width: Constants.updateTab.upgradeSerialDialogWidth
        height: Constants.updateTab.popupLargeHeight
        modal: true
        focus: true
        title: "Update device over serial connection?"
        standardButtons: Dialog.Ok | Dialog.Cancel
        onAccepted: {
            let downloadLatestFirmware = false;
            let updateFirmware = true;
            let sendFileToDevice = false;
            let serialPromptConfirm = true;
            let updateLocalFilepath = null;
            let downloadDirectory = null;
            let fileioLocalFilepath = null;
            let fileioDestinationFilepath = null;
            let updateLocalFilename = null;
            backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
        }

        contentItem: Label {
            text: upgradeSerialConfirmDialogText()
            verticalAlignment: Qt.AlignVCenter
            elide: Text.ElideRight
            clip: true
            wrapMode: Text.Wrap
        }
    }

    Timer {
        id: timer

        property var currentCallback: null

        function startTimer(callback) {
            currentCallback = callback;
            timer.start();
        }

        interval: Constants.updateTab.popupDelayMilliseconds
        repeat: false
        onTriggered: {
            currentCallback();
        }
    }
}
