/*
 *   Copyright 2016 Marco Martin <mart@kde.org>
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

import QtQuick 2.1
import QtGraphicalEffects 1.0
import QtQuick.Templates 2.0 as T2
import org.kde.kirigami 2.4

import "private"
import "templates" as T

/**
 * Overlay Drawers are used to expose additional UI elements needed for
 * small secondary tasks for which the main UI elements are not needed.
 * For example in Okular Active, an Overlay Drawer is used to display
 * thumbnails of all pages within a document along with a search field.
 * This is used for the distinct task of navigating to another page.
 */
T.OverlayDrawer {
    id: root

//BEGIN Properties
    background: Rectangle {
        color: Theme.backgroundColor

        Item {
            parent: root.handle
            anchors.fill: parent

            DropShadow {
                anchors.fill: handleGraphics
                horizontalOffset: 0
                verticalOffset: Units.devicePixelRatio
                radius: Units.gridUnit /2
                samples: 16
                color: Qt.rgba(0, 0, 0, root.handle.pressed ? 0.6 : 0.4)
                source: handleGraphics
            }
            Rectangle {
                id: handleGraphics
                anchors.centerIn: parent
                Theme.colorSet: Theme.Button
                Theme.inherit: false
                color: root.handle.pressed ? Theme.highlightColor : Theme.backgroundColor
                width: Units.iconSizes.smallMedium + Units.smallSpacing * 2
                height: width
                radius: Units.devicePixelRatio * 2
                Loader {
                    anchors.centerIn: parent
                    width: height
                    height: Units.iconSizes.smallMedium
                    source: {
                        switch(root.edge) {
                        case Qt.LeftEdge:
                            return Qt.resolvedUrl("templates/private/MenuIcon.qml");
                        case Qt.RightEdge: {
                            if (root.hasOwnProperty("actions")) {
                                return Qt.resolvedUrl("templates/private/ContextIcon.qml");
                            } else {
                                return Qt.resolvedUrl("templates/private/GenericDrawerIcon.qml");
                            }
                        }
                        default:
                            return "";
                        }
                    }
                    onItemChanged: {
                        if(item) {
                            item.drawer = Qt.binding(function(){return root});
                            item.color = Qt.binding(function(){return root.handle.pressed ? Theme.highlightedTextColor : Theme.textColor});
                        }
                    }
                }
                Behavior on color {
                    ColorAnimation {
                        duration: Units.longDuration
                        easing.type: Easing.InOutQuad
                    }
                }
            }
        }


        EdgeShadow {
            z: -2
            edge: root.edge
            anchors {
                right: root.edge == Qt.RightEdge ? parent.left : (root.edge == Qt.LeftEdge ? undefined : parent.right)
                left: root.edge == Qt.LeftEdge ? parent.right : (root.edge == Qt.RightEdge ? undefined : parent.left)
                top: root.edge == Qt.TopEdge ? parent.bottom : (root.edge == Qt.BottomEdge ? undefined : parent.top)
                bottom: root.edge == Qt.BottomEdge ? parent.top : (root.edge == Qt.TopEdge ? undefined : parent.bottom)
            }

            opacity: root.position == 0 ? 0 : 1

            Behavior on opacity {
                NumberAnimation {
                    duration: Units.longDuration
                    easing.type: Easing.InOutQuad
                }
            }
        }
    }
}
