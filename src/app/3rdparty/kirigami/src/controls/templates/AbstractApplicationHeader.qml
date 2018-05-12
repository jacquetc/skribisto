/*
 *   Copyright 2015 Marco Martin <mart@kde.org>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Library General Public License as
 *   published by the Free Software Foundation; either version 2 or
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
import QtQuick.Layouts 1.2
import "private"
import org.kde.kirigami 2.4


/**
 * An item that can be used as a title for the application.
 * Scrolling the main page will make it taller or shorter (trough the point of going away)
 * It's a behavior similar to the typical mobile web browser adressbar
 * the minimum, preferred and maximum heights of the item can be controlled with
 * * minimumHeight: default is 0, i.e. hidden
 * * preferredHeight: default is Units.gridUnit * 1.6
 * * preferredHeight: default is Units.gridUnit * 3
 *
 * To achieve a titlebar that stays completely fixed just set the 3 sizes as the same
 * @inherit QtQuick.Item
 */
Item {
    id: root
    z: 90
    property int minimumHeight: 0
    property int preferredHeight: Units.gridUnit * 2
    property int maximumHeight: Units.gridUnit * 3
    default property alias contentItem: mainItem.data
    readonly property int paintedHeight: headerItem.y + headerItem.height - 1
    LayoutMirroring.enabled: Qt.application.layoutDirection == Qt.RightToLeft 
    LayoutMirroring.childrenInherit: true

    //FIXME: remove
    property QtObject __appWindow: applicationWindow();

    anchors {
        left: parent.left
        right: parent.right
    }
    height: preferredHeight

    /**
     * background: Item
     * This property holds the background item.
     * Note: the background will be automatically sized as the whole control
     */
    property Item background

    onBackgroundChanged: {
        background.z = -1;
        background.parent = headerItem;
        background.anchors.fill = headerItem;
    }

    opacity: height > 0 ? 1 : 0
    Behavior on opacity {
        OpacityAnimator {
            duration: Units.longDuration
            easing.type: Easing.InOutQuad
        }
    }

    Behavior on height {
        enabled: __appWindow.pageStack.currentItem && __appWindow.pageStack.currentItem.flickable && !__appWindow.pageStack.currentItem.flickable.moving
        NumberAnimation {
            duration: Units.longDuration
            easing.type: Easing.InOutQuad
        }
    }

    Connections {
        target: __appWindow
        onControlsVisibleChanged: root.height = __appWindow.controlsVisible ? root.preferredHeight : 0;
    }

    Item {
        id: headerItem
        property real computedRootHeight: root.preferredHeight
        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
        }

        height: __appWindow.reachableMode && __appWindow.reachableModeEnabled ? root.maximumHeight : root.preferredHeight

        Connections {
            id: headerSlideConnection
            target: __appWindow.pageStack.currentItem ? __appWindow.pageStack.currentItem.flickable : null
            property int oldContentY
            onContentYChanged: {
                if (!Settings.isMobile ||
                    !__appWindow.controlsVisible ||
                    !__appWindow.pageStack.currentItem ||
                    __appWindow.pageStack.currentItem.flickable.atYBeginning ||
                    __appWindow.pageStack.currentItem.flickable.atYEnd) {
                    return;
                //if moves but not dragging, just update oldContentY
                } else if (!__appWindow.pageStack.currentItem.flickable.dragging) {
                    oldContentY = __appWindow.pageStack.currentItem.flickable.contentY;
                    return;
                } 
                

                if (__appWindow.wideScreen || !Settings.isMobile) {
                    root.height = root.preferredHeight;
                } else {
                    var oldHeight = root.height;

                    root.height = Math.max(root.minimumHeight,
                                            Math.min(root.preferredHeight,
                                                 root.height + oldContentY - __appWindow.pageStack.currentItem.flickable.contentY));
                
                    //if the height is changed, use that to simulate scroll
                    if (oldHeight != height) {
                        __appWindow.pageStack.currentItem.flickable.contentY = oldContentY;
                    } else {
                        oldContentY = __appWindow.pageStack.currentItem.flickable.contentY;
                    }
                }
            }
            onMovementEnded: {
                if (__appWindow.wideScreen || !Settings.isMobile) {
                    return;
                }
                if (root.height > (root.preferredHeight - root.minimumHeight)/2 ) {
                    root.height = root.preferredHeight;
                } else {
                    root.height = root.minimumHeight;
                }
            }
        }
        Connections {
            target: __appWindow.pageStack
            onCurrentItemChanged: {
                if (!__appWindow.pageStack.currentItem) {
                    return;
                }
                if (__appWindow.pageStack.currentItem.flickable) {
                    headerSlideConnection.oldContentY = __appWindow.pageStack.currentItem.flickable.contentY;
                } else {
                    headerSlideConnection.oldContentY = 0;
                }
                root.height = root.preferredHeight;
            }
        }

        Item {
            id: mainItem
            anchors {
                fill: parent
            }
        }
    }
}

