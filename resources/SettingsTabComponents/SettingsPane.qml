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
import "../BaseComponents"
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Rectangle {
    id: settingsPane

    property var floatValidator
    property var intValidator
    property var stringValidator

    function shouldShowField(name) {
        let row = settingsPane.selectedRow();
        if (!row)
            return false;
        return !!row[name];
    }

    function selectedRowField(name) {
        let row = settingsPane.selectedRow();
        if (!row)
            return "";
        return row[name] || "";
    }

    function isLongTextField(name) {
        return selectedRowField(name).length > 70;
    }

    clip: true

    Flickable {
        anchors.fill: parent
        contentHeight: grid.height
        boundsBehavior: Flickable.StopAtBounds

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
                            Layout.preferredHeight: isLongTextField(_fieldName) ? 5 * parent.smallRowHeight : 3 * parent.smallRowHeight
                        }
                    },
                    State {
                        name: "enumOrBool"
                        when: valOnDevice.item instanceof SwiftComboBox

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
                Layout.preferredHeight: isLongTextField(_fieldName) ? 4 * parent.smallRowHeight : parent.smallRowHeight
                sourceComponent: settingRowLabel
            }

            Loader {
                property string _fieldName: "defaultValue"

                sourceComponent: settingRowText
                Layout.rowSpan: 2
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: isLongTextField(_fieldName) ? 4 * parent.smallRowHeight : parent.smallRowHeight
            }

            Loader {
                property string _title: "Description"
                property string _fieldName: "description"

                visible: !!selectedRowField(_fieldName)
                Layout.rowSpan: parents.rows - 8
                Layout.columnSpan: 1
                Layout.preferredWidth: parent.colWidthLabel
                Layout.preferredHeight: Math.max(1, parent.height - 8 * parent.smallRowHeight)
                sourceComponent: settingRowLabel
            }

            Loader {
                property string _fieldName: "description"

                visible: !!selectedRowField(_fieldName)
                Layout.rowSpan: parents.rows - 8
                Layout.columnSpan: parent.columns - 1
                Layout.preferredWidth: parent.colWidthField
                Layout.preferredHeight: Math.max(1, parent.height - 8 * parent.smallRowHeight)
                sourceComponent: settingRowText
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

            MouseArea {
                anchors.fill: parent
                onClicked: parent.forceActiveFocus()
            }
        }
    }

    Component {
        id: settingRowText

        Rectangle {
            anchors.fill: parent

            TextEdit {
                text: {
                    if (_fieldName == "description") {
                        let desc = selectedRowField("description");
                        let notes = selectedRowField("notes");
                        if (notes)
                            return desc + "\n\nNotes:\n" + notes;

                        return notes;
                    }
                    return selectedRowField(_fieldName);
                }
                anchors.fill: parent
                wrapMode: Text.WordWrap
                readOnly: true
                selectByMouse: true
                selectionColor: Constants.swiftOrange
                onSelectedTextChanged: {
                    if (selectedText.length > 0)
                        Globals.copyClipboard = selectedText;
                }

                font {
                    family: Constants.fontFamily
                    pixelSize: Constants.largePixelSize
                }
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
            font.pixelSize: Constants.largePixelSize
            selectByMouse: true
            anchors.centerIn: parent
            anchors.verticalCenterOffset: 5
            onEditingFinished: {
                let isNumericField = settingType === "float" || settingType === "double";
                backend_request_broker.settings_write_request(settingGroup, settingName, isNumericField ? text.replace(",", ".") : text);
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

        SwiftComboBox {
            model: ["True", "False"]
            currentIndex: model.indexOf(selectedRowField("valueOnDevice"))
            onActivated: {
                if (currentIndex != -1 && selectedRowField("valueOnDevice") != model[currentIndex])
                    backend_request_broker.settings_write_request(selectedRowField("group"), selectedRowField("name"), model[currentIndex]);
            }
        }
    }

    Component {
        id: settingRowEnum

        SwiftComboBox {
            model: selectedRowField("enumeratedPossibleValues").split(",")
            currentIndex: model.indexOf(selectedRowField("valueOnDevice"))
            onActivated: {
                if (currentIndex != -1 && selectedRowField("valueOnDevice") != model[currentIndex])
                    backend_request_broker.settings_write_request(selectedRowField("group"), selectedRowField("name"), model[currentIndex]);
            }
        }
    }

    floatValidator: RegularExpressionValidator {
        regularExpression: /[-+]?[0-9]*[.,]?[0-9]+/
    }

    intValidator: IntValidator {
    }

    stringValidator: RegularExpressionValidator {
    }
}
