import "../Constants"
import "../BaseComponents"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: advancedSpectrumAnalyzerTab

    property variant line: null

    AdvancedSpectrumAnalyzerPoints {
        id: advancedSpectrumAnalyzerPoints
    }

    ColumnLayout {
        id: advancedSpectrumAnalyzerArea

        anchors.fill: parent
        visible: false

        ChartView {
            id: advancedSpectrumAnalyzerChart

            Layout.fillWidth: true
            Layout.fillHeight: true
            margins {
                top: 0
                bottom: 0
                left: 0
                right: 0
            }
            title: Constants.advancedSpectrumAnalyzer.title
            titleColor: Constants.commonChart.titleColor
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: true
            titleFont: Constants.commonChart.titleFont

            SwiftValueAxis {
                id: advancedSpectrumAnalyzerXAxis

                titleText: Constants.advancedSpectrumAnalyzer.xAxisTitleText
                tickInterval: Constants.advancedSpectrumAnalyzer.xAxisTickCount
                tickType: ValueAxis.TicksDynamic

            }

            SwiftValueAxis {
                id: advancedSpectrumAnalyzerYAxis

                titleText: Constants.advancedSpectrumAnalyzer.yAxisTitleText
                tickInterval: Constants.advancedSpectrumAnalyzer.yAxisTickCount
                tickType: ValueAxis.TicksDynamic

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
            Layout.preferredHeight: Constants.advancedSpectrumAnalyzer.dropdownRowHeight
            Layout.alignment: Qt.AlignBottom
        }

    }

}
