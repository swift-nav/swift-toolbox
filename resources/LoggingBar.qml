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
    property variant sbp_logging_labels: []

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
            id: csvLoggingButton

            Layout.preferredWidth: Constants.loggingBar.csvLoggingButtonWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight
            text: "CSV Log"
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "On" : "Off"
            checkable: true
            visible: Globals.showCsvLog
            onClicked: data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.currentText], folderPathBar.editText)
        }

        ComboBox {
            id: sbpLoggingButton

            Layout.preferredWidth: Constants.loggingBar.sbpLoggingButtonWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight
            model: sbp_logging_labels
            ToolTip.visible: hovered
            ToolTip.text: "SBP Log"
            onActivated: data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.currentText], folderPathBar.editText)

            background: Rectangle {
                border.width: 3
                border.color: sbpLoggingButton.currentIndex === 0 ? "dimgrey" : "crimson"
            }

        }

        ComboBox {
            id: folderPathBar

            Layout.fillWidth: true
            Layout.preferredHeight: Constants.loggingBar.folderPathBarHeight
            model: previous_folders
            editable: true
            selectTextByMouse: true
            onActivated: {
                var text = folderPathBar.currentText;
                folderPathBar.currentIndex = -1;
                folderPathBar.editText = text;
                data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.currentText], folderPathBar.editText);
            }

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
                var filepath = Utils.fileUrlToString(fileDialog.folder);
                folderPathBar.editText = filepath;
                data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.currentText, logLevelButton.currentText], folderPathBar.editText);
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
                if (!sbp_logging_labels.length)
                    sbp_logging_labels = loggingBarData.sbp_logging_labels;

                sbpLoggingButton.currentIndex = sbp_logging_labels.indexOf(loggingBarData.sbp_logging);
                csvLoggingButton.checked = loggingBarData.csv_logging;
            }
        }

    }

}
