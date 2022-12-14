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
    id: advancedSpectrumAnalyzerTab

    property variant line: null

    AdvancedSpectrumAnalyzerPoints {
        id: advancedSpectrumAnalyzerPoints
    }

    ColumnLayout {
        id: advancedSpectrumAnalyzerArea

        anchors.fill: parent
        visible: true
        spacing: 0

        ChartView {
            id: advancedSpectrumAnalyzerChart

            Layout.fillWidth: true
            Layout.fillHeight: true
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

            SwiftValueAxis {
                id: advancedSpectrumAnalyzerXAxis

                titleText: Constants.advancedSpectrumAnalyzer.xAxisTitleText
                tickInterval: Constants.advancedSpectrumAnalyzer.xAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
            }

            SwiftValueAxis {
                id: advancedSpectrumAnalyzerYAxis

                titleText: Constants.advancedSpectrumAnalyzer.yAxisTitleText
                tickInterval: Constants.advancedSpectrumAnalyzer.yAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
            }

            LineSeries {
                name: "emptySeries"
                axisYRight: advancedSpectrumAnalyzerYAxis
                axisX: advancedSpectrumAnalyzerXAxis
                color: "transparent"
                useOpenGL: Globals.useOpenGL

                XYPoint {
                    x: -1
                    y: -1
                }

                XYPoint {
                    x: 1
                    y: 1
                }
            }

            Timer {
                id: advancedSpectrumAnalyzerTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedSpectrumAnalyzerTab.visible)
                        return;
                    advanced_spectrum_analyzer_model.fill_console_points(advancedSpectrumAnalyzerPoints);
                    if (!advancedSpectrumAnalyzerPoints.points.length)
                        return;
                    if (!line) {
                        var line_ = advancedSpectrumAnalyzerChart.createSeries(ChartView.SeriesTypeLine, 0, advancedSpectrumAnalyzerXAxis);
                        line_.color = Constants.advancedSpectrumAnalyzer.lineColors[0];
                        line_.width = Constants.commonChart.lineWidth;
                        line_.axisYRight = advancedSpectrumAnalyzerYAxis;
                        line_.useOpenGL = Globals.useOpenGL;
                        line = line_;
                    }
                    advancedSpectrumAnalyzerArea.visible = true;
                    advancedSpectrumAnalyzerYAxis.min = advancedSpectrumAnalyzerPoints.ymin;
                    advancedSpectrumAnalyzerYAxis.max = advancedSpectrumAnalyzerPoints.ymax;
                    advancedSpectrumAnalyzerXAxis.min = advancedSpectrumAnalyzerPoints.xmin;
                    advancedSpectrumAnalyzerXAxis.max = advancedSpectrumAnalyzerPoints.xmax;
                    channelSelectionRow.dropdownIdx = advancedSpectrumAnalyzerPoints.channel;
                    advancedSpectrumAnalyzerPoints.fill_series(line);
                }
            }
        }

        AdvancedSpectrumAnalyzerTabChannelBar {
            id: channelSelectionRow

            Layout.fillWidth: true
            Layout.maximumHeight: Constants.advancedSpectrumAnalyzer.dropdownRowHeight
        }
    }
}
