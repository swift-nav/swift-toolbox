import "../Constants"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: baselinePlot

    property variant cur_scatters: []
    property variant scatters: []
    property variant colors: []
    property real mouse_x: 0
    property real mouse_y: 0
    property real orig_n_max: 0
    property real orig_n_min: 0
    property real orig_e_max: 0
    property real orig_e_min: 0
    property bool is_moving: false


    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    BaselinePlotPoints {
        id: baselinePlotPoints
    }

    Rectangle {
        id: baselinePlotArea

        width: parent.width
        height: parent.height
        visible: false

        ColumnLayout {
            id: baselinePlotAreaRowLayout

            anchors.fill: parent
            spacing: Constants.baselinePlot.navBarSpacing

            ButtonGroup {
                id: baselineButtonGroup

                exclusive: false
            }

            RowLayout {
                Layout.alignment: Qt.AlignLeft
                Layout.leftMargin: Constants.baselinePlot.navBarMargin

                Button {
                    id: baselinePauseButton

                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "| |"
                    ToolTip.visible: hovered
                    ToolTip.text: "Pause"
                    checkable: true
                    onClicked: {
                        data_model.baseline_plot([baselineButtonGroup.buttons[4].checked, baselineButtonGroup.buttons[3].pressed, baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].checked, baselineButtonGroup.buttons[0].pressed]);
                    }
                }

                Button {
                    id: baselineClearButton

                    onPressed: data_model.baseline_plot([baselineButtonGroup.buttons[4].checked, baselineButtonGroup.buttons[3].pressed, baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].checked, baselineButtonGroup.buttons[0].pressed])
                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: " X "
                    ToolTip.visible: hovered
                    ToolTip.text: "Clear"
                }

                Button {
                    id: baselineZoomAllButton

                    onClicked: data_model.baseline_plot([baselineButtonGroup.buttons[4].checked, baselineButtonGroup.buttons[3].pressed, baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].checked, baselineButtonGroup.buttons[0].pressed])
                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "[ ]"
                    ToolTip.visible: hovered
                    ToolTip.text: "Zoom All"
                    checkable: true
                }

                Button {
                    id: baselineCenterButton

                    onClicked: data_model.baseline_plot([baselineButtonGroup.buttons[4].checked, baselineButtonGroup.buttons[3].pressed, baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].checked, baselineButtonGroup.buttons[0].pressed])
                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.navBarButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "(><)"
                    ToolTip.visible: hovered
                    ToolTip.text: "Center On Solution"
                    checkable: true
                }

                Button {
                    id: baselineResetFiltersButton

                    onPressed: data_model.baseline_plot([baselineButtonGroup.buttons[4].checked, baselineButtonGroup.buttons[3].pressed, baselineButtonGroup.buttons[2].checked, baselineButtonGroup.buttons[1].checked, baselineButtonGroup.buttons[0].pressed])
                    ButtonGroup.group: baselineButtonGroup
                    Layout.preferredWidth: Constants.baselinePlot.resetFiltersButtonWidth
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "Reset Filters"
                    ToolTip.visible: hovered
                    ToolTip.text: "Reset Filters"
                }

            }

            ChartView {
                id: baselinePlotChart

                Layout.preferredWidth: parent.width
                Layout.preferredHeight: parent.height - Constants.commonChart.heightOffset
                Layout.alignment: Qt.AlignBottom
                Layout.fillHeight: true
                backgroundColor: Constants.commonChart.backgroundColor
                plotAreaColor: Constants.commonChart.areaColor
                legend.visible: false
                antialiasing: true
                Component.onCompleted: {
                }

                Rectangle {
                    id: lineLegend

                    border.color: Constants.commonLegend.borderColor
                    border.width: Constants.commonLegend.borderWidth
                    anchors.top: baselinePlotChart.top
                    anchors.right: baselinePlotChart.right
                    anchors.topMargin: Constants.commonLegend.topMargin
                    anchors.rightMargin: Constants.commonLegend.rightMargin
                    implicitHeight: lineLegendRepeater.height
                    width: lineLegendRepeater.width

                    Column {
                        id: lineLegendRepeater

                        padding: Constants.commonLegend.padding
                        anchors.bottom: lineLegend.bottom

                        Repeater {
                            id: lineLegendRepeaterRows

                            model: Constants.baselinePlot.legendLabels

                            Row {
                                Text {
                                    id: marker

                                    text: "+ "
                                    font.pointSize: (Constants.mediumPointSize + Constants.commonLegend.markerPointSizeOffset)
                                    font.bold: true
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                }

                                Text {
                                    id: label

                                    text: modelData
                                    font.pointSize: Constants.mediumPointSize
                                    font.bold: true
                                    anchors.verticalCenter: parent.verticalCenter
                                    anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                                }

                            }

                        }

                    }

                }

                ValueAxis {
                    id: baselinePlotXAxis

                    titleText: Constants.baselinePlot.xAxisTitleText
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: Constants.commonChart.minorGridLineColor
                    gridLineColor: Constants.commonChart.gridLineColor
                    labelsColor: Constants.commonChart.labelsColor

                    labelsFont {
                        pointSize: Constants.mediumPointSize
                        bold: true
                    }

                }

                ValueAxis {
                    id: baselinePlotYAxis

                    titleText: Constants.baselinePlot.yAxisTitleText
                    gridVisible: true
                    lineVisible: true
                    minorGridVisible: true
                    minorGridLineColor: Constants.commonChart.minorGridLineColor
                    gridLineColor: Constants.commonChart.gridLineColor
                    labelsColor: Constants.commonChart.labelsColor

                    labelsFont {
                        pointSize: Constants.mediumPointSize
                        bold: true
                    }

                }
                // MouseArea{
                //     anchors.fill: parent
                //     drag.target: item
                //     drag.axis: Drag.XAndYAxis
                //     onClicked: resetPosition()
                // }
                MouseArea {
                    anchors.fill: baselinePlotChart
                    onDoubleClicked: baselinePlotChart.zoomReset();
                    onWheel: {
                        if (wheel.angleDelta.y > 0) {
                            baselinePlotChart.zoom(1.1);
                        } else {
                            baselinePlotChart.zoom(0.9);
                        }
                    }
                    onPositionChanged: {
                        if (pressed && !is_moving) {
                            is_moving = true
                            var current = baselinePlotChart.plotArea


                            var x_unit = Math.abs(baselinePlotXAxis.max - baselinePlotXAxis.min)/current.width
                            var y_unit = Math.abs(baselinePlotYAxis.max - baselinePlotYAxis.min)/current.height
                            
                            var delta_x = (mouse_x - mouseX)*x_unit
                            var delta_y = (mouse_y - mouseY)*y_unit
                            // var r = Qt.rect((delta_x + current.x), (delta_y + current.y), current.width, current.height)
                            // print(r)
                            // baselinePlotChart.plotArea = r
                            // baselinePlotChart.zoomReset()
                            // baselinePlotChart.zoomIn(r)
                            baselinePlotXAxis.max += delta_x
                            baselinePlotXAxis.min += delta_x
                            baselinePlotYAxis.max -= delta_y
                            baselinePlotYAxis.min -= delta_y
                            mouse_x = mouseX
                            mouse_y = mouseY
                            // print(delta_x, delta_y)
                            // print(baselinePlotChart.plotArea)
                            is_moving = false

                        }
                        
                    }
                    onPressed: {
                        mouse_x = mouseX
                        mouse_y = mouseY
                    }
                }
                


                Timer {
                    interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                    running: true
                    repeat: true
                    onTriggered: {
                        if (!baselineTab.visible)
                            return ;

                        baseline_plot_model.fill_console_points(baselinePlotPoints);
                        if (!baselinePlotPoints.points.length)
                            return ;
                        // print(baselinePlotChart.x)
                        // print(baselinePlotChart.scale)
                        // print(Object.keys(baselinePlotChart))
                        baselinePlotArea.visible = true;
                        var points = baselinePlotPoints.points;
                        if (!scatters.length || !cur_scatters.length) {
                            for (var idx in Constants.baselinePlot.legendLabels) {
                                if (lineLegendRepeaterRows.itemAt(idx))
                                    lineLegendRepeaterRows.itemAt(idx).children[0].color = Constants.baselinePlot.colors[idx];

                                var cur_scatter = baselinePlotChart.createSeries(ChartView.SeriesTypeScatter, Constants.baselinePlot.legendLabels[idx] + "cur-scatter", baselinePlotXAxis, baselinePlotYAxis);
                                cur_scatter.color = Constants.baselinePlot.colors[idx];
                                cur_scatter.markerSize = Constants.commonChart.currentSolutionMarkerSize;
                                cur_scatter.useOpenGL = Globals.useOpenGL;
                                if (idx == 0) {
                                    cur_scatter.append(0, 0);
                                    cur_scatter.pointsVisible = true;
                                    continue;
                                }
                                var scatter = baselinePlotChart.createSeries(ChartView.SeriesTypeScatter, Constants.baselinePlot.legendLabels[idx] + "scatter", baselinePlotXAxis, baselinePlotYAxis);
                                scatter.color = Constants.baselinePlot.colors[idx];
                                scatter.markerSize = Constants.commonChart.solutionMarkerSize;
                                scatter.useOpenGL = Globals.useOpenGL;
                                scatters.push(scatter);
                                cur_scatters.push(cur_scatter);
                            }
                        }
                        var combined = [scatters, cur_scatters];
                        baselinePlotPoints.fill_series(combined);
                        // if (baselinePlotYAxis.min != baselinePlotPoints.n_min || baselinePlotYAxis.max != baselinePlotPoints.n_max) {
                        //     baselinePlotYAxis.min = baselinePlotPoints.n_min;
                        //     baselinePlotYAxis.max = baselinePlotPoints.n_max;
                        // }
                        // if (baselinePlotXAxis.min != baselinePlotPoints.e_min || baselinePlotXAxis.max != baselinePlotPoints.e_max) {
                        //     baselinePlotXAxis.min = baselinePlotPoints.e_min;
                        //     baselinePlotXAxis.max = baselinePlotPoints.e_max;
                        // }
                    }
                }

            }

        }

    }

}
