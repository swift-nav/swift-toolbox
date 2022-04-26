import "../BaseComponents"
import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

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
            anchors.topMargin: Constants.updateTab.innerMargins
            anchors.leftMargin: Constants.updateTab.innerMargins
            anchors.rightMargin: Constants.updateTab.innerMargins
            width: parent.width
            height: parent.height

            SelectFirmwareDownloadDirectory {
                id: selectFirmwareDownloadDirectory

                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins
            }

            SwiftButton {
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
                    backend_request_broker.update_tab([downloadLatestFirmware, updateFirmware, sendFileToDevice, serialPromptConfirm], updateLocalFilepath, downloadDirectory, fileioLocalFilepath, fileioDestinationFilepath, updateLocalFilename);
                }

                Label {
                    text: Constants.updateTab.downloadLatestFirmwareButtonLabel
                    anchors.centerIn: parent
                    font.family: Constants.genericTable.fontFamily
                    font.pixelSize: Constants.largePixelSize
                }

            }

        }

    }

}
