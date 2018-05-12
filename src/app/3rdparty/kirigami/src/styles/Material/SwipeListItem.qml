/*
 *   Copyright 2010 Marco Martin <notmart@gmail.com>
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
 *   51 Franklin Street, Fifth Floor, Boston, MA  2.010-1301, USA.
 */

import QtQuick 2.5
import org.kde.kirigami 2.4
import "../../private"
import "../../templates" as T

/**
 * An item delegate Intended to support extra actions obtainable
 * by uncovering them by dragging away the item with the handle
 * This acts as a container for normal list items.
 * Any subclass of AbstractListItem can be assigned as the contentItem property.
 * @code
 * ListView {
 *     model: myModel
 *     delegate: SwipeListItem {
 *         Label {
 *             text: model.text
 *         }
 *         actions: [
 *              Action {
 *                  iconName: "document-decrypt"
 *                  onTriggered: print("Action 1 clicked")
 *              },
 *              Action {
 *                  iconName: model.action2Icon
 *                  onTriggered: //do something
 *              }
 *         ]
 *     }
 * 
 * }
 * @endcode
 *
 * @inherit QtQuick.Item
 */
T.SwipeListItem {
    id: root

    onPressedChanged: {
        if (pressed) {
            clickAnim.running = true
        }
    }
    background: DefaultListItemBackground {
        clip: true
        //TODO: this will have to reuse QQC2.1 Ripple
        Rectangle {
            id: ripple
            anchors.centerIn: parent
            width: parent.width
            height: parent.width
            radius: width
            color: Qt.rgba(1,1,1,0.3)
            scale: 0
            opacity: 1
            ParallelAnimation {
                id: clickAnim
                ScaleAnimator {
                    target: ripple
                    from: 0
                    to: 1
                    duration: Units.longDuration
                }
                OpacityAnimator {
                    target: ripple
                    from: 0
                    to: 1
                    duration: Units.longDuration
                }
            }
        }
    }
    implicitHeight: contentItem.implicitHeight + Units.smallSpacing * 6
}
