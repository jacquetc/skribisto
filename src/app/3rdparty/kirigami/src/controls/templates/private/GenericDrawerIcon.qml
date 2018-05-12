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
import org.kde.kirigami 2.4 as Kirigami

Item {
    width: height
    height: Kirigami.Units.iconSizes.smallMedium
    property Kirigami.OverlayDrawer drawer
    property color color: Theme.textColor
    opacity: 0.8
    layer.enabled: true

    Kirigami.Icon {
        selected: drawer.handle.pressed
        opacity: 1 - drawer.position
        anchors.fill: parent
        source: drawer.handleClosedIcon.source
        color: drawer.handleClosedIcon.color
    }
    Kirigami.Icon {
        selected: drawer.handle.pressed
        opacity: drawer.position
        anchors.fill: parent
        source: drawer.handleOpenIcon.source
        color: drawer.handleOpenIcon.color
    }
}

