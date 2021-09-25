// Note: To use draggable features, your model must have the following:
//   * Must be a QSortFilterProxyModel
//   * Must have a method called reorderColumn(column, pos), where column is the index of the
//     column that is being reordered, and pos is the position of the mouse press. The method
//     should do the reordering of the columns in the QSortFilterProxyModel, making sure to wrap
//     the logic between calls to beginMoveColumns and endMoveColumns so that the TableView knows
//     what to update.
//   * Must provide a javascript function that, when called, will reset the layout of all of the
//     SortableColumnHeadings. Assign that function to the headerRelayoutProvider property.
//     When the user drags a column to a new position, the header will be re-laid out.
//     Here is an example of this for a header made from a repeater:
//         function relayout() {
//             headerRepeater.model = 0
//             headerRepeater.model = table.model.columnCount()
//         }
// Note: To use sortable features, your model must have the following:
//   * Must be a QSortFilterProxyModel
//   * Must provide an implementation of the sort(column, sortOrder) method, where column is the
//     index of the column to sort, and sortOrder is Qt.AscendingOrder or Qt.DescendingOrder
//   * Code that uses this SortableColumnHeading will need to provide logic to reset all other
//     columns' sorting visual state by calling clearSorting(). Here is an example where the column
//     headings are created from a repeater:
//         onSorting: {
//             for (var i = 0; i < headerRepeater.model; ++i)
//                 if (i != index)
//                     headerRepeater.itemAt(i).clearSorting()
//         }

import QtQuick 2.15

Rectangle {
    id: root

    property int initialSortOrder: Qt.AscendingOrder
    property alias text: label.text
    property real initialWidth: 100
    property alias sortable: tap.enabled
    property alias reorderable: dragHandler.enabled
    property QtObject table: undefined
    property var headerRelayoutProvider: function() {
    }

    signal sorting()
    signal dropped(real x)

    function clearSorting() {
        state = "";
    }

    function nextState() {
        if (state == "")
            state = (initialSortOrder == Qt.DescendingOrder ? "down" : "up");
        else if (state == "up")
            state = "down";
        else
            state = "up";
        root.sorting();
    }

    color: "Dark Grey"
    implicitHeight: label.implicitHeight
    width: splitter.x + 6
    z: dragHandler.active ? 1 : 0
    onSortableChanged: {
        if (sortable && typeof table.model.sort === "undefined")
            console.warn("SortableColumnHeading: Model does not support sorting, but sortable enabled.");

    }
    onReorderableChanged: {
        if (reorderable && typeof table.model.reorderColumn === "undefined")
            console.warn("SortableColumnHeading: Model does not support reordering columns, but reorderable enabled.");

    }
    onDropped: (x) => {
        if (typeof table.model.reorderColumn !== "undefined") {
            table.model.reorderColumn(index, x);
            if (typeof headerRelayoutProvider === "undefined")
                console.warn("SortableColumnHeading: No headerRelayoutProvider specified. Undefined behavior when reordering headers.");
            else
                headerRelayoutProvider();
        }
    }
    onSorting: {
        if (typeof table.model.sort !== "undefined")
            table.model.sort(index, state == "up" ? Qt.AscendingOrder : Qt.DescendingOrder);

    }
    states: [
        State {
            name: "up"

            PropertyChanges {
                target: upDownIndicator
                visible: true
                rotation: 0
            }

            PropertyChanges {
                target: root
                color: "orange"
            }

        },
        State {
            name: "down"

            PropertyChanges {
                target: upDownIndicator
                visible: true
                rotation: 180
            }

            PropertyChanges {
                target: root
                color: "orange"
            }

        }
    ]

    Text {
        id: label

        anchors.verticalCenter: parent.verticalCenter
        padding: 3
        x: 4
        width: parent.width - 4
        text: (table && table.model) ? table.model.headerData(index, Qt.Horizontal) : ""
    }

    Text {
        id: upDownIndicator

        anchors.right: parent.right
        anchors.margins: 4
        anchors.verticalCenter: parent.verticalCenter
        text: "^"
        visible: false
    }

    TapHandler {
        id: tap

        enabled: false
        onTapped: nextState()
    }

    Item {
        id: splitter

        x: root.initialWidth - 6
        width: 12
        height: parent.height + 10

        DragHandler {
            yAxis.enabled: false
            onActiveChanged: {
                if (!active)
                    table.forceLayout();

            }
        }

    }

    DragHandler {
        id: dragHandler

        enabled: false
        onActiveChanged: {
            if (!active)
                root.dropped(centroid.scenePosition.x);

        }
    }

}
