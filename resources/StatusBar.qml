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
import "Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Rectangle {
    property string position: Constants.statusBar.defaultValue
    property string rtk_val: Constants.statusBar.defaultValue
    property string ins_val: Constants.statusBar.defaultValue
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

        function update() {
            status_bar_model.fill_data(statusBarData);
            if (statusBarData.title) {
                position = statusBarData.pos;
                rtk_val = statusBarData.rtk;
                ins_val = statusBarData.ins;
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
                    "valueText": rtk_val
                }, {
                    "labelText": Constants.statusBar.insLabel,
                    "valueText": ins_val
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
}
