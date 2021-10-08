import QtQuick 2.12
import QtQuick.Templates 2.12 as T
import QtQuick.Controls.Material 2.12
import QtQuick.Controls.Material.impl 2.12

T.TabBar {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset,
                            contentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset,
                             contentHeight + topPadding + bottomPadding)

    spacing: -1

    contentItem: ListView {
        model: control.contentModel
        currentIndex: control.currentIndex

        spacing: control.spacing
        orientation: ListView.Horizontal
        boundsBehavior: Flickable.StopAtBounds
        flickableDirection: Flickable.AutoFlickIfNeeded
        snapMode: ListView.SnapToItem

        highlightMoveDuration: 250
        highlightResizeDuration: 0
        highlightFollowsCurrentItem: true
        highlightRangeMode: ListView.ApplyRange
        preferredHighlightBegin: 48
        preferredHighlightEnd: width - 48

        highlight: Item {
            z: 2
            Rectangle {
                x: 1
                height: 2
                width: parent.width - 2
                y: control.position === T.TabBar.Footer ? 0 : parent.height - height
                color: control.Material.accentColor
            }
        }
    }

    background: Item {
        Rectangle {
            anchors.fill: parent
            color: "white"

            layer.enabled: control.Material.elevation > 0
            layer.effect: ElevationEffect {
                elevation: control.Material.elevation
                fullWidth: true
            }
        }

        Rectangle {
            z: 2
            anchors.top: parent.top
            width: parent.width
            height: 1
            color: "#C2C2C2"
        }
        Rectangle {
            z: 2
            anchors.bottom: parent.bottom
            width: parent.width
            height: 1
            color: "#C2C2C2"
        }
    }
}
