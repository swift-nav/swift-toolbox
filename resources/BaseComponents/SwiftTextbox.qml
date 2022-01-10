import "../Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    property string text: ""
    property int rightMargin: 0
    property int leftMargin: 0
    property int topMargin: 0
    property int bottomMargin: 0
    property var labelHorizontalAlignment: Text.AlignRight
    property var fontFamily: Constants.genericTable.fontFamily
    property font labelFont: Qt.font({
        "family": Constants.genericTable.fontFamily,
        "pointSize": Constants.largePointSize
    })

    Label {
        text: parent.text
        anchors.fill: parent
        anchors.rightMargin: rightMargin
        anchors.leftMargin: leftMargin
        anchors.topMargin: topMargin
        anchors.bottomMargin: bottomMargin
        horizontalAlignment: labelHorizontalAlignment
        font: labelFont
    }

}
