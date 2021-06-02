import "../Constants"
import QtCharts 2.3
import QtQuick 2.6
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    // property variant labels: []
    // property variant colors: []
    // property variant check_labels: []
    // property variant check_visibility: []

    id: advancedInsTab

    property variant lines: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    AdvancedInsPoints {
        id: advancedInsPoints
    }

    Rectangle {
        // GridLayout {
        //     id: advancedInsCheckboxes
        //     columns: parent.width / Constants.advancedIns.checkBoxPreferredWidth
        //     anchors.horizontalCenter: advancedInsChart.horizontalCenter
        //     anchors.top: advancedInsChart.bottom
        //     Repeater {
        //         id: advancedInsCheckbox
        //         model: check_labels
        //         Column {
        //             CheckBox {
        //                 checked: true
        //                 text: modelData
        //                 verticalPadding: Constants.advancedIns.checkBoxVerticalPadding
        //                 onClicked: {
        //                     check_visibility[index] = checked;
        //                     if (index == 0) {
        //                         lineLegend.visible = !lineLegend.visible;
        //                         return ;
        //                     }
        //                     var labels_not_visible = [];
        //                     for (var idx in check_visibility) {
        //                         if (!check_visibility[idx])
        //                             labels_not_visible.push(check_labels[idx]);
        //                     }
        //                     data_model.advanced_ins_check_visibility(labels_not_visible);
        //                 }
        //                 Component.onCompleted: {
        //                     check_visibility.push(checked);
        //                 }
        //             }
        //         }
        //     }
        // }

        id: advancedInsArea

        width: parent.width
        height: parent.height

        ChartView {
            id: advancedInsChart

            visible: false
            title: Constants.advancedIns.title
            titleColor: Constants.advancedIns.titleColor
            width: parent.width
            height: parent.height
            // height: parent.height - advancedInsCheckboxes.height
            anchors.top: parent.top
            backgroundColor: Constants.commonChart.backgroundColor
            plotAreaColor: Constants.commonChart.areaColor
            legend.visible: false
            antialiasing: true
            Component.onCompleted: {
            }

            titleFont {
                pointSize: Constants.advancedIns.titlePointSize
                bold: true
            }

            Rectangle {
                id: lineLegend

                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                anchors.bottom: advancedInsChart.bottom
                anchors.left: advancedInsChart.left
                anchors.bottomMargin: Constants.advancedIns.legendBottomMargin
                anchors.leftMargin: Constants.advancedIns.legendLeftMargin

                Column {
                    id: lineLegendRepeater

                    padding: Constants.commonLegend.padding
                    anchors.bottom: lineLegend.bottom

                    Repeater {
                        id: lineLegendRepeaterRows

                        model: Constants.advancedIns.legendLabels

                        Row {
                            Component.onCompleted: {
                                for (var idx in Constants.advancedIns.lineColors) {
                                    if (lineLegendRepeaterRows.itemAt(idx))
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = Constants.advancedIns.lineColors[idx];

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
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

            ValueAxis {
                id: advancedInsXAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickInterval: Constants.advancedIns.xAxisTickCount
                tickType: ValueAxis.TicksDynamic

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            ValueAxis {
                id: advancedInsYAxis

                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickInterval: Constants.advancedIns.yAxisTickCount
                tickType: ValueAxis.TicksDynamic

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            Timer {
                id: advancedInsTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!advancedTab.visible)
                        return ;

                    advanced_ins_model.fill_console_points(advancedInsPoints);
                    if (!advancedInsPoints.points.length)
                        return ;

                    var points = advancedInsPoints.points;
                    // colors = advancedInsPoints.colors;
                    // labels = advancedInsPoints.labels;
                    advancedInsChart.visible = true;
                    // check_labels = advancedInsPoints.check_labels;
                    if (!lines.length) {
                        for (var idx in advancedInsPoints.points) {
                            var line = advancedInsChart.createSeries(ChartView.SeriesTypeLine, idx, advancedInsXAxis);
                            // line.color = colors[idx];
                            line.width = Constants.commonChart.lineWidth;
                            line.axisYRight = advancedInsYAxis;
                            line.useOpenGL = Globals.useOpenGL;
                            lines.push(line);
                        }
                    }
                    advancedInsPoints.fill_series(lines);
                    advancedInsXAxis.min = 0;
                    advancedInsXAxis.max = 200;
                    if (advancedInsYAxis.min != advancedInsPoints.min_) {
                        advancedInsYAxis.min = advancedInsPoints.min_;
                        advancedInsYAxis.max = advancedInsPoints.max_;
                    }
                }
            }

        }

    }

}
