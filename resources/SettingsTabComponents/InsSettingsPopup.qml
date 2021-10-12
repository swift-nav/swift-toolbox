import "../Constants"
import Qt.labs.qmlmodels 1.0
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    property variant columnWidths: [layout.width / 3, layout.width / 3, layout.width / 3]
    property real mouse_x: 0
    property int selectedRow: -1
    property variant isVisible: false
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
        visible: isVisible
        title: "Confirm Inertial Navigation Change?"
        onAccepted: {
            data_model.confirm_ins_change();
            isVisible = false;
        }
        onRejected: {
            isVisible = false;
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

            Text {
                text: settingsChangeConfirmText()
                verticalAlignment: Qt.AlignVCenter
                elide: Text.ElideRight
                clip: true
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
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
                    z: Constants.genericTable.headerZOffset

                    delegate: Rectangle {
                        implicitWidth: columnWidths[index]
                        implicitHeight: Constants.genericTable.cellHeight
                        border.color: Constants.genericTable.borderColor

                        Text {
                            width: parent.width
                            anchors.centerIn: parent
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            text: tableView.model.columns[index].display
                            elide: Text.ElideRight
                            clip: true
                            font.family: Constants.genericTable.fontFamily
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

                TableView {
                    id: tableView

                    columnSpacing: -1
                    rowSpacing: -1
                    columnWidthProvider: function(column) {
                        return columnWidths[column];
                    }
                    reuseItems: true
                    boundsBehavior: Flickable.StopAtBounds
                    height: parent.height - horizontalHeader.height
                    width: parent.width

                    ScrollBar.horizontal: ScrollBar {
                    }

                    ScrollBar.vertical: ScrollBar {
                    }

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

                    delegate: Rectangle {
                        implicitHeight: Constants.genericTable.cellHeight
                        implicitWidth: tableView.columnWidthProvider(column)
                        border.color: Constants.genericTable.borderColor
                        color: row == selectedRow ? Constants.genericTable.cellHighlightedColor : Constants.genericTable.cellColor

                        Text {
                            width: parent.width
                            horizontalAlignment: Text.AlignLeft
                            clip: true
                            font.family: Constants.genericTable.fontFamily
                            font.pointSize: Constants.largePointSize
                            text: model.display
                            elide: Text.ElideRight
                            padding: Constants.genericTable.padding
                        }

                        MouseArea {
                            width: parent.width
                            height: parent.height
                            anchors.centerIn: parent
                            onPressed: {
                                if (selectedRow == row)
                                    selectedRow = -1;
                                else
                                    selectedRow = row;
                            }
                        }

                    }

                }

            }

            Text {
                text: settingBottomText()
                verticalAlignment: Qt.AlignVCenter
                elide: Text.ElideRight
                clip: true
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
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
