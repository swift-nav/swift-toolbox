import QtQuick 2.5
import QtQuick.Controls 2.12
import QtCharts 2.2
import QtQuick.Layouts 1.15

import SwiftConsole 1.0

ApplicationWindow {

    width: 640
    height: 480

    font.pointSize: 8

    ConsolePoints {
        id: console_points
    }

    TrackingSignalsPoints {
        id: tracking_signals_points
    }
    property variant lines: []
    
    ColumnLayout {

        anchors.fill: parent
        anchors.margins: 4
        spacing: 2

        TabBar {
            id: bar
            width: parent.width
            
            Repeater {
                model: ["Tracking", "Solution", "Baseline", "Observations", "Settings", "Update", "Advanced"]

                TabButton {
                    text: modelData
                    width: implicitWidth
                }
            }
        }
        StackLayout {
            width: parent.width
            currentIndex: bar.currentIndex
            Item {
                
                id: trackingTab
                TabBar {
                    id: trackingbar
                    Repeater {
                        model: ["Signals", "Sky Plot"]
                        TabButton {
                            text: modelData
                            width: implicitWidth
                        }
                    }
                }
                StackLayout {
                    anchors.top: trackingbar.bottom
                    anchors.bottom: trackingTab.bottom
                    id: trackingbarlayout
                    width: parent.width
                    currentIndex: trackingbar.currentIndex
                    Item {
                        id: trackingsignalsTab

                        ChartView {
                            id: tracking_signals_chart
                            title: "Tracking C/N0"
                            titleFont.pointSize: 14
                            antialiasing: true
                            width: trackingTab.width
                            height: trackingTab.height

                            // legend.font.pointSize: 7
                            legend.alignment: Qt.AlignTop
                            legend.showToolTips: true

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
                                    var colors = tracking_signals_points.colors;
                                    var labels = tracking_signals_points.labels;
                                

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
                    Item {
                        id: trackingskyplotTab
                    }

                }
                
            }
            Item {
                id: solutionTab
                RowLayout{
                    anchors.fill: parent
                    spacing: 2
                    ListView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignLeft
                        

                        Component {
                            id: contactsDelegate
                            Rectangle {
                                id: wrapper
                                width: 180
                                height: contactInfo.height
                                color: ListView.isCurrentItem ? "black" : "red"
                                Text {
                                    id: contactInfo
                                    text: name + ": " + number
                                    color: wrapper.ListView.isCurrentItem ? "red" : "black"
                                }
                            }
                        }

                        model: ContactModel {}
                        delegate: contactsDelegate
                        focus: true
                    }

                    ChartView {

                        
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignLeft
                        titleFont.pointSize: 8
                        antialiasing: true
                        width: parent.width/2
                        legend.font.pointSize: 7
                        legend.alignment: Qt.AlignTop
                        legend.showToolTips: true

                        ValueAxis {
                            id: x_axis
                            labelsFont.pointSize: 7
                            titleText: "GPS Time of Week"
                        }
                        ValueAxis {
                            id: y_axis
                            min: -1.0
                            max: 1.0
                            labelsFont.pointSize: 7
                        }

                        LineSeries {
                            id: hseries
                            name: "Horizontal [m/s]"
                            axisX: x_axis
                            axisY: y_axis
                            //useOpenGL: true
                        }
                        LineSeries {
                            id: vseries
                            name: "Vertical [m/s]"
                            axisX: x_axis
                            axisY: y_axis
                            //useOpenGL: true
                        }

                        Timer {
                            interval: 1000/5 // 5 Hz refresh
                            running: true
                            repeat: true
                            onTriggered: {
                                if (!solutionTab.visible) {
                                    return;
                                }
                                data_model.fill_console_points(console_points);
                                if (!console_points.valid) {
                                    return;
                                }
                                var hpoints = console_points.hpoints;
                                var last = hpoints[hpoints.length - 1];
                                x_axis.min = last.x - 10;
                                x_axis.max = last.x;
                                y_axis.min = console_points.min_;
                                y_axis.max = console_points.max_;
                                console_points.fill_hseries(hseries);
                                console_points.fill_vseries(vseries);
                            }
                        }
                    }
                }
                
            }    
            Item {
                id: baselineTab
            }
            Item {
                id: observationsTab
            }
            Item {
                id: settingsTab
            }
            Item {
                id: updateTab
            }
            Item {
                id: advancedTab
            }
        }
        
        RowLayout {
            Button {
                text: "Connect"
                onClicked: data_model.connect()
            }
            Button {
                text: "File In"
                onClicked: data_model.readfile()
            }
        }
        
    }

    Component.onCompleted: {
        visible = true;
    }
}
