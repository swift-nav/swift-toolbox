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
import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: advancedImuTab

    property variant lines: []

    AdvancedImuPoints {
        id: advancedImuPoints

        function update() {
            advanced_imu_model.fill_console_points(advancedImuPoints);
            if (!advancedImuPoints.points.length)
                return;
            var points = advancedImuPoints.points;
            advancedImuArea.visible = true;
            let commonChart = Constants.commonChart;
            let advancedImu = Constants.advancedImu;
            if (!lines.length) {
                const tempLines = [];
                for (var idx in points) {
                    var line = advancedImuChart.createSeries(ChartView.SeriesTypeLine, idx, advancedImuXAxis);
                    line.color = advancedImu.lineColors[idx];
                    line.width = commonChart.lineWidth;
                    line.axisYRight = advancedImuYAxis;
                    line.useOpenGL = Globals.useOpenGL;
                    tempLines.push(line);
                }
                lines = lines.concat(tempLines);
            }
            let fieldDatum = advancedImuPoints.fields_data;
            imuTempText.text = `${fieldDatum[0].toFixed(2)} C`;
            imuConfText.text = `0x${fieldDatum[1].toString(16).padStart(2, "0")}`;
            rmsAccXText.text = `${fieldDatum[2].toFixed(2)} g`;
            rmsAccYText.text = `${fieldDatum[3].toFixed(2)} g`;
            rmsAccZText.text = `${fieldDatum[4].toFixed(2)} g`;
            advancedImuPoints.fill_series(lines);
        }
    }

    ColumnLayout {
        id: advancedImuArea

        anchors.fill: parent
        visible: true

        ChartView {
            id: advancedImuChart

            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
            visible: true
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: Globals.useAntiAliasing
            titleFont: Constants.commonChart.titleFont

            margins {
                top: 0
                bottom: 0
                left: 0
                right: 0
            }

            Rectangle {
                id: lineLegend

                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.bottom: advancedImuChart.bottom
                anchors.left: advancedImuChart.left
                anchors.bottomMargin: Constants.advancedImu.legendBottomMargin
                anchors.leftMargin: Constants.advancedImu.legendLeftMargin

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: lineLegend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: Constants.advancedImu.legendLabels

                        Row {
                            Component.onCompleted: {
                                let imuLineColors = Constants.advancedImu.lineColors;
                                for (var idx in imuLineColors) {
                                    let item = lineLegendRepeaterRows.itemAt(idx);
                                    if (item)
                                        item.children[0].color = imuLineColors[idx];
                                }
                            }

                            Rectangle {
                                id: marker

                                width: Constants.commonLegend.markerWidth
                                height: Constants.commonLegend.markerHeight
                                anchors.verticalCenter: parent.verticalCenter
                            }

                            Label {
                                id: label

                                text: modelData
                                font.pixelSize: Constants.smallPixelSize
                                font.bold: true
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }
                        }
                    }
                }
            }

            SwiftValueAxis {
                id: advancedImuXAxis

                tickInterval: Constants.advancedImu.xAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
                min: Constants.advancedImu.xAxisMin
                max: Constants.advancedImu.xAxisMax
                reverse: true
            }

            SwiftValueAxis {
                id: advancedImuYAxis

                tickInterval: Constants.advancedImu.yAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
                min: Constants.advancedImu.yAxisMin
                max: Constants.advancedImu.yAxisMax
            }

            ScatterSeries {
                name: "emptySeries"
                axisYRight: advancedImuYAxis
                axisX: advancedImuXAxis
                color: "transparent"
                useOpenGL: Globals.useOpenGL
                markerSize: 0.1

                XYPoint {
                    x: 0
                    y: 0
                }
            }
        }

        RowLayout {
            id: textDataRow

            property real preferredChildWidths: advancedImuArea.width / 15

            visible: true
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.advancedImu.urlBarHeight
            Layout.alignment: Qt.AlignBottom

            Label {
                text: Constants.advancedImu.textDataLabels[0]
            }

            Rectangle {
                Layout.preferredWidth: parent.preferredChildWidths
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: imuTempText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 C"
                }
            }

            Label {
                text: Constants.advancedImu.textDataLabels[1]
            }

            Rectangle {
                Layout.preferredWidth: parent.preferredChildWidths
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: imuConfText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0x00"
                }
            }

            Label {
                text: Constants.advancedImu.textDataLabels[2]
            }

            Rectangle {
                Layout.preferredWidth: parent.preferredChildWidths
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccXText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 g"
                }
            }

            Label {
                text: Constants.advancedImu.textDataLabels[3]
            }

            Rectangle {
                Layout.preferredWidth: parent.preferredChildWidths
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccYText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 g"
                }
            }

            Label {
                text: Constants.advancedImu.textDataLabels[4]
            }

            Rectangle {
                Layout.preferredWidth: parent.preferredChildWidths
                Layout.preferredHeight: Constants.advancedImu.textDataBarHeight
                Layout.alignment: Qt.AlignVCenter
                border.width: Constants.advancedImu.textDataBarBorderWidth

                Label {
                    id: rmsAccZText

                    clip: true
                    anchors.fill: parent
                    anchors.margins: Constants.advancedImu.textDataBarMargin
                    font.pixelSize: Constants.mediumPixelSize
                    text: "0.00 g"
                }
            }

            Item {
                Layout.fillWidth: true
                Layout.preferredHeight: Constants.advancedMagnetometer.suggestionTextRowHeight

                Label {
                    text: Constants.advancedMagnetometer.suggestionText
                    font.italic: true
                    antialiasing: Globals.useAntiAliasing
                    anchors.horizontalCenter: parent.horizontalCenter
                }
            }
        }
    }
}
