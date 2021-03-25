
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtCharts 2.2
import QtQuick.Layouts 1.15

import SwiftConsole 1.0

Item {

    TrackingSignalsPoints {
        id: trackingSignalsPoints
    }

    id: trackingSignalsTab
    width: parent.width
    height: parent.height
    property variant lines: []
    property variant labels: []
    property variant colors: []
    property variant check_labels: []
    Rectangle {
        id: trackingSignalsArea
        width: parent.width
        height: parent.height

        ChartView {
            id: trackingSignalsChart
            title: "Tracking C/N0"
            titleFont { pointSize: 14; bold: true }
            titleColor: "#00006E"
            width: parent.width
            height: parent.height - trackingSignalsCheckboxes.height
            anchors.top: parent.top
            backgroundColor: "#CDC9C9"
            plotAreaColor:  "#FFFFFF"
            legend.visible: false

            Rectangle {
                id: lineLegend
                border.color: "#000000"
                border.width: 1
                
                anchors.bottom: trackingSignalsChart.bottom
                anchors.left: trackingSignalsChart.left
                anchors.bottomMargin: parent.height/6
                anchors.leftMargin: parent.width/10
                implicitHeight: lineLegendRepeater.height
                width: lineLegendRepeater.width
                

                Column {
                    id: lineLegendRepeater
                    spacing: -1
                    anchors.bottom: lineLegend.bottom
                    Repeater {
                        model: labels
                        id: lineLegendRepeaterRows
                        Row {
                            Rectangle {
                                id: marker
                                width: 20
                                height: 3
                                color: "#000000"
                                anchors.verticalCenter: parent.verticalCenter
                            }
                            Text { 
                                id: label
                                text: modelData
                                font.pointSize: 6
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.verticalCenterOffset: -1
                            }
                            Component.onCompleted: {
                                for (var idx in colors) {
                                    if (lineLegendRepeaterRows.itemAt(idx)){
                                        lineLegendRepeaterRows.itemAt(idx).children[0].color = colors[idx];
                                    }                                
                                }
                            }
                        }
                    }
                    
                }
            }
            ValueAxis {
                id: trackingSignalsXAxis
                labelsFont { pointSize: 10; bold: true }
                titleText: "seconds"
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: "#CDC9C9"
                visible: true
            }
            ValueAxis {
                id: trackingSignalsYAxis
                titleText: "dB-Hz"
                min: 0
                max: 1
                labelsFont { pointSize: 10; bold: true }
                gridVisible: true
                lineVisible: true
                minorGridVisible: true
                minorGridLineColor: "#CDC9C9"
                visible: true
            }
            Timer {
                interval: 1000/5 // 5 Hz refresh
                running: true
                repeat: true
                onTriggered: {
                    if (!trackingTab.visible) {
                        return;
                    }
                    tracking_signals_model.fill_console_points(trackingSignalsPoints);
                    if (!trackingSignalsPoints.points.length) {
                        return;
                    }
                    var points = trackingSignalsPoints.points;
                    colors = trackingSignalsPoints.colors;
                    labels = trackingSignalsPoints.labels;
                    if (check_labels != trackingSignalsPoints.check_labels) {
                        check_labels = trackingSignalsPoints.check_labels;
                    }
                    for (var idx in labels) {
                        if (idx < lines.length) {
                            if (labels[idx]!=lines[idx][1]){
                                trackingSignalsChart.removeSeries(lines[idx][0])
                                var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                                line.color = colors[idx];
                                line.axisYRight = trackingSignalsYAxis;
                                lines[idx] = [line, labels[idx]];
                            }
                        } else {
                            var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                            line.color = colors[idx];
                            line.axisYRight = trackingSignalsYAxis;
                            lines.push([line, labels[idx]]);
                        }
                    }
                    trackingSignalsPoints.fill_series(lines);
                    
                    var last = points[0][points[0].length - 1];
                    trackingSignalsXAxis.min = last.x - 10;
                    trackingSignalsXAxis.max = last.x;
                    
                    if (trackingSignalsYAxis.min!=trackingSignalsPoints.min_){
                        trackingSignalsYAxis.min = trackingSignalsPoints.min_;
                        trackingSignalsYAxis.max = trackingSignalsPoints.max_;
                    }
                    
                    
                }
            }
            Component.onCompleted: {
            }
        }
        RowLayout {
            id: trackingSignalsCheckboxes
            width: parent.width
            height: 25
            anchors.top: trackingSignalsChart.bottom
            
            Repeater {
                model: check_labels
                id: trackingSignalsCheckbox
                Column {
                    CheckBox {
                        checked: true
                        text: modelData
                    }
                }
            }
        }
    }
    Component.onCompleted: {
    }
}