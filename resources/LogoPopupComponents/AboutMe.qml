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
        }

    }

    Label {
        Layout.alignment: Qt.AlignHCenter
        text: "Swift Navigation Console " + Globals.consoleVersion
        font.pointSize: Constants.logoPopup.aboutMe.titlePointSize
        font.bold: true
    }

    Label {
        Layout.alignment: Qt.AlignHCenter
        text: Constants.logoPopup.aboutMe.copyrightText
        font.pointSize: Constants.logoPopup.aboutMe.secondaryPointSize
    }

    Label {
        readonly property string website: Constants.logoPopup.aboutMe.supportWebsite
        readonly property string websiteDisplay: website.slice(12) // trim https://www.

        Layout.alignment: Qt.AlignHCenter
        text: `Find help online at <a href='${website}'>${websiteDisplay}</a>`
        font.pointSize: Constants.logoPopup.aboutMe.secondaryPointSize
        onLinkActivated: {
            Qt.openUrlExternally(website);
        }
    }

    Label {
        readonly property string website: Constants.logoPopup.aboutMe.website
        readonly property string websiteDisplay: website.slice(12)

        Layout.alignment: Qt.AlignHCenter
        text: `Learn more at <a href='${website}'>${websiteDisplay}</a>`
        font.pointSize: Constants.logoPopup.aboutMe.secondaryPointSize
        onLinkActivated: {
            Qt.openUrlExternally(Constants.logoPopup.aboutMe.website);
        }
    }

}
