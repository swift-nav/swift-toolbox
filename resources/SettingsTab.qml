import "Constants"
import QtCharts 2.2
import QtQuick 2.7
import QtQuick.Controls 1.4
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Dialogs 1.3
import "SettingsTabComponents" as SettingsTabComponents

Item {
    id: settingsTab

    width: parent.width
    height: parent.height

    function selectedRow() {
        var rowIdx = settingsTable.selectedRow;
        if (rowIdx < 0) {
            return;
        }
        return settingsTable.table[settingsTable.rowOffsets[rowIdx]];
    }

    function shouldShowField(name) {
        var row = selectedRow();
        if (!row) {
            return false;
        }
        return !!row[name];
    }

    function selectedRowField(name) {
        var row = selectedRow();
        if (!row) {
            return "";
        }
        return row[name] || "";
    }

    FileDialog {
        id: exportDialog
        defaultSuffix: "ini"
        selectExisting: false
        nameFilters: ["*.ini"]
        folder: shortcuts.home
        onAccepted: data_model.settings_export_request(exportDialog.fileUrl)
    }

    FileDialog {
        id: importDialog
        defaultSuffix: "ini"
        selectExisting: true
        nameFilters: ["*.ini"]
        folder: shortcuts.home
        onAccepted: data_model.settings_import_request(importDialog.fileUrl)
    }

    RowLayout {
        anchors.fill: parent

        Rectangle {
            id: leftPanel
            width: settingsTable.width
            Layout.fillHeight: true

            SettingsTabComponents.SettingsTable {
                id: settingsTable
            }
        }

        ColumnLayout {
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Layout.maximumWidth: parent.width - leftPanel.width
            spacing: 3

            RowLayout {
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.preferredHeight: 50

                Button {
                    text: "Save to Device"
                    icon.source: Constants.icons.savePath
                    icon.width: 20
                    icon.height: 20
                }

                Button {
                    text: "Export to file"
                    icon.source: Constants.icons.exportPath
                    icon.width: 20
                    icon.height: 20
                    onClicked: exportDialog.visible = true
                }

                Button {
                    text: "Import from File"
                    icon.source: Constants.icons.importPath
                    icon.width: 20
                    icon.height: 20
                    onClicked: importDialog.visible = true
                }

                Button {
                    text: "Reset to Defaults"
                    icon.source: Constants.icons.warningPath
                    icon.width: 20
                    icon.height: 20
                }
            }

            RowLayout {
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.preferredHeight: 50

                Button {
                    text: "Refresh from device"
                    icon.source: Constants.icons.refreshPath
                    icon.width: 20
                    icon.height: 20
                    onClicked: data_model.settings_refresh()
                }

                CheckBox {
                    text: "Show Advance Settings"
                    onClicked: {
                        settingsTable.showExpert = checked
                        settingsTable.selectedRow = -1
                    }
                }
            }

            ToolSeparator {
                orientation: Qt.Horizontal
                Layout.fillWidth: true
            }

            ColumnLayout {
                Component {
                    id: settingRowLabel
                    Label {
                        text: _title
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        font.bold: true
                    }
                }
                Component {
                    id: settingRowText
                    Row {
                        width: 550
                        Text {
                            text: selectedRowField(_fieldName)
                            width: parent.width
                            elide: Text.ElideRight
                            wrapMode: Text.WordWrap
                            font.family: Constants.genericTable.fontFamily
                            font.pointSize: Constants.largePointSize
                        }
                    }
                }
                Component {
                    id: settingRowEditable
                    TextField {
                        text: selectedRowField(_fieldName)
                        wrapMode: Text.WordWrap
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        Keys.onReturnPressed: {
                            data_model.settings_save_request(
                                selectedRowField("group"),
                                selectedRowField("name"),
                                text
                            )
                        }
                    }
                }
                Component {
                    id: settingRowBool
                    ComboBox {
                        model: ["True", "False"]
                        currentIndex: model.indexOf(selectedRowField("valueOnDevice"))
                        onCurrentIndexChanged: {
                            data_model.settings_save_request(
                                selectedRowField("group"),
                                selectedRowField("name"),
                                model[currentIndex]
                            )
                        }
                    }
                }
                Component {
                    id: settingRowEnum
                    ComboBox {
                        model: selectedRowField("enumeratedPossibleValues").split(",")
                        currentIndex: model.indexOf(selectedRowField("valueOnDevice"))
                        onCurrentIndexChanged: {
                            data_model.settings_save_request(
                                selectedRowField("group"),
                                selectedRowField("name"),
                                model[currentIndex]
                            )
                        }
                    }
                }
                Component {
                    id: settingRow
                    RowLayout {
                        visible: shouldShowField(fieldName)
                        Loader {
                            property string _title: title
                            property string _fieldName: fieldName
                            sourceComponent: settingRowLabel
                        }
                        Loader {
                            property string _fieldName: fieldName
                            sourceComponent: component
                        }
                    }
                }

                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.maximumWidth: parent.width

                Loader {
                    property string title: "Name"
                    property string fieldName: "name"
                    property Component component: settingRowText
                    sourceComponent: settingRow
                }

                Loader {
                    property string title: "Value"
                    property string fieldName: "valueOnDevice"
                    property Component component: {
                        if (selectedRowField("readonly")) {
                            return settingRowText
                        }
                        var ty = selectedRowField("type")
                        if (ty === "boolean") {
                            return settingRowBool
                        } else if (ty === "enum") {
                            return settingRowEnum
                        } else {
                            return settingRowEditable
                        }
                    }
                    sourceComponent: settingRow
                }

                Repeater {
                    model: [
                        { title: "Units", fieldName: "units" },
                        { title: "Setting Type", fieldName: "type" },
                        { title: "Default Value", fieldName: "defaultValue" },
                        { title: "Description", fieldName: "description" },
                        { title: "Notes", fieldName: "notes" },
                    ].filter(el => shouldShowField(el.fieldName))
                    Loader {
                        property string title: modelData.title
                        property string fieldName: modelData.fieldName
                        property Component component: settingRowText
                        sourceComponent: settingRow
                    }
                }

                Item {
                    Layout.fillHeight: true
                }
            }
        }
    }
}
