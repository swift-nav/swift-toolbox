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

    function bytesToString(bytes, decimals = 2) {
        if (bytes === 0)
            return '0 Bytes';
        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return (bytes / Math.pow(k, i)).toFixed(dm) + ' ' + sizes[i];
    }

    color: Constants.swiftControlBackground

    LoggingBarData {
        id: loggingBarData

        function update() {
            logging_bar_model.fill_data(loggingBarData);
            if (sbpLoggingFormat.currentIndex == -1) {
                sbpLoggingFormat.currentIndex = loggingBarData.sbp_logging_format_index;
                sbpLoggingButton.checked = loggingBarData.sbp_logging;
                csvLoggingButton.checked = loggingBarData.csv_logging;
            }
            if (loggingBarData.recording_filename)
                recordingFilenameText.editText = loggingBarData.recording_filename;
            let recording = loggingBarData.sbp_logging;
            recordingTime.text = recording ? loggingDurationFormat(loggingBarData.recording_duration_sec) : "00:00:00";
            let recSize = loggingBarData.recording_size.toFixed(0);
            recordingSize.text = recSize > 0 && recording ? bytesToString(recSize) : "0.00 MB";
        }
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
    }
}
