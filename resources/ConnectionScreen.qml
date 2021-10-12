import "Constants"
import QtQuick 2.15
import QtQuick.Controls 2.15

Item {

    Rectangle {
        // color: "red"
        // // visible: true
        width: parent.width
        height: parent.height
        anchors.centerIn: parent

        Dialog {
            id: dialog

            visible: true
            height: parent.height - 50
            width: parent.height
            // width: Constants.width / 2
            // height: parent.height - Constants.licensesPopup.dialogPopupHeightPadding
            // anchors.fill: parent
            anchors.centerIn: parent

            Image {
                source: Constants.icons.splashScreenPath
            }
            onRejected: {
                stack.pop()
            }
        }
    }
    
    
    
}
