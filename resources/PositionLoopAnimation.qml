import QtQuick

SequentialAnimation {
    id: anim

    property Item target: undefined
    property string property: ""
    property real startingPropertyValue: 0
    property real totalDuration: 700
    property bool reverse: false
    property real targetParentHeight: target.parent.height

    NumberAnimation {
        target: anim.target
        property: anim.property
        duration: anim.totalDuration / 2
        easing.type: Easing.InQuad
        to: reverse ? anim.targetParentHeight : -anim.target.height
    }

    PropertyAction {
        target: anim.target
        property: anim.property
        value: reverse ? -anim.target.height : anim.targetParentHeight
    }

    NumberAnimation {
        target: anim.target
        property: anim.property
        duration: anim.totalDuration / 2
        easing.type: Easing.OutQuad
        to: anim.startingPropertyValue
    }

}
