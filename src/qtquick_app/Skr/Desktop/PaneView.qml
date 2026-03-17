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
import Skr
import Skr.Desktop.Views

Rectangle {
    id: root

    required property bool isActive
    required property int paneIndex
    required property var paneState

    signal closeRequested
    signal focusRequested

    border.color: isActive ? "#0078d4" : "#e0e0e0"
    border.width: isActive ? 2 : 1
    color: isActive ? "#f5f5f5" : "#ffffff"

    FocusScope {
        anchors.fill: parent
        focus: true

        onActiveFocusChanged: {
            if (activeFocus)
                root.focusRequested();
        }

        TapHandler {
            onTapped: root.forceActiveFocus()
        }

        // --- Pane header ---
        Rectangle {
            id: header

            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            color: paneState.isLocked ? "#fff4e6" : "#f0f0f0"
            height: 32

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 8
                anchors.rightMargin: 8
                spacing: 4

                // Back / Forward buttons
                ToolButton {
                    font.pixelSize: 10
                    enabled: paneState.havePrevious
                    text: "<"
                    onClicked: paneState.navigateToPrevious()
                }
                ToolButton {
                    font.pixelSize: 10
                    enabled: paneState.haveNext
                    text: ">"
                    onClicked: paneState.navigateToNext()
                }

                // Title label
                Label {
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                    font.bold: paneState.isLocked
                    font.pixelSize: 12
                    text: {
                        let view = paneState.currentPaneViewState;
                        if (!view)
                            return "Empty";
                        switch (view.paneViewContentType()) {
                        case PaneViewState.Tree:
                            return paneState.isLocked ? "Tree (locked)" : "Tree";
                        case PaneViewState.CascadingList:
                            return "List " + view.contentId();
                        case PaneViewState.TextContent:
                            return "Content " + view.contentId();
                        case PaneViewState.Overview:
                            return "Overview";
                        default:
                            return "Empty";
                        }
                    }
                }

                // Context menu button
                ToolButton {
                    text: "..."
                    visible: !paneState.isLocked

                    onClicked: contextMenu.popup()

                    Menu {
                        id: contextMenu

                        MenuItem {
                            text: "Show Tree Here"
                            onTriggered: PaneLayoutController.openInPane(paneIndex, PaneViewState.Tree)
                        }
                        MenuItem {
                            text: "Show Column View Here"
                            onTriggered: PaneLayoutController.openInPane(paneIndex, PaneViewState.CascadingList)
                        }
                        MenuItem {
                            text: "Show Card View Here"
                            onTriggered: PaneLayoutController.openInPane(paneIndex, PaneViewState.Overview)
                        }
                        MenuSeparator {}
                        MenuItem {
                            enabled: PaneLayoutController.canSplitMore
                            text: "Split Right"
                            onTriggered: PaneLayoutController.splitPane(paneIndex, PaneLayoutController.Right, PaneViewState.Tree)
                        }
                        MenuSeparator {}
                        MenuItem {
                            checkable: true
                            checked: paneState.isLocked
                            enabled: paneIndex === 0
                            text: "Lock as Tree Pane"
                            onTriggered: PaneLayoutController.treeLockedLeft = checked
                        }
                    }
                }

                // Close button
                ToolButton {
                    text: "x"
                    visible: !paneState.isLocked
                    onClicked: closeRequested()
                }
            }
        }

        // --- Content area ---
        Item {
            id: contentArea

            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.margins: 1
            anchors.right: parent.right
            anchors.top: header.bottom

            Loader {
                id: contentLoader

                anchors.fill: parent
                sourceComponent: {
                    let view = paneState.currentPaneViewState;
                    if (!view)
                        return emptyComponent;
                    switch (view.paneViewContentType()) {
                    case PaneViewState.Tree:
                        return treeComponent;
                    case PaneViewState.CascadingList:
                        return cascadingListComponent;
                    case PaneViewState.TextContent:
                        return contentComponent;
                    case PaneViewState.Overview:
                        return overviewComponent;
                    default:
                        return emptyComponent;
                    }
                }
            }

            // --- Pane-level drop zones (edges only, don't cover center) ---
            property int edgeZoneWidth: Math.max(16, Math.min(contentArea.width * 0.05, 32))

            // Left edge drop zone
            DropArea {
                id: leftDropArea
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                width: contentArea.edgeZoneWidth
                keys: ["binderItem"]
                enabled: PaneLayoutController.canSplitMore

                onDropped: drop => {
                    let binderItemId = parseInt(drop.text);
                    if (!isNaN(binderItemId)) {
                        PaneLayoutController.handleDrop(paneIndex, PaneLayoutController.LeftEdge,
                                                         PaneViewState.TextContent, binderItemId);
                        drop.accept();
                    }
                }

                Rectangle {
                    anchors.fill: parent
                    color: "transparent"
                    Rectangle {
                        anchors.left: parent.left
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        width: 4
                        color: "#600078d4"
                        visible: leftDropArea.containsDrag
                    }
                }
            }

            // Right edge drop zone
            DropArea {
                id: rightDropArea
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                anchors.right: parent.right
                width: contentArea.edgeZoneWidth
                keys: ["binderItem"]
                enabled: PaneLayoutController.canSplitMore

                onDropped: drop => {
                    let binderItemId = parseInt(drop.text);
                    if (!isNaN(binderItemId)) {
                        PaneLayoutController.handleDrop(paneIndex, PaneLayoutController.RightEdge,
                                                         PaneViewState.TextContent, binderItemId);
                        drop.accept();
                    }
                }

                Rectangle {
                    anchors.fill: parent
                    color: "transparent"
                    Rectangle {
                        anchors.right: parent.right
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        width: 4
                        color: "#600078d4"
                        visible: rightDropArea.containsDrag
                    }
                }
            }

            // Bottom edge drop zone (open here)
            DropArea {
                id: bottomDropArea
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: contentArea.edgeZoneWidth
                anchors.rightMargin: contentArea.edgeZoneWidth
                height: contentArea.edgeZoneWidth
                keys: ["binderItem"]

                onDropped: drop => {
                    let binderItemId = parseInt(drop.text);
                    if (!isNaN(binderItemId)) {
                        PaneLayoutController.handleDrop(paneIndex, PaneLayoutController.Center,
                                                         PaneViewState.TextContent, binderItemId);
                        drop.accept();
                    }
                }

                Rectangle {
                    anchors.bottom: parent.bottom
                    anchors.left: parent.left
                    anchors.right: parent.right
                    height: 4
                    color: "#600078d4"
                    visible: bottomDropArea.containsDrag
                }
            }

            // Locked forward overlay (full area, only for locked panes)
            DropArea {
                id: lockedDropArea
                anchors.fill: parent
                keys: ["binderItem"]
                enabled: paneState.isLocked && PaneLayoutController.treeLockedLeft

                onDropped: drop => {
                    let binderItemId = parseInt(drop.text);
                    if (!isNaN(binderItemId)) {
                        PaneLayoutController.handleDrop(paneIndex, PaneLayoutController.LockedForward,
                                                         PaneViewState.TextContent, binderItemId);
                        drop.accept();
                    }
                }

                Rectangle {
                    anchors.fill: parent
                    color: "#20ffa040"
                    visible: lockedDropArea.containsDrag
                }
            }
        }

        // --- Content components ---
        Component {
            id: emptyComponent

            Rectangle {
                color: "#fafafa"

                Label {
                    anchors.centerIn: parent
                    color: "#aaa"
                    text: "Empty pane"
                }
            }
        }

        Component {
            id: treeComponent

            NavigationTree {
                paneIndex: root.paneIndex
            }
        }

        Component {
            id: cascadingListComponent

            BinderItemCascadingListView {}
        }

        Component {
            id: contentComponent

            Rectangle {
                color: "#ffffff"

                Label {
                    anchors.centerIn: parent
                    color: "#666"
                    text: "Content Editor (stub)"
                }
            }
        }

        Component {
            id: overviewComponent

            BinderItemCardView {}
        }
    }
}
