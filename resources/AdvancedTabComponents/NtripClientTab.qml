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
                id: generalRepeater
                model: ["Url", "Username", "Password"]
                RowLayout {
                    width: 500
                    height: 30
                    Label {
                        text: modelData + ": "
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }
                    TextField {
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
                width: 500
                height: 30
                Label {
                    text: "Epoch: "
                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                }
                TextField {
                    id: epochField
                    width: 200
                    placeholderText: "Epoch"
                    font.family: Constants.genericTable.fontFamily
                    font.pixelSize: Constants.largePixelSize
                    selectByMouse: true
                    Layout.alignment: Qt.AlignVCenter| Qt.AlignRight
                    validator: floatValidator
                }
            }

            Repeater {
                id: positionRepeater
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
                        validator: floatValidator
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
                        let url = generalRepeater.itemAt(0).children[1].text;
                        let username = generalRepeater.itemAt(1).children[1].text;
                        let password = generalRepeater.itemAt(2).children[1].text;
                        let epoch = epochField.text;
                        let lat = null;
                        let lon = null;
                        let alt = null;
                        if (staticRadio.checked){
                            lat = positionRepeater.itemAt(0).children[1].text;
                            lon = positionRepeater.itemAt(1).children[1].text;
                            alt = positionRepeater.itemAt(2).children[1].text;
                        }
                        backend_request_broker.ntrip_connect(url, username, password, epoch, lat, lon, alt);
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
