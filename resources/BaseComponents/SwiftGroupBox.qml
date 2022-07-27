import QtQuick
import QtQuick.Controls

GroupBox {
    id: control

    property color color: "transparent"
    property color borderColor: control.palette.mid
    property real borderWidth: 1

    padding: 7
    spacing: 0
    bottomPadding: padding

    background: Rectangle {
        y: control.topPadding - control.bottomPadding
        width: parent.width
        height: parent.height - control.topPadding + control.bottomPadding
        color: control.color
        border.color: control.borderColor
        border.width: control.borderWidth
    }
}
