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

import QtQuick 2.1
import QtQuick.Layouts 1.2
import QtGraphicalEffects 1.0
import org.kde.kirigami 2.4

Item {
    id: canvas
    width: height
    height: Units.iconSizes.smallMedium
    property OverlayDrawer drawer
    property color color: Theme.textColor
    opacity: 0.8
    layer.enabled: true

    Item {
        id: iconRoot
        anchors {
            fill: parent
            margins: Units.smallSpacing
        }
        property int thickness: Math.floor(Units.devicePixelRatio)*2

        Rectangle {
            anchors {
                right: parent.right
                top: parent.top
                topMargin: -iconRoot.thickness/2 * drawer.position
            }
            antialiasing: true
            transformOrigin: Item.Right
            width: (1 - drawer.position) * parent.width + drawer.position * (Math.sqrt(2*(parent.width*parent.width)))
            height: iconRoot.thickness
            color: canvas.color
            rotation: -45 * drawer.position
        }

        Rectangle {
            anchors.centerIn: parent
            width: parent.width - parent.width * drawer.position
            height: iconRoot.thickness
            color: canvas.color
        }

        Rectangle {
            anchors {
                right: parent.right
                bottom: parent.bottom
                bottomMargin: -iconRoot.thickness/2 * drawer.position
            }
            antialiasing: true
            transformOrigin: Item.Right
            width: (1 - drawer.position) * parent.width + drawer.position * (Math.sqrt(2*(parent.width*parent.width)))
            height: iconRoot.thickness
            color: canvas.color
            rotation: 45 * drawer.position
        }
    }
}

