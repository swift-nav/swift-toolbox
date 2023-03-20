/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
import "../BaseComponents"
import "../Constants"
import "../SolutionPlotCommon/SolutionPlotLoop.js" as SolutionPlotLoop
import QtCharts
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SwiftConsole

Item {
    id: solutionPositionTab

    property variant available_units: ["degrees", "meters"]
    property variant cur_scatters: []
    property variant scatters: []
    property variant line: null
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
    readonly property int num_buttons: 4

    SolutionPositionPoints {
        id: solutionPositionPoints

        function update() {
            solution_position_model.fill_console_points(solutionPositionPoints);
            if (!solutionPositionPoints.points.length)
                return;
            if (available_units != solutionPositionPoints.available_units)
                available_units = solutionPositionPoints.available_units;
            if (!line || !scatters.length || !cur_scatters.length)
                [scatters, cur_scatters, line] = SolutionPlotLoop.setupScatterSeries(solutionPositionChart, Constants, Globals, solutionPositionXAxis, solutionPositionYAxis, Constants.solutionPosition.legendLabels, Constants.solutionPosition.colors, false, true);
            var combined = [line, scatters, cur_scatters];
            solutionPositionPoints.fill_series(combined);
            let point = SolutionPlotLoop.getCurSolution(solutionPositionPoints.cur_points);
            if (point)
                cur_solution = point;
            if (center_solution)
                solutionPositionChart.centerToSolution();
            let hasData = false;
            let solnPoints = solutionPositionPoints.points;
            for (let idx in solnPoints) {
                if (solnPoints[idx].length) {
                    hasData = true;
                    break;
                }
            }
            let solnPos = Constants.solutionPosition;
            let new_lat_min = solnPos.axesDefaultMin;
            let new_lat_max = solnPos.axesDefaultMax;
            let new_lon_min = solnPos.axesDefaultMin;
            let new_lon_max = solnPos.axesDefaultMax;
            solutionZoomAllButton.enabled = hasData;
            solutionCenterButton.enabled = hasData;
            if (hasData) {
                new_lat_min = solutionPositionPoints.lat_min_;
                new_lat_max = solutionPositionPoints.lat_max_;
                new_lon_min = solutionPositionPoints.lon_min_;
                new_lon_max = solutionPositionPoints.lon_max_;
            } else {
                zoom_all = true;
                center_solution = false;
                solutionZoomAllButton.checked = true;
                solutionCenterButton.checked = false;
                solutionPositionChart.resetChartZoom();
            }
            if (orig_lat_min != new_lat_min || orig_lat_max != new_lat_max || orig_lon_min != new_lon_min || orig_lon_max != new_lon_max) {
                orig_lat_min = new_lat_min;
                orig_lat_max = new_lat_max;
                orig_lon_min = new_lon_min;
                orig_lon_max = new_lon_max;
                if (zoom_all)
                    solutionPositionChart.resetChartZoom();
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        visible: true
        spacing: Constants.solutionPosition.navBarSpacing

        ButtonGroup {
            id: solutionButtonGroup

            exclusive: false
        }

        RowLayout {
            property real labelComboWidth: solutionPositionSelectedUnit.width + solutionPositionSelectedUnitLabel.width

            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: Constants.solutionPosition.navBarMargin

            Button {
                id: solutionPauseButton

                ButtonGroup.group: solutionButtonGroup
                Layout.preferredWidth: parent.labelComboWidth / num_buttons
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Pause"
                checkable: true
                onClicked: backend_request_broker.solution_position([solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].pressed])

                icon {
                    source: Constants.icons.pauseButtonUrl
                    color: !solutionPauseButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }
            }

            Button {
                id: solutionClearButton

                ButtonGroup.group: solutionButtonGroup
                Layout.preferredWidth: parent.labelComboWidth / num_buttons
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Clear"
                onPressed: backend_request_broker.solution_position([solutionButtonGroup.buttons[1].checked, solutionButtonGroup.buttons[0].pressed])

                icon {
                    source: Constants.icons.clearButtonUrl
                    color: !solutionClearButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }
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
                Layout.preferredWidth: parent.labelComboWidth / num_buttons
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Zoom All"
                checkable: true
                checked: true

                icon {
                    source: Constants.icons.zoomAllButtonUrl
                    color: !solutionZoomAllButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }
            }

            Button {
                id: solutionCenterButton

                onClicked: {
                    if (checked) {
                        solutionZoomAllButton.checked = false;
                        x_axis_half = Utils.spanBetweenValues(solutionPositionXAxis.max, solutionPositionXAxis.min) / 2;
                        y_axis_half = Utils.spanBetweenValues(solutionPositionYAxis.max, solutionPositionYAxis.min) / 2;
                        center_solution = true;
                        zoom_all = false;
                    } else {
                        center_solution = false;
                    }
                }
                Layout.preferredWidth: parent.labelComboWidth / num_buttons
                Layout.preferredHeight: Constants.commonChart.buttonHeight
                ToolTip.visible: hovered
                ToolTip.text: "Center On Solution"
                checkable: true

                icon {
                    source: Constants.icons.centerOnButtonUrl
                    color: !solutionCenterButton.checked ? Constants.materialGrey : Constants.swiftOrange
                }
            }

            Label {
                id: solutionPositionSelectedUnitLabel

                text: "Display Units: "
            }

            SwiftComboBox {
                id: solutionPositionSelectedUnit

                model: available_units
                Layout.preferredWidth: Constants.commonChart.unitDropdownWidth
                onCurrentIndexChanged: {
                    if (!scatters.length)
                        return;
                    backend_request_broker.solution_position_unit(available_units[currentIndex]);
                    zoom_all = true;
                    solutionZoomAllButton.checked = true;
                    solutionCenterButton.checked = false;
                    center_solution = false;
                    solutionPositionChart.resetChartZoom();
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

            onWidthChanged: {
                solutionPositionChart.freezeTicks();
                solutionPositionChart.fixAspectRatio();
                solutionPositionChart.setTicks();
            }

            onHeightChanged: {
                solutionPositionChart.freezeTicks();
                solutionPositionChart.fixAspectRatio();
                solutionPositionChart.setTicks();
            }

            function freezeTicks() {
                // fix the interval so tick number will not be too large.
                solutionPositionXAxis.freezeTicks();
                solutionPositionYAxis.freezeTicks();
            }

            function fixAspectRatio() {
                const aspect_ratio = height / width;
                const x_range = Math.abs(solutionPositionXAxis.max - solutionPositionXAxis.min);
                const y_range = Math.abs(solutionPositionYAxis.max - solutionPositionYAxis.min);
                const range_diff = aspect_ratio * x_range - y_range;
                if (range_diff < 0) {
                    const correction = Math.abs(range_diff / aspect_ratio / 2);
                    solutionPositionXAxis.min -= correction;
                    solutionPositionXAxis.max += correction;
                } else {
                    const correction = Math.abs(range_diff / 2);
                    solutionPositionYAxis.min -= correction;
                    solutionPositionYAxis.max += correction;
                }
            }

            // This function make the ticks on the x & y axes have the
            // same interval, and have them land on evenish numbers.
            // It also ensures the ranges of the two axes are the same.
            function setTicks() {
                const x_tick_interval = solutionPositionXAxis.getGoodTickInterval();
                const y_tick_interval = solutionPositionYAxis.getGoodTickInterval();
                const max_tick_interval = Math.max(x_tick_interval, y_tick_interval);
                solutionPositionXAxis.setGoodTicks(max_tick_interval);
                solutionPositionYAxis.setGoodTicks(max_tick_interval);
            }

            function resetChartZoom() {
                solutionPositionChart.freezeTicks();
                // update the chart lims
                solutionPositionChart.zoomReset();
                solutionPositionXAxis.max = orig_lon_max;
                solutionPositionXAxis.min = orig_lon_min;
                solutionPositionYAxis.max = orig_lat_max;
                solutionPositionYAxis.min = orig_lat_min;
                solutionPositionChart.fixAspectRatio();
                // update ticks
                solutionPositionChart.setTicks();
            }

            function centerToSolution() {
                solutionPositionChart.freezeTicks();
                // update chart lims
                solutionPositionChart.zoomReset();
                if (cur_scatters.length) {
                    solutionPositionXAxis.max = cur_solution.x + x_axis_half;
                    solutionPositionXAxis.min = cur_solution.x - x_axis_half;
                    solutionPositionYAxis.max = cur_solution.y + y_axis_half;
                    solutionPositionYAxis.min = cur_solution.y - y_axis_half;
                }
                solutionPositionChart.fixAspectRatio();
                // update ticks
                solutionPositionChart.setTicks();
            }

            function chartZoomByDirection(delta) {
                solutionPositionChart.freezeTicks();
                if (delta > 0) {
                    solutionPositionChart.zoom(Constants.commonChart.zoomInMult);
                } else {
                    solutionPositionChart.zoom(Constants.commonChart.zoomOutMult);
                }
                solutionPositionChart.setTicks();
            }

            function stopZoomFeatures() {
                solutionCenterButton.checked = false;
                center_solution = false;
                solutionZoomAllButton.checked = false;
                zoom_all = false;
            }

            Layout.preferredWidth: parent.width
            Layout.minimumHeight: parent.height - Constants.commonChart.heightOffset
            Layout.alignment: Qt.AlignBottom
            Layout.fillHeight: true
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
                            spacing: Constants.solutionPosition.legendLabelSpacing

                            Label {
                                id: marker

                                text: "● "
                                font.pixelSize: (Constants.mediumPixelSize + Constants.commonLegend.markerPixelSizeOffset)
                                font.bold: true
                                color: Constants.solutionPosition.colors[index]
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
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
                id: solutionPositionXAxis

                titleText: Constants.solutionPosition.xAxisTitleText + " (" + available_units[solutionPositionSelectedUnit.currentIndex] + ")"
                tickType: ValueAxis.TicksDynamic
            }

            SwiftValueAxis {
                id: solutionPositionYAxis

                titleText: Constants.solutionPosition.yAxisTitleText + " (" + available_units[solutionPositionSelectedUnit.currentIndex] + ")"
                tickType: ValueAxis.TicksDynamic
            }

            MouseArea {
                anchors.fill: solutionPositionChart
                onDoubleClicked: {
                    solutionPositionChart.stopZoomFeatures();
                    solutionZoomAllButton.checked = true;
                    solutionPositionChart.resetChartZoom();
                }
                onWheel: wheel => {
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

            LineSeries {
                name: "emptySeries"
                axisY: solutionPositionYAxis
                axisX: solutionPositionXAxis
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
        }
    }
}
