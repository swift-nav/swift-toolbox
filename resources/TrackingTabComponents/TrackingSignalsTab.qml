import "../BaseComponents"
import "../Constants"
import QtCharts 2.3
import QtQuick 2.15
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.15
import SwiftConsole 1.0

Item {
    id: trackingSignalsTab

    property alias all_series: trackingSignalsPoints.all_series
    property alias check_labels: trackingSignalsPoints.check_labels
    property variant check_visibility: []

    width: parent.width
    height: parent.height
    Component.onCompleted: {
    }

    TrackingSignalsPoints {
        id: trackingSignalsPoints
    }

    ColumnLayout {
        id: trackingSignalsArea

        width: parent.width
        height: parent.height
        spacing: 0

        ChartView {
            id: trackingSignalsChart

            Layout.bottomMargin: -(Constants.margins * 2)
            Layout.fillHeight: true
            Layout.fillWidth: true
            visible: false
            title: Constants.trackingSignals.title
            titleColor: Constants.trackingSignals.titleColor
            //width: parent.width
            //height: parent.height - trackingSignalsCheckboxes.height
            backgroundColor: Constants.commonChart.backgroundColor
            plotAreaColor: Constants.commonChart.areaColor
            legend.visible: false
            antialiasing: true
            Component.onCompleted: {
            }

            titleFont {
                pointSize: Constants.trackingSignals.titlePointSize
                bold: true
            }

            Rectangle {
                id: lineLegend

                property int openedHeight: parent.height - Constants.trackingSignals.legendTopMargin - Constants.trackingSignals.legendBottomMargin
                property int openCloseSpeed: 350

                visible: false
                radius: 5
                border.color: Constants.commonLegend.borderColor
                border.width: Constants.commonLegend.borderWidth
                x: Constants.trackingSignals.legendTopMargin
                y: Constants.trackingSignals.legendLeftMargin
                height: openedHeight
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
                    id: legendHideBar

                    color: "dark grey"
                    anchors.top: parent.top
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.topMargin: 1
                    height: 10
                    radius: lineLegend.radius

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            lineLegend.state = lineLegend.state == "opened" ? "closed" : "opened";
                        }
                        cursorShape: pressed ? Qt.ClosedHandCursor : Qt.OpenHandCursor
                        drag.target: lineLegend
                        hoverEnabled: true
                    }

                }

                GridView {
                    id: gridView

                    anchors.top: legendHideBar.bottom
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    clip: true
                    model: all_series
                    flow: GridView.FlowTopToBottom
                    cellWidth: 10
                    cellHeight: 10
                    boundsBehavior: Flickable.StopAtBounds
                    onCellWidthChanged: {
                        lineLegend.width = cellWidth * 2;
                    }

                    delegate: Row {
                        padding: 4
                        Component.onCompleted: {
                            if (gridView.cellWidth < implicitWidth)
                                gridView.cellWidth = implicitWidth;

                            if (gridView.cellHeight < implicitHeight)
                                gridView.cellHeight = implicitHeight;

                            if (lineLegend.visible != true && gridView.cellWidth > 50)
                                lineLegend.visible = true;

                        }

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
                            font.pointSize: Constants.smallPointSize
                            anchors.verticalCenter: parent.verticalCenter
                            anchors.verticalCenterOffset: Constants.commonLegend.verticalCenterOffset
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

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

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

                labelsFont {
                    pointSize: Constants.mediumPointSize
                    bold: true
                }

            }

            Timer {
                id: trackingSignalsTimer

                interval: Utils.hzToMilliseconds(Globals.currentRefreshRate)
                running: true
                repeat: true
                onTriggered: {
                    if (!trackingTab.visible)
                        return ;

                    if (trackingSignalsPoints.all_series.length < trackingSignalsPoints.num_labels) {
                        for (var i = trackingSignalsPoints.all_series.length; i < trackingSignalsPoints.num_labels; i++) {
                            var series = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, trackingSignalsPoints.getLabel(i), trackingSignalsXAxis);
                            series.axisYRight = trackingSignalsYAxis;
                            series.width = Constants.commonChart.lineWidth;
                            // Color and useOpenGL will be set in Python with fill_all_series call.
                            // series.color = sourceSeries.color
                            // series.useOpenGL = sourceSeries.useOpenGL
                            trackingSignalsPoints.addSeries(series);
                        }
                    }
                    trackingSignalsPoints.fill_all_series(Constants.commonChart.lineWidth, Globals.useOpenGL);
                    if (trackingSignalsPoints.all_series.length) {
                        trackingSignalsChart.visible = true;
                        trackingSignalsXAxis.min = trackingSignalsPoints.xaxis_min;
                        trackingSignalsXAxis.max = trackingSignalsPoints.xaxis_max;
                    }
                }
            }

        }

        GridLayout {
            id: trackingSignalsCheckboxes

            flow: GridLayout.TopToBottom
            columns: Math.floor(parent.width / Constants.trackingSignals.checkBoxPreferredWidth)
            rows: Math.ceil(check_labels.length / trackingSignalsCheckboxes.columns)
            rowSpacing: 0
            Layout.margins: 0
            Layout.alignment: Qt.AlignHCenter

            Repeater {
                id: trackingSignalsCheckbox

                model: check_labels

                SmallCheckBox {
                    Layout.margins: 0
                    Layout.rowSpan: index === 0 ? trackingSignalsCheckboxes.rows : 1
                    checked: true
                    text: modelData
                    onClicked: {
                        check_visibility[index] = checked;
                        if (index == 0) {
                            lineLegend.visible = !lineLegend.visible;
                            return ;
                        }
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
