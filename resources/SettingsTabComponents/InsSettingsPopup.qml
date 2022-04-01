import "../Constants"
import "../TableComponents"
import Qt.labs.qmlmodels 1.0
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

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
            id: layout

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
                property variant columnWidths: [layout.width / 3, layout.width / 3, layout.width / 3]

                spacing: 0
                width: parent.width
                height: Constants.insSettingsPopup.tableHeight
                visible: settings.length > 0

                HorizontalHeaderView {
                    id: horizontalHeader

                    interactive: false
                    syncView: tableView

                    delegate: Rectangle {
                        implicitWidth: columnWidths[index]
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
                                    let oldcols = columnWidths.slice();
                                    var delta_x = (mouseX - mouse_x);
                                    columnWidths[index] += delta_x;
                                    columnWidths[(index + 1) % 3] -= delta_x;
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
                    columnWidths: parent.columnWidths

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
            for (var idx in settings) {
                var new_row = {
                };
                new_row[Constants.insSettingsPopup.columnHeaders[0]] = settings[idx][0];
                new_row[Constants.insSettingsPopup.columnHeaders[1]] = settings[idx][1];
                new_row[Constants.insSettingsPopup.columnHeaders[2]] = settings[idx][2];
                tableView.model.setRow(idx, new_row);
            }
        }
    }

}
