import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: settingsPane

    property var selectedRow
    property var floatValidator
    property var intValidator
    property var stringValidator

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

    GridLayout {
        columns: 2
        rowSpacing: 4
        width: parent.width

        Loader {
            property string _title: "Name"
            property string _fieldName: "name"

            Layout.alignment: Qt.AlignRight
            sourceComponent: settingRowLabel
        }

        Loader {
            property string _fieldName: "name"

            sourceComponent: settingRowText
        }

        Loader {
            property string _title: "Value"
            property string _fieldName: "valueOnDevice"

            Layout.alignment: Qt.AlignRight
            sourceComponent: settingRowLabel
        }

        Loader {
            id: valOnDevice

            property string _fieldName: "valueOnDevice"

            Layout.fillWidth: false
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
                    name: "long text field"
                    when: {
                        if (valOnDevice.item instanceof TextField) {
                            var textLen = valOnDevice.item.font.pointSize * valOnDevice.item.length;
                            return (textLen >= 150);
                        } else {
                            return false;
                        }
                    }

                    PropertyChanges {
                        target: valOnDevice
                        Layout.fillWidth: true
                    }

                }
            ]
        }

        Loader {
            property string _title: "Units"
            property string _fieldName: "units"

            visible: !!selectedRowField(_fieldName)
            Layout.alignment: Qt.AlignRight
            sourceComponent: settingRowLabel
        }

        Loader {
            property string _fieldName: "units"

            visible: !!selectedRowField(_fieldName)
            sourceComponent: settingRowText
        }

        Loader {
            property string _title: "Setting Type"
            property string _fieldName: "type"

            visible: !!selectedRowField(_fieldName)
            Layout.alignment: Qt.AlignRight
            sourceComponent: settingRowLabel
        }

        Loader {
            property string _fieldName: "type"

            visible: !!selectedRowField(_fieldName)
            sourceComponent: settingRowText
        }

        Loader {
            property string _title: "Default Value"
            property string _fieldName: "defaultValue"

            Layout.alignment: Qt.AlignRight
            sourceComponent: settingRowLabel
        }

        Loader {
            property string _fieldName: "defaultValue"

            sourceComponent: settingRowText
        }

        Loader {
            property string _title: "Description"
            property string _fieldName: "description"

            visible: !!selectedRowField(_fieldName)
            Layout.alignment: Qt.AlignRight
            sourceComponent: settingRowLabel
        }

        Loader {
            property string _fieldName: "description"

            visible: !!selectedRowField(_fieldName)
            sourceComponent: settingRowText
        }

        Loader {
            property string _title: "Notes"
            property string _fieldName: "notes"

            visible: !!selectedRowField(_fieldName)
            Layout.alignment: Qt.AlignRight
            sourceComponent: settingRowLabel
        }

        Loader {
            property string _fieldName: "notes"

            visible: !!selectedRowField(_fieldName)
            sourceComponent: settingRowText
        }

    }

    Component {
        id: settingRowLabel

        Label {
            text: _title + ":"
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
            wrapMode: Text.Wrap
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

    floatValidator: DoubleValidator {
    }

    intValidator: IntValidator {
    }

    stringValidator: RegularExpressionValidator {
    }

}
