import "BaseComponents"
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
    property int preferredHeight: Constants.loggingBar.preferredHeight

    function loggingDurationFormat(duration) {
        let hours = Math.floor(duration / 3600).toFixed(0).padStart(2, 0);
        let minutes = Math.floor((duration / 60) % 60).toFixed(0).padStart(2, 0);
        let seconds = (duration % 60).toFixed(0).padStart(2, 0);
        return hours + ":" + minutes + ":" + seconds;
    }

    color: Constants.swiftControlBackground

    LoggingBarData {
        id: loggingBarData
    }

    RowLayout {
        id: loggingBarRowLayout

        property real preferredButtonHeight: height * Constants.loggingBar.buttonHeightRatio

        anchors.fill: parent
        anchors.leftMargin: Constants.loggingBar.loggingBarMargin
        anchors.rightMargin: Constants.loggingBar.loggingBarMargin * 2

        SwiftButton {
            id: csvLoggingButton

            Layout.preferredHeight: parent.preferredButtonHeight
            invertColor: true
            text: "CSV Log"
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "On" : "Off"
            checkable: true
            visible: Globals.showCsvLog
            onClicked: data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText)
        }

        SwiftButton {
            id: sbpLoggingButton

            invertColor: true
            icon.source: checked ? Constants.icons.squareSolidPath : Constants.icons.solidCirclePath
            icon.color: checked ? "red" : Constants.materialGrey
            checkable: true
            showAccent: false
            Layout.preferredWidth: parent.preferredButtonHeight
            Layout.preferredHeight: parent.preferredButtonHeight
            ToolTip.visible: hovered
            ToolTip.text: !checked ? "Start Recording" : "Stop Recording"
            onClicked: {
                data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
            }
        }

        RowLayout {
            property int spacerMargin: 5
            property color spacerColor: Constants.spacerColor

            Layout.preferredHeight: Constants.loggingBar.buttonHeight

            Label {
                text: "Recording"
                visible: false
                Layout.alignment: Qt.AlignVCenter
                Layout.topMargin: 6
                font.pointSize: Constants.mediumPointSize
                font.family: Constants.fontFamily
                font.capitalization: Font.AllUppercase
                font.bold: true
            }

            Label {
                id: recordingTime

                Layout.minimumWidth: 100
                Layout.alignment: Qt.AlignHCenter
                horizontalAlignment: Text.AlignHCenter
                font.pointSize: Constants.xxlPointSize
                font.family: Constants.lightFontFamily
            }

            Rectangle {
                Layout.fillHeight: true
                Layout.topMargin: parent.spacerMargin
                Layout.bottomMargin: parent.spacerMargin
                Layout.alignment: Qt.AlignVCenter
                width: 1
                color: parent.spacerColor
            }

            Label {
                id: recordingSize

                Layout.minimumWidth: 100
                Layout.alignment: Qt.AlignHCenter
                horizontalAlignment: Text.AlignHCenter
                font.pointSize: Constants.xxlPointSize
                font.family: Constants.lightFontFamily
            }

            Rectangle {
                Layout.fillHeight: true
                Layout.topMargin: parent.spacerMargin
                Layout.bottomMargin: parent.spacerMargin
                Layout.alignment: Qt.AlignVCenter
                width: 1
                color: parent.spacerColor
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

                    Layout.preferredWidth: Constants.loggingBar.sbpLoggingButtonWidth
                    Layout.preferredHeight: loggingBarRowLayout.preferredButtonHeight
                    enabled: !sbpLoggingButton.checked
                    font: Constants.loggingBar.comboBoxFont
                    model: sbp_logging_labels
                    ToolTip.visible: hovered
                    ToolTip.text: "SBP Log Format"
                    onActivated: data_model.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText)
                }

                ComboBox {
                    id: recordingFilenameText

                    Layout.fillWidth: true
                    Layout.preferredHeight: loggingBarRowLayout.preferredButtonHeight
                    enabled: !sbpLoggingButton.checked
                    font: Constants.loggingBar.comboBoxFont
                    model: previous_folders
                    editable: true
                    selectTextByMouse: true
                    visible: sbpLoggingButton.checked
                }

                ComboBox {
                    id: folderPathBar

                    Layout.fillWidth: true
                    Layout.preferredHeight: loggingBarRowLayout.preferredButtonHeight
                    enabled: !sbpLoggingButton.checked
                    font: Constants.loggingBar.comboBoxFont
                    model: previous_folders
                    editable: true
                    selectTextByMouse: true
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

                SwiftButton {
                    id: folderBarButton

                    invertColor: true
                    Layout.preferredWidth: Constants.loggingBar.folderButtonWidth
                    Layout.preferredHeight: loggingBarRowLayout.preferredButtonHeight
                    ToolTip.visible: hovered
                    ToolTip.text: "File Browser"
                    enabled: !sbpLoggingButton.checked
                    padding: 9
                    icon.source: Constants.icons.folderPath
                    onClicked: {
                        fileDialog.visible = !fileDialog.visible;
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

        Timer {
            property bool mockTime: false
            property bool mockSize: false
            property int mockRecordingTime: 0
            property real mockRecordingSize: 0

            function bytesToString(bytes, decimals = 2) {
                if (bytes === 0)
                    return '0 Bytes';

                const k = 1024;
                const dm = decimals < 0 ? 0 : decimals;
                const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
                const i = Math.floor(Math.log(bytes) / Math.log(k));
                return (bytes / Math.pow(k, i)).toFixed(dm) + ' ' + sizes[i];
            }

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
                recordingFilenameText.editText = loggingBarData.recording_filename;
                if (mockTime) {
                    mockRecordingTime += interval;
                    recordingTime.text = loggingDurationFormat(mockRecordingTime / 1000);
                } else {
                    recordingTime.text = loggingDurationFormat(loggingBarData.recording_duration_sec);
                }
                if (mockSize) {
                    mockRecordingSize += 15.15;
                    recordingSize.text = bytesToString(mockRecordingSize);
                } else {
                    if (loggingBarData.recording_size > 0)
                        recordingSize.text = bytesToString(loggingBarData.recording_size);
                    else
                        recordingSize.text = "0.00 MiB";
                }
            }
        }

    }

}
