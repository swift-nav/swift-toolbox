import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: advancedMagnetometerTab

    property variant lines: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    AdvancedMagnetometerPoints {
        id: advancedMagnetometerPoints
    }

    ColumnLayout {
        id: advancedMagnetometerArea

        width: parent.width
        height: parent.height

        ChartView {
            id: advancedMagnetometerChart

            visible: false
            title: Constants.advancedMagnetometer.title
            titleColor: Constants.advancedMagnetometer.titleColor
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
            backgroundColor: Constants.commonChart.backgroundColor
            plotAreaColor: Constants.commonChart.areaColor
            legend.visible: false
            antialiasing: true
            Component.onCompleted: {
            }

            titleFont {
                pointSize: Constants.advancedMagnetometer.titlePointSize
                bold: true
            }

            Rectangle {
                id: lineLegend

                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.bottom: advancedMagnetometerChart.bottom
                anchors.left: advancedMagnetometerChart.left
                anchors.bottomMargin: Constants.advancedMagnetometer.legendBottomMargin
                anchors.leftMargin: Constants.advancedMagnetometer.legendLeftMargin

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: lineLegend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: Constants.advancedMagnetometer.legendLabels

                        Row {
                            spacing: Constants.commonLegend.spacing
                            Component.onCompleted: {
                                for (var idx in Constants.advancedMagnetometer.lineColors) {
                                    if (lineLegendRepeaterRows.itemAt(idx))
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = Constants.advancedMagnetometer.lineColors[idx];

                                }
                            }

                            Rectangle {
                                id: marker

                                width: Constants.commonLegend.markerWidth
                                height: Constants.commonLegend.markerHeight
                                anchors.verticalCenter: parent.verticalCenter
                            }

                            Text {
                                id: label

                                text: modelData
                                font.pointSize: Constants.smallPointSize
                                font.bold: true
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

            ValueAxis {
                id: advancedMagnetometerXAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                min: Constants.advancedMagnetometer.xAxisMin
                max: Constants.advancedMagnetometer.xAxisMax
                tickInterval: Constants.advancedMagnetometer.xAxisTickCount
                tickType: ValueAxis.TicksDynamic

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            ValueAxis {
                id: advancedMagnetometerYAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickInterval: Constants.advancedMagnetometer.yAxisTickCount
                tickType: ValueAxis.TicksDynamic

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            Timer {
                id: advancedMagnetometerTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedTab.visible)
                        return ;

                    advancedMagnetometerChart.visible = true;
                    advanced_magnetometer_model.fill_console_points(advancedMagnetometerPoints);
                    if (!advancedMagnetometerPoints.points.length)
                        return ;

                    var points = advancedMagnetometerPoints.points;
                    if (!lines.length) {
                        for (var idx in advancedMagnetometerPoints.points) {
                            var line = advancedMagnetometerChart.createSeries(ChartView.SeriesTypeLine, idx, advancedMagnetometerXAxis);
                            line.color = Constants.advancedMagnetometer.lineColors[idx];
                            line.width = Constants.commonChart.lineWidth;
                            line.axisYRight = advancedMagnetometerYAxis;
                            line.useOpenGL = Globals.useOpenGL;
                            lines.push(line);
                        }
                    }
                    advancedMagnetometerYAxis.min = advancedMagnetometerPoints.ymin;
                    advancedMagnetometerYAxis.max = advancedMagnetometerPoints.ymax;
                    advancedMagnetometerPoints.fill_series(lines);
                }
            }

        }

    }

}
