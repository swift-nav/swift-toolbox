import "../Constants"
import QtQuick
import QtQuick.Controls.Material
import QtQuick.Controls.Material.impl
import QtQuick.Controls.impl
import QtQuick.Templates as T

T.TabButton {
    id: control

    property color labelColor: !control.enabled ? control.Material.hintTextColor : control.down || control.checked ? Constants.swiftWhite : Constants.tabButtonUnselectedTextColor
    property color gradientStartColor: down || checked ? Qt.lighter(Constants.swiftGrey, 1.7) : hovered ? Qt.lighter(Constants.swiftControlBackground, 1.1) : Constants.swiftWhite
    property color backgroundColor: down || checked ? Constants.swiftGrey : hovered ? Qt.darker(Constants.swiftControlBackground, 1.1) : Constants.swiftControlBackground
    property bool border: true

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: 12
    spacing: 6
    icon.width: 24
    icon.height: 24
    icon.color: !enabled ? Material.hintTextColor : down || checked ? Constants.swiftWhite : Constants.tabButtonUnselectedTextColor

    font {
        family: "Roboto"
        pixelSize: Constants.largePixelSize
        bold: true
        capitalization: Font.MixedCase
    }

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        icon: control.icon
        text: control.text
        font: control.font
        color: control.labelColor
    }

    background: Rectangle {
        border.width: control.border ? 1 : 0
        border.color: Constants.spacerColor
        implicitHeight: control.Material.touchTarget
        clip: true
        color: backgroundColor

        gradient: Gradient {
            GradientStop {
                position: 0
                color: gradientStartColor
            }

            GradientStop {
                position: 1
                color: backgroundColor
            }
        }
    }
}
