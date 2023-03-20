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
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: fusionStatusFlags

    property variant lines: []
    property string last_gnsspos: "UNKNOWN"
    property string last_gnssvel: "UNKNOWN"
    property string last_wheelticks: "UNKNOWN"
    property string last_speed: "UNKNOWN"
    property string last_nhc: "UNKNOWN"
    property string last_zerovel: "UNKNOWN"

    Component.onCompleted: {
    }

    FusionStatusFlagsData {
        id: fusionStatusFlagsData

        function updateStatus(status, unknown, warning, ok) {
            unknown.visible = status == "UNKNOWN";
            warning.visible = status != "UNKNOWN" && status == "WARNING";
            ok.visible = status != "UNKNOWN" && status != "WARNING";
        }

        function update() {
            fusion_engine_flags_model.fill_console_points(fusionStatusFlagsData);
            if (!fusionStatusFlagsData.gnsspos)
                return;
            fusionStatusFlagsArea.visible = true;
            var gnsspos = fusionStatusFlagsData.gnsspos;
            if (gnsspos != last_gnsspos) {
                updateStatus(gnsspos, gnssposUnknown, gnssposWarning, gnssposOk)
                last_gnsspos = gnsspos;
            }
            var gnssvel = fusionStatusFlagsData.gnssvel;
            if (gnssvel != last_gnssvel) {
                updateStatus(gnssvel, gnssvelUnknown, gnssvelWarning, gnssvelOk)
                last_gnssvel = gnssvel;
            }
            var wheelticks = fusionStatusFlagsData.wheelticks;
            if (wheelticks != last_wheelticks) {
                updateStatus(wheelticks, wheelticksUnknown, wheelticksWarning, wheelticksOk)
                last_wheelticks = wheelticks;
            }
            var speed = fusionStatusFlagsData.speed;
            if (speed != last_speed) {
                updateStatus(speed, speedUnknown, speedWarning, speedOk)
                last_speed = speed;
            }
            var nhc = fusionStatusFlagsData.nhc;
            if (nhc != last_nhc) {
                updateStatus(nhc, nhcUnknown, nhcWarning, nhcOk)
                last_nhc = nhc;
            }
            var zerovel = fusionStatusFlagsData.zerovel;
            if (zerovel != last_zerovel) {
                updateStatus(zerovel, zerovelUnknown, zerovelWarning, zerovelOk)
                last_zerovel = zerovel;
            }
        }
    }

    GroupBox {
        anchors.centerIn: parent

        GridLayout {
            id: fusionStatusFlagsArea

            columns: 2

            Label {
                text: Constants.advancedImu.insStatusLabels[0]
                font.pixelSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: gnssposUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: gnssposWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: gnssposOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[1]
                font.pixelSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: gnssvelUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: gnssvelWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: gnssvelOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[2]
                font.pixelSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: wheelticksUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: wheelticksWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: wheelticksOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[3]
                font.pixelSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: speedUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: speedWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: speedOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[4]
                font.pixelSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: nhcUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: nhcWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: nhcOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Label {
                text: Constants.advancedImu.insStatusLabels[5]
                font.pixelSize: Constants.fusionStatusFlags.labelFontSize
            }

            UnknownStatus {
                id: zerovelUnknown

                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            WarningStatus {
                id: zerovelWarning

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            OkStatus {
                id: zerovelOk

                visible: false
                Layout.preferredWidth: Constants.fusionStatusFlags.fusionStatusWidth
            }

            Item {
                Layout.fillWidth: true
            }
        }

        label: Label {
            text: Constants.fusionStatusFlags.title
            font.pixelSize: Constants.fusionStatusFlags.titleFontSize
        }
    }
}
