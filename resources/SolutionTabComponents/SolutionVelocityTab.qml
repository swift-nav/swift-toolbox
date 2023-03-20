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
    id: solutionVelocityTab

    property variant labels: ["Horizontal", "Vertical"]
    property variant lines: []
    property variant colors: []
    property variant available_units: ["m/s", "mph", "kph"]

    SolutionVelocityPoints {
        id: solutionVelocityPoints

        function update() {
            solution_velocity_model.fill_console_points(solutionVelocityPoints);
            if (!solutionVelocityPoints.points.length)
                return;
            solutionVelocityArea.visible = true;
            var points = solutionVelocityPoints.points;
            if (colors != solutionVelocityPoints.colors) {
                colors = solutionVelocityPoints.colors;
                for (var idx in colors) {
                    if (lineLegendRepeaterRows.itemAt(idx))
                        lineLegendRepeaterRows.itemAt(idx).children[0].color = colors[idx];
                }
            }
            if (available_units != solutionVelocityPoints.available_units)
                available_units = solutionVelocityPoints.available_units;
            let commonChart = Constants.commonChart;
            if (!lines.length) {
                const tempLines = [];
                for (var idx in labels) {
                    var line = solutionVelocityChart.createSeries(ChartView.SeriesTypeLine, Constants.solutionVelocity.labels[idx], solutionVelocityXAxis);
                    line.color = colors[idx];
                    line.width = commonChart.lineWidth;
                    line.axisYRight = solutionVelocityYAxis;
                    line.useOpenGL = Globals.useOpenGL;
                    tempLines.push(line);
                }
                lines = lines.concat(tempLines);
            }
            solutionVelocityPoints.fill_series(lines);
            var last = points[0][points[0].length - 1];
            solutionVelocityXAxis.min = last.x - Constants.solutionVelocity.xAxisMinOffsetFromMaxSeconds;
            solutionVelocityXAxis.max = last.x;
            if (solutionVelocityYAxis.min != solutionVelocityPoints.min_ || solutionVelocityYAxis.max != solutionVelocityPoints.max_) {
                solutionVelocityYAxis.min = solutionVelocityPoints.min_;
                solutionVelocityYAxis.max = solutionVelocityPoints.max_;
            }
        }
    }

    ColumnLayout {
        id: solutionVelocityArea

        anchors.fill: parent
        spacing: 0

        RowLayout {
            Layout.alignment: Qt.AlignHCenter

            Label {
                text: "Display Units:"
            }

            SwiftComboBox {
                id: solutionVelocitySelectedUnit

                Component.onCompleted: {
                    solutionVelocitySelectedUnit.indicator.width = Constants.solutionVelocity.unitDropdownWidth / 3;
                }
                Layout.alignment: Qt.AlignCenter | Qt.AlignTop
                Layout.preferredWidth: Constants.solutionVelocity.unitDropdownWidth
                model: available_units
                onCurrentIndexChanged: {
                    if (!lines.length)
                        return;
                    backend_request_broker.solution_velocity_unit(available_units[currentIndex]);
                }
            }
        }

        ChartView {
            id: solutionVelocityChart

            Layout.alignment: Qt.AlignBottom
            Layout.fillHeight: true
            Layout.fillWidth: true
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: Globals.useAntiAliasing

            margins {
                top: 0
                bottom: 0
                left: 0
                right: 0
            }

            Rectangle {
                id: lineLegend

                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.bottom: solutionVelocityChart.bottom
                anchors.left: solutionVelocityChart.left
                anchors.bottomMargin: Constants.solutionVelocity.legendBottomMargin
                anchors.leftMargin: Constants.solutionVelocity.legendLeftMargin
                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: lineLegend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: Constants.solutionVelocity.labels

                        Row {
                            spacing: Constants.solutionVelocity.legendLabelSpacing
                            Component.onCompleted: {
                                for (var idx in colors) {
                                    if (lineLegendRepeaterRows.itemAt(idx))
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = colors[idx];
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
                                font.pixelSize: Constants.mediumPixelSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }
                        }
                    }
                }
            }

            SwiftValueAxis {
                id: solutionVelocityXAxis

                titleText: Constants.solutionVelocity.xAxisTitleText
                labelFormat: "%d"
            }

            SwiftValueAxis {
                id: solutionVelocityYAxis

                titleText: solutionVelocitySelectedUnit.currentText
            }

            LineSeries {
                name: "emptySeries"
                axisYRight: solutionVelocityYAxis
                axisX: solutionVelocityXAxis
                color: "transparent"
                useOpenGL: Globals.useOpenGL

                XYPoint {
                    x: 0
                    y: 0
                }

                XYPoint {
                    x: 1
                    y: 1
                }
            }
        }
    }
}
