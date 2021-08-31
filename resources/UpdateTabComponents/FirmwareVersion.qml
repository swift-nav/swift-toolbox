import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property alias currentVersion: currentVersionText.text
    property alias latestVersion: latestVersionText.text
    property alias localFileText: selectLocalFile.localFileText

    Rectangle {
        width: parent.width
        height: parent.height
        border.width: Constants.updateTab.borderWidth
        border.color: Constants.genericTable.borderColor

        ColumnLayout {
            anchors.fill: parent
            width: parent.width
            height: parent.height
            spacing: Constants.updateTab.firmwareVersionColumnSpacing

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins

                Rectangle {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height

                    Text {
                        text: Constants.updateTab.firmwareVersionCurrentLabel
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        anchors.fill: parent
                        anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                        horizontalAlignment: Text.AlignRight
                    }

                }

                Rectangle {
                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    border.width: Constants.advancedIns.textDataBarBorderWidth

                    Text {
                        id: currentVersionText

                        text: ""
                        clip: true
                        color: Constants.updateTab.placeholderTextColor
                        anchors.centerIn: parent
                        font.pointSize: Constants.largePointSize
                        font.family: Constants.genericTable.fontFamily
                    }

                }

            }

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins

                Rectangle {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height

                    Text {
                        text: Constants.updateTab.firmwareVersionLatestLabel
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        anchors.fill: parent
                        anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                        horizontalAlignment: Text.AlignRight
                    }

                }

                Rectangle {
                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    border.width: Constants.advancedIns.textDataBarBorderWidth

                    Text {
                        id: latestVersionText

                        text: ""
                        clip: true
                        color: Constants.updateTab.placeholderTextColor
                        anchors.centerIn: parent
                        font.pointSize: Constants.largePointSize
                        font.family: Constants.genericTable.fontFamily
                    }

                }

            }

            SelectLocalFile {
                id: selectLocalFile
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins
            }

            Row {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.updateTab.textHeight
                Layout.leftMargin: Constants.updateTab.innerMargins
                Layout.rightMargin: Constants.updateTab.innerMargins
                Layout.bottomMargin: Constants.updateTab.innerMargins
                Layout.alignment: Qt.AlignBottom

                Rectangle {
                    width: Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                }

                Button {
                    id: updateFirmwareButton

                    width: parent.width - Constants.updateTab.hardwareVersionElementsLabelWidth
                    height: parent.height
                    topInset: Constants.updateTab.buttonInset
                    bottomInset: Constants.updateTab.buttonInset
                    onClicked: {
                        data_model.update_tab([true, false, false], null, null, null, null);
                    }

                    Text {
                        text: Constants.updateTab.updateFirmwareButtonLabel
                        anchors.centerIn: parent
                        font.pointSize: Constants.largePointSize
                        font.family: Constants.genericTable.fontFamily
                    }

                }

            }

        }

    }

}
