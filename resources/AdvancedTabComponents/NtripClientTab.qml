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
