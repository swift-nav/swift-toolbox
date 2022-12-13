import "BaseComponents"
import "Constants"
import Qt.labs.platform as LP
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Dialogs
import QtQuick.Layouts
import SwiftConsole

Rectangle {
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
            onClicked: backend_request_broker.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText)
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
            onClicked: backend_request_broker.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText)
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
                font.pixelSize: Constants.mediumPixelSize
                font.family: Constants.fontFamily
                font.capitalization: Font.AllUppercase
                font.bold: true
            }

            Label {
                id: recordingTime

                Layout.minimumWidth: 100
                Layout.alignment: Qt.AlignHCenter
                horizontalAlignment: Text.AlignHCenter
                font.pixelSize: Constants.xxlPixelSize
                font.family: Constants.lightFontFamily
                text: "00:00:00"
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
                font.pixelSize: Constants.xxlPixelSize
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

                SwiftComboBox {
                    id: sbpLoggingFormat

                    currentIndex: 0
                    Layout.preferredWidth: Constants.loggingBar.sbpLoggingButtonWidth
                    Layout.preferredHeight: loggingBarRowLayout.preferredButtonHeight
                    enabled: !sbpLoggingButton.checked
                    font: Constants.loggingBar.comboBoxFont
                    model: loggingBarData.sbp_logging_labels
                    textRole: "display"
                    ToolTip.visible: hovered
                    ToolTip.text: "SBP Log Format"
                    onActivated: {
                        backend_request_broker.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
                        loggingBarData.sbp_logging_format = sbpLoggingFormat.currentText;
                    }
                }

                SwiftComboBox {
                    id: recordingFilenameText

                    Layout.fillWidth: true
                    Layout.preferredHeight: loggingBarRowLayout.preferredButtonHeight
                    enabled: !sbpLoggingButton.checked
                    font: Constants.loggingBar.comboBoxFont
                    model: []
                    textRole: "display"
                    editable: true
                    selectTextByMouse: true
                    visible: sbpLoggingButton.checked
                }

                SwiftComboBox {
                    id: folderPathBar

                    Layout.fillWidth: true
                    Layout.preferredHeight: loggingBarRowLayout.preferredButtonHeight
                    enabled: !sbpLoggingButton.checked
                    font: Constants.loggingBar.comboBoxFont
                    model: loggingBarData.previous_folders
                    textRole: "display"
                    editable: true
                    selectTextByMouse: true
                    visible: !sbpLoggingButton.checked
                    currentIndex: 0
                    onActivated: {
                        if (folderPathBar.editText == folderPathBar.currentText) {
                            backend_request_broker.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
                            folderPathBar.currentIndex = 0;
                        } else {
                            folderPathBar.editText = folderPathBar.currentText;
                        }
                    }
                    onAccepted: {
                        if (folderPathBar.editText != folderPathBar.currentText)
                            backend_request_broker.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
                    }
                    onCurrentIndexChanged: {
                        if (folderPathBar.currentIndex == -1)
                            folderPathBar.currentIndex = 0;
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
            currentFolder: LP.StandardPaths.writableLocation(LP.StandardPaths.HomeLocation)
            fileMode: FileDialog.SaveFile
            onAccepted: {
                var filepath = Utils.fileUrlToString(fileDialog.folder);
                folderPathBar.editText = filepath;
                backend_request_broker.logging_bar([csvLoggingButton.checked, sbpLoggingButton.checked, sbpLoggingFormat.currentText], folderPathBar.editText);
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
                sbpLoggingButton.checked = loggingBarData.sbp_logging;
                csvLoggingButton.checked = loggingBarData.csv_logging;
                if (loggingBarData.recording_filename)
                    recordingFilenameText.editText = loggingBarData.recording_filename;

                if (sbpLoggingButton.checked) {
                    if (mockTime) {
                        mockRecordingTime += interval;
                        recordingTime.text = loggingDurationFormat(mockRecordingTime / 1000);
                    } else {
                        recordingTime.text = loggingDurationFormat(loggingBarData.recording_duration_sec);
                    }
                }
                if (mockSize) {
                    mockRecordingSize += 15.15;
                    recordingSize.text = bytesToString(mockRecordingSize);
                } else {
                    let recSize = loggingBarData.recording_size.toFixed(0);
                    if (recSize > 0)
                        recordingSize.text = bytesToString(recSize);
                    else
                        recordingSize.text = "0.00 MB";
                }
            }
        }
    }
}
