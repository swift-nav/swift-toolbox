import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Dialogs
import QtQuick.Layouts 1.15

Item {
    property alias fwDirectory: selectFirmwareDownloadDirectory.fwDirectory
    property alias downloadButtonEnable: downloadFirmwareButton.enabled
    property alias fwDirectoryEditing: selectFirmwareDownloadDirectory.fwDirectoryEditing

    Rectangle {
        width: parent.width
        height: parent.height
        border.width: Constants.updateTab.borderWidth
        border.color: Constants.genericTable.borderColor

        ColumnLayout {
            anchors.fill: parent
            width: parent.width
            height: parent.height

            SelectFirmwareDownloadDirectory {
                id: selectFirmwareDownloadDirectory

                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins
            }

            Button {
                id: downloadFirmwareButton

                Layout.alignment: Qt.AlignBottom
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins
                Layout.bottomMargin: Constants.updateTab.innerMargins
                topInset: Constants.updateTab.buttonInset
                bottomInset: Constants.updateTab.buttonInset
                onClicked: {
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

                Label {
                    text: Constants.updateTab.downloadLatestFirmwareButtonLabel
                    anchors.centerIn: parent
                }

            }

        }

    }

}
