import "../BaseComponents"
import "../Constants"
import QtCharts 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: trackingSignalsTab

    property alias all_series: trackingSignalsPoints.all_series
    property alias enabled_series: trackingSignalsPoints.enabled_series
    property alias check_labels: trackingSignalsPoints.check_labels
    property alias num_labels: trackingSignalsPoints.num_labels
    property variant check_visibility: []

    TrackingSignalsPoints {
        id: trackingSignalsPoints
    }

    ColumnLayout {
        id: trackingSignalsArea

        anchors.fill: parent
        spacing: 0

        ChartView {
            id: trackingSignalsChart

            Layout.fillHeight: true
            Layout.fillWidth: true
            visible: false
            title: Constants.trackingSignals.title
            titleFont.family: Constants.fontFamily
            titleFont.pointSize: Constants.trackingSignals.titlePointSize
            titleFont.bold: true
            titleColor: Constants.trackingSignals.titleColor
            plotAreaColor: Constants.commonChart.areaColor
            legend.visible: false
            antialiasing: true

            margins {
                top: 0
                bottom: 0
                left: 0
                right: 0
            }

            Rectangle {
                id: lineLegend

                property int maximumHeight: parent.height - Constants.trackingSignals.legendTopMargin - Constants.trackingSignals.legendBottomMargin
                property int openedHeight: gridView.count < maxCellsPerColumn ? gridView.cellHeight * (gridView.count + 1) : maximumHeight
                property int openCloseSpeed: Constants.trackingSignals.legendShadeSpeed
                property int maxCellsPerColumn: Math.floor((maximumHeight - gridView.cellHeight) / gridView.cellHeight)

                visible: gridView.count > 0
                radius: 5
                x: Constants.trackingSignals.legendLeftMargin
                y: Constants.trackingSignals.legendTopMargin
                height: openedHeight
                // Size to two cols if there are cells for 2+ cols.
                width: gridView.cellWidth * (gridView.count <= maxCellsPerColumn ? 1 : 2) + 1
                state: "opened"
                states: [
                    State {
                        name: "opened"

                        PropertyChanges {
                            target: lineLegend
                            height: lineLegend.openedHeight
                        }

                        PropertyChanges {
                            target: gridView
                            visible: true
                        }

                    },
                    State {
                        name: "closed"

                        PropertyChanges {
                            target: lineLegend
                            height: legendHideBar.height + 2
                        }

                        PropertyChanges {
                            target: gridView
                            visible: false
                        }

                    }
                ]
                transitions: [
                    Transition {
                        to: "closed"

                        // reversible property should be what we want here instead of duplicating this,
                        // but it doesn't seem to work right in this situation.
                        SequentialAnimation {
                            SmoothedAnimation {
                                property: "height"
                                duration: lineLegend.openCloseSpeed
                            }

                            PropertyAction {
                                property: "visible"
                            }

                        }

                    },
                    Transition {
                        to: "opened"

                        SequentialAnimation {
                            PropertyAction {
                                property: "visible"
                            }

                            SmoothedAnimation {
                                property: "height"
                                duration: lineLegend.openCloseSpeed
                            }

                        }

                    }
                ]

                Rectangle {
                    // This rectangle ensures that the border of the legend is painted nicely.
                    anchors.fill: parent
                    z: 2
                    color: "transparent"
                    radius: parent.radius
                    border.color: Constants.commonLegend.borderColor
                    border.width: Constants.commonLegend.borderWidth
                }

                ColumnLayout {
                    anchors.fill: parent
                    spacing: 0

                    Rectangle {
                        id: legendHideBar

                        Layout.fillWidth: true
                        color: Constants.trackingSignals.legendShadeColor
                        height: Constants.trackingSignals.legendShadeHeight
                        radius: lineLegend.radius

                        MouseArea {
                            anchors.fill: parent
                            onClicked: {
                                lineLegend.state = lineLegend.state == "opened" ? "closed" : "opened";
                            }
                            cursorShape: pressed ? Qt.ClosedHandCursor : Qt.OpenHandCursor
                            //drag.target: lineLegend
                            hoverEnabled: true
                        }

                    }

                    GridView {
                        id: gridView

                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true
                        model: enabled_series
                        flow: GridView.FlowTopToBottom
                        cellWidth: Constants.commonLegend.markerWidth + legendTextMetrics.width + 4
                        cellHeight: legendTextMetrics.height + 2
                        boundsBehavior: Flickable.StopAtBounds

                        TextMetrics {
                            id: legendTextMetrics

                            font.family: Constants.fontFamily
                            font.pointSize: Constants.xSmallPointSize
                            text: Constants.trackingSignals.legendCellTextSample
                        }

                        ScrollBar.horizontal: ScrollBar {
                        }

                        delegate: Row {
                            padding: 1
                            leftPadding: 4
                            rightPadding: leftPadding

                            Rectangle {
                                id: marker

                                color: modelData.color
                                width: Constants.commonLegend.markerWidth
                                height: Constants.commonLegend.markerHeight
                                anchors.verticalCenter: parent.verticalCenter
                            }

                            Label {
                                id: label

                                text: modelData.name
                                font: legendTextMetrics.font
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
                            }

                        }

                    }

                }

            }

            ValueAxis {
                id: trackingSignalsXAxis

                titleText: Constants.trackingSignals.xAxisTitleText
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.xAxisTickInterval
                labelFormat: "%d"
                titleFont.family: Constants.fontFamily
                titleFont.pointSize: Constants.smallPointSize
                labelsFont.family: Constants.fontFamily
                labelsFont.pointSize: Constants.xSmallPointSize
            }

            ValueAxis {
                id: trackingSignalsYAxis

                titleText: Constants.trackingSignals.yAxisTitleText
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: Constants.commonChart.minorGridLineColor
                gridLineColor: Constants.commonChart.gridLineColor
                labelsColor: Constants.commonChart.labelsColor
                max: Constants.trackingSignals.yAxisMax
                min: Constants.trackingSignals.snrThreshold
                tickType: ValueAxis.TicksDynamic
                tickInterval: Constants.trackingSignals.yAxisTickInterval
                labelFormat: "%d"
                titleFont.family: Constants.fontFamily
                titleFont.pointSize: Constants.smallPointSize
                labelsFont.family: Constants.fontFamily
                labelsFont.pointSize: Constants.xSmallPointSize
            }

            Timer {
                id: trackingSignalsTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!trackingTab.visible)
                        return ;

                    if (all_series.length < num_labels) {
                        for (var i = all_series.length; i < num_labels; i++) {
                            var series = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, trackingSignalsPoints.getLabel(i), trackingSignalsXAxis);
                            series.axisYRight = trackingSignalsYAxis;
                            series.width = Constants.commonChart.lineWidth;
                            // Color and useOpenGL will be set in Python with fill_all_series call.
                            trackingSignalsPoints.addSeries(series);
                        }
                    }
                    trackingSignalsPoints.fill_all_series(Constants.commonChart.lineWidth, Globals.useOpenGL);
                    if (all_series.length) {
                        trackingSignalsChart.visible = true;
                        trackingSignalsXAxis.min = trackingSignalsPoints.xaxis_min;
                        trackingSignalsXAxis.max = trackingSignalsPoints.xaxis_max;
                    }
                }
            }

        }

        GridLayout {
            id: trackingSignalsCheckboxes

            property int numChecked: trackingSignalsCbRepeater.count

            flow: GridLayout.TopToBottom
            columns: Math.floor(parent.width / Constants.trackingSignals.checkBoxPreferredWidth)
            rows: Math.ceil(check_labels.length / trackingSignalsCheckboxes.columns)
            rowSpacing: 0
            Layout.margins: 0
            Layout.alignment: Qt.AlignHCenter

            SmallCheckBox {
                id: toggleAllCheckBox

                Layout.margins: 0
                Layout.rowSpan: parent.rows == 0 ? 1 : parent.rows
                tristate: true
                checkState: (parent.numChecked == trackingSignalsCbRepeater.count ? Qt.Checked : parent.numChecked > 0 ? Qt.PartiallyChecked : Qt.Unchecked)
                text: "Toggle All"
                onClicked: {
                    var curCheckState = checkState;
                    for (var i = 0; i < trackingSignalsCbRepeater.count; i++) {
                        var cb = trackingSignalsCbRepeater.itemAt(i);
                        if ((curCheckState == Qt.Checked && !cb.checked) || (curCheckState != Qt.Checked && cb.checked))
                            cb.toggle();

                    }
                }
                nextCheckState: function() {
                    return (checkState == Qt.Checked) ? Qt.Unchecked : Qt.Checked;
                }
            }

            Repeater {
                id: trackingSignalsCbRepeater

                model: check_labels

                SmallCheckBox {
                    Layout.margins: 0
                    Layout.rowSpan: index === 0 ? trackingSignalsCheckboxes.rows : 1
                    checked: true
                    text: modelData
                    onCheckedChanged: {
                        trackingSignalsCheckboxes.numChecked += checked ? 1 : -1;
                        check_visibility[index] = checked;
                        var labels_not_visible = [];
                        for (var idx in check_visibility) {
                            if (!check_visibility[idx])
                                labels_not_visible.push(check_labels[idx]);

                        }
                        data_model.tracking_signals_check_visibility(labels_not_visible);
                    }
                    Component.onCompleted: {
                        check_visibility.push(checked);
                    }
                }

            }

        }

    }

}
