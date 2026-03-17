/*
 * Copyright (C) 2026 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Skr.Models
import Skr.Controllers
import "TreeHelper.js" as Tree

Item {
    id: root

    property int binderId: 1

    // Multi-selection
    property var selectedIds: new Set()
    property int selectionVersion: 0
    property int lastSelectedIndex: -1

    // Drop state
    property int dropTargetItemId: -1
    property string dropZone: ""

    // Column path
    property var columnPath: [{ parentItemId: -1, selectedItemId: -1 }]
    property int pathVersion: 0

    BinderBinderItemsListModel {
        id: binderModel
        binderId: root.binderId
    }

    BinderItemManagementController {
        id: moveController
    }

    // Drag state
    property var draggedItemIds: []
    property string draggedTitle: ""
    property bool dragging: false

    Rectangle {
        id: dragGhost
        visible: root.dragging
        z: 1000
        width: ghostRow.implicitWidth + 20
        height: 28
        color: "#e8f0ff"
        border.color: "#4488cc"
        radius: 4
        opacity: 0.9

        Row {
            id: ghostRow
            anchors.centerIn: parent
            spacing: 6

            Label {
                text: root.draggedTitle
                font.pixelSize: 11
            }
            Rectangle {
                visible: root.draggedItemIds.length > 1
                width: cntLabel.implicitWidth + 8
                height: 18
                radius: 9
                color: "#0078d4"
                anchors.verticalCenter: parent.verticalCenter

                Label {
                    id: cntLabel
                    anchors.centerIn: parent
                    text: root.draggedItemIds.length
                    font.pixelSize: 10
                    font.bold: true
                    color: "white"
                }
            }
        }
    }

    ScrollView {
        anchors.fill: parent

        Row {
            id: columnsRow
            height: root.height
            spacing: 0

            Repeater {
                id: columnsRepeater
                model: {
                    root.pathVersion;
                    return root.columnPath.length;
                }

                delegate: Rectangle {
                    id: columnRect
                    width: 220
                    height: columnsRow.height
                    border.color: "#e0e0e0"
                    border.width: 1
                    color: "#fafafa"

                    required property int index

                    property int parentItemId: {
                        root.pathVersion;
                        return root.columnPath[index].parentItemId;
                    }
                    property int columnSelectedId: {
                        root.pathVersion;
                        return root.columnPath[index].selectedItemId;
                    }
                    property var columnItems: {
                        root.pathVersion;
                        return Tree.getChildItems(binderModel, parentItemId);
                    }

                    Rectangle {
                        id: columnHeader
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        height: 28
                        color: "#f0f0f0"
                        border.color: "#e0e0e0"
                        border.width: 1

                        Label {
                            anchors.centerIn: parent
                            font.pixelSize: 11
                            font.bold: true
                            color: "#666"
                            text: {
                                if (parentItemId === -1) return "Root";
                                var idx = Tree.findIndexByItemId(binderModel, parentItemId);
                                return idx >= 0 ? binderModel.get(idx).title : "...";
                            }
                            elide: Text.ElideRight
                            width: parent.width - 16
                            horizontalAlignment: Text.AlignHCenter
                        }
                    }

                    ListView {
                        id: columnListView
                        anchors.top: columnHeader.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        clip: true
                        model: columnItems

                        // Drag proxy inside each ListView for correct coordinate space
                        Item {
                            id: colDragProxy
                            parent: columnListView.contentItem
                            width: 2; height: 2
                            visible: false

                            Drag.active: root.dragging
                            Drag.keys: ["binderItem"]
                            Drag.hotSpot: Qt.point(1, 1)
                            Drag.source: colDragProxy
                        }

                        delegate: Item {
                            id: columnItem
                            width: columnListView.width
                            height: 36

                            required property var modelData
                            required property int index

                            property bool isColumnSelected: modelData.itemId === columnRect.columnSelectedId
                            property bool isItemSelected: {
                                root.selectionVersion;
                                return root.selectedIds.has(modelData.itemId);
                            }
                            property bool isDragged: root.dragging && root.draggedItemIds.indexOf(modelData.itemId) >= 0
                            property bool isDropTarget: root.dropTargetItemId === modelData.itemId
                            property string currentDropZone: isDropTarget ? root.dropZone : ""
                            property bool itemHasChildren: Tree.hasChildren(binderModel, modelData.modelIndex)

                            opacity: isDragged ? 0.4 : 1.0

                            // Drop indicators
                            Rectangle {
                                visible: currentDropZone === "before"
                                anchors.top: parent.top
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.margins: 2
                                height: 2
                                color: "#0078d4"
                                z: 5
                            }
                            Rectangle {
                                visible: currentDropZone === "after"
                                anchors.bottom: parent.bottom
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.margins: 2
                                height: 2
                                color: "#0078d4"
                                z: 5
                            }

                            Rectangle {
                                anchors.fill: parent
                                anchors.margins: 2
                                radius: 4
                                color: currentDropZone === "into" ? "#d0e0ff"
                                     : currentDropZone !== "" ? "transparent"
                                     : isItemSelected ? "#0078d4"
                                     : isColumnSelected ? "#e0e8ff"
                                     : (columnHover.hovered && !root.dragging) ? "#f0f0f0"
                                     : "transparent"
                                border.color: currentDropZone === "into" ? "#0078d4" : "transparent"
                                border.width: currentDropZone === "into" ? 1 : 0

                                RowLayout {
                                    anchors.fill: parent
                                    anchors.leftMargin: 8
                                    anchors.rightMargin: 8
                                    spacing: 4

                                    Label {
                                        Layout.preferredWidth: 18
                                        text: Tree.roleIcon(modelData.role)
                                        font.pixelSize: 13
                                    }

                                    Label {
                                        Layout.fillWidth: true
                                        text: modelData.title
                                        elide: Text.ElideRight
                                        font.pixelSize: 12
                                        font.italic: !modelData.activated
                                        color: isItemSelected ? "white"
                                             : modelData.activated ? "#1a1a1a" : "#999"
                                    }

                                    Label {
                                        visible: modelData.isFavorite
                                        text: "\u2605"
                                        font.pixelSize: 10
                                        color: isItemSelected ? "#ffd700" : "#e6a800"
                                    }

                                    Label {
                                        visible: itemHasChildren
                                        text: "\u25B8"
                                        font.pixelSize: 11
                                        color: isItemSelected ? "white" : "#aaa"
                                    }
                                }
                            }

                            HoverHandler {
                                id: columnHover
                            }

                            TapHandler {
                                onTapped: {
                                    root.handleTap(modelData.modelIndex, modelData.itemId, point.modifiers);
                                    root.selectInColumn(columnRect.index, modelData.itemId);
                                }
                            }

                            DragHandler {
                                id: colDragHandler
                                target: null

                                onActiveChanged: {
                                    if (active) {
                                        root.startDrag(modelData.modelIndex, modelData.itemId, modelData.title);
                                    } else {
                                        root.endDrag();
                                    }
                                }
                            }

                            // Move drag proxy (in ListView contentItem coords)
                            Binding {
                                when: colDragHandler.active
                                target: colDragProxy
                                property: "x"
                                value: columnItem.x + colDragHandler.centroid.position.x
                                restoreMode: Binding.RestoreNone
                            }
                            Binding {
                                when: colDragHandler.active
                                target: colDragProxy
                                property: "y"
                                value: columnItem.y + colDragHandler.centroid.position.y
                                restoreMode: Binding.RestoreNone
                            }

                            // Ghost position (in root coords)
                            Binding {
                                when: colDragHandler.active
                                target: dragGhost
                                property: "x"
                                value: columnItem.mapToItem(root,
                                    colDragHandler.centroid.position.x,
                                    colDragHandler.centroid.position.y).x + 12
                                restoreMode: Binding.RestoreNone
                            }
                            Binding {
                                when: colDragHandler.active
                                target: dragGhost
                                property: "y"
                                value: columnItem.mapToItem(root,
                                    colDragHandler.centroid.position.x,
                                    colDragHandler.centroid.position.y).y - 14
                                restoreMode: Binding.RestoreNone
                            }

                            DropArea {
                                anchors.fill: parent
                                keys: ["binderItem"]

                                onPositionChanged: function(drag) {
                                    if (Tree.isInvalidDropTarget(binderModel, root.draggedItemIds, modelData.itemId)) {
                                        root.dropTargetItemId = -1;
                                        root.dropZone = "";
                                        return;
                                    }
                                    root.dropTargetItemId = modelData.itemId;
                                    root.dropZone = Tree.computeDropZone(drag.y, columnItem.height, modelData.role);
                                }
                                onExited: {
                                    if (root.dropTargetItemId === modelData.itemId) {
                                        root.dropTargetItemId = -1;
                                        root.dropZone = "";
                                    }
                                }
                                onDropped: function(drop) {
                                    if (Tree.isInvalidDropTarget(binderModel, root.draggedItemIds, modelData.itemId))
                                        return;
                                    root.performDrop(root.draggedItemIds,
                                                     modelData.itemId, root.dropZone);
                                    root.dropTargetItemId = -1;
                                    root.dropZone = "";
                                    drop.accept();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    function handleTap(modelIndex, itemId, modifiers) {
        if (modifiers & Qt.ControlModifier) {
            if (selectedIds.has(itemId))
                selectedIds.delete(itemId);
            else
                selectedIds.add(itemId);
            lastSelectedIndex = modelIndex;
        } else if (modifiers & Qt.ShiftModifier) {
            if (lastSelectedIndex >= 0) {
                var range = Tree.visibleRange(binderModel, lastSelectedIndex, modelIndex, new Set());
                for (var i = 0; i < range.length; i++)
                    selectedIds.add(range[i]);
            }
        } else {
            selectedIds.clear();
            selectedIds.add(itemId);
            lastSelectedIndex = modelIndex;
        }
        selectionVersion++;
    }

    function startDrag(modelIndex, itemId, title) {
        if (!selectedIds.has(itemId)) {
            selectedIds.clear();
            selectedIds.add(itemId);
            lastSelectedIndex = modelIndex;
            selectionVersion++;
        }
        var ids = Tree.pruneSelection(binderModel, selectedIds);
        selectedIds.clear();
        for (var i = 0; i < ids.length; i++)
            selectedIds.add(ids[i]);
        selectionVersion++;

        draggedItemIds = ids;
        draggedTitle = ids.length === 1 ? title : ids.length + " items";
        dragging = true;
    }

    function endDrag() {
        dragging = false;
        dropTargetItemId = -1;
        dropZone = "";
    }

    function performDrop(itemIds, targetId, zone) {
        var movePlace;
        switch (zone) {
        case "before": movePlace = 0; break;
        case "after":  movePlace = 1; break;
        case "into":   movePlace = 2; break;
        default: return;
        }

        var dto = moveController.getMoveDto();
        dto.itemIds = itemIds;
        dto.targetId = targetId;
        dto.movePlace = movePlace;
        moveController.moveItems(dto);
        console.log("moveItems:", JSON.stringify(itemIds), "->", targetId, zone);
    }

    function selectInColumn(columnIndex, itemId) {
        var newPath = columnPath.slice(0, columnIndex + 1);
        newPath[columnIndex] = {
            parentItemId: newPath[columnIndex].parentItemId,
            selectedItemId: itemId
        };

        var idx = Tree.findIndexByItemId(binderModel, itemId);
        if (idx >= 0 && Tree.hasChildren(binderModel, idx))
            newPath.push({ parentItemId: itemId, selectedItemId: -1 });

        columnPath = newPath;
        pathVersion++;
    }

    function refreshColumns() {
        pathVersion++;
    }
}
