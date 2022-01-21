import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    id: settingsPane

    property var selectedRow
    property var floatValidator
    property var intValidator
    property var stringValidator
    property alias textFieldFocus: valOnDevice.item

    function shouldShowField(name) {
        if (!selectedRow)
            return false;

        return !!selectedRow[name];
    }

    function selectedRowField(name) {
        if (!selectedRow)
            return "";

        return selectedRow[name] || "";
    }

    clip: true

    Flickable {
        anchors.fill: parent
        contentHeight: grid.height

        GridLayout {
            id: grid

            property int colWidth: this.width / columns
            property int colWidthLabel: colWidth
            property int colWidthField: this.width - colWidth
            property int smallRowHeight: Constants.settingsTab.paneSmallRowHeight
            property int labelColumnWidth: this.width

            columns: 7
            rows: 12
            rowSpacing: 1
            height: parent.parent.height + Constants.settingsTab.paneScrollBufferHeight
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.leftMargin: 5
            anchors.rightMargin: 2 * scrollBar.width

            Loader {
                property string _title: "Name"
                property string _fieldName: "name"

                Layout.rowSpan: 1
                Layout.columnSpan: 1
                Layout.preferredWidth: parent.colWidthLabel
                Layout.preferredHeight: parent.smallRowHeight
                sourceComponent: settingRowLabel
            }

            Loader {
                property string _fieldName: "name"

                sourceComponent: settingRowText
                Layout.rowSpan: 1
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: parent.smallRowHeight
            }

            Loader {
                property string _title: "Value"
                property string _fieldName: "valueOnDevice"

                Layout.rowSpan: 1
                Layout.columnSpan: 1
                Layout.preferredWidth: parent.colWidthLabel
                Layout.preferredHeight: parent.smallRowHeight
                sourceComponent: settingRowLabel
            }

            Loader {
                id: valOnDevice

                property string _fieldName: "valueOnDevice"

                Layout.rowSpan: 1
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: parent.smallRowHeight
                Layout.alignment: Qt.AlignVCenter
                sourceComponent: {
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
                states: [
                    State {
                        name: "longTextField"
                        when: valOnDevice.item instanceof TextField

                        PropertyChanges {
                            target: valOnDevice
                            Layout.preferredHeight: 3 * parent.smallRowHeight
                        }

                    },
                    State {
                        name: "enumOrBool"
                        when: valOnDevice.item instanceof ComboBox

                        PropertyChanges {
                            target: valOnDevice
                            Layout.preferredHeight: 2 * parent.smallRowHeight
                        }

                    },
                    State {
                        name: "label"
                        when: valOnDevice.item instanceof Rectangle

                        PropertyChanges {
                            target: valOnDevice
                            Layout.preferredHeight: parent.smallRowHeight
                        }

                    }
                ]
            }

            Loader {
                property string _title: "Units"
                property string _fieldName: "units"

                visible: !!selectedRowField(_fieldName)
                Layout.rowSpan: 1
                Layout.columnSpan: 1
                Layout.preferredWidth: parent.colWidthLabel
                Layout.preferredHeight: parent.smallRowHeight
                sourceComponent: settingRowLabel
            }

            Loader {
                property string _fieldName: "units"

                visible: !!selectedRowField(_fieldName)
                sourceComponent: settingRowText
                Layout.rowSpan: 1
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: parent.smallRowHeight
            }

            Loader {
                property string _title: "Setting Type"
                property string _fieldName: "type"

                visible: !!selectedRowField(_fieldName)
                Layout.rowSpan: 1
                Layout.columnSpan: 1
                Layout.preferredWidth: parent.colWidthLabel
                Layout.preferredHeight: parent.smallRowHeight
                sourceComponent: settingRowLabel
            }

            Loader {
                property string _fieldName: "type"

                visible: !!selectedRowField(_fieldName)
                sourceComponent: settingRowText
                Layout.rowSpan: 1
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: parent.smallRowHeight
            }

            Loader {
                property string _title: "Default Value"
                property string _fieldName: "defaultValue"

                Layout.rowSpan: 2
                Layout.columnSpan: 1
                Layout.preferredWidth: parent.colWidthLabel
                Layout.preferredHeight: parent.smallRowHeight
                sourceComponent: settingRowLabel
            }

            Loader {
                property string _fieldName: "defaultValue"

                sourceComponent: settingRowText
                Layout.rowSpan: 2
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: parent.smallRowHeight
            }

            Loader {
                property string _title: "Description"
                property string _fieldName: "description"

                visible: !!selectedRowField(_fieldName)
                Layout.rowSpan: 2
                Layout.columnSpan: 1
                Layout.preferredWidth: parent.colWidthLabel
                Layout.preferredHeight: parent.smallRowHeight
                sourceComponent: settingRowLabel
            }

            Loader {
                property string _fieldName: "description"

                visible: !!selectedRowField(_fieldName)
                Layout.rowSpan: 2
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: parent.smallRowHeight
                sourceComponent: settingRowText
            }

            Loader {
                property string _title: "Notes"
                property string _fieldName: "notes"

                visible: !!selectedRowField(_fieldName)
                Layout.columnSpan: 1
                Layout.rowSpan: parent.rows - 7
                Layout.preferredHeight: Math.max(1, parent.height - 7 * parent.smallRowHeight)
                Layout.preferredWidth: parent.colWidthLabel
                sourceComponent: settingRowLabel
            }

            Loader {
                id: notes

                property string _fieldName: "notes"

                visible: !!selectedRowField(_fieldName)
                Layout.columnSpan: parent.columns - 1
                Layout.rowSpan: parent.rows - 8
                Layout.preferredHeight: Math.max(1, parent.height - 8 * parent.smallRowHeight)
                Layout.preferredWidth: parent.colWidthField
                sourceComponent: settingRowText
            }

            Loader {
                property string _fieldName: "notes"

                visible: !notes.visible
                Layout.columnSpan: parent.columns
                Layout.rowSpan: parent.rows - 8
                Layout.fillHeight: true
                Layout.fillWidth: true
                sourceComponent: emptyRow
            }

        }

        ScrollBar.vertical: ScrollBar {
            id: scrollBar

            policy: ScrollBar.AlwaysOn
        }

    }

    Component {
        id: emptyRow

        Rectangle {
            anchors.fill: parent
        }

    }

    Component {
        id: settingRowLabel

        Label {
            text: _title + ":"
            font.bold: true
            horizontalAlignment: Text.AlignRight
        }

    }

    Component {
        id: settingRowText

        Rectangle {
            anchors.fill: parent

            Label {
                text: selectedRowField(_fieldName)
                anchors.fill: parent
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
            wrapMode: Text.Wrap
            font.family: Constants.genericTable.fontFamily
            font.pointSize: Constants.largePointSize
            selectByMouse: true
            anchors.centerIn: parent
            anchors.verticalCenterOffset: 5
            onEditingFinished: {
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
                if (currentIndex != -1 && selectedRowField("valueOnDevice") != model[currentIndex])
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
                if (currentIndex != -1 && selectedRowField("valueOnDevice") != model[currentIndex])
                    data_model.settings_write_request(selectedRowField("group"), selectedRowField("name"), model[currentIndex]);

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
