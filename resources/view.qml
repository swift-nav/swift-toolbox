import "BaseComponents"
import "Constants"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

ApplicationWindow {
    id: trackingSignalsTab

    property alias all_series: trackingSignalsPoints.all_series
    property alias enabled_series: trackingSignalsPoints.enabled_series
    property alias check_labels: trackingSignalsPoints.check_labels
    property alias num_labels: trackingSignalsPoints.num_labels
    property variant check_visibility: []

    Material.accent: Constants.swiftOrange
    width: 1050
    minimumWidth: 1050
    height: 600
    minimumHeight: 600
    visible: true
    color: Constants.swiftWhite

    TrackingSignalsPoints {
        id: trackingSignalsPoints
    }

    ColumnLayout {
        id: trackingSignalsArea

        anchors.fill: parent
        spacing: 0

        ChartView {
            // Timer {

            id: trackingSignalsChart

            Layout.fillHeight: true
            Layout.fillWidth: true
            visible: all_series.length > 0
            title: Constants.trackingSignals.title
            titleFont: Constants.commonChart.titleFont
            titleColor: Constants.commonChart.titleColor
            plotAreaColor: Constants.commonChart.areaColor
            backgroundColor: "transparent"
            legend.visible: false
            antialiasing: true

            margins {
                top: 0
                bottom: 0
                left: 0
                right: 0
            }

            ChartLegend {
                x: Constants.trackingSignals.legendLeftMargin
                y: Constants.trackingSignals.legendTopMargin
                maximumHeight: parent.height - Constants.trackingSignals.legendTopMargin - Constants.trackingSignals.legendBottomMargin
                cellTextSample: Constants.trackingSignals.legendCellTextSample
                model: enabled_series
            }

            SwiftValueAxis {
                id: trackingSignalsXAxis

                titleText: Constants.trackingSignals.xAxisTitleText
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.xAxisTickInterval
                labelFormat: "%d"
            }

            SwiftValueAxis {
                id: trackingSignalsYAxis

                titleText: Constants.trackingSignals.yAxisTitleText
                max: Constants.trackingSignals.yAxisMax
                min: Constants.trackingSignals.snrThreshold
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.yAxisTickInterval
                labelFormat: "%d"
                titleFont: Constants.trackingSignals.yAxisTitleFont
            }

            Timer {
                id: trackingSignalsTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                triggeredOnStart: true
                onTriggered: {
                    if (!trackingSignalsTab.visible)
                        return ;

                    if (all_series.length < num_labels) {
                        for (var i = all_series.length; i < num_labels; i++) {
                            var series = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, trackingSignalsPoints.getLabel(i), trackingSignalsXAxis);
                            series.axisYRight = trackingSignalsYAxis;
                            series.width = Constants.commonChart.lineWidth;
                            series.useOpenGL = Globals.useOpenGL;
                            // Color will be set in Python with fill_all_series call.
                            trackingSignalsPoints.addSeries(series);
                        }
                    }
                    var series = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, "", trackingSignalsXAxis);
                    series.axisYRight = trackingSignalsYAxis;
                    series.width = Constants.commonChart.lineWidth;
                    series.useOpenGL = Globals.useOpenGL;
                    trackingSignalsPoints.addSeries(series);
                    trackingSignalsPoints.fill_all_series();
                    trackingSignalsChart.visible = true;
                    trackingSignalsXAxis.min = trackingSignalsPoints.xaxis_min;
                    trackingSignalsXAxis.max = trackingSignalsPoints.xaxis_max;
                }
            }

        }

    }

}
