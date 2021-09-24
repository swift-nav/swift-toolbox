import "../Constants"
import QtCharts 2.2
import QtQuick 2.15
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: trackingSkyPlotTab

    property var series: []
    property var checkVisibility: [false, true, true, true, true, true, true]

    TrackingSkyPlotPoints {
        id: trackingSkyPlotPoints
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: Constants.trackingSkyPlot.checkboxSpacing

        PolarChartView {
            id: trackingSkyPlotChart

            Layout.fillWidth: true
            Layout.fillHeight: true
            legend.visible: false
            antialiasing: true

            CategoryAxis {
                id: compassAxis

                min: Constants.trackingSkyPlot.axisAngularMin
                max: Constants.trackingSkyPlot.axisAngularMax
                labelsColor: Constants.commonChart.labelsColor
                labelsPosition: CategoryAxis.AxisLabelsPositionOnValue
                startValue: Constants.trackingSkyPlot.axisAngularMin

                CategoryRange {
                    label: "N"
                    endValue: Constants.trackingSkyPlot.axisAngularMin
                }

                CategoryRange {
                    label: "E"
                    endValue: Constants.trackingSkyPlot.axisAngularMax / 4
                }

                CategoryRange {
                    label: "S"
                    endValue: Constants.trackingSkyPlot.axisAngularMax / 2
                }

                CategoryRange {
                    label: "W"
                    endValue: 3 * Constants.trackingSkyPlot.axisAngularMax / 4
                }

            }

            ValueAxis {
                id: axisAngular

                min: Constants.trackingSkyPlot.axisAngularMin
                max: Constants.trackingSkyPlot.axisAngularMax
                tickCount: Constants.trackingSkyPlot.axisAngularTickCount
                labelsVisible: false
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
            }

            CategoryAxis {
                id: axisRadial

                labelFormat: "%d°"
                min: Constants.trackingSkyPlot.axisRadialMin
                max: Constants.trackingSkyPlot.axisRadialMax
                tickCount: Constants.trackingSkyPlot.axisRadialTickCount
                labelsPosition: CategoryAxis.AxisLabelsPositionOnValue
                labelsColor: Constants.commonChart.labelsColor
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor

                CategoryRange {
                    label: "0°"
                    endValue: Constants.trackingSkyPlot.axisRadialMax
                }

                CategoryRange {
                    label: "30°"
                    endValue: 2 * Constants.trackingSkyPlot.axisRadialMax / 3
                }

                CategoryRange {
                    label: "60°"
                    endValue: Constants.trackingSkyPlot.axisRadialMax / 3
                }

                CategoryRange {
                    label: "90°"
                    endValue: Constants.trackingSkyPlot.axisRadialMin
                }

            }

            ScatterSeries {
                id: series_none

                axisAngular: compassAxis
                axisRadial: axisRadial
            }

            Rectangle {
                id: legend

                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.top: trackingSkyPlotChart.top
                anchors.right: trackingSkyPlotChart.right
                anchors.topMargin: Constants.trackingSkyPlot.legendTopMargin
                anchors.rightMargin: Constants.trackingSkyPlot.legendRightMargin
                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                visible: checkVisibility[0]

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: legend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: Constants.trackingSkyPlot.scatterLabels

                        Row {
                            Text {
                                id: marker

                                text: "●"
                                font.pointSize: Constants.smallPointSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                color: Constants.trackingSkyPlot.colors[index]
                            }

                            Text {
                                id: label

                                text: modelData
                                font.pointSize: Constants.smallPointSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: Constants.trackingSkyPlot.checkboxHeight
            Layout.alignment: Qt.AlignBottom
            Layout.leftMargin: Constants.trackingSkyPlot.checkboxMargins
            Layout.rightMargin: Constants.trackingSkyPlot.checkboxMargins
            spacing: Constants.trackingSkyPlot.checkboxSpacing

            Text {
                Layout.fillWidth: true
                text: "Enabled with SBP message MSG_SV_AZ_EL (0x0097 | 151), * indicates satellite is being tracked"
                font.family: Constants.genericTable.fontFamily
                font.pointSize: Constants.largePointSize
            }

            Row {
                id: trackingSignalsCheckboxes

                Layout.fillWidth: true
                Layout.preferredHeight: Constants.trackingSkyPlot.checkboxHeight
                Layout.alignment: Qt.AlignBottom
                spacing: Constants.trackingSkyPlot.checkboxSpacing

                CheckBox {
                    checked: false
                    text: "Show Legend"
                    font.family: Constants.genericTable.fontFamily
                    font.pointSize: Constants.largePointSize
                    height: Constants.trackingSkyPlot.checkboxHeight
                    width: Constants.trackingSkyPlot.checkboxLegendWidth
                    onClicked: {
                        checkVisibility[0] = checked;
                        legend.visible = checkVisibility[0];
                    }
                }

                Repeater {
                    id: trackingSignalsCheckbox

                    model: Constants.trackingSkyPlot.scatterLabels

                    CheckBox {
                        checked: true
                        text: modelData
                        font.family: Constants.genericTable.fontFamily
                        font.pointSize: Constants.largePointSize
                        height: Constants.trackingSkyPlot.checkboxHeight
                        width: Constants.trackingSkyPlot.checkboxLabelWidth
                        onClicked: {
                            checkVisibility[index + 1] = checked;
                        }
                    }

                }

            }

        }

    }

    Timer {
        interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
        running: true
        repeat: true
        onTriggered: {
            if (!trackingTab.visible)
                return ;

            tracking_sky_plot_model.fill_console_points(trackingSkyPlotPoints);
            if (!trackingSkyPlotPoints.sats.length)
                return ;

            if (!series.length) {
                for (var idx in Constants.trackingSkyPlot.scatterLabels) {
                    var scatter = trackingSkyPlotChart.createSeries(ChartView.SeriesTypeScatter, Constants.trackingSkyPlot.scatterLabels[idx], axisAngular, axisRadial);
                    scatter.color = Constants.trackingSkyPlot.colors[idx];
                    scatter.markerSize = Constants.trackingSkyPlot.markerSize;
                    scatter.useOpenGL = Globals.useOpenGL;
                    series.push(scatter);
                }
            }
            trackingSkyPlotPoints.fill_series(series);
            let labels = trackingSkyPlotPoints.labels;
            for (var idx in labels) {
                var kdx = parseInt(idx) + 1;
                if (!checkVisibility[kdx]) {
                    series[idx].clear();
                    continue;
                }
                for (var jdx in labels[idx]) {
                    var pose = trackingSkyPlotChart.mapToPosition(series[idx].at(jdx), series[idx]);
                    let qmlStr = "import QtQuick 2.15; Text {color: 'black'; text: '" + labels[idx][jdx] + "'; width: 20; height: 20; x: " + pose.x + "; y: " + pose.y + ";}";
                    var obj = Qt.createQmlObject(qmlStr, trackingSkyPlotChart, labels[idx][jdx]);
                    obj.destroy(Utils.hzToMilliseconds(Globals.currentRefreshRate));
                }
            }
        }
    }

}
