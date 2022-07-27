import QtQuick
import QtQuick.Controls

Item {
    property alias dialogStack: stackView

    StackView {
        id: stackView

        anchors.fill: parent
        anchors.centerIn: parent
    }
}
