import QtQuick 2.15

Item {
    property alias font: cellText.font
    property real rowSpacing: 0
    property real columnSpacing: 0
    implicitHeight: cellText.implicitHeight

    Rectangle {
        visible: row === 0
        height: 1
        color: "black"

        anchors {
            top: parent.top
            left: parent.left
            right: parent.right
            leftMargin: column === 0 ? 0 : -1 * (columnSpacing + 1) / 2
            rightMargin: -1 * (columnSpacing + 1) / 2
        }

    }

    Rectangle {
        visible: column === 0
        width: 1
        color: "black"

        anchors {
            top: parent.top
            bottom: parent.bottom
            left: parent.left
            topMargin: row === 0 ? 0 : -1 * (rowSpacing + 1) / 2
            bottomMargin: -1 * (rowSpacing + 1) / 2
        }

    }

    Rectangle {
        height: 1
        color: "black"

        anchors {
            bottom: parent.bottom
            left: parent.left
            right: parent.right
            bottomMargin: -1 * (rowSpacing + 1) / 2
            leftMargin: column === 0 ? 0 : -1 * (columnSpacing + 1) / 2
            rightMargin: -1 * (columnSpacing + 1) / 2
        }

    }

    Rectangle {
        width: 1
        color: "black"

        anchors {
            top: parent.top
            bottom: parent.bottom
            right: parent.right
            topMargin: row === 0 ? 0 : -1 * (rowSpacing + 1) / 2
            bottomMargin: -1 * (rowSpacing + 1) / 2
            rightMargin: -1 * (columnSpacing + 1) / 2
        }

    }

    Text {
        id: cellText

        text: display
        font: parent.font
        anchors.centerIn: parent
        padding: 3
    }

}