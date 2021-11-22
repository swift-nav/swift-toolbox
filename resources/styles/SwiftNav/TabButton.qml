/****************************************************************************
**
** Copyright (C) 2017 The Qt Company Ltd.
** Contact: http://www.qt.io/licensing/
**
** This file is part of the Qt Quick Controls 2 module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL3$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see http://www.qt.io/terms-conditions. For further
** information use the contact form at http://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPLv3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or later as published by the Free
** Software Foundation and appearing in the file LICENSE.GPL included in
** the packaging of this file. Please review the following information to
** ensure the GNU General Public License version 2.0 requirements will be
** met: http://www.gnu.org/licenses/gpl-2.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

import "../Constants"
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Controls.Material 2.12
import QtQuick.Controls.Material.impl 2.12
import QtQuick.Controls.impl 2.12
import QtQuick.Templates 2.12 as T

T.TabButton {
    id: control

    property color labelColor: !control.enabled ? control.Material.hintTextColor : control.down || control.checked ? "white" : Constants.tabButtonUnselectedTextColor
    property color gradientStartColor: down || checked ? Qt.lighter(Constants.swiftGrey, 1.7) : hovered ? Qt.lighter(Constants.swiftControlBackground, 1.1) : "white"
    property color backgroundColor: down || checked ? Constants.swiftGrey : hovered ? Qt.darker(Constants.swiftControlBackground, 1.1) : Constants.swiftControlBackground
    property bool border: true

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: 12
    spacing: 6
    font: Qt.font({
        "family": "Roboto",
        "pointSize": Constants.largePointSize,
        "bold": true,
        "capitalization": Font.MixedCase
    })
    icon.width: 24
    icon.height: 24
    icon.color: !enabled ? Material.hintTextColor : down || checked ? "white" : Constants.tabButtonUnselectedTextColor

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
