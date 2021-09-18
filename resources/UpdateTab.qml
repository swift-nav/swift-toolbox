import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0
import "UpdateTabComponents" as UpdateTabComponents

Item {
    id: updateTab

    property bool consoleVersionPopupAlready: false
    property bool firmwareVersionPopupAlready: false
    property bool v2DownloadDialogAlready: false
    property bool popupLock: false

    function consoleOutdatedPopupText(currentVersion, latestVersion) {
        let text = "";
        text += "Your console is incompatible with your hardware revision.\n";
        text += "We highly recommend using a compatible console version\n";
        text += "to ensure proper behavior.\n\n";
        text += "Please visit support.swiftnav.com to ";
        text += "download the latest compatible version.\n\n";
        text += "Current Console version :\n\t";
        text += currentVersion;
        text += "\nLast supported Console Version: \n\t";
        text += latestVersion;
        text += +"\n\n";
        return text;
    }

    function upgradeSerialConfirmDialogText() {
        let text = "";
        text += "Upgrading your device via UART / RS232 may take up to 30 minutes.\n\n";
        text += "If the device you are upgrading has an accessible USB host port, it\n";
        text += "is recommended to instead  follow the \'USB Flashdrive Upgrade\n";
        text += "Procedure\' that now appears in the Firmware upgrade status box.\n\n";
        text += "Are you sure you want to continue upgrading over serial?";
        return text;
    }

    function firmwareV2OutdatedDialogText() {
        let text = "";
        text += "Upgrading to firmware v2.1.0 or later requires that the device be\n";
        text += "running firmware v2.0.0 or later. Please upgrade to firmware\n";
        text += "version 2.0.0.\n\n";
        text += "Would you like to download firmware version v2.0.0 now?\n\n";
        return text;
    }

    function firmwareOutdatedPopupText(latestVersion) {
        let text = "";
        text += "New Piksi firmware available.\n\n";
        text += "Please use the Update tab to update.\n\n";
        text += "Newest Firmware Version :\n\t";
        text += latestVersion + "\n\n";
        return text;
    }

    width: parent.width
    height: parent.height

    UpdateTabData {
        id: updateTabData
    }

    ColumnLayout {
        anchors.fill: parent
        width: parent.width
        height: parent.height
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

            Text {
                text: Constants.updateTab.firmwareUpgradeStatusTitle
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
            }

        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.leftMargin: Constants.updateTab.innerMargins
            Layout.rightMargin: Constants.updateTab.innerMargins
            Layout.bottomMargin: Constants.updateTab.innerMargins

            TextArea {
                id: fwLogTextArea

                readOnly: true
                selectByMouse: true
                selectByKeyboard: true
                cursorVisible: true
                activeFocusOnPress: false
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.updateTab.textHeight

            Text {
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
        height: Constants.updateTab.popupHeight
        modal: true
        focus: true
        title: "Update to v2.0.0"
        standardButtons: Dialog.Ok | Dialog.Cancel
        onAccepted: {
            data_model.update_tab([true, false, false, false], null, null, null, null, null);
        }

        Label {
            text: firmwareV2OutdatedDialogText()
        }

    }

    Dialog {
        id: upgradeSerialDialog

        x: (parent.width - Constants.sideNavBar.tabBarWidth - Constants.updateTab.upgradeSerialDialogWidth) / 2
        y: parent.height / 2
        width: Constants.updateTab.upgradeSerialDialogWidth
        height: Constants.updateTab.popupHeight
        modal: true
        focus: true
        title: "Update device over serial connection?"
        standardButtons: Dialog.Ok | Dialog.Cancel
        onAccepted: {
            data_model.update_tab([false, true, false, true], null, null, null, null, null);
        }

        Label {
            text: upgradeSerialConfirmDialogText()
        }

    }

    Popup {
        id: consoleVersionPopup

        x: (parent.width - Constants.sideNavBar.tabBarWidth - Constants.updateTab.consoleVersionPopupWidth) / 2
        y: parent.height / 2
        width: Constants.updateTab.consoleVersionPopupWidth
        height: Constants.updateTab.popupHeight
        modal: true
        focus: true
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        onClosed: {
            popupLock = false;
        }

        contentItem: Text {
            verticalAlignment: Qt.AlignVCenter
            elide: Text.ElideRight
            clip: true
            font.family: Constants.genericTable.fontFamily
            font.pointSize: Constants.largePointSize
            wrapMode: Text.Wrap
        }

    }

    Popup {
        id: fwVersionPopup

        x: (parent.width - Constants.sideNavBar.tabBarWidth - Constants.updateTab.fwVersionPopupWidth) / 2
        y: parent.height / 2
        width: Constants.updateTab.fwVersionPopupWidth
        height: Constants.updateTab.popupHeight
        modal: true
        focus: true
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        onClosed: {
            popupLock = false;
        }

        contentItem: Text {
            verticalAlignment: Qt.AlignVCenter
            elide: Text.ElideRight
            clip: true
            font.family: Constants.genericTable.fontFamily
            font.pointSize: Constants.largePointSize
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
                if (!consoleVersionPopupAlready) {
                    if (updateTabData.console_outdated && !popupLock) {
                        popupLock = true;
                        consoleVersionPopup.contentItem.text = consoleOutdatedPopupText(updateTabData.console_version_current, updateTabData.console_version_latest);
                        consoleVersionPopupAlready = true;
                        timer.startTimer(consoleVersionPopup.open);
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
                if (!firmwareVersionPopupAlready && !updateTabData.fw_v2_outdated && !popupLock) {
                    if (updateTabData.fw_outdated) {
                        popupLock = true;
                        fwVersionPopup.contentItem.text = firmwareOutdatedPopupText(updateTabData.fw_version_latest);
                        firmwareVersionPopupAlready = true;
                        timer.startTimer(fwVersionPopup.open);
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
