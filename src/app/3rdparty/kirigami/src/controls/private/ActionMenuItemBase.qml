/*
 *   Copyright 2018 Aleix Pol Gonzalez <aleixpol@kde.org>
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

import QtQuick 2.3
import QtQuick.Controls 2.1 as Controls

Controls.MenuItem {
    id: menuItem

    property QtObject ourAction

    text: ourAction.text
    visible: ourAction.visible !== undefined ? ourAction.visible : true
    enabled: ourAction.enabled
    checkable: ourAction.checkable
    checked: ourAction.checked
    onTriggered: {
        ourAction.trigger()
    }

    readonly property var ourMenu: theMenu.submenuComponent ? theMenu.submenuComponent.createObject(menuItem, { actions: ourAction.children }) : null
}
