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

import "../Constants"
import QtQuick 2.15

Rectangle {
    id: sortableColumnHeading

    property int initialSortOrder: Qt.AscendingOrder
    property alias text: label.text
    property real initialWidth: 100
    property alias horizontalAlignment: label.horizontalAlignment
    property alias sortable: tap.enabled
    property alias reorderable: dragHandler.enabled
    property QtObject table: undefined
    property var headerRelayoutProvider: function() {
    }
    property font font: Qt.font({
        "family": Constants.genericTable.fontFamily
    })
    property color gradientStartColor: Constants.genericTable.cellColor
    property color gradientStopColor: Constants.genericTable.gradientColor
    property color selectedCellColor: Constants.genericTable.selectedCellColor

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
        sortableColumnHeading.sorting();
    }

    border.color: Constants.genericTable.borderColor
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
                target: sortableColumnHeading
                gradientStopColor: selectedCellColor
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
                target: sortableColumnHeading
                gradientStopColor: selectedCellColor
            }

        }
    ]

    Text {
        id: label

        anchors.fill: parent
        text: (table && table.model) ? table.model.headerData(index, Qt.Horizontal) : ""
        font: sortableColumnHeading.font
        elide: Text.ElideMiddle
        verticalAlignment: Text.AlignVCenter
        horizontalAlignment: Text.AlignHCenter
        padding: 3
    }

    Text {
        id: upDownIndicator

        anchors.right: parent.right
        anchors.margins: 4
        anchors.verticalCenter: parent.verticalCenter
        visible: false
        text: "^"
        font: sortableColumnHeading.font
    }

    TapHandler {
        id: tap

        enabled: false
        onTapped: nextState()
    }

    Item {
        id: splitter

        x: sortableColumnHeading.initialWidth - 6
        onXChanged: {
            if (x < 0)
                x = 0;

        } // Prevent resizing cell smaller than 0
        width: 12
        height: parent.height + 10

        HoverHandler {
            cursorShape: Qt.SizeHorCursor
        }

        DragHandler {
            id: splitDragHandler

            yAxis.enabled: false
            dragThreshold: 1
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
                sortableColumnHeading.dropped(centroid.scenePosition.x);

        }
    }

    gradient: Gradient {
        GradientStop {
            position: 0
            color: sortableColumnHeading.gradientStartColor
        }

        GradientStop {
            position: 1
            color: sortableColumnHeading.gradientStopColor
        }

    }

}
