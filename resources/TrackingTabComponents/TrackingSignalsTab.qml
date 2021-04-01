
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
    property variant check_visibility: []
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
            antialiasing: true

            Rectangle {
                id: lineLegend
                border.color: "#000000"
                border.width: 1
                
                anchors.bottom: trackingSignalsChart.bottom
                anchors.left: trackingSignalsChart.left
                anchors.bottomMargin: 85
                anchors.leftMargin: 60
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
                                line.width = 1;
                                line.axisYRight = trackingSignalsYAxis;
                                lines[idx] = [line, labels[idx]];
                            }
                        } else {
                            var line = trackingSignalsChart.createSeries(ChartView.SeriesTypeLine, labels[idx], trackingSignalsXAxis);
                            line.color = colors[idx];
                            line.width = 1;
                            line.axisYRight = trackingSignalsYAxis;
                            lines.push([line, labels[idx]]);
                        }
                    }
                    trackingSignalsPoints.fill_series(lines);
                    
                    var last = points[0][points[0].length - 1];
                    trackingSignalsXAxis.min = last.x - 100;
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
        GridLayout {
            id: trackingSignalsCheckboxes
            columns: 6
            anchors.horizontalCenter: trackingSignalsChart.horizontalCenter
            anchors.top: trackingSignalsChart.bottom
            
            
            Repeater {
                model: check_labels
                id: trackingSignalsCheckbox
                Column {
                    CheckBox {
                        checked: true
                        text: modelData
                        verticalPadding: 0
                        onClicked: {
                            check_visibility[index] = checked;
                            if (index == 0){
                                lineLegend.visible = !lineLegend.visible;
                                return;
                            }
                            var labels_not_visible = [];
                            for (var idx in check_visibility){
                                if (!check_visibility[idx]){
                                    labels_not_visible.push(check_labels[idx]);
                                }
                            }
                            data_model.check_visibility(labels_not_visible);
                        } 
                        Component.onCompleted: {
                            check_visibility.push(checked);
                        }
                    }

                }
            }
        }
    }
    Component.onCompleted: {
    }
}
