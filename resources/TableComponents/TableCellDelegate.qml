import "../Constants"
import QtQuick
import QtQuick.Controls

Rectangle {
    property alias font: cellText.font

    implicitHeight: cellText.implicitHeight
    border.color: Constants.genericTable.borderColor
    color: Constants.genericTable.cellColor

    Label {
        id: cellText

        anchors.fill: parent
        text: display
        font.family: Constants.genericTable.fontFamily
        font.pointSize: Constants.largePointSize
        elide: Text.ElideRight
        verticalAlignment: Text.AlignVCenter
        horizontalAlignment: Text.AlignLeft
        padding: Constants.genericTable.padding
    }

}
