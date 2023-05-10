import "../BaseComponents"
import "../Constants"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: ntripClientTab

    property var floatValidator
    property var intValidator
    property var stringValidator

    floatValidator: DoubleValidator {
    }

    intValidator: IntValidator {
    }

    stringValidator: RegExpValidator {
    }
    RowLayout {
        ColumnLayout {
            Repeater {
                model: ["Url", "Username", "Password"]
                RowLayout {
                    width: 500
                    height: 30
                    Label {
                        text: modelData + ": "
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }
                    TextField {
                        id: textField
                        width: 200
                        placeholderText: modelData
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                        selectByMouse: true
                        Layout.alignment: Qt.AlignVCenter| Qt.AlignRight
                        validator: stringValidator
                    }
                }
            }

            Repeater {
                model: ["Lat", "Lon", "Alt"]
                RowLayout {
                    width: 500
                    height: 30
                    visible: staticRadio.checked
                    Label {
                        text: modelData + ": "
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }
                    TextField {
                        id: textField
                        width: 200
                        placeholderText: modelData
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                        selectByMouse: true
                        Layout.alignment: Qt.AlignVCenter| Qt.AlignRight
                        validator: stringValidator
                    }
                }
            }
        }
        ColumnLayout {
            RadioButton {
                checked: true
                text: "Dynamic"
                ToolTip.visible: hovered
                ToolTip.text: "Allow automatically fetching position from device"
            }

            RadioButton {
                id: staticRadio
                text: "Static"
                ToolTip.visible: hovered
                ToolTip.text: "Allow user input position"
            }
        }
        ColumnLayout {
            RowLayout {
                SwiftButton {
                    invertColor: true
                    icon.width: 10
                    icon.height: 10
                    icon.source: Constants.icons.playPath
                    icon.color: Constants.materialGrey
                    ToolTip.visible: hovered
                    ToolTip.text: "Start"
                    onClicked: {
                        console.log("hello");
                    }
                }
                SwiftButton {
                    invertColor: true
                    icon.width: 10
                    icon.height: 10
                    icon.source: Constants.icons.pauseButtonUrl
                    icon.color: Constants.materialGrey
                    ToolTip.visible: hovered
                    ToolTip.text: "Stop"
                    onClicked: {
                        console.log("hello");
                    }
                }
            }
        }
    }
}
