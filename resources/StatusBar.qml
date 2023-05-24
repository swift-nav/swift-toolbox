import "Constants"
import QtCharts 2.2
import QtQuick 2.5
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Rectangle {
    property string position: Constants.statusBar.defaultValue
    property string rtk: Constants.statusBar.defaultValue
    property string ins: Constants.statusBar.defaultValue
    property int satellites: -1
    property real correctionAge: -1
    property string antennaStatus: Constants.statusBar.defaultValue
    property real dataRate: 0
    property bool solidConnection: false
    property string title: ""
    property string ntrip: "off"
    property int verticalPadding: Constants.statusBar.verticalPadding

    color: Constants.swiftOrange
    border.width: Constants.statusBar.borderWidth
    border.color: Constants.statusBar.borderColor
    implicitWidth: rowLayout.implicitWidth
    implicitHeight: rowLayout.implicitHeight

    StatusBarData {
        id: statusBarData
    }

    RowLayout {
        id: rowLayout

        anchors.left: parent.left
        anchors.leftMargin: Constants.statusBar.leftMargin
        spacing: Constants.statusBar.spacing

        Repeater {
            model: [{
                "labelText": Constants.statusBar.posLabel,
                "valueText": position
            }, {
                "labelText": Constants.statusBar.rtkLabel,
                "valueText": rtk
            }, {
                "labelText": Constants.statusBar.insLabel,
                "valueText": ins
            }, {
                "labelText": Constants.statusBar.satsLabel,
                "valueText": satellites < 0 ? Constants.statusBar.defaultValue : satellites
            }, {
                "labelText": Constants.statusBar.corrAgeLabel,
                "valueText": correctionAge <= 0 ? Constants.statusBar.defaultValue : Utils.padFloat(correctionAge, 1, 1) + " s"
            }, {
                "labelText": Constants.statusBar.antennaLabel,
                "valueText": antennaStatus
            }, {
                "labelText": Constants.statusBar.ntripLabel,
                "valueText": ntrip
            }]

            RowLayout {
                spacing: Constants.statusBar.keyValueSpacing

                Label {
                    visible: modelData.valueText
                    topPadding: Constants.statusBar.verticalPadding
                    bottomPadding: Constants.statusBar.verticalPadding
                    text: modelData.labelText
                    color: Constants.statusBar.textColor
                    font.pixelSize: Constants.statusBar.textPixelSize
                }

                Label {
                    id: statusBarPos

                    visible: modelData.valueText
                    Layout.minimumWidth: Constants.statusBar.valueMinimumWidth
                    topPadding: Constants.statusBar.verticalPadding
                    bottomPadding: Constants.statusBar.verticalPadding
                    text: modelData.valueText
                    color: Constants.statusBar.textColor
                    font.pixelSize: Constants.statusBar.textPixelSize
                    font.bold: true
                }

            }

        }

    }

    Timer {
        interval: Utils.hzToMilliseconds(Constants.staticTimerIntervalRate)
        running: true
        repeat: true
        onTriggered: {
            status_bar_model.fill_data(statusBarData);
            if (statusBarData.title) {
                position = statusBarData.pos;
                rtk = statusBarData.rtk;
                ins = statusBarData.ins;
                satellites = statusBarData.solid_connection ? statusBarData.sats : -1;
                correctionAge = statusBarData.corr_age;
                antennaStatus = statusBarData.antenna_status;
                dataRate = statusBarData.data_rate;
                solidConnection = statusBarData.solid_connection;
                title = statusBarData.title;
                ntrip = statusBarData.ntrip_display;
            }
        }
    }

}
