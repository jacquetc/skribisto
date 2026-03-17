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

    required property int paneIndex

    property int binderId: 1

    // Multi-selection
    property var selectedIds: new Set()
    property int selectionVersion: 0
    property int lastSelectedIndex: -1

    // Expand/collapse
    property var expandedIds: new Set()
    property int expandedVersion: 0

    // Drop state
    property int dropTargetItemId: -1
    property string dropZone: ""

    // Drag state
    property var draggedItemIds: []
    property string draggedTitle: ""
    property bool dragging: false

    BinderBinderItemsListModel {
        id: binderModel
        binderId: root.binderId
    }

    BinderItemManagementController {
        id: moveController
    }

    // Visual drag ghost
    Rectangle {
        id: dragGhost
        visible: root.dragging
        z: 1000
        width: dragGhostRow.implicitWidth + 20
        height: 28
        color: "#e8f0ff"
        border.color: "#4488cc"
        radius: 4
        opacity: 0.9

        Row {
            id: dragGhostRow
            anchors.centerIn: parent
            spacing: 6

            Label {
                text: root.draggedTitle
                font.pixelSize: 11
            }
            Rectangle {
                visible: root.draggedItemIds.length > 1
                width: countLabel.implicitWidth + 8
                height: 18
                radius: 9
                color: "#0078d4"
                anchors.verticalCenter: parent.verticalCenter

                Label {
                    id: countLabel
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
        id: scrollView
        anchors.fill: parent

        Flickable {
            id: flickable
            contentWidth: treeColumn.width
            contentHeight: treeColumn.height
            clip: true

            Column {
                id: treeColumn
                width: scrollView.availableWidth

                // Drag proxy — lives inside the Column so it shares
                // coordinate space with the delegates' DropAreas
                Item {
                    id: dragProxy
                    width: 2; height: 2
                    visible: false

                    Drag.active: root.dragging
                    Drag.keys: ["binderItem"]
                    Drag.hotSpot: Qt.point(1, 1)
                    Drag.source: dragProxy
                }

                Repeater {
                    model: binderModel

                    delegate: Item {
                        id: treeItem
                        width: treeColumn.width
                        height: itemVisible ? 36 : 0
                        visible: itemVisible

                        property bool itemVisible: {
                            root.expandedVersion;
                            return Tree.isVisible(binderModel, index, root.expandedIds);
                        }
                        property bool itemHasChildren: Tree.hasChildren(binderModel, index)
                        property bool itemExpanded: {
                            root.expandedVersion;
                            return root.expandedIds.has(model.itemId);
                        }
                        property bool isSelected: {
                            root.selectionVersion;
                            return root.selectedIds.has(model.itemId);
                        }
                        property bool isDragged: root.dragging && root.draggedItemIds.indexOf(model.itemId) >= 0
                        property bool isDropTarget: root.dropTargetItemId === model.itemId
                        property string currentDropZone: isDropTarget ? root.dropZone : ""

                        opacity: isDragged ? 0.4 : 1.0

                        // Drop indicator — top line
                        Rectangle {
                            visible: currentDropZone === "before"
                            anchors.top: parent.top
                            anchors.left: parent.left
                            anchors.leftMargin: model.indent * 24 + 4
                            anchors.right: parent.right
                            anchors.rightMargin: 4
                            height: 2
                            color: "#0078d4"
                            z: 5
                        }

                        // Drop indicator — bottom line
                        Rectangle {
                            visible: currentDropZone === "after"
                            anchors.bottom: parent.bottom
                            anchors.left: parent.left
                            anchors.leftMargin: model.indent * 24 + 4
                            anchors.right: parent.right
                            anchors.rightMargin: 4
                            height: 2
                            color: "#0078d4"
                            z: 5
                        }

                        Rectangle {
                            id: bg
                            anchors.fill: parent
                            anchors.leftMargin: model.indent * 24 + 4
                            anchors.rightMargin: 4
                            color: currentDropZone === "into" ? "#d0e0ff"
                                 : currentDropZone !== "" ? "transparent"
                                 : isSelected ? "#0078d4"
                                 : (hoverHandler.hovered && !root.dragging) ? "#f0f0f0"
                                 : "transparent"
                            radius: 4
                            border.color: currentDropZone === "into" ? "#0078d4" : "transparent"
                            border.width: currentDropZone === "into" ? 1 : 0

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 4
                                anchors.rightMargin: 8
                                spacing: 2

                                Label {
                                    id: arrowLabel
                                    Layout.preferredWidth: 16
                                    Layout.alignment: Qt.AlignVCenter
                                    text: treeItem.itemHasChildren
                                          ? (treeItem.itemExpanded ? "\u25BE" : "\u25B8")
                                          : ""
                                    font.pixelSize: 12
                                    color: isSelected ? "#c0d8ff" : "#666"
                                    horizontalAlignment: Text.AlignHCenter

                                    TapHandler {
                                        gesturePolicy: TapHandler.ReleaseWithinBounds
                                        onTapped: root.toggleExpanded(model.itemId)
                                    }
                                }

                                Label {
                                    Layout.preferredWidth: 18
                                    Layout.alignment: Qt.AlignVCenter
                                    text: Tree.roleIcon(model.role)
                                    font.pixelSize: 13
                                }

                                Label {
                                    Layout.fillWidth: true
                                    Layout.alignment: Qt.AlignVCenter
                                    text: model.title
                                    elide: Text.ElideRight
                                    font.pixelSize: 13
                                    font.bold: model.role === "folder" && model.indent === 0
                                    font.italic: !model.activated
                                    color: isSelected ? "white"
                                         : model.activated ? "#1a1a1a" : "#999"
                                }

                                Label {
                                    visible: model.isFavorite
                                    Layout.alignment: Qt.AlignVCenter
                                    text: "\u2605"
                                    font.pixelSize: 11
                                    color: isSelected ? "#ffd700" : "#e6a800"
                                }
                            }
                        }

                        HoverHandler {
                            id: hoverHandler
                        }

                        TapHandler {
                            onTapped: root.handleTap(index, model.itemId, point.modifiers)
                            onDoubleTapped: {
                                if (treeItem.itemHasChildren)
                                    root.toggleExpanded(model.itemId);
                            }
                        }

                        DragHandler {
                            id: itemDragHandler
                            target: null

                            onActiveChanged: {
                                if (active) {
                                    root.startDrag(index, model.itemId, model.title);
                                } else {
                                    root.endDrag();
                                }
                            }
                        }

                        // Move drag proxy to cursor position (in treeColumn coords)
                        Binding {
                            when: itemDragHandler.active
                            target: dragProxy
                            property: "x"
                            value: treeItem.x + itemDragHandler.centroid.position.x
                            restoreMode: Binding.RestoreNone
                        }
                        Binding {
                            when: itemDragHandler.active
                            target: dragProxy
                            property: "y"
                            value: treeItem.y + itemDragHandler.centroid.position.y
                            restoreMode: Binding.RestoreNone
                        }

                        // Ghost position (in root coords)
                        Binding {
                            when: itemDragHandler.active
                            target: dragGhost
                            property: "x"
                            value: treeItem.mapToItem(root,
                                itemDragHandler.centroid.position.x,
                                itemDragHandler.centroid.position.y).x + 12
                            restoreMode: Binding.RestoreNone
                        }
                        Binding {
                            when: itemDragHandler.active
                            target: dragGhost
                            property: "y"
                            value: treeItem.mapToItem(root,
                                itemDragHandler.centroid.position.x,
                                itemDragHandler.centroid.position.y).y - 14
                            restoreMode: Binding.RestoreNone
                        }

                        // Drop target with 3-zone detection
                        DropArea {
                            anchors.fill: parent
                            keys: ["binderItem"]

                            onPositionChanged: function(drag) {
                                if (Tree.isInvalidDropTarget(binderModel, root.draggedItemIds, model.itemId)) {
                                    root.dropTargetItemId = -1;
                                    root.dropZone = "";
                                    return;
                                }
                                root.dropTargetItemId = model.itemId;
                                root.dropZone = Tree.computeDropZone(drag.y, treeItem.height, model.role);
                            }

                            onExited: {
                                if (root.dropTargetItemId === model.itemId) {
                                    root.dropTargetItemId = -1;
                                    root.dropZone = "";
                                }
                            }

                            onDropped: function(drop) {
                                if (Tree.isInvalidDropTarget(binderModel, root.draggedItemIds, model.itemId))
                                    return;
                                root.performDrop(root.draggedItemIds,
                                                 model.itemId, root.dropZone);
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

    // --- Functions ---

    function handleTap(index, itemId, modifiers) {
        if (modifiers & Qt.ControlModifier) {
            if (selectedIds.has(itemId))
                selectedIds.delete(itemId);
            else
                selectedIds.add(itemId);
            lastSelectedIndex = index;
        } else if (modifiers & Qt.ShiftModifier) {
            if (lastSelectedIndex >= 0) {
                var range = Tree.visibleRange(binderModel, lastSelectedIndex, index, expandedIds);
                for (var i = 0; i < range.length; i++)
                    selectedIds.add(range[i]);
            }
        } else {
            selectedIds.clear();
            selectedIds.add(itemId);
            lastSelectedIndex = index;
        }
        selectionVersion++;
    }

    function startDrag(index, itemId, title) {
        if (!selectedIds.has(itemId)) {
            selectedIds.clear();
            selectedIds.add(itemId);
            lastSelectedIndex = index;
            selectionVersion++;
        }
        // Prune children whose parent is also selected
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

    function toggleExpanded(itemId) {
        if (expandedIds.has(itemId))
            expandedIds.delete(itemId);
        else
            expandedIds.add(itemId);
        expandedVersion++;
    }

    Component.onCompleted: {
        for (var i = 0; i < binderModel.count; i++) {
            if (Tree.hasChildren(binderModel, i))
                expandedIds.add(binderModel.get(i).itemId);
        }
        expandedVersion++;
    }
}
