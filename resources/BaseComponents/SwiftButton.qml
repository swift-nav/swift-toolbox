/****************************************************************************
 **
 ** Copyright (c) 2022 Swift Navigation
 **
 ** Permission is hereby granted, free of charge, to any person obtaining a copy of
 ** this software and associated documentation files (the "Software"), to deal in
 ** the Software without restriction, including without limitation the rights to
 ** use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 ** the Software, and to permit persons to whom the Software is furnished to do so,
 ** subject to the following conditions:
 **
 ** The above copyright notice and this permission notice shall be included in all
 ** copies or substantial portions of the Software.
 **
 ** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 ** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 ** FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 ** COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 ** IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 ** CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 **
 ****************************************************************************/
// This is the source to QtQuick2's Material CheckBox with some changes to make it smaller
import "../Constants"
import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Controls.Material.impl
import QtQuick.Controls.impl
import QtQuick.Templates as T

T.Button {
    id: control

    property color disabledColor: control.Material.buttonDisabledColor
    property color highlightedColor: control.Material.highlightedButtonColor
    property color color: control.Material.buttonColor
    property color invertedColor: Constants.swiftWhite
    property bool showAccent: true
    property bool invertColor: false

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    topInset: 6
    bottomInset: 6
    padding: 12
    horizontalPadding: padding - 4
    spacing: 6
    icon.width: 24
    icon.height: 24
    icon.color: {
        var color;
        if (!enabled) {
            color = Material.hintTextColor;
        } else {
            if (flat && highlighted) {
                color = Material.accentColor;
            } else {
                if (highlighted) {
                    color = Material.primaryHighlightedTextColor;
                } else {
                    color = Material.foreground;
                }
            }
        }
        return color;
    }
    Material.elevation: flat ? control.down || control.hovered ? 2 : 0 : control.down ? 8 : 2
    Material.background: flat ? "transparent" : undefined

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        icon: control.icon
        text: control.text
        font: control.font
        color: {
            var color;
            if (!control.enabled) {
                color = control.Material.hintTextColor;
            } else {
                if (control.flat && control.highlighted) {
                    color = control.Material.accentColor;
                } else {
                    if (control.highlighted) {
                        color = control.Material.primaryHighlightedTextColor;
                    } else {
                        color = control.Material.foreground;
                    }
                }
            }
            return color;
        }
    }

    background: Rectangle {
        implicitWidth: 64
        implicitHeight: control.Material.buttonHeight
        radius: 2
        color: {
            var color;
            if (!control.enabled) {
                color = control.disabledColor;
            } else {
                if (control.highlighted) {
                    color = control.highlightedColor;
                } else {
                    if (control.invertColor) {
                        color = control.invertedColor;
                    } else {
                        color = control.color;
                    }
                }
            }
            return color;
        }
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
            visible: showAccent && control.checkable && (!control.highlighted || control.flat)
            color: control.checked && control.enabled ? control.Material.accentColor : control.Material.secondaryTextColor
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
