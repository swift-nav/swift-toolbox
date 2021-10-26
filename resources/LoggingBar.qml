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
    property alias sbpRecording: sbpLoggingButton.checked
    property string recordingFilename: ""
    property string lastEdittedLogDirectoryText: ""

    function loggingDurationFormat(duration) {
        let hours = Math.floor(duration / 3600).toFixed(0).padStart(2, 0);
        let minutes = Math.floor(duration / 60).toFixed(0).padStart(2, 0);
        let seconds = (duration % 60).toFixed(0).padStart(2, 0);
        return hours + ":" + minutes + ":" + seconds + " s";
    }

    border.width: Constants.statusBar.borderWidth
    border.color: Constants.statusBar.borderColor

    LoggingBarData {
        id: loggingBarData
    }

    RowLayout {
        id: loggingBarRowLayout

        anchors.fill: parent
        anchors.leftMargin: Constants.loggingBar.loggingBarMargin
        anchors.rightMargin: Constants.loggingBar.loggingBarMargin * 2

        Button {
            id: csvLoggingButton

            Layout.preferredWidth: Constants.loggingBar.csvLoggingButtonWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight
            text: "CSV Log"
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "On" : "Off"
            checkable: true
            visible: Globals.showCsvLog
            onClicked: data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText)
        }

        Button {
            id: sbpLoggingButton

            icon.source: checked ? Constants.icons.squareSolidPath : Constants.icons.solidCirclePath
            icon.color: checked ? Constants.swiftOrange : Constants.materialGrey
            checkable: true
            Layout.preferredWidth: Constants.loggingBar.buttonHeight
            Layout.preferredHeight: Constants.loggingBar.buttonHeight
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "Start Recording" : "Stop Recording"
            onClicked: {
                data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
            }
            Component.onCompleted: {
                this.background.children[0].visible = false;
            }
        }

        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true

                ToolTip {
                    visible: parent.containsMouse && sbpLoggingButton.checked
                    text: "Currently logging, stop logging to adjust."
                }

            }

            RowLayout {
                anchors.fill: parent

                ComboBox {
                    id: sbpLoggingFormat

                    enabled: !sbpLoggingButton.checked
                    Layout.preferredWidth: Constants.loggingBar.sbpLoggingButtonWidth
                    Layout.preferredHeight: Constants.loggingBar.buttonHeight
                    model: sbp_logging_labels
                    ToolTip.visible: hovered
                    ToolTip.text: "SBP Log Format"
                    onActivated: data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText)
                }

                ComboBox {
                    id: recordingFilenameText

                    Layout.fillWidth: true
                    Layout.preferredHeight: Constants.loggingBar.folderPathBarHeight
                    model: previous_folders
                    editable: true
                    selectTextByMouse: true
                    enabled: !sbpLoggingButton.checked
                    visible: sbpLoggingButton.checked
                }

                ComboBox {
                    id: folderPathBar

                    Layout.fillWidth: true
                    Layout.preferredHeight: Constants.loggingBar.folderPathBarHeight
                    model: previous_folders
                    editable: true
                    selectTextByMouse: true
                    enabled: !sbpLoggingButton.checked
                    visible: !sbpLoggingButton.checked
                    onActivated: {
                        var text = folderPathBar.currentText;
                        folderPathBar.currentIndex = -1;
                        folderPathBar.editText = text;
                        data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
                    }

                    Label {
                        anchors.fill: parent.contentItem
                        anchors.leftMargin: 4
                        verticalAlignment: Text.AlignVCenter
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
                    enabled: !sbpLoggingButton.checked
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
                        color: Constants.materialGrey
                        antialiasing: true
                    }

                }

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
                data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
            }
            onRejected: {
            }
        }

        Rectangle {
            Layout.preferredWidth: Constants.loggingBar.recordingLabelWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight

            Label {
                anchors.centerIn: parent
                text: "Recording:"
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
            }

        }

        Rectangle {
            Layout.preferredWidth: Constants.loggingBar.recordingTimeLabelWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight

            Label {
                id: recordingTime

                anchors.centerIn: parent
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
            }

        }

        Rectangle {
            Layout.preferredWidth: Constants.loggingBar.recordingDividerLabelWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight

            Label {
                anchors.centerIn: parent
                text: "|"
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
            }

        }

        Rectangle {
            Layout.preferredWidth: Constants.loggingBar.recordingSizeLabelWidth
            Layout.preferredHeight: Constants.loggingBar.buttonHeight

            Label {
                id: recordingSize

                anchors.centerIn: parent
                font.pointSize: Constants.largePointSize
                font.family: Constants.genericTable.fontFamily
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

                sbpLoggingButton.checked = loggingBarData.sbp_logging;
                sbpLoggingFormat.currentIndex = sbp_logging_labels.indexOf(loggingBarData.sbp_logging_format);
                csvLoggingButton.checked = loggingBarData.csv_logging;
                recordingTime.text = loggingDurationFormat(loggingBarData.recording_duration_sec);
                recordingSize.text = loggingBarData.recording_size;
                recordingFilenameText.editText = loggingBarData.recording_filename;
            }
        }

    }

}
