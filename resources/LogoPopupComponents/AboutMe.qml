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
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    Item {
        Layout.fillWidth: true
        Layout.fillHeight: true

        Image {
            anchors.centerIn: parent
            height: Constants.logoPopup.aboutMe.logoWidth
            width: Constants.logoPopup.aboutMe.logoWidth
            source: Constants.icons.swiftLogoPath
            asynchronous: true
        }
    }

    Label {
        Layout.alignment: Qt.AlignHCenter
        text: "Swift Navigation Console " + Globals.consoleVersion
        font.pixelSize: Constants.logoPopup.aboutMe.titlePixelSize
        font.bold: true
    }

    Label {
        Layout.alignment: Qt.AlignHCenter
        text: Constants.logoPopup.aboutMe.copyrightText
        font.pixelSize: Constants.logoPopup.aboutMe.secondaryPixelSize
    }

    Label {
        readonly property string website: Constants.logoPopup.aboutMe.supportWebsite
        readonly property string websiteDisplay: website.slice(12) // trim https://www.

        Layout.alignment: Qt.AlignHCenter
        text: `Find help online at <a href='${website}'>${websiteDisplay}</a>`
        font.pixelSize: Constants.logoPopup.aboutMe.secondaryPixelSize
        onLinkActivated: {
            Qt.openUrlExternally(website);
        }
    }

    Label {
        readonly property string website: Constants.logoPopup.aboutMe.website
        readonly property string websiteDisplay: website.slice(12)

        Layout.alignment: Qt.AlignHCenter
        text: `Learn more at <a href='${website}'>${websiteDisplay}</a>`
        font.pixelSize: Constants.logoPopup.aboutMe.secondaryPixelSize
        onLinkActivated: {
            Qt.openUrlExternally(Constants.logoPopup.aboutMe.website);
        }
    }
}
