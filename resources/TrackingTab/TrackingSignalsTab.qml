
import QtQuick 2.5
import QtQuick.Controls 2.12
import QtCharts 2.2
import QtQuick.Layouts 1.15

Item {
    id: trackingsignalsTab
    property variant lines: []
    property variant labels: []
    property variant colors: []

    ChartView {
        id: tracking_signals_chart
        title: "Tracking C/N0"
        titleFont.pointSize: 14
        antialiasing: true
        width: trackingTab.width
        height: trackingTab.height

        
        legend.visible: false

        Rectangle {
            id: rect
            
            border.color: "#000000"
            border.width: 1
            anchors.bottom: tracking_signals_chart.bottom
            // anchors.left: tracking_signals_chart.left
            // radius: 20
            // implicitWidth: 200//parent.width/4//label.implicitWidth + marker.implicitWidth + 30
            // implicitHeight: tracking_signals_chart.height
            implicitHeight: legendRow.height
            width: legendRow.width

            Column {
                id: legendRow
                spacing: -1
                anchors.bottom: rect.bottom
                Repeater {
                    model: labels
                    id: rows
                    Row {
                        Rectangle {
                            id: marker
                            width: 20
                            height: 2
                            // color: "#000000"//
                            color: colors[index]
                            anchors.verticalCenter: parent.verticalCenter
                        }
                        Text { 
                            id: label
                            text: modelData
                            font.pointSize: 6
                            anchors.verticalCenter: parent.verticalCenter
                            anchors.verticalCenterOffset: -1
                        }
                    }
                    
                }
            }
        }


        ValueAxis {
            id: tracking_signals_x_axis
            labelsFont.pointSize: 10
            titleText: "seconds"
        }
        ValueAxis {
            id: tracking_signals_y_axis
            titleText: "dB-Hz"
            min: -1.0
            max: 0.0
            labelsFont.pointSize: 10
        }
        Timer {
            interval: 1000/5 // 5 Hz refresh
            running: true
            repeat: true
            onTriggered: {

                if (!trackingTab.visible) {
                    return;
                }
                
                tracking_signals_model.fill_console_points(tracking_signals_points);
                if (!tracking_signals_points.points.length) {
                    return;
                }
                
                var points = tracking_signals_points.points;
                colors = tracking_signals_points.colors;
                labels = tracking_signals_points.labels;
            

                for (var idx in labels) {
                    
                    if (idx < lines.length) {
                        if (labels[idx]!=lines[idx][1]){
                            var line = tracking_signals_chart.createSeries(ChartView.SeriesTypeLine, labels[idx], tracking_signals_x_axis);
                            line.color = colors[idx];
                            line.axisYRight = tracking_signals_y_axis;
                            lines[idx] = [line, labels[idx]];
                        }
                    } else {
                        var line = tracking_signals_chart.createSeries(ChartView.SeriesTypeLine, labels[idx], tracking_signals_x_axis);
                        line.color = colors[idx];
                        line.axisYRight = tracking_signals_y_axis;
                        lines.push([line, labels[idx]]);
                    }
                }
                tracking_signals_points.fill_series(lines);
                
                var last = points[0][points[0].length - 1];
                tracking_signals_x_axis.min = last.x - 10;
                tracking_signals_x_axis.max = last.x;
                
                if (tracking_signals_y_axis.min!=tracking_signals_points.min_){
                    tracking_signals_y_axis.min = tracking_signals_points.min_;
                    tracking_signals_y_axis.max = tracking_signals_points.max_;
                }
                
                
            }
        }
        Component.onCompleted: {
        }
    }

}