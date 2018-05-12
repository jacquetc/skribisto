/*
 *   Copyright 2016 Marco Martin <notmart@gmail.com>
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

import QtQuick 2.7
import org.kde.kirigami 2.4
import QtQuick.Controls.Material 2.1 as Mat
import QtQuick.Controls.Material.impl 2.1 as MatImp
import "../../private"
import "../../templates" as T

T.AbstractListItem {
    id: listItem

    background: DefaultListItemBackground {

        MatImp.Ripple {
            anchors.fill: parent
            clip: visible
            pressed: listItem.pressed
            anchor: listItem
            active: listItem.down || listItem.visualFocus 
            color: Qt.rgba(0,0,0,0.2)
        }
    }
    implicitHeight: contentItem.implicitHeight + Units.smallSpacing * 6
}
