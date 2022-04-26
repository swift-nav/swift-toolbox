import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Controls.impl

// This defines a Side Navigation button as is specified in the Style mockup guidelines listed here:
// https://snav.slack.com/archives/C020JLK6PK8/p1637094700036500
// To make the buttons within exclusive, the user needs to create a ButtonGroup, and assign
// all buttons to that buttongroup
Button {
    id: control

    property QtObject buttonGroup
    property QtObject view: ListView.view
    property bool separator: true
    property bool silenceButtonGroupWarning: false

    ButtonGroup.group: buttonGroup
    width: view ? view.width : 0
    z: visualFocus ? 10 : control.checked || control.highlighted ? 5 : 1
    display: AbstractButton.TextUnderIcon
    checkable: true
    icon.width: 22
    icon.height: 22
    icon.color: control.checked || control.highlighted ? Qt.darker(Constants.swiftOrange, control.enabled ? 1 : 1.4) : control.flat && !control.down ? (control.visualFocus ? Constants.swiftOrange : control.palette.windowText) : Qt.darker("white", control.enabled ? 1 : 1.4)
    ToolTip.delay: 1000
    ToolTip.timeout: 5000
    ToolTip.visible: ToolTip.text.length != 0 && hovered
    font.pointSize: Constants.smallPointSize
    font.capitalization: Font.MixedCase
    font.letterSpacing: 0
    // No idea why the insets are set, but they need to be 0 so there are no gaps between buttons.
    topInset: 0
    bottomInset: 0
    // Padding controls the padding around items within the button. We want to minimize it, so set
    // to 0. Realistically if we weren't setting an explicit width (set by the SideBar width), we'd
    // want more padding than 0.
    padding: 0
    // Spacing controls the spacing between the icon and the text. Default is 6, we reduce this to
    // 3 to match the new style mockups.
    spacing: 3
    Component.onCompleted: {
        if (!silenceButtonGroupWarning)
            console.assert(buttonGroup != undefined, "No buttonGroup assigned to SideNavButton! Undesired behavior will result.");

    }

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        icon: control.icon
        text: control.text
        font: control.font
        color: control.checked || control.highlighted ? Qt.darker(control.palette.dark, control.enabled ? 1 : 1.5) : control.flat && !control.down ? (control.visualFocus ? Constants.swiftOrange : control.palette.windowText) : Qt.darker("white", control.enabled ? 1 : 1.4)
    }

    background: Rectangle {
        implicitWidth: 100
        implicitHeight: 40
        visible: !control.flat || control.down || control.checked || control.highlighted
        color: Color.blend(control.checked || control.highlighted ? Qt.darker("white", control.enabled ? 1 : 1.4) : Constants.sideNavBar.backgroundColor, control.palette.mid, control.down ? 0.5 : 0)
        border.color: Constants.swiftOrange
        border.width: control.visualFocus ? 1 : 0

        Rectangle {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            height: 1
            color: Qt.darker("white", control.enabled ? 1 : 1.4)
            visible: control.separator && !control.visualFocus && !(control.view && control.view.itemAtIndex(index + 1) ? control.view.itemAtIndex(index + 1).visualFocus : false)
        }

        Repeater {
            model: 2

            Rectangle {
                anchors.left: index == 0 ? parent.left : undefined
                anchors.right: index == 1 ? parent.right : undefined
                y: -1
                height: parent.height + 1
                width: 1
                color: Constants.swiftGrey
                visible: !control.visualFocus && (control.checked || control.highlighted) && control.enabled
            }

        }

    }

}
