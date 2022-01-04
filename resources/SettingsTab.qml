import "BaseComponents"
import "Constants"
import Qt.labs.platform 1.1 as LabsPlatform
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.15
import "SettingsTabComponents" as SettingsTabComponents
import SwiftConsole 1.0

MainTab {
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

    SplitView {
        anchors.fill: parent
        anchors.margins: 5
        orientation: Qt.Horizontal

        SettingsTabComponents.SettingsTable {
            id: settingsTable

            SplitView.minimumWidth: Constants.settingsTable.minimumWidth
            onSelectedRowIdxChanged: {
                if (!!selectedRow())
                    settingsPane.selectedRow = selectedRow();

            }
        }

        ColumnLayout {
            SplitView.minimumWidth: parent.width * 0.55
            spacing: 0

            GridLayout {
                property int colWidth: Math.max(Constants.settingsTab.buttonIconWidth, ((parent.width / (columns)) - columnSpacing * (columns)))
                property int buttonPadding: 3

                Layout.fillWidth: true
                rowSpacing: 0
                columnSpacing: 2
                columns: 5
                rows: 2

                Button {
                    Layout.columnSpan: 1
                    Layout.rowSpan: 1
                    Layout.preferredWidth: parent.colWidth
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    padding: parent.buttonPadding
                    text: "Save to\nDevice"
                    icon.source: Constants.icons.savePath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    display: AbstractButton.TextUnderIcon
                    flat: true
                    onClicked: data_model.settings_save_request()
                }

                Button {
                    Layout.columnSpan: 1
                    Layout.rowSpan: 1
                    Layout.preferredWidth: parent.colWidth
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    padding: parent.buttonPadding
                    text: "Export to\nfile"
                    icon.source: Constants.icons.exportPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    display: AbstractButton.TextUnderIcon
                    flat: true
                    onClicked: exportDialog.visible = true
                }

                Button {
                    Layout.columnSpan: 1
                    Layout.rowSpan: 1
                    Layout.preferredWidth: parent.colWidth
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    padding: parent.buttonPadding
                    text: "Import from\nFile"
                    icon.source: Constants.icons.importPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    display: AbstractButton.TextUnderIcon
                    flat: true
                    onClicked: importDialog.visible = true
                }

                Button {
                    Layout.columnSpan: 1
                    Layout.rowSpan: 1
                    Layout.preferredWidth: parent.colWidth
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    padding: parent.buttonPadding
                    text: "Reset to\nDefaults"
                    icon.source: Constants.icons.warningPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    display: AbstractButton.TextUnderIcon
                    flat: true
                    onClicked: resetDialog.visible = true
                }

                Button {
                    id: autoSurveyButton

                    property bool buttonEnabled: (selectedRowField("group") === "surveyed_position")

                    Layout.columnSpan: 1
                    Layout.rowSpan: 1
                    Layout.preferredWidth: parent.colWidth
                    Layout.preferredHeight: refreshButton.height
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    ToolTip.text: "Select element under \'surveyed_position\' group to enable."
                    ToolTip.visible: !buttonEnabled && hovered
                    background.visible: buttonEnabled
                    padding: parent.buttonPadding
                    text: "Auto Survey\n"
                    opacity: buttonEnabled ? 1 : 0.5
                    icon.source: Constants.icons.centerOnButtonUrl
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    display: AbstractButton.TextUnderIcon
                    flat: true
                    onClicked: {
                        if (buttonEnabled)
                            autoSurveyDialog.visible = true;

                    }
                }

                Button {
                    id: refreshButton

                    Layout.columnSpan: 1
                    Layout.rowSpan: 1
                    Layout.preferredWidth: parent.colWidth
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    padding: parent.buttonPadding
                    text: "Refresh from\ndevice"
                    icon.source: Constants.icons.refreshPath
                    icon.width: Constants.settingsTab.buttonIconWidth
                    icon.height: Constants.settingsTab.buttonIconHeight
                    display: AbstractButton.TextUnderIcon
                    flat: true
                    onClicked: data_model.settings_refresh()
                }

                SmallCheckBox {
                    Layout.columnSpan: 1
                    Layout.rowSpan: 1
                    Layout.preferredWidth: parent.colWidth
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    padding: parent.buttonPadding
                    bottomPadding: refreshButton.bottomPadding
                    text: "SHOW ADVANCED SETTINGS"
                    font.pointSize: refreshButton.font.pointSize
                    font.family: Constants.fontFamily
                    font.bold: false
                    onClicked: {
                        if (this.enabled) {
                            this.enabled = false;
                            settingsTable.showExpert = checked;
                            settingsTable.selectedRowIdx = -1;
                            this.enabled = true;
                        }
                    }
                }

            }

            ToolSeparator {
                orientation: Qt.Horizontal
                Layout.fillWidth: true
            }

            SettingsTabComponents.SettingsPane {
                id: settingsPane

                Layout.rightMargin: 10
                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                visible: {
                    var row = settingsTab.selectedRow();
                    if (row && row.hasOwnProperty("valueOnDevice"))
                        return true;
                    else
                        return false;
                }
            }

            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                visible: !settingsPane.visible
            }

        }

    }

}
