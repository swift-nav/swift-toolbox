import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: updateTab

    property bool consoleVersionDialogAlready: false
    property bool firmwareVersionDialogAlready: false
    property bool v2DownloadDialogAlready: false
    property bool popupLock: false
    property int dialogWidthDivisor: 3
    property int dialogHeightDivisor: 2

    function consoleOutdatedDialogText(currentVersion, latestVersion) {
        let text = "";
        text += "Current Console version:\n";
        text += "\t" + currentVersion + "\n";
        text += "Latest Console version:\n";
        text += "\t" + latestVersion;
        return text;
    }

    function firmwareV2OutdatedDialogText() {
        let text = "";
        text += "Upgrading to firmware v2.1.0 or later requires that the device be running ";
        text += "firmware v2.0.0 or later. Please upgrade to firmware version 2.0.0.\n\n";
        text += "Would you like to download firmware version v2.0.0 now?\n";
        return text;
    }

    function firmwareOutdatedDialogText(latestVersion) {
        let text = "";
        text += "New Piksi firmware available.\n\n";
        text += "Please use the Update tab to update.\n\n";
        text += "Newest Firmware Version:\n";
        text += "\t" + latestVersion + "\n";
        return text;
    }

    Dialog {
        id: v2DownloadDialog

        anchors.centerIn: parent
        width: Globals.width / dialogWidthDivisor
        height: Globals.height / dialogHeightDivisor
        x: Globals.width / 2 - width / 2
        y: Globals.height / 2 - height / 2
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
        id: consoleVersionDialog

        property alias versionText: versionTextLabel.text

        anchors.centerIn: parent
        width: Globals.width / dialogWidthDivisor
        height: Globals.height / dialogHeightDivisor
        x: Globals.width / 2 - width / 2
        y: Globals.height / 2 - height / 2
        modal: true
        focus: true
        title: "Swift Console Outdated"
        standardButtons: Dialog.Close
        onRejected: {
            popupLock = false;
        }

        contentItem: ColumnLayout {
            anchors.centerIn: parent
            spacing: 0

            Label {
                Layout.fillWidth: true
                wrapMode: Text.Wrap
                text: {
                    let text = ``;
                    text += `Your console is out of date and may be incompatible with current firmware. `;
                    text += `We highly recommend upgrading to ensure proper behavior.`;
                    text;
                }
            }

            Label {
                readonly property string website: Constants.logoPopup.aboutMe.supportWebsite
                readonly property string websiteDisplay: website.slice(12)

                Layout.fillWidth: true
                wrapMode: Text.Wrap
                text: `Please visit <a href='${website}'>${websiteDisplay}</a> to download the latest version.\n\n`
                onLinkActivated: {
                    Qt.openUrlExternally(website);
                }
            }

            Label {
                id: versionTextLabel

                Layout.fillWidth: true
                wrapMode: Text.Wrap
            }

        }

    }

    Dialog {
        id: fwVersionDialog

        anchors.centerIn: parent
        width: Globals.width / dialogWidthDivisor
        height: Globals.height / dialogHeightDivisor
        x: Globals.width / 2 - width / 2
        y: Globals.height / 2 - height / 2
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
        interval: Utils.hzToMilliseconds(Constants.staticTimerNotificationIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            if (!popupLock && Globals.showPrompts) {
                if (Globals.updateTabData.consoleVersionLatest) {
                    if (!consoleVersionDialogAlready) {
                        if (Globals.updateTabData.consoleOutdated) {
                            popupLock = true;
                            consoleVersionDialog.versionText = consoleOutdatedDialogText(Globals.updateTabData.consoleVersionCurrent, Globals.updateTabData.consoleVersionLatest);
                            timer.startTimer(consoleVersionDialog.open);
                        }
                        consoleVersionDialogAlready = true;
                        return ;
                    }
                }
                if (Globals.updateTabData.fwVersionCurrent) {
                    if (!v2DownloadDialogAlready) {
                        if (Globals.updateTabData.fwV2Outdated) {
                            popupLock = true;
                            timer.startTimer(v2DownloadDialog.open);
                        }
                        v2DownloadDialogAlready = true;
                        return ;
                    }
                } else {
                    // This will clear between device connections.
                    v2DownloadDialogAlready = false;
                    firmwareVersionDialogAlready = false;
                    return ;
                }
                if (Globals.updateTabData.fwVersionCurrent && Globals.updateTabData.fwVersionLatest) {
                    if (!firmwareVersionDialogAlready && !Globals.updateTabData.fwV2Outdated) {
                        if (Globals.updateTabData.fwOutdated) {
                            popupLock = true;
                            fwVersionDialog.contentItem.text = firmwareOutdatedDialogText(Globals.updateTabData.fwVersionLatest);
                            timer.startTimer(fwVersionDialog.open);
                        }
                        firmwareVersionDialogAlready = true;
                    }
                }
            }
        }
    }

}
