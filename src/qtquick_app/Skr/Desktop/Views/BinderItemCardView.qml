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
    property int currentParentId: -1
    property int itemsVersion: 0

    // Multi-selection
    property var selectedIds: new Set()
    property int selectionVersion: 0
    property int lastSelectedIndex: -1

    // Drop state
    property int dropTargetItemId: -1
    property string dropZone: ""

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
        width: ghostRow2.implicitWidth + 20
        height: 28
        color: "#e8f0ff"
        border.color: "#4488cc"
        radius: 4
        opacity: 0.9

        Row {
            id: ghostRow2
            anchors.centerIn: parent
            spacing: 6

            Label {
                text: root.draggedTitle
                font.pixelSize: 11
            }
            Rectangle {
                visible: root.draggedItemIds.length > 1
                width: cntLabel2.implicitWidth + 8
                height: 18
                radius: 9
                color: "#0078d4"
                anchors.verticalCenter: parent.verticalCenter

                Label {
                    id: cntLabel2
                    anchors.centerIn: parent
                    text: root.draggedItemIds.length
                    font.pixelSize: 10
                    font.bold: true
                    color: "white"
                }
            }
        }
    }

    // Breadcrumb navigation
    Rectangle {
        id: breadcrumb
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 36
        color: "#f5f5f5"
        border.color: "#e0e0e0"
        border.width: 1

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 8
            anchors.rightMargin: 8
            spacing: 4

            ToolButton {
                text: "\u2190"
                font.pixelSize: 14
                enabled: root.currentParentId !== -1
                onClicked: root.navigateUp()
            }

            Label {
                Layout.fillWidth: true
                text: root.breadcrumbText()
                elide: Text.ElideLeft
                font.pixelSize: 12
                color: "#333"
            }
        }
    }

    GridView {
        id: cardGrid
        anchors.top: breadcrumb.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: 8
        clip: true

        cellWidth: 180
        cellHeight: 220

        // Drag proxy inside GridView for correct coordinate space
        Item {
            id: cardDragProxy
            parent: cardGrid.contentItem
            width: 2; height: 2
            visible: false

            Drag.active: root.dragging
            Drag.keys: ["binderItem"]
            Drag.hotSpot: Qt.point(1, 1)
            Drag.source: cardDragProxy
        }

        model: {
            root.itemsVersion;
            return Tree.getChildItems(binderModel, root.currentParentId);
        }

        delegate: Item {
            id: cardWrapper
            width: cardGrid.cellWidth
            height: cardGrid.cellHeight

            required property var modelData
            required property int index

            property bool isCardSelected: {
                root.selectionVersion;
                return root.selectedIds.has(modelData.itemId);
            }
            property bool isDragged: root.dragging && root.draggedItemIds.indexOf(modelData.itemId) >= 0
            property bool isDropTarget: root.dropTargetItemId === modelData.itemId
            property string currentDropZone: isDropTarget ? root.dropZone : ""
            property bool cardHasChildren: Tree.hasChildren(binderModel, modelData.modelIndex)

            opacity: isDragged ? 0.4 : 1.0

            // Drop indicators for cards — left/right lines
            Rectangle {
                visible: currentDropZone === "before"
                anchors.top: card.top
                anchors.bottom: card.bottom
                anchors.left: card.left
                width: 3
                color: "#0078d4"
                radius: 1
                z: 5
            }
            Rectangle {
                visible: currentDropZone === "after"
                anchors.top: card.top
                anchors.bottom: card.bottom
                anchors.right: card.right
                width: 3
                color: "#0078d4"
                radius: 1
                z: 5
            }

            Rectangle {
                id: card
                anchors.fill: parent
                anchors.margins: 6
                radius: 8
                color: currentDropZone === "into" ? "#d0e0ff"
                     : currentDropZone !== "" ? "#f0f8ff"
                     : "white"
                border.color: currentDropZone === "into" ? "#0078d4"
                            : currentDropZone !== "" ? "#80b0e0"
                            : isCardSelected ? "#0078d4"
                            : (cardHover.hovered && !root.dragging) ? "#b0b0b0"
                            : "#e0e0e0"
                border.width: (isCardSelected || currentDropZone !== "") ? 2 : 1

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 6

                    Rectangle {
                        Layout.preferredHeight: 22
                        Layout.preferredWidth: roleBadgeRow.implicitWidth + 12
                        radius: 11
                        color: modelData.role === "folder" ? "#e3f2fd" : "#e8f5e9"

                        Row {
                            id: roleBadgeRow
                            anchors.centerIn: parent
                            spacing: 4

                            Label {
                                text: Tree.roleIcon(modelData.role)
                                font.pixelSize: 11
                            }
                            Label {
                                text: modelData.role
                                font.pixelSize: 10
                                font.bold: true
                                color: "#555"
                            }
                        }
                    }

                    Label {
                        Layout.fillWidth: true
                        text: modelData.title
                        wrapMode: Text.WordWrap
                        maximumLineCount: 3
                        elide: Text.ElideRight
                        font.pixelSize: 14
                        font.bold: true
                        font.italic: !modelData.activated
                        color: modelData.activated ? "#1a1a1a" : "#999"
                    }

                    Label {
                        Layout.fillWidth: true
                        visible: modelData.subTitle !== ""
                        text: modelData.subTitle
                        wrapMode: Text.WordWrap
                        maximumLineCount: 2
                        elide: Text.ElideRight
                        font.pixelSize: 11
                        color: "#777"
                    }

                    Item { Layout.fillHeight: true }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        Rectangle {
                            visible: modelData.label !== ""
                            Layout.preferredHeight: 18
                            Layout.preferredWidth: labelText.implicitWidth + 10
                            radius: 9
                            color: {
                                switch (modelData.label) {
                                case "done": return "#c8e6c9";
                                case "draft": return "#fff9c4";
                                case "outline": return "#ffe0b2";
                                default: return "#e0e0e0";
                                }
                            }

                            Label {
                                id: labelText
                                anchors.centerIn: parent
                                text: modelData.label
                                font.pixelSize: 9
                                color: "#555"
                            }
                        }

                        Item { Layout.fillWidth: true }

                        Label {
                            visible: modelData.isFavorite
                            text: "\u2605"
                            font.pixelSize: 12
                            color: "#e6a800"
                        }

                        Label {
                            visible: cardHasChildren
                            text: "\u25B6"
                            font.pixelSize: 10
                            color: "#aaa"
                        }
                    }
                }
            }

            HoverHandler {
                id: cardHover
            }

            TapHandler {
                onTapped: root.handleTap(modelData.modelIndex, modelData.itemId, point.modifiers)
                onDoubleTapped: {
                    if (cardHasChildren) {
                        root.currentParentId = modelData.itemId;
                        root.selectedIds.clear();
                        root.selectionVersion++;
                        root.itemsVersion++;
                    }
                }
            }

            DragHandler {
                id: cardDragHandler
                target: null

                onActiveChanged: {
                    if (active) {
                        root.startDrag(modelData.modelIndex, modelData.itemId, modelData.title);
                    } else {
                        root.endDrag();
                    }
                }
            }

            // Move drag proxy (in GridView contentItem coords)
            Binding {
                when: cardDragHandler.active
                target: cardDragProxy
                property: "x"
                value: cardWrapper.x + cardDragHandler.centroid.position.x
                restoreMode: Binding.RestoreNone
            }
            Binding {
                when: cardDragHandler.active
                target: cardDragProxy
                property: "y"
                value: cardWrapper.y + cardDragHandler.centroid.position.y
                restoreMode: Binding.RestoreNone
            }

            // Ghost position (in root coords)
            Binding {
                when: cardDragHandler.active
                target: dragGhost
                property: "x"
                value: cardWrapper.mapToItem(root,
                    cardDragHandler.centroid.position.x,
                    cardDragHandler.centroid.position.y).x + 12
                restoreMode: Binding.RestoreNone
            }
            Binding {
                when: cardDragHandler.active
                target: dragGhost
                property: "y"
                value: cardWrapper.mapToItem(root,
                    cardDragHandler.centroid.position.x,
                    cardDragHandler.centroid.position.y).y - 14
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
                    root.dropZone = Tree.computeDropZoneH(drag.x, cardWrapper.width, modelData.role);
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

    function navigateUp() {
        if (currentParentId === -1) return;
        var idx = Tree.findIndexByItemId(binderModel, currentParentId);
        if (idx < 0) { currentParentId = -1; itemsVersion++; return; }

        var item = binderModel.get(idx);
        if (item.indent === 0) {
            currentParentId = -1;
        } else {
            for (var i = idx - 1; i >= 0; i--) {
                if (binderModel.get(i).indent === item.indent - 1) {
                    currentParentId = binderModel.get(i).itemId;
                    break;
                }
            }
        }
        selectedIds.clear();
        selectionVersion++;
        itemsVersion++;
    }

    function breadcrumbText() {
        if (currentParentId === -1) return "All Items";
        var parts = [];
        var itemId = currentParentId;
        while (itemId !== -1) {
            var idx = Tree.findIndexByItemId(binderModel, itemId);
            if (idx < 0) break;
            var item = binderModel.get(idx);
            parts.unshift(item.title);
            if (item.indent === 0) break;
            var found = false;
            for (var i = idx - 1; i >= 0; i--) {
                if (binderModel.get(i).indent === item.indent - 1) {
                    itemId = binderModel.get(i).itemId;
                    found = true;
                    break;
                }
            }
            if (!found) break;
        }
        return parts.join(" \u203A ");
    }
}
