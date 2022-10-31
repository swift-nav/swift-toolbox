import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: advancedMagnetometerTab

    property variant lines: []

    AdvancedMagnetometerPoints {
        id: advancedMagnetometerPoints
    }

    ColumnLayout {
        id: advancedMagnetometerArea

        anchors.fill: parent
        visible: true
        spacing: 0

        ChartView {
            id: advancedMagnetometerChart

            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.alignment: Qt.AlignTop
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
                                let magnetometer = Constants.advancedMagnetometer;
                                for (var idx in magnetometer.lineColors) {
                                    let item = lineLegendRepeaterRows.itemAt(idx);
                                    if (item)
                                        item.children[0].color = magnetometer.lineColors[idx];
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
                id: advancedMagnetometerXAxis

                min: Constants.advancedMagnetometer.xAxisMin
                max: Constants.advancedMagnetometer.xAxisMax
                tickInterval: Constants.advancedMagnetometer.xAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
                reverse: true
            }

            SwiftValueAxis {
                id: advancedMagnetometerYAxis

                tickInterval: Constants.advancedMagnetometer.yAxisTickCount
                tickType: ValueAxis.TicksDynamic
                labelFormat: "%d"
            }

            LineSeries {
                name: "emptySeries"
                axisYRight: advancedMagnetometerYAxis
                axisX: advancedMagnetometerXAxis
                color: "transparent"
                useOpenGL: Globals.useOpenGL

                XYPoint {
                    x: 0
                    y: -10
                }

                XYPoint {
                    x: 1
                    y: 10
                }
            }

            Timer {
                id: advancedMagnetometerTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedMagnetometerTab.visible)
                        return;
                    advanced_magnetometer_model.fill_console_points(advancedMagnetometerPoints);
                    let magnetometerPoints = advancedMagnetometerPoints.points;
                    if (!magnetometerPoints.length)
                        return;
                    var points = advancedMagnetometerPoints.points;
                    let magnetometer = Constants.advancedMagnetometer;
                    let commonChart = Constants.commonChart;
                    if (!lines.length) {
                        const tempLines = [];
                        for (var idx in magnetometerPoints) {
                            var line = advancedMagnetometerChart.createSeries(ChartView.SeriesTypeLine, idx, advancedMagnetometerXAxis);
                            line.color = magnetometer.lineColors[idx];
                            line.width = commonChart.lineWidth;
                            line.axisYRight = advancedMagnetometerYAxis;
                            line.useOpenGL = Globals.useOpenGL;
                            tempLines.push(line);
                        }
                        line = line.concat(tempLines);
                    }
                    advancedMagnetometerArea.visible = true;
                    advancedMagnetometerYAxis.min = advancedMagnetometerPoints.ymin - magnetometer.yAxisPadding;
                    advancedMagnetometerYAxis.max = advancedMagnetometerPoints.ymax + magnetometer.yAxisPadding;
                    advancedMagnetometerPoints.fill_series(lines);
                }
            }
        }

        Rectangle {
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
