/*
 *   Copyright 2015 Marco Martin <mart@kde.org>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Library General Public License as
 *   published by the Free Software Foundation; either version 2, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU Library General Public License for more details
 *
 *   You should have received a copy of the GNU Library General Public
 *   License along with this program; if not, write to the
 *   Free Software Foundation, Inc.,
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

import QtQuick 2.5
import QtQuick.Window 2.2
import QtQuick.Controls 2.0 as QQC2
import QtGraphicalEffects 1.0
import QtQuick.Layouts 1.2
import org.kde.kirigami 2.4
import "../templates/private" as P

/**
 * RefreshableScrollView is a scroll view for any Flickable that supports the
 * "scroll down to refresh" behavior, and also allows the contents of the
 * flickable to have more top margins in order to make possible to scroll down the list
 * to reach it with the thumb while using the phone with a single hand.
 *
 * Example usage:
 *
 * @code
 * import org.kde.kirigami 2.4 as Kirigami
 * [...]
 * 
 * Kirigami.RefreshableScrollView {
 *     id: view
 *     supportsRefreshing: true
 *     onRefreshingChanged: {
 *         if (refreshing) {
 *             myModel.refresh();
 *         }
 *     }
 *     ListView {
 *         //NOTE: MyModel doesn't come from the components,
 *         //it's purely an example on how it can be used together
 *         //some application logic that can update the list model
 *         //and signals when it's done.
 *         model: MyModel {
 *             onRefreshDone: view.refreshing = false;
 *         }
 *         delegate: BasicListItem {}
 *     }
 * }
 * [...]
 *
 * @endcode
 * 
 */
