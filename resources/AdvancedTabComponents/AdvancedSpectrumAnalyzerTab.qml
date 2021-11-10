import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Layouts
import SwiftConsole

Item {
    id: advancedSpectrumAnalyzerTab

    property variant line: null

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    AdvancedSpectrumAnalyzerPoints {
        id: advancedSpectrumAnalyzerPoints
    }

    ColumnLayout {
        id: advancedSpectrumAnalyzerArea

        width: parent.width
        height: parent.height

        ChartView {
            id: advancedSpectrumAnalyzerChart

            visible: false
            title: Constants.advancedSpectrumAnalyzer.title
            titleColor: Constants.advancedSpectrumAnalyzer.titleColor
            width: parent.width
            height: parent.height - Constants.advancedSpectrumAnalyzer.dropdownRowHeight
            backgroundColor: Constants.commonChart.backgroundColor
            plotAreaColor: Constants.commonChart.areaColor
            legend.visible: false
            antialiasing: true
            Component.onCompleted: {
            }

            titleFont {
                pointSize: Constants.advancedSpectrumAnalyzer.titlePointSize
                bold: true
            }

            ValueAxis {
                id: advancedSpectrumAnalyzerXAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                titleText: Constants.advancedSpectrumAnalyzer.xAxisTitleText
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickInterval: Constants.advancedSpectrumAnalyzer.xAxisTickCount
                tickType: ValueAxis.TicksDynamic

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            ValueAxis {
                id: advancedSpectrumAnalyzerYAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                titleText: Constants.advancedSpectrumAnalyzer.yAxisTitleText
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickInterval: Constants.advancedSpectrumAnalyzer.yAxisTickCount
                tickType: ValueAxis.TicksDynamic

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            Timer {
                id: advancedSpectrumAnalyzerTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedTab.visible)
                        return ;

                    advancedSpectrumAnalyzerChart.visible = true;
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
            Layout.preferredHeight: Constants.advancedSpectrumAnalyzer.dropdownRowHeight
            Layout.alignment: Qt.AlignBottom
        }

    }

}
