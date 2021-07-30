import QtQuick 2.5
import QtQuick.Controls 2.15

Item {
    property alias dialogStack: stackView

    StackView {
        id: stackView

        anchors.fill: parent
        anchors.centerIn: parent
    }

}
