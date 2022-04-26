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
            title: Constants.advancedSpectrumAnalyzer.title
            titleColor: Constants.commonChart.titleColor
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
                        return ;

                    advanced_spectrum_analyzer_model.fill_console_points(advancedSpectrumAnalyzerPoints);
                    if (!advancedSpectrumAnalyzerPoints.points.length)
                        return ;

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
