import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
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
                    data_model.update_tab([true, false, false], null, null, null, null, null);
                }

                Text {
                    text: Constants.updateTab.downloadLatestFirmwareButtonLabel
                    anchors.centerIn: parent
                    font.pointSize: Constants.largePointSize
                    font.family: Constants.genericTable.fontFamily
                }

            }

        }

    }

}
