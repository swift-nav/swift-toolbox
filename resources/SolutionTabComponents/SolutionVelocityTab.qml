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

            ComboBox {
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

            Timer {
                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!solutionVelocityTab.visible)
                        return;
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
                    if (!lines.length) {
                        for (var idx in labels) {
                            var line = solutionVelocityChart.createSeries(ChartView.SeriesTypeLine, Constants.solutionVelocity.labels[idx], solutionVelocityXAxis);
                            line.color = colors[idx];
                            line.width = Constants.commonChart.lineWidth;
                            line.axisYRight = solutionVelocityYAxis;
                            line.useOpenGL = Globals.useOpenGL;
                            lines.push(line);
                        }
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
        }
    }
}
