import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    RowLayout {
        anchors.fill: parent

        Rectangle {
            Layout.preferredWidth: Constants.updateTab.hardwareVersionElementsLabelWidth
            Layout.fillHeight: true

            Text {
                text: Constants.updateTab.fileioLocalFileLabel
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
                anchors.fill: parent
                anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                horizontalAlignment: Text.AlignRight
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.width: Constants.advancedIns.textDataBarBorderWidth

            TextInput {
                id: localFileTextInput

                text: ""
                cursorVisible: true
                selectByMouse: true
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
                anchors.fill: parent
                anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                onTextEdited: {
                    data_model.update_tab([false, false, false], null, null, text, null);
                }
            }

        }

        Item {
            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonSpacing
            Layout.fillHeight: true
        }

        Button {
            id: localFileSelectionButton

            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonWidth
            Layout.fillHeight: true
            topInset: Constants.updateTab.buttonInset
            bottomInset: Constants.updateTab.buttonInset
            onClicked: {
                fileDialog.visible = !fileDialog.visible;
            }

            Text {
                text: Constants.updateTab.dotDotDotLabel
                anchors.centerIn: parent
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
            }

        }

        FileDialog {
            id: fileDialog

            visible: false
            title: "Please choose a file."
            folder: shortcuts.home
            selectFolder: false
            selectMultiple: false
            selectExisting: true
            nameFilters: ["All Files (*)"]
            onAccepted: {
                var filepath = Utils.fileUrlToString(fileDialog.file);
                // Fix for fileUrlToString which removes file:/// prefix but leaves unix
                // path without leading forward slash.
                if (Qt.platform.os !== "windows")
                    filepath = "/" + filepath;

                data_model.update_tab([false, false, false], null, null, filepath, null);
            }
            onRejected: {
            }
        }

        Rectangle {
            Layout.preferredWidth: 100 //Constants.updateTab.hardwareVersionElementsLabelWidth
            Layout.fillHeight: true

            Text {
                text: Constants.updateTab.fileioDestinationPathLabel
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
                anchors.fill: parent
                anchors.rightMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                horizontalAlignment: Text.AlignRight
            }

        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.width: Constants.advancedIns.textDataBarBorderWidth

            TextInput {
                id: destinationPathTextInput

                text: ""
                cursorVisible: true
                selectByMouse: true
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
                anchors.fill: parent
                anchors.leftMargin: Constants.updateTab.firmwareVersionElementsLabelRightMargin
                onTextEdited: {
                    data_model.update_tab([false, false, false], null, null, null, text);
                }
            }

        }

        Item {
            Layout.preferredWidth: Constants.updateTab.firmwareVersionLocalFileButtonSpacing
            Layout.fillHeight: true
        }

        Button {
            id: sendFileToDeviceButton

            Layout.preferredWidth: Constants.updateTab.fileioDestinationPathButtonWidth
            Layout.fillHeight: true
            topInset: Constants.updateTab.buttonInset
            bottomInset: Constants.updateTab.buttonInset
            onClicked: {
                data_model.update_tab([false, false, true], null, null, null, null);
            }

            Text {
                text: Constants.updateTab.fileioSendFileToDeviceButtonLabel
                anchors.centerIn: parent
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
            }

        }

    }

}
