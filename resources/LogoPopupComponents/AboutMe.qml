import "../Constants"
import QtQuick 2.5
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

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

        Layout.alignment: Qt.AlignHCenter
        text: "Find help at the Swift Navigation <a href='" + website + "'>support portal</a>"
        font.pointSize: Constants.logoPopup.aboutMe.secondaryPointSize
        onLinkActivated: {
            Qt.openUrlExternally(website);
        }
    }

    Label {
        Layout.alignment: Qt.AlignHCenter
        text: "Learn more at the <a href='" + Constants.logoPopup.aboutMe.website + "'>Swift Navigation website</a>"
        font.pointSize: Constants.logoPopup.aboutMe.secondaryPointSize
        onLinkActivated: {
            Qt.openUrlExternally(Constants.logoPopup.aboutMe.website);
        }
    }

}
