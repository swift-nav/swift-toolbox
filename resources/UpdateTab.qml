import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0
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
                font.pointSize: Constants.largePointSize
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.width: Constants.updateTab.borderWidth
            border.color: Constants.genericTable.borderColor

            ScrollView {
                anchors.fill: parent
                anchors.margins: Constants.updateTab.innerMargins

                TextEdit {
                    id: fwLogTextArea

                    readOnly: true
                    selectByMouse: true
                    selectByKeyboard: true
                    cursorVisible: true
                    activeFocusOnPress: false
                    font.family: Constants.genericTable.fontFamily
                    font.pointSize: Constants.largePointSize
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
                font.pointSize: Constants.largePointSize
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
            data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
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

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
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
                return ;

            if (!firmwareDownload.fwDirectoryEditing)
                firmwareDownload.fwDirectory = updateTabData.directory;

            fwLogTextArea.text = updateTabData.fw_text;
            firmwareDownload.downloadButtonEnable = !updateTabData.downloading && !updateTabData.upgrading;
            firmwareVersion.upgradeButtonEnable = !updateTabData.upgrading && !updateTabData.downloading;
            if (!firmwareVersion.localFileTextEditing)
                firmwareVersion.localFileText = updateTabData.fw_local_filename;

            if (!fileioSelect.destinationTextEditing)
                fileioSelect.destinationText = updateTabData.fileio_destination_filepath;

            if (!fileioSelect.localTextEditing)
                fileioSelect.localText = updateTabData.fileio_local_filepath;

        }
    }

}
