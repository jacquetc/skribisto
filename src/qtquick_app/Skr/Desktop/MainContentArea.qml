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
import Skr.Desktop

Item {
    id: mainContentArea

    Component.onCompleted: {
        PaneLayoutController.updateAvailableWidth(width);
        PaneLayoutController.initialize();
    }

    onWidthChanged: {
        PaneLayoutController.updateAvailableWidth(width);
    }

    // Breadcrumb / toolbar bar at top
    ToolBar {
        id: breadcrumbBar

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        height: 40

        RowLayout {
            anchors.fill: parent
            anchors.margins: 4

            Label {
                font.pixelSize: 14
                text: "Work > Binder > Chapter"
            }

            Item {
                Layout.fillWidth: true
            }

            // Overflow indicator
            ToolButton {
                text: PaneLayoutController.overflowCount + " hidden"
                visible: PaneLayoutController.overflowCount > 0

                onClicked: PaneLayoutController.restoreOverflowPane()
            }
        }
    }

    // Main split view area
    SplitView {
        id: splitView

        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: breadcrumbBar.bottom
        orientation: Qt.Horizontal

        Repeater {
            model: PaneLayoutController.panes

            PaneView {
                id: paneView

                required property int index
                required property var modelData

                SplitView.minimumWidth: PaneLayoutController.minPaneWidth
                SplitView.preferredWidth: modelData.widthRatio * splitView.width
                isActive: index === PaneLayoutController.activePaneIndex
                paneIndex: index
                paneState: modelData

                onCloseRequested: {
                    PaneLayoutController.closePane(index);
                }
                onFocusRequested: {
                    PaneLayoutController.focusPane(index, PaneLayoutController.Click);
                }
            }
        }
    }

    // --- Keyboard shortcuts ---

    // Esc or Ctrl+T: navigate to tree in active pane
    Shortcut {
        sequence: "Ctrl+T"
        onActivated: PaneLayoutController.navigateToTreeInActivePane()
    }

    // Ctrl+\: toggle split
    Shortcut {
        sequence: "Ctrl+\\"
        onActivated: {
            if (PaneLayoutController.canSplitMore) {
                if (PaneLayoutController.panes.length > 1) {
                    PaneLayoutController.closePane(PaneLayoutController.panes.length - 1);
                } else {
                    PaneLayoutController.splitPane(0, PaneLayoutController.Right, PaneViewState.Tree);
                }
            }
        }
    }

    // Ctrl+W: close active pane
    Shortcut {
        sequence: "Ctrl+W"
        onActivated: PaneLayoutController.closePane(PaneLayoutController.activePaneIndex)
    }

    // Ctrl+1/2/3: focus pane by number
    Shortcut {
        sequence: "Ctrl+1"
        onActivated: PaneLayoutController.focusPane(0, PaneLayoutController.KeyboardNav)
    }
    Shortcut {
        sequence: "Ctrl+2"
        onActivated: {
            if (PaneLayoutController.panes.length > 1)
                PaneLayoutController.focusPane(1, PaneLayoutController.KeyboardNav);
        }
    }
    Shortcut {
        sequence: "Ctrl+3"
        onActivated: {
            if (PaneLayoutController.panes.length > 2)
                PaneLayoutController.focusPane(2, PaneLayoutController.KeyboardNav);
        }
    }

    // Alt+Left / Alt+Right: per-pane back/forward history
    Shortcut {
        sequence: "Alt+Left"
        onActivated: {
            let pane = PaneLayoutController.panes[PaneLayoutController.activePaneIndex];
            if (pane && pane.havePrevious)
                pane.navigateToPrevious();
        }
    }
    Shortcut {
        sequence: "Alt+Right"
        onActivated: {
            let pane = PaneLayoutController.panes[PaneLayoutController.activePaneIndex];
            if (pane && pane.haveNext)
                pane.navigateToNext();
        }
    }
}
