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
    Repeater {
        model: ["Url", "Lat", "Lon", "Alt", "Username", "Password"]
        Rectangle {
            TextField {
                id: textField
                placeholderText: modelData
                wrapMode: Text.Wrap
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
                selectByMouse: true
                anchors.centerIn: parent
                anchors.verticalCenterOffset: 5
                onEditingFinished: {
                    console.log(text);
                }
                validator: stringValidator
            }
        }
    }
}
