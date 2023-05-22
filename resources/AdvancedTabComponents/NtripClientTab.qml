import "../BaseComponents"
import "../Constants"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: ntripClientTab

    property bool connected: false
    property var floatValidator
    property var intValidator
    property var stringValidator
    property string url
    property string mountpoint
    property string port

    RowLayout {
        anchors.fill: parent

        ColumnLayout {
            RowLayout {
                ComboBox {
                    Layout.fillWidth: true
                    editable: true
                    validator: stringValidator
                    onAccepted: url = currentText
                    model: ListModel {
                        ListElement { 
                            text: "na.l1l2.skylark.swiftnav.com"
                        }

                        ListElement { 
                            text: "na.l1l5.skylark.swiftnav.com" 
                        }

                        ListElement { 
                            text: "eu.l1l2.skylark.swiftnav.com" 
                        }

                        ListElement { 
                            text: "eu.l1l5.skylark.swiftnav.com" 
                        }

                        ListElement { 
                            text: "ap.l1l2.skylark.swiftnav.com" 
                        }

                        ListElement { 
                            text: "ap.l1l5.skylark.swiftnav.com" 
                        }

                    }

                }

                ComboBox {
                    Layout.fillWidth: true
                    editable: true
                    validator: intValidator
                    onAccepted: port = currentText
                    model: ListModel {
                        ListElement { 
                            text: "2101" 
                        }

                        ListElement { 
                            text: "2102" 
                        }

                    }

                }

                ComboBox {
                    Layout.fillWidth: true
                    editable: true
                    validator: stringValidator
                    onAccepted: mountpoint = currentText

                    model: ListModel {
                        ListElement { 
                            text: "OSR" 
                        }

                        ListElement { 
                            text: "MSM5" 
                        }

                    }

                }

            }

            Repeater {
                id: generalRepeater

                model: ["Username", "Password", "GGA Period"]

                RowLayout {
                    height: 30

                    Label {
                        text: modelData + ": "
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }

                    TextField {
                        width: 400
                        Layout.fillWidth: true
                        text: {
                            if (modelData == "GGA Period")
                                return "10";

                            return "";
                        }
                        placeholderText: modelData
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                        selectByMouse: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                        validator: {
                            if (modelData == "GGA Period")
                                return intValidator;

                            return stringValidator;
                        }
                        readOnly: connected
                    }

                }

            }

            Repeater {
                id: positionRepeater

                model: ["Lat", "Lon", "Alt"]

                RowLayout {
                    height: 30
                    visible: staticRadio.checked

                    Label {
                        text: modelData + ": "
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }

                    TextField {
                        id: textField

                        width: 400
                        Layout.fillWidth: true
                        placeholderText: modelData
                        font.family: Constants.genericTable.fontFamily
                        font.pixelSize: Constants.largePixelSize
                        selectByMouse: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                        validator: floatValidator
                        text: {
                            if (modelData == "Lat")
                                return "37.77101999622968";

                            if (modelData == "Lon")
                                return "-122.40315159140708";

                            if (modelData == "Alt")
                                return "-5.549358852471994";

                            return "";
                        }
                        readOnly: connected
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
                enabled: !connected
            }

            RadioButton {
                id: staticRadio

                text: "Static"
                ToolTip.visible: hovered
                ToolTip.text: "Allow user input position"
                enabled: !connected
            }

        }

        ColumnLayout {
            Label {
                id: inputErrorLabel

                visible: false
                text: ""
                font.family: Constants.genericTable.fontFamily
                font.pixelSize: Constants.largePixelSize
                color: "red"
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
                    enabled: !connected
                    onClicked: {
                        if (!url) {
                            inputErrorLabel.text = "URL is not provided!";
                            inputErrorLabel.visible = true;
                            return ;
                        }
                        let address = url + ":" + port + "/" + mountpoint;
                        let username = generalRepeater.itemAt(1).children[1].text;
                        let password = generalRepeater.itemAt(2).children[1].text;
                        let ggaPeriod = generalRepeater.itemAt(3).children[1].text;
                        if (!ggaPeriod) {
                            inputErrorLabel.text = "GGA Period is not provided!";
                            inputErrorLabel.visible = true;
                            return ;
                        }
                        let lat = null;
                        let lon = null;
                        let alt = null;
                        if (staticRadio.checked) {
                            lat = positionRepeater.itemAt(0).children[1].text;
                            lon = positionRepeater.itemAt(1).children[1].text;
                            alt = positionRepeater.itemAt(2).children[1].text;
                            if (!lat || !lon || !alt) {
                                inputErrorLabel.text = "Position missing!";
                                inputErrorLabel.visible = true;
                                return ;
                            }
                        }
                        backend_request_broker.ntrip_connect(address, username, password, ggaPeriod, lat, lon, alt);
                        connected = true;
                        inputErrorLabel.visible = false;
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
                    enabled: connected
                    onClicked: {
                        backend_request_broker.ntrip_disconnect();
                        connected = false;
                        inputErrorLabel.visible = false;
                    }
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
