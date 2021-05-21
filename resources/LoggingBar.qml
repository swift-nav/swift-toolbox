import "Constants"
import QtCharts 2.2
import QtGraphicalEffects 1.15
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Dialogs 1.0
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    property variant previous_folders: []

    anchors.fill: parent
    border.width: Constants.statusBar.borderWidth
    border.color: Constants.statusBar.borderColor

    LoggingBarData {
        id: loggingBarData
    }

    RowLayout {
        id: loggingBarRowLayout

        anchors.fill: parent
        anchors.leftMargin: Constants.loggingBar.loggingBarMargin
        anchors.rightMargin: Constants.loggingBar.loggingBarMargin

        Button {
            id: solutionLoggingButton

            Layout.preferredWidth: Constants.loggingBar.solutionLoggingButtonWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight
            text: "Log Solution"
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "On" : "Off"
            checkable: true
            onClicked: data_model.logging_bar([solutionLoggingButton.checked, sbpLoggingButton.checked, sbpFileFormatSwitch.checked], folderPathBar.editText)
        }

        Button {
            id: sbpLoggingButton

            Layout.preferredWidth: Constants.loggingBar.sbpLoggingButtonWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight
            text: "Log SBP"
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "On" : "Off"
            checkable: true
            onClicked: data_model.logging_bar([solutionLoggingButton.checked, sbpLoggingButton.checked, sbpFileFormatSwitch.checked], folderPathBar.editText)
        }

        Switch {
            id: sbpFileFormatSwitch

            Component.onCompleted: {
                sbpFileFormatSwitch.checked = true;
            }
            text: !checked ? "SBP Json" : "SBP"
            ToolTip.visible: hovered
            ToolTip.text: "SBP File Type"
            onClicked: data_model.logging_bar([solutionLoggingButton.checked, sbpLoggingButton.checked, sbpFileFormatSwitch.checked], folderPathBar.editText)
        }

        ComboBox {
            id: folderPathBar

            Layout.fillWidth: true
            Layout.preferredHeight: Constants.loggingBar.folderPathBarHeight
            model: previous_folders
            editable: true
            selectTextByMouse: true

            Text {
                text: "Enter folder path"
                color: Constants.loggingBar.placeholderTextColor
                visible: !folderPathBar.editText
            }

        }

        Button {
            id: folderBarButton

            Layout.preferredWidth: Constants.loggingBar.folderButtonWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight
            ToolTip.visible: hovered
            ToolTip.text: "File Browser"
            onClicked: {
                fileDialog.visible = !fileDialog.visible;
            }

            Image {
                id: loggingBarFolder

                anchors.centerIn: parent
                width: Constants.loggingBar.buttonSvgHeight
                height: Constants.loggingBar.buttonSvgHeight
                smooth: true
                source: Constants.loggingBar.folderButtonPath
                antialiasing: true
            }

            ColorOverlay {
                anchors.fill: loggingBarFolder
                source: loggingBarFolder
                color: "dimgrey"
                antialiasing: true
            }

        }

        FileDialog {
            id: fileDialog

            visible: false
            title: "Please choose a folder."
            folder: shortcuts.home
            selectFolder: true
            onAccepted: {
                folderPathBar.editText = Utils.fileUrlToString(fileDialog.folder);
                data_model.logging_bar([solutionLoggingButton.checked, sbpLoggingButton.checked, sbpFileFormatSwitch.checked], folderPathBar.editText);
            }
            onRejected: {
            }
        }

        Timer {
            interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
            running: true
            repeat: true
            onTriggered: {
                logging_bar_model.fill_data(loggingBarData);
                previous_folders = loggingBarData.previous_folders;
                if (!folderPathBar.focus)
                    folderPathBar.editText = loggingBarData.folder;

            }
        }

    }

}
