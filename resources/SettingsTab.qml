import "Constants"
import Qt.labs.platform 1.1 as LP
import QtQuick
import QtQuick.Controls
import QtQuick.Dialogs
import QtQuick.Layouts
import "SettingsTabComponents" as SettingsTabComponents
import SwiftConsole

Item {
    id: settingsTab

    function selectedRow() {
        var rowIdx = settingsTable.selectedRowIdx;
        if (rowIdx < 0)
            return ;

        return settingsTable.table[settingsTable.rowOffsets[rowIdx]];
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

    LP.FileDialog {
        id: exportDialog

        defaultSuffix: "ini"
        nameFilters: ["*.ini"]
        fileMode: LP.FileDialog.SaveFile
        currentFile: {
            let text = LP.StandardPaths.writableLocation(LP.StandardPaths.HomeLocation);
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
//        selectExisting: true
        nameFilters: ["*.ini"]
        currentFolder: shortcuts.home + "/" + Constants.settingsTab.defaultImportExportRelativePathFromHome
        onAccepted: {
            var filepath = Utils.fileUrlToString(importDialog.fileUrl);
            data_model.settings_import_request(filepath);
        }
    }

    LP.MessageDialog {
        id: resetDialog

        title: "Reset to Factory Defaults?"
        text: "This will erase all settings and then reset the device.\nAre you sure you want to reset to factory defaults?"
        buttons: StandardButton.RestoreDefaults | StandardButton.No
        onResetClicked: data_model.settings_reset_request()
    }

    LP.MessageDialog {
        id: importSuccess

        title: "Successfully imported settings from file."
        text: "Settings import from file complete.  Click OK to save the settings\nto the device's persistent storage."
        buttons: StandardButton.Yes | StandardButton.No
        onYesClicked: data_model.settings_save_request()
    }

    LP.MessageDialog {
        id: autoSurveyDialog

        title: "Auto populate surveyed position?"
        text: autoSurveyDialogText()
        buttons: StandardButton.Yes | StandardButton.No
        onYesClicked: data_model.auto_survey_request()
    }

    SettingsTabComponents.InsSettingsPopup {
        id: insSettingsPopup
    }

    LP.MessageDialog {
        id: importFailure

        title: "Failed to import settings from file."
        buttons: StandardButton.Ok
    }

    RowLayout {
        anchors.fill: parent

        Rectangle {
            id: leftPanel

            width: settingsTable.width
            Layout.fillHeight: true

            SettingsTabComponents.SettingsTable {
                id: settingsTable

                onSelectedRowIdxChanged: {
                    if (!!selectedRow())
                        settingsPane.selectedRow = selectedRow();

                }
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
                    text: "Show Advanced Settings"
                    onClicked: {
                        settingsTable.showExpert = checked;
                        settingsTable.selectedRowIdx = -1;
                    }
                }

            }

            ToolSeparator {
                orientation: Qt.Horizontal
                Layout.fillWidth: true
            }

            RowLayout {
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.maximumWidth: parent.width
                visible: {
                    var row = selectedRow();
                    if (row && row.hasOwnProperty("valueOnDevice"))
                        return true;
                    else
                        return false;
                }

                SettingsTabComponents.SettingsPane {
                    id: settingsPane
                }

            }

        }

    }

}
