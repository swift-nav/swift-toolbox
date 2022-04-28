/****************************************************************************
**
** Copyright (C) 2017 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the Qt Quick Controls 2 module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

import "../Constants"
import QtQuick
import QtQuick.Templates as T
import QtQuick.Controls.impl
import QtQuick.Controls.Material
import QtQuick.Controls.Material.impl

T.TabButton {
    id: control

    property color labelColor: !control.enabled ? control.Material.hintTextColor : control.down || control.checked ? "white" : Constants.tabButtonUnselectedTextColor
    property color gradientStartColor: down || checked ? Qt.lighter(Constants.swiftGrey, 1.7) : hovered ? Qt.lighter(Constants.swiftControlBackground, 1.1) : "white"
    property color backgroundColor: down || checked ? Constants.swiftGrey : hovered ? Qt.darker(Constants.swiftControlBackground, 1.1) : Constants.swiftControlBackground
    property bool border: true

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset,
                            implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset,
                             implicitContentHeight + topPadding + bottomPadding)

    padding: 12
    spacing: 6

    icon.width: 24
    icon.height: 24
    icon.color: !enabled ? Material.hintTextColor : down || checked ? "white" : Constants.tabButtonUnselectedTextColor

    font {
        family: "Roboto"
        pointSize: Constants.largePointSize
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
        border.color: "#C2C2C2"
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
