import "../Constants"
// This is the source to QtQuick2's Material CheckBox with some changes to make it smaller
import QtQuick 2.12
import QtQuick.Controls.Material 2.12
import QtQuick.Controls.Material.impl 2.12
import QtQuick.Controls.impl 2.12
import QtQuick.Templates 2.12 as T

T.CheckBox {
    id: control

    spacing: 2
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    Component.onCompleted: {
        // changes size of inner checkmark
        control.indicator.children[0].height = 10;
        control.indicator.children[0].width = 10;
    }

    font {
        family: Constants.fontFamily
        pointSize: Constants.smallPointSize
        bold: true
    }

    indicator: CheckIndicator {
        height: 14
        width: 14
        x: control.text ? (control.mirrored ? control.width - width - control.rightPadding : control.leftPadding) : control.leftPadding + (control.availableWidth - width) / 2
        y: control.topPadding + (control.availableHeight - height) / 2
        control: control
        border.color: !control.enabled ? control.Material.hintTextColor : Color.blend(Constants.sideNavBar.backgroundColor, control.palette.mid, control.checked ? 0.5 : 0)

        Ripple {
            x: (parent.width - width) / 2
            y: (parent.height - height) / 2
            width: 20
            height: 20
            z: -1
            anchor: control
            pressed: control.pressed
            active: control.down || control.visualFocus || control.hovered
            color: control.Material.rippleColor
        }

    }

    contentItem: Text {
        leftPadding: control.indicator && !control.mirrored ? control.indicator.width + control.spacing : 0
        rightPadding: control.indicator && control.mirrored ? control.indicator.width + control.spacing : 0
        text: control.text
        font: control.font
        color: control.enabled ? control.Material.foreground : control.Material.hintTextColor
        elide: Text.ElideRight
        verticalAlignment: Text.AlignVCenter
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.WordWrap
    }

}
