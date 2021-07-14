import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: solutionPositionTab

    property variant available_units: []
    property variant cur_scatters: []
    property variant scatters: []
    property variant lines: []
    property real mouse_x: 0
    property real mouse_y: 0
    property real orig_lat_max: 0
    property real orig_lat_min: 0
    property real orig_lon_max: 0
    property real orig_lon_min: 0
    property real x_axis_half: 0
    property real y_axis_half: 0
    property variant cur_solution: null
    property bool zoom_all: true
    property bool center_solution: false

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    SolutionPositionPoints {
        id: solutionPositionPoints
    }

    Rectangle {
        id: solutionPositionArea

        width: parent.width
        height: parent.height
        visible: false

        ColumnLayout {
            id: solutionPositionAreaRowLayout

            anchors.fill: parent
            width: parent.width
            height: parent.height
            spacing: Constants.solutionPosition.navBarSpacing

            ButtonGroup {
                id: solutionButtonGroup

                exclusive: false
            }

            RowLayout {
                Layout.alignment: Qt.AlignLeft
                Layout.leftMargin: Constants.solutionPosition.navBarMargin

                Button {
                    id: solutionPauseButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "| |"
                    ToolTip.visible: hovered
                    ToolTip.text: "Pause"
                    checkable: true
                    onClicked: data_model.solution_position([solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].pressed])
                }

                Button {
                    id: solutionClearButton

                    ButtonGroup.group: solutionButtonGroup
                    Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: " X "
                    ToolTip.visible: hovered
                    ToolTip.text: "Clear"
                    onPressed: data_model.solution_position([solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].pressed])
                }

                Button {
                    id: solutionZoomAllButton

                    onClicked: {
                        if (checked) {
                            zoom_all = true;
                            solutionCenterButton.checked = false;
                            center_solution = false;
                            solutionPositionChart.resetChartZoom();
                        } else {
                            zoom_all = false;
                        }
                    }
                    Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "[ ]"
                    ToolTip.visible: hovered
                    ToolTip.text: "Zoom All"
                    checkable: true
                    checked: true
                }

                Button {
                    id: solutionCenterButton

                    onClicked: {
                        if (checked) {
                            solutionZoomAllButton.checked = false;
                            y_axis_half = Utils.spanBetweenValues(solutionPositionXAxis.max, solutionPositionXAxis.min) / 2;
                            x_axis_half = Utils.spanBetweenValues(solutionPositionYAxis.max, solutionPositionYAxis.min) / 2;
                            center_solution = true;
                            zoom_all = false;
                        } else {
                            center_solution = false;
                        }
                    }
                    Layout.preferredWidth: parent.width * Constants.solutionPosition.navBarButtonProportionOfParent
                    Layout.preferredHeight: Constants.commonChart.buttonHeight
                    text: "(><)"
                    ToolTip.visible: hovered
                    ToolTip.text: "Center On Solution"
                    checkable: true
                }

                Text {
                    text: "Display Units: "
                    font.family: Constants.monoSpaceFont
                    font.pointSize: Constants.mediumPointSize
                }

                ComboBox {
                    id: solutionPositionSelectedUnit

                    model: available_units
                    Layout.preferredWidth: Constants.commonChart.unitDropdownWidth
                    onCurrentIndexChanged: {
                        if (!available_units)
                            return ;

                        data_model.solution_position_unit(available_units[currentIndex]);
                    }

                    states: State {
                        when: solutionPositionSelectedUnit.down

                        PropertyChanges {
                            target: solutionPositionSelectedUnit
                            width: Constants.commonChart.unitDropdownWidth * 1.5
                        }

                    }

                }

            }

            ChartView {
                id: solutionPositionChart

                function resetChartZoom() {
                    solutionPositionChart.zoomReset();
                    solutionPositionXAxis.max = orig_lon_max;
                    solutionPositionXAxis.min = orig_lon_min;
                    solutionPositionYAxis.max = orig_lat_max;
                    solutionPositionYAxis.min = orig_lat_min;
                }

                function centerToSolution() {
                    solutionPositionChart.zoomReset();
                    if (cur_scatters.length) {
                        solutionPositionXAxis.max = cur_solution.x + x_axis_half;
                        solutionPositionXAxis.min = cur_solution.x - x_axis_half;
                        solutionPositionYAxis.max = cur_solution.y + y_axis_half;
                        solutionPositionYAxis.min = cur_solution.y - y_axis_half;
                    }
                }

                function chartZoomByDirection(delta) {
                    if (delta > 0)
                        solutionPositionChart.zoom(Constants.commonChart.zoomInMult);
                    else
                        solutionPositionChart.zoom(Constants.commonChart.zoomOutMult);
                }

                function stopZoomFeatures() {
                    solutionCenterButton.checked = false;
                    center_solution = false;
                    solutionZoomAllButton.checked = false;
                    zoom_all = false;
                }

                Layout.preferredWidth: parent.width
                Layout.preferredHeight: parent.height - Constants.commonChart.heightOffset
                Layout.alignment: Qt.AlignBottom
                Layout.bottomMargin: Constants.commonChart.margin
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
                    anchors.top: solutionPositionChart.top
                    anchors.right: solutionPositionChart.right
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

                            model: Constants.solutionPosition.legendLabels

                            Row {
                                Text {
                                    id: marker

                                    text: "+ "
                                    font.pointSize: (Constants.mediumPointSize + Constants.commonLegend.markerPointSizeOffset)
                                    font.bold: true
                                    color: Constants.solutionPosition.colors[index]
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
                    id: solutionPositionXAxis

                    titleText: Constants.solutionPosition.xAxisTitleText
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
                    id: solutionPositionYAxis

                    titleText: Constants.solutionPosition.yAxisTitleText
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

                MouseArea {
                    anchors.fill: solutionPositionChart
                    onDoubleClicked: {
                        solutionPositionChart.stopZoomFeatures();
                        solutionZoomAllButton.checked = true;
                        solutionPositionChart.resetChartZoom();
                    }
                    onWheel: {
                        solutionPositionChart.stopZoomFeatures();
                        solutionPositionChart.chartZoomByDirection(wheel.angleDelta.y);
                    }
                    onPositionChanged: {
                        if (pressed) {
                            solutionPositionChart.stopZoomFeatures();
                            var current = solutionPositionChart.plotArea;
                            var x_unit = Utils.spanBetweenValues(solutionPositionXAxis.max, solutionPositionXAxis.min) / current.width;
                            var y_unit = Utils.spanBetweenValues(solutionPositionYAxis.max, solutionPositionYAxis.min) / current.height;
                            var delta_x = (mouse_x - mouseX) * x_unit;
                            var delta_y = (mouse_y - mouseY) * y_unit;
                            solutionPositionXAxis.max += delta_x;
                            solutionPositionXAxis.min += delta_x;
                            solutionPositionYAxis.max -= delta_y;
                            solutionPositionYAxis.min -= delta_y;
                            mouse_x = mouseX;
                            mouse_y = mouseY;
                        }
                    }
                    onPressed: {
                        mouse_x = mouseX;
                        mouse_y = mouseY;
                    }
                }

                Timer {
                    interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                    running: true
                    repeat: true
                    onTriggered: {
                        if (!solutionTab.visible)
                            return ;

                        solution_position_model.fill_console_points(solutionPositionPoints);
                        if (!solutionPositionPoints.points.length)
                            return ;

                        solutionPositionArea.visible = true;
                        if (available_units != solutionPositionPoints.available_units)
                            available_units = solutionPositionPoints.available_units;

                        if (!lines.length || !scatters.length || !cur_scatters.length)
                            [scatters, cur_scatters, lines] = SolutionPlotLoop.setupScatterSeries(solutionPositionChart, Constants, Globals, solutionPositionXAxis, solutionPositionYAxis, Constants.solutionPosition.legendLabels, Constants.solutionPosition.colors, false, true);

                        var combined = [lines, scatters, cur_scatters];
                        solutionPositionPoints.fill_series(combined);
                        let point = SolutionPlotLoop.getCurSolution(solutionPositionPoints.cur_points);
                        if (point)
                            cur_solution = point;

                        if (center_solution)
                            solutionPositionChart.centerToSolution();

                        if (orig_lat_min != solutionPositionPoints.lat_min_ || orig_lat_max != solutionPositionPoints.lat_max_ || orig_lon_min != solutionPositionPoints.lon_min_ || orig_lon_max != solutionPositionPoints.lon_max_) {
                            orig_lat_min = solutionPositionPoints.lat_min_;
                            orig_lat_max = solutionPositionPoints.lat_max_;
                            orig_lon_min = solutionPositionPoints.lon_min_;
                            orig_lon_max = solutionPositionPoints.lon_max_;
                            if (zoom_all)
                                solutionPositionChart.resetChartZoom();

                        }
                    }
                }

            }

        }

    }

}
