import QtQuick 2.9
import QtQuick.Shapes 1.0

MouseArea {
    id: main

    readonly property real strokeWidth: width / 12

    opacity: pressed ? 0.5 : 1
    state: "pause"

    Rectangle {
        id: border

        anchors.fill: parent
        border.width: 1
        radius: width / 2
        color: "transparent"
        border.color: "#353535"
    }

    Loader {
        anchors.fill: parent
        sourceComponent: main.state == "pause" ? pause : play
    }

    Component {
        id: pause

        Item {
            Item {
                width: parent.width * 0.33
                height: parent.height * 0.475
                anchors.centerIn: parent

                Rectangle {
                    width: strokeWidth
                    anchors.left: parent.left
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    radius: width / 2
                    color: "#353535"
                }

                Rectangle {
                    width: strokeWidth
                    anchors.right: parent.right
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    radius: width / 2
                    color: "#353535"
                }

            }

        }

    }

    Component {
        id: play

        Item {
            Shape {
                id: shape

                width: parent.width * 0.4
                height: parent.height * 0.4
                anchors.centerIn: parent
                anchors.horizontalCenterOffset: path.strokeWidth / 2

                ShapePath {
                    id: path

                    strokeWidth: 4
                    strokeColor: "#353535"
                    fillColor: "#353535"
                    fillRule: ShapePath.WindingFill
                    joinStyle: ShapePath.RoundJoin
                    startX: 0
                    startY: 0

                    PathLine {
                        x: shape.width
                        y: shape.height / 2
                    }

                    PathLine {
                        x: 0
                        y: shape.height
                    }

                    PathLine {
                        x: 0
                        y: 0
                    }

                }

            }

        }

    }

}
