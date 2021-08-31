import "Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0
import "UpdateTabComponents" as UpdateTabComponents

Item {
    id: updateTab

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
                anchors.fill: parent
            }

        }

    }

    Timer {
        
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            if (!updateTab.visible)
                return ;
            update_tab_model.fill_data(updateTabData);
            firmwareRevision.revision = updateTabData.hardware_revision;
            firmwareVersion.currentVersion = updateTabData.fw_version_current;
            firmwareVersion.latestVersion = updateTabData.fw_version_latest;
            firmwareDownload.fwDirectory = updateTabData.directory;
            fwLogTextArea.text = updateTabData.fw_text;
            firmwareDownload.downloadButtonEnable = !updateTabData.downloading;
            firmwareVersion.localFileText = updateTabData.fw_local_filename;
        }
    }

}
