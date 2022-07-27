import "../BaseComponents"
import "../Constants"
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: trackingSkyPlotTab

    property alias all_series: trackingSkyPlotPoints.all_series
    property var series: []
    property bool labelsVisible: labelsVisibleCheckBox.checked
    property bool polarChartWidthChanging: false

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
            antialiasing: Globals.useAntiAliasing
            backgroundColor: "transparent"
            onWidthChanged: {
                polarChartWidthChanging = true;
            }
            onHeightChanged: {
                polarChartWidthChanging = true;
            }

            margins {
                bottom: Constants.trackingSkyPlot.directionLabelOffset
                left: 0
                right: 0
                top: Constants.trackingSkyPlot.directionLabelOffset
            }

            Label {
                text: "N"
                font.pixelSize: Constants.trackingSkyPlot.directionLabelFontSize
                font.bold: true
                x: trackingSkyPlotChart.plotArea.x + trackingSkyPlotChart.plotArea.width / 2 - Constants.trackingSkyPlot.directionLabelFontSize / 2
                y: trackingSkyPlotChart.plotArea.y - Constants.trackingSkyPlot.directionLabelOffset
            }

            Label {
                // This label just for testing whether Label is honoring the font it has set.
                // set it visible to test Label font. If this label is entirely ontop of the other
                // N label such that you cannot tell there are two N labels, then Label is not
                // honoring the font set in the label.
                visible: false
                text: "N"
                font.family: "Roboto"
                font.pixelSize: Constants.trackingSkyPlot.directionLabelFontSize
                font.bold: true
                x: trackingSkyPlotChart.plotArea.x + trackingSkyPlotChart.plotArea.width / 2 - width / 2
                y: trackingSkyPlotChart.plotArea.y - Constants.trackingSkyPlot.directionLabelOffset
            }

            Label {
                text: "E"
                font.pixelSize: Constants.trackingSkyPlot.directionLabelFontSize
                font.bold: true
                x: trackingSkyPlotChart.plotArea.x + trackingSkyPlotChart.plotArea.width + Constants.trackingSkyPlot.directionLabelOffset / 3
                y: trackingSkyPlotChart.plotArea.y + trackingSkyPlotChart.plotArea.height / 2 - height / 2
            }

            Label {
                text: "S"
                font.pixelSize: Constants.trackingSkyPlot.directionLabelFontSize
                font.bold: true
                x: trackingSkyPlotChart.plotArea.x + trackingSkyPlotChart.plotArea.width / 2 - width / 2
                y: trackingSkyPlotChart.plotArea.y + trackingSkyPlotChart.plotArea.height + Constants.trackingSkyPlot.directionLabelOffset / 5
            }

            Label {
                text: "W"
                font.pixelSize: Constants.trackingSkyPlot.directionLabelFontSize
                font.bold: true
                x: trackingSkyPlotChart.plotArea.x - Constants.trackingSkyPlot.directionLabelOffset
                y: trackingSkyPlotChart.plotArea.y + trackingSkyPlotChart.plotArea.height / 2 - height / 2
            }

            SwiftValueAxis {
                id: axisAngular

                min: Constants.trackingSkyPlot.axisAngularMin
                max: Constants.trackingSkyPlot.axisAngularMax
                tickCount: Constants.trackingSkyPlot.axisAngularTickCount
                labelsVisible: false
            }

            SwiftCategoryAxis {
                id: axisRadial

                labelFormat: "%d°"
                min: Constants.trackingSkyPlot.axisRadialMin
                max: Constants.trackingSkyPlot.axisRadialMax
                tickCount: Constants.trackingSkyPlot.axisRadialTickCount

                CategoryRange {
                    label: " "
                    endValue: Constants.trackingSkyPlot.axisRadialMax
                }

                CategoryRange {
                    label: "0°"
                    endValue: Constants.trackingSkyPlot.axisRadialMax - 0.1
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
                name: "emptySeries"
                axisYRight: axisRadial
                axisX: axisAngular
                color: "transparent"
                useOpenGL: Globals.useOpenGL

                XYPoint {
                    x: 0
                    y: 0
                }
            }

            Rectangle {
                id: legend

                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                radius: Constants.commonLegend.borderRadius
                anchors.top: trackingSkyPlotChart.top
                anchors.right: trackingSkyPlotChart.right
                anchors.topMargin: Constants.trackingSkyPlot.legendTopMargin
                anchors.rightMargin: Constants.trackingSkyPlot.legendRightMargin
                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                visible: showLegendCheckBox.checked && all_series.filter(x => {
                        return x.visible;
                    }).length > 0

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: legend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: all_series

                        Row {
                            visible: modelData.visible

                            Label {
                                id: marker

                                text: "●"
                                font.pixelSize: Constants.smallPixelSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                color: Constants.trackingSkyPlot.colors[index]
                            }

                            Label {
                                id: label

                                text: modelData.name
                                font.pixelSize: Constants.smallPixelSize
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }
                        }
                    }
                }
            }
        }

        Label {
            Layout.alignment: Qt.AlignHCenter
            text: "Enabled with SBP message MSG_SV_AZ_EL (0x0097 | 151), * indicates satellite is being tracked"
        }

        Row {
            Layout.alignment: Qt.AlignHCenter
            spacing: Constants.trackingSkyPlot.checkboxSpacing

            SmallCheckBox {
                id: showLegendCheckBox

                checked: false
                text: "Show Legend"
            }

            SmallCheckBox {
                id: labelsVisibleCheckBox

                checked: false
                text: "Show Labels"
                onCheckedChanged: {
                    updateTimer.restart();
                }
            }

            Repeater {
                model: all_series

                SmallCheckBox {
                    checked: true
                    text: modelData.name
                    onCheckedChanged: {
                        modelData.visible = checked;
                        updateTimer.restart();
                    }
                }
            }
        }
    }

    Timer {
        id: updateTimer

        interval: Utils.hzToMilliseconds(Constants.staticTimerSlowIntervalRate)
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: {
            if (!trackingSkyPlotTab.visible)
                return;
            let labels = trackingSkyPlotPoints.labels;
            if (all_series.length < labels.length) {
                for (var i = all_series.length; i < labels.length; i++) {
                    var series = trackingSkyPlotChart.createSeries(ChartView.SeriesTypeScatter, Constants.trackingSkyPlot.scatterLabels[i], axisAngular, axisRadial);
                    series.color = Constants.trackingSkyPlot.colors[i];
                    series.markerSize = Constants.trackingSkyPlot.markerSize;
                    series.useOpenGL = Globals.useOpenGL;
                    trackingSkyPlotPoints.addSeries(series);
                }
            }
            trackingSkyPlotPoints.fill_all_series();
            if (polarChartWidthChanging) {
                polarChartWidthChanging = false;
                return;
            }
            for (var idx in labels) {
                if (!all_series[idx].visible)
                    continue;
                if (labelsVisible) {
                    for (var jdx in labels[idx]) {
                        var pose = trackingSkyPlotChart.mapToPosition(all_series[idx].at(jdx), all_series[idx]);
                        let qmlStr = "import QtQuick.Controls; Label {color: 'black'; text: '" + labels[idx][jdx] + "'; visible: (!polarChartWidthChanging && labelsVisible && all_series[" + idx + "].visible); width: 20; height: 20; x: " + pose.x + "; y: " + pose.y + ";}";
                        var obj = Qt.createQmlObject(qmlStr, trackingSkyPlotChart, labels[idx][jdx]);
                        obj.destroy(Utils.hzToMilliseconds(Constants.staticTimerSlowIntervalRate));
                    }
                }
            }
        }
    }
}
