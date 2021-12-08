// This is the source to QtQuick2's Material CheckBox with some changes to make it smaller
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Controls.Material 2.12
import QtQuick.Controls.Material.impl 2.12
import QtQuick.Controls.impl 2.12
import QtQuick.Templates 2.12 as T

T.Button {
    id: control

    property color disabledColor: control.Material.buttonDisabledColor
    property color highlightedColor: control.Material.highlightedButtonColor
    property color color: control.Material.buttonColor

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    topInset: 6
    bottomInset: 6
    padding: 12
    horizontalPadding: padding - 4
    spacing: 6
    icon.width: 24
    icon.height: 24
    icon.color: !enabled ? Material.hintTextColor : flat && highlighted ? Material.accentColor : highlighted ? Material.primaryHighlightedTextColor : Material.foreground
    Material.elevation: flat ? control.down || control.hovered ? 2 : 0 : control.down ? 8 : 2
    Material.background: flat ? "transparent" : undefined

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        icon: control.icon
        text: control.text
        font: control.font
        color: !control.enabled ? control.Material.hintTextColor : control.flat && control.highlighted ? control.Material.accentColor : control.highlighted ? control.Material.primaryHighlightedTextColor : control.Material.foreground
    }

    background: Rectangle {
        implicitWidth: 64
        implicitHeight: control.Material.buttonHeight
        radius: 2
        color: !control.enabled ? control.disabledColor : control.highlighted ? highlightedColor : color
        // The layer is disabled when the button color is transparent so you can do
        // Material.background: "transparent" and get a proper flat button without needing
        // to set Material.elevation as well
        layer.enabled: control.enabled && control.Material.buttonColor.a > 0

        PaddedRectangle {
            y: parent.height - 4
            width: parent.width
            height: 4
            radius: 2
            topPadding: -2
            clip: true
            visible: control.checkable && (!control.highlighted || control.flat)
            // color: control.checked && control.enabled ? control.Material.accentColor : control.Material.secondaryTextColor
            color: control.checked && control.enabled ? (control.invertColor ? control.Material.accentColor : control.Material.secondaryTextColor) : (control.invertColor ? control.Material.secondaryTextColor : control.Material.accentColor)
        }

        Ripple {
            clipRadius: 2
            width: parent.width
            height: parent.height
            pressed: control.pressed
            anchor: control
            active: control.down || control.visualFocus || control.hovered
            color: control.flat && control.highlighted ? control.Material.highlightedRippleColor : control.Material.rippleColor
        }

        layer.effect: ElevationEffect {
            elevation: control.Material.elevation
        }

    }

}
