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