P.ScrollView {
    id: root

    /**
     * type: bool
     * If true the list is asking for refresh and will show a loading spinner.
     * it will automatically be set to true when the user pulls down enough the list.
     * This signals the application logic to start its refresh procedure.
     * The application itself will have to set back this property to false when done.
     */
    property bool refreshing: false

    /**
     * type: bool
     * If true the list supports the "pull down to refresh" behavior.
     */
    property bool supportsRefreshing: false

    /**
     * leftPadding: int
     * default contents padding at left
     */
    property int leftPadding: Units.gridUnit

    /**
     * topPadding: int
     * default contents padding at top
     */
    property int topPadding: Units.gridUnit

    /**
     * rightPadding: int
     * default contents padding at right
     */
    property int rightPadding: Units.gridUnit

    /**
     * bottomPadding: int
     * default contents padding at bottom
     */
    property int bottomPadding: Units.gridUnit

    /**
     * Set when this scrollview manages a whole page
     */
    property Page page

    property Item _swipeFilter

    children: [
        Item {
            id: busyIndicatorFrame
            z: 99
            y: root.flickableItem.verticalLayoutDirection === ListView.BottomToTop
                ? -root.flickableItem.contentY+height
                : -root.flickableItem.contentY-height
            width: root.flickableItem.width
            height: busyIndicator.height + Units.gridUnit * 2
            QQC2.BusyIndicator {
                id: busyIndicator
                anchors.centerIn: parent
                running: root.refreshing
                visible: root.refreshing
                //Android busywidget QQC seems to be broken at custom sizes
            }
            property int headerItemHeight: (root.flickableItem.headerItem
                    ? (root.flickableItem.headerItem.maximumHeight ? root.flickableItem.headerItem.maximumHeight : root.flickableItem.headerItem.height)
                    : 0)
            Rectangle {
                id: spinnerProgress
                anchors {
                    fill: busyIndicator
                    margins: Math.ceil(Units.smallSpacing/2)
                }
                radius: width
                visible: supportsRefreshing && !refreshing && progress > 0
                color: "transparent"
                opacity: 0.8
                border.color: Theme.backgroundColor
                border.width: Math.ceil(Units.smallSpacing/4)
                //also take into account the listview header height if present
                property real progress: supportsRefreshing && !refreshing ? ((parent.y - busyIndicatorFrame.headerItemHeight)/busyIndicatorFrame.height) : 0
            }
            ConicalGradient {
                source: spinnerProgress
                visible: spinnerProgress.visible
                anchors.fill: spinnerProgress
                gradient: Gradient {
                    GradientStop { position: 0.00; color: Theme.highlightColor }
                    GradientStop { position: spinnerProgress.progress; color: Theme.highlightColor }
                    GradientStop { position: spinnerProgress.progress + 0.01; color: "transparent" }
                    GradientStop { position: 1.00; color: "transparent" }
                }
            }

            onYChanged: {
                //it's overshooting enough and not reachable: start countdown for reachability

                if (y - busyIndicatorFrame.headerItemHeight > root.topPadding + Units.gridUnit && !applicationWindow().reachableMode) {
                    overshootResetTimer.running = true;
                //not reachable and not overshooting enough, stop reachability countdown
                } else if (typeof(applicationWindow) == "undefined" || !applicationWindow().reachableMode) {
                    //it's important it doesn't restart
                    overshootResetTimer.running = false;
                }

                if (!supportsRefreshing) {
                    return;
                }

                //also take into account the listview header height if present
                if (!root.refreshing && y - busyIndicatorFrame.headerItemHeight > busyIndicatorFrame.height/2 + topPadding) {
                    refreshTriggerTimer.running = true;
                } else {
                    refreshTriggerTimer.running = false;
                }
            }
            Timer {
                id: refreshTriggerTimer
                interval: 500
                onTriggered: {
                    //also take into account the listview header height if present
                    if (!root.refreshing && parent.y - busyIndicatorFrame.headerItemHeight > busyIndicatorFrame.height/2 + topPadding) {
                        root.refreshing = true;
                    }
                }
            }
            Connections {
                enabled: typeof applicationWindow !== "undefined" 
                target: typeof applicationWindow !== "undefined" ? applicationWindow() : null
                onReachableModeChanged: {
                    overshootResetTimer.running = applicationWindow().reachableMode;
                }
            }
            Timer {
                id: overshootResetTimer
                interval: (typeof applicationWindow !== "undefined"  && applicationWindow().reachableMode) ? 8000 : 2000
                onTriggered: {
                    //put it there because widescreen may have changed since timer start
                    if (!Settings.isMobile || (typeof applicationWindow !== "undefined"  && applicationWindow().wideScreen) || root.flickableItem.verticalLayoutDirection === ListView.BottomToTop) {
                        return;
                    }
                    applicationWindow().reachableMode = !applicationWindow().reachableMode;
                }
            }

            Binding {
                target: root.flickableItem
                property: "flickableDirection"
                value: Flickable.VerticalFlick
            }

            Binding {
                target: root.flickableItem
                property: "bottomMargin"
                value: root.page.bottomPadding
            }

            Binding {
                target: root.contentItem
                property: "width"
                value: root.flickableItem.width
            }
        }
    ]

    onHeightChanged: {
        if (!Window.window) {
            return;
        }
        var focusItem = Window.window.activeFocusItem;

        if (!focusItem) {
            return;
        }

        //NOTE: there is no function to know if an item is descended from another,
        //so we have to walk the parent hyerarchy by hand
        var isDescendent = false;
        var candidate = focusItem.parent;
        while (candidate) {
            if (candidate == root) {
                isDescendent = true;
                break;
            }
            candidate = candidate.parent;
        }
        if (!isDescendent) {
            return;
        }

        var cursorY = 0;
        if (focusItem.cursorPosition !== undefined) {
            cursorY = focusItem.positionToRectangle(focusItem.cursorPosition).y;
        }

        var pos = focusItem.mapToItem(root.contentItem, 0, cursorY);

        //focused item alreqady visible? add some margin for the space of the action buttons
        if (pos.y >= root.flickableItem.contentY && pos.y <= root.flickableItem.contentY + root.flickableItem.height - Units.gridUnit * 8) {
            return;
        }
        root.flickableItem.contentY = pos.y;
    }

    onLeftPaddingChanged: {
        //for gridviews do apply margins
        if (root.contentItem == root.flickableItem) {
            if (typeof root.flickableItem.cellWidth != "undefined") {
                flickableItem.anchors.leftMargin = leftPadding;
                flickableItem.anchors.rightMargin = rightPadding;
            } else {
                flickableItem.anchors.leftMargin = 0;
                flickableItem.anchors.rightMargin = 0;
            }
            flickableItem.anchors.rightMargin = 0;
            flickableItem.anchors.bottomMargin = 0;
        } else {
            flickableItem.anchors.leftMargin = leftPadding;
            flickableItem.anchors.topMargin = topPadding;
            flickableItem.anchors.rightMargin = rightPadding;
            flickableItem.anchors.bottomMargin = bottomPadding;
        }
    }
}
