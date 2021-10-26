import "Constants"
import Qt.labs.platform 1.1 as LabsPlatform
import QtCharts 2.2
import QtQuick 2.7
import QtQuick.Controls 1.4
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.15
import "SettingsTabComponents" as SettingsTabComponents
import SwiftConsole 1.0

Item {
    id: settingsTab

    property var floatValidator
    property var intValidator
    property var stringValidator

    function selectedRow() {
        var rowIdx = settingsTable.selectedRow;
        if (rowIdx < 0)
            return ;

        return settingsTable.table[settingsTable.rowOffsets[rowIdx]];
    }

    function shouldShowField(name) {
        var row = selectedRow();
        if (!row)
            return false;

        return !!row[name];
    }

    function selectedRowField(name) {
        var row = selectedRow();
        if (!row)
            return "";

        return row[name] || "";
    }

    function autoSurveyDialogText() {
        var text = "This will set the Surveyed Position section to the mean position of up to the last 1000 position solutions. ";
        text += "The fields that will be auto-populated are: \n\n";
        text += "Surveyed Lat\n";
        text += "Surveyed Lon\n";
        text += "Surveyed Alt\n\n";
        text += "The surveyed position will be an approximate value. ";
        text += "This may affect the relative accuracy of Piksi.\n\n";
        text += "Are you sure you want to auto-populate the Surveyed Position section?";
        return text;
    }

    width: parent.width
    height: parent.height

    SettingsTabData {
        id: settingsTabData
    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            settings_tab_model.fill_data(settingsTabData);
            if (settingsTabData.import_status !== "") {
                if (settingsTabData.import_status === "success") {
                    importSuccess.visible = true;
                } else {
                    importFailure.text = "Error: " + settingsTabData.import_status;
                    importFailure.visible = true;
                }
                settings_tab_model.clear_import_status(settingsTabData);
            }
            if (settingsTabData.new_ins_confirmation) {
                insSettingsPopup.settings = settingsTabData.recommended_ins_settings;
                insSettingsPopup.insPopup.open();
                settings_tab_model.clear_new_ins_confirmation(settingsTabData);
            }
        }
    }

    LabsPlatform.FileDialog {
        id: exportDialog

        defaultSuffix: "ini"
        nameFilters: ["*.ini"]
        fileMode: LabsPlatform.FileDialog.SaveFile
        currentFile: {
            let text = LabsPlatform.StandardPaths.writableLocation(LabsPlatform.StandardPaths.HomeLocation);
            text += "/" + Constants.settingsTab.defaultImportExportRelativePathFromHome;
            text += "/" + Constants.settingsTab.defaultExportFileName;
            return text;
        }
        onAccepted: {
            var filepath = Utils.fileUrlToString(exportDialog.file);
            data_model.settings_export_request(filepath);
        }
    }

    FileDialog {
        id: importDialog

        defaultSuffix: "ini"
        selectExisting: true
        nameFilters: ["*.ini"]
        folder: shortcuts.home + "/" + Constants.settingsTab.defaultImportExportRelativePathFromHome
        onAccepted: {
            var filepath = Utils.fileUrlToString(importDialog.fileUrl);
            data_model.settings_import_request(filepath);
        }
    }

    MessageDialog {
        id: resetDialog

        title: "Reset to Factory Defaults?"
        icon: StandardIcon.Warning
        text: "This will erase all settings and then reset the device.\nAre you sure you want to reset to factory defaults?"
        standardButtons: StandardButton.RestoreDefaults | StandardButton.No
        onReset: data_model.settings_reset_request()
    }

    MessageDialog {
        id: importSuccess

        title: "Successfully imported settings from file."
        text: "Settings import from file complete.  Click OK to save the settings\nto the device's persistent storage."
        standardButtons: StandardButton.Yes | StandardButton.No
        onYes: data_model.settings_save_request()
    }

    MessageDialog {
        id: autoSurveyDialog

        title: "Auto populate surveyed position?"
        text: autoSurveyDialogText()
        standardButtons: StandardButton.Yes | StandardButton.No
        onYes: data_model.auto_survey_request()
    }

    SettingsTabComponents.InsSettingsPopup {
        id: insSettingsPopup
    }

    MessageDialog {
        id: importFailure

        title: "Failed to import settings from file."
        standardButtons: StandardButton.Ok
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
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    onClicked: data_model.settings_save_request()
                }

                Button {
                    text: "Export to file"
                    icon.source: Constants.icons.exportPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    onClicked: exportDialog.visible = true
                }

                Button {
                    text: "Import from File"
                    icon.source: Constants.icons.importPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    onClicked: importDialog.visible = true
                }

                Button {
                    text: "Reset to Defaults"
                    icon.source: Constants.icons.warningPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    onClicked: resetDialog.visible = true
                }

                Button {
                    text: "Auto Survey"
                    visible: selectedRowField("group") === "surveyed_position"
                    icon.source: Constants.icons.centerOnButtonUrl
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    onClicked: autoSurveyDialog.visible = true
                }

            }

            RowLayout {
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.preferredHeight: 50

                Button {
                    text: "Refresh from device"
                    icon.source: Constants.icons.refreshPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    onClicked: data_model.settings_refresh()
                }

                CheckBox {
                    text: "Show Advance Settings"
                    onClicked: {
                        settingsTable.showExpert = checked;
                        settingsTable.selectedRow = -1;
                    }
                }

            }

            ToolSeparator {
                orientation: Qt.Horizontal
                Layout.fillWidth: true
            }

            ColumnLayout {
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.maximumWidth: parent.width

                Component {
                    id: settingRowLabel

                    Label {
                        text: _title
                        font.bold: true
                    }

                }

                Component {
                    id: settingRowText

                    Row {
                        width: Constants.settingsTab.textSettingWidth

                        Label {
                            text: selectedRowField(_fieldName)
                            width: parent.width
                            elide: Text.ElideRight
                            wrapMode: Text.WordWrap
                        }

                    }

                }

                Component {
                    id: settingRowEditable

                    TextField {
                        id: textField

                        // these are properties because the selected row will have changed before
                        // the onDestruction event has triggered
                        property string settingGroup: selectedRowField("group")
                        property string settingName: selectedRowField("name")
                        property string settingType: selectedRowField("type")

                        text: selectedRowField(_fieldName)
                        wrapMode: Text.WordWrap
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        onEditingFinished: {
                            data_model.settings_write_request(settingGroup, settingName, text);
                        }
                        Component.onDestruction: {
                            data_model.settings_write_request(settingGroup, settingName, text);
                        }
                        validator: {
                            if (settingType === "integer")
                                return intValidator;
                            else if (settingType === "float" || settingType === "double")
                                return floatValidator;
                            else
                                return stringValidator;
                        }
                    }

                }

                Component {
                    id: settingRowBool

                    ComboBox {
                        model: ["True", "False"]
                        currentIndex: model.indexOf(selectedRowField("valueOnDevice"))
                        onCurrentIndexChanged: {
                            if (selectedRowField("valueOnDevice") != model[currentIndex])
                                data_model.settings_write_request(selectedRowField("group"), selectedRowField("name"), model[currentIndex]);

                        }
                    }

                }

                Component {
                    id: settingRowEnum

                    ComboBox {
                        model: selectedRowField("enumeratedPossibleValues").split(",")
                        currentIndex: model.indexOf(selectedRowField("valueOnDevice"))
                        onCurrentIndexChanged: {
                            if (selectedRowField("valueOnDevice") != model[currentIndex])
                                data_model.settings_write_request(selectedRowField("group"), selectedRowField("name"), model[currentIndex]);

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
                        if (selectedRowField("readonly"))
                            return settingRowText;

                        var ty = selectedRowField("type");
                        if (ty === "boolean")
                            return settingRowBool;
                        else if (ty === "enum")
                            return settingRowEnum;
                        else
                            return settingRowEditable;
                    }

                    sourceComponent: settingRow
                }

                Repeater {
                    model: [{
                        "title": "Units",
                        "fieldName": "units"
                    }, {
                        "title": "Setting Type",
                        "fieldName": "type"
                    }, {
                        "title": "Default Value",
                        "fieldName": "defaultValue"
                    }, {
                        "title": "Description",
                        "fieldName": "description"
                    }, {
                        "title": "Notes",
                        "fieldName": "notes"
                    }].filter((el) => {
                        return shouldShowField(el.fieldName);
                    })

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

    floatValidator: DoubleValidator {
    }

    intValidator: IntValidator {
    }

    stringValidator: RegExpValidator {
    }

}
