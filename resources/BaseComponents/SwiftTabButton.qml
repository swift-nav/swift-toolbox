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
