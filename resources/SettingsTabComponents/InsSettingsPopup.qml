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
import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    property real mouse_x: 0
    property alias insPopup: dialog
    property variant settings: []

    function settingsChangeConfirmText() {
        let text = "";
        text += "In order for the \"Ins Output Mode\" setting to take effect, it is necessary to save the current settings to device ";
        text += "flash and then power cycle your device.\n\n";
        if (settings.length > 0) {
            text += "Additionally, in order to enable INS output, it is necessary to enable and configure the IMU. ";
            text += "The current settings indicate that the IMU raw ouptut is currently disabled and/or improperly ";
            text += "configured as shown in the table below.";
        }
        return text;
    }

    function settingBottomText() {
        let text = "";
        text += "Choose \"Ok\" to";
        if (settings.length > 0)
            text += " allow the console to change the above settings on your device to help enable INS output and then";
        text += " immediately save settings to device flash and send the software reset command.";
        text += " The software reset will temporarily interrupt the console's connection to the device but it ";
        text += " will recover on its own. ";
        return text;
    }

    Dialog {
        id: dialog

        property variant columnWidths: [width / 3, width / 3, width / 3]

        parent: Overlay.overlay
        title: "Confirm Inertial Navigation Change?"
        onAccepted: {
            backend_request_broker.confirm_ins_change();
        }
        standardButtons: Dialog.Ok | Dialog.Cancel
        width: Constants.insSettingsPopup.dialogWidth
        anchors.centerIn: parent
        modal: true
        closePolicy: Popup.CloseOnEscape
        focus: true

        contentItem: Column {
            width: parent.width
            spacing: Constants.insSettingsPopup.columnSpacing

            Label {
                text: settingsChangeConfirmText()
                verticalAlignment: Qt.AlignVCenter
                elide: Text.ElideRight
                clip: true
                wrapMode: Text.Wrap
                width: parent.width
            }

            ColumnLayout {
                spacing: 0
                width: parent.width
                height: Constants.insSettingsPopup.tableHeight
                visible: settings.length > 0

                HorizontalHeaderView {
                    id: horizontalHeader

                    interactive: false
                    syncView: tableView

                    delegate: Rectangle {
                        implicitWidth: dialog.columnWidths[index]
                        implicitHeight: Constants.genericTable.cellHeight
                        border.color: Constants.genericTable.borderColor

                        Label {
                            width: parent.width
                            anchors.centerIn: parent
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            text: tableView.model.columns[index].display
                            elide: Text.ElideRight
                            clip: true
                            font.family: Constants.genericTable.fontFamily
                            font.pixelSize: Constants.largePixelSize
                        }

                        MouseArea {
                            width: Constants.genericTable.mouseAreaResizeWidth
                            height: parent.height
                            anchors.right: parent.right
                            cursorShape: Qt.SizeHorCursor
                            onPressed: {
                                mouse_x = mouseX;
                            }
                            onPositionChanged: {
                                if (pressed) {
                                    let oldcols = dialog.columnWidths.slice();
                                    var delta_x = (mouseX - mouse_x);
                                    dialog.columnWidths[index] += delta_x;
                                    dialog.columnWidths[(index + 1) % 3] -= delta_x;
                                    tableView.forceLayout();
                                }
                            }
                        }

                        gradient: Gradient {
                            GradientStop {
                                position: 0
                                color: Constants.genericTable.cellColor
                            }

                            GradientStop {
                                position: 1
                                color: Constants.genericTable.gradientColor
                            }
                        }
                    }
                }

                SwiftTableView {
                    id: tableView

                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    columnWidths: dialog.columnWidths

                    model: TableModel {
                        id: tableModel

                        rows: []

                        TableModelColumn {
                            display: Constants.insSettingsPopup.columnHeaders[0]
                        }

                        TableModelColumn {
                            display: Constants.insSettingsPopup.columnHeaders[1]
                        }

                        TableModelColumn {
                            display: Constants.insSettingsPopup.columnHeaders[2]
                        }
                    }
                }
            }

            Label {
                text: settingBottomText()
                verticalAlignment: Qt.AlignVCenter
                elide: Text.ElideRight
                clip: true
                wrapMode: Text.Wrap
                width: parent.width
            }
        }
    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTableTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            let columnHeaders = Constants.insSettingsPopup.columnHeaders;
            for (var idx in settings) {
                var new_row = {};
                var entry = settings[idx];
                new_row[columnHeaders[0]] = entry[0];
                new_row[columnHeaders[1]] = entry[1];
                new_row[columnHeaders[2]] = entry[2];
                tableView.model.setRow(idx, new_row);
            }
        }
    }
}
