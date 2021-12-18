import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0
import "UpdateTabComponents" as UpdateTabComponents

MainTab {
    id: updateTab

    property bool consoleVersionDialogAlready: false
    property bool firmwareVersionDialogAlready: false
    property bool v2DownloadDialogAlready: false
    property bool popupLock: false

    function consoleOutdatedDialogText(currentVersion, latestVersion) {
        let text = "";
        text += "Your console is out of date and may be incompatible with current firmware. We highly recommend upgrading to ensure proper behavior.\n\n";
        text += "Please visit support.swiftnav.com to download the latest version.\n\n";
        text += "Current Console version:\n";
        text += "\t" + currentVersion + "\n";
        text += "Latest Console version:\n";
        text += "\t" + latestVersion;
        return text;
    }

    function upgradeSerialConfirmDialogText() {
        let text = "";
        text += "Upgrading your device via UART / RS232 may take up to 30 minutes.\n\n";
        text += "If the device you are upgrading has an accessible USB host port, it is recommended to instead follow the \
        \'USB Flashdrive Upgrade Procedure\' that now appears in the Firmware upgrade status box.\n\n";
        text += "Are you sure you want to continue upgrading over serial?\n";
        return text;
    }

    function firmwareV2OutdatedDialogText() {
        let text = "";
        text += "Upgrading to firmware v2.1.0 or later requires that the device be running firmware v2.0.0 or later. \
        Please upgrade to firmware version 2.0.0.\n\n";
        text += "Would you like to download firmware version v2.0.0 now?\n";
        return text;
    }

    function firmwareOutdatedDialogText(latestVersion) {
        let text = "";
        text += "New Piksi firmware available.\n\n";
        text += "Please use the Update \
        tab to update.\n\n";
        text += "Newest Firmware Version:\n";
        text += "\t" + latestVersion + "\n";
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
            }

        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.leftMargin: Constants.updateTab.innerMargins
            Layout.rightMargin: Constants.updateTab.innerMargins
            Layout.bottomMargin: Constants.updateTab.innerMargins

            TextEdit {
                id: fwLogTextArea

                readOnly: true
                selectByMouse: true
                selectByKeyboard: true
                cursorVisible: true
                activeFocusOnPress: false
                font.family: Constants.fontFamily
                font.pointSize: Constants.largePointSize
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.updateTab.textHeight
            visible: Globals.showFileio

            Label {
                text: Constants.updateTab.fileioAndProductFeatureToolTitle
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
        id: v2DownloadDialog

        x: (parent.width - Constants.sideNavBar.tabBarWidth - Constants.updateTab.v2DownloadDialogWidth) / 2
        y: parent.height / 2
        width: Constants.updateTab.v2DownloadDialogWidth
        height: Constants.updateTab.popupSmallHeight
        modal: true
        focus: true
        title: "Update to v2.0.0"
        standardButtons: Dialog.Ok | Dialog.Cancel
        onAccepted: {
            let downloadLatestFirmware = true;
            let updateFirmware = false;
            let sendFileToDevice = false;
            let serialPromptConfirm = false;
            let updateLocalFilepath = null;
            let downloadDirectory = null;
            let fileioLocalFilepath = null;
            let fileioDestinationFilepath = null;
            let updateLocalFilename = null;
            data_model.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
        }

        contentItem: Label {
            text: firmwareV2OutdatedDialogText()
            verticalAlignment: Qt.AlignVCenter
            elide: Text.ElideRight
            clip: true
            wrapMode: Text.Wrap
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

    Dialog {
        id: consoleVersionDialog

        x: (parent.width - Constants.sideNavBar.tabBarWidth - Constants.updateTab.consoleVersionDialogWidth) / 2
        y: parent.height / 2
        width: Constants.updateTab.consoleVersionDialogWidth
        height: Constants.updateTab.popupLargeHeight
        modal: true
        focus: true
        title: "Swift Console Outdated"
        standardButtons: Dialog.Close
        onRejected: {
            popupLock = false;
        }

        contentItem: Label {
            verticalAlignment: Qt.AlignVCenter
            elide: Text.ElideRight
            clip: true
            wrapMode: Text.Wrap
        }

    }

    Dialog {
        id: fwVersionDialog

        x: (parent.width - Constants.sideNavBar.tabBarWidth - Constants.updateTab.fwVersionDialogWidth) / 2
        y: parent.height / 2
        width: Constants.updateTab.fwVersionDialogWidth
        height: Constants.updateTab.popupSmallHeight
        modal: true
        focus: true
        title: "Firmware Update"
        standardButtons: Dialog.Close
        onRejected: {
            popupLock = false;
        }

        contentItem: Label {
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
            if (updateTabData.console_version_latest) {
                if (!consoleVersionDialogAlready) {
                    if (updateTabData.console_outdated && !popupLock) {
                        popupLock = true;
                        consoleVersionDialog.contentItem.text = consoleOutdatedDialogText(updateTabData.console_version_current, updateTabData.console_version_latest);
                        consoleVersionDialogAlready = true;
                        timer.startTimer(consoleVersionDialog.open);
                    }
                }
            }
            if (!v2DownloadDialogAlready) {
                if (updateTabData.fw_v2_outdated && !popupLock) {
                    popupLock = true;
                    v2DownloadDialogAlready = true;
                    timer.startTimer(v2DownloadDialog.open);
                }
            }
            if (updateTabData.fw_version_latest) {
                firmwareRevision.revision = updateTabData.hardware_revision;
                firmwareVersion.currentVersion = updateTabData.fw_version_current;
                firmwareVersion.latestVersion = updateTabData.fw_version_latest;
                if (!firmwareVersionDialogAlready && !updateTabData.fw_v2_outdated && !popupLock) {
                    if (updateTabData.fw_outdated) {
                        popupLock = true;
                        fwVersionDialog.contentItem.text = firmwareOutdatedDialogText(updateTabData.fw_version_latest);
                        firmwareVersionDialogAlready = true;
                        timer.startTimer(fwVersionDialog.open);
                    }
                }
            }
            if (updateTabData.serial_prompt)
                firmwareVersion.isSerialConnected = updateTabData.serial_prompt;

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
