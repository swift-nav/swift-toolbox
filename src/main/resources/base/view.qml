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
    
    ColumnLayout {

        anchors.fill: parent
        anchors.margins: 2
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
                    width: parent.width
                    
                    Repeater {
                        model: ["Signals", "Sky Plot"]
                        TabButton {
                            text: modelData
                            width: implicitWidth
                        }
                    }
                }
                Item {
                    id: trackingsignalsTab
                    ChartView {
                        id: tracking_signals_chart
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignLeft
                        titleFont.pointSize: 8
                        antialiasing: true
                        width: parent.width/2
                        //height: parent.height

                        legend.font.pointSize: 7
                        legend.alignment: Qt.AlignTop
                        legend.showToolTips: true

                        ValueAxis {
                            id: tracking_signals_x_axis
                            labelsFont.pointSize: 7
                            titleText: "GPS Time of Week"
                        }
                        ValueAxis {
                            id: tracking_signals_y_axis
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
                        

                        Timer {
                            interval: 1000/5 // 5 Hz refresh
                            running: true
                            repeat: true
                            onTriggered: {
                                data_model.fill_console_points(console_points);
                                if (!console_points.valid) {
                                    return;
                                }
                                var hpoints = console_points.hpoints;
                                var last = hpoints[hpoints.length - 1];
                                x_axis.min = last.x - 10;
                                x_axis.max = last.x;
                                y_axis.min = console_points.min;
                                y_axis.max = console_points.max;
                                console_points.fill_hseries(hseries);
                                console_points.fill_vseries(vseries);
                            }
                        }
                    }
                
                }
                Item {
                    id: trackingskyplotTab
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
                        //height: parent.height

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
                                data_model.fill_console_points(console_points);
                                if (!console_points.valid) {
                                    return;
                                }
                                var hpoints = console_points.hpoints;
                                var last = hpoints[hpoints.length - 1];
                                x_axis.min = last.x - 10;
                                x_axis.max = last.x;
                                y_axis.min = console_points.min;
                                y_axis.max = console_points.max;
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
