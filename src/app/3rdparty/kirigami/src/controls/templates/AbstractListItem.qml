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

import QtQuick 2.1
import QtQuick.Layouts 1.0
import org.kde.kirigami 2.4
import QtQuick.Templates 2.0 as T2

/**
 * An item delegate for the primitive ListView component.
 *
 * It's intended to make all listviews look coherent.
 *
 * @inherit QtQuick.Templates.ItemDelegate
 */
T2.ItemDelegate {
    id: listItem
    
    /**
     * supportsMouseEvents: bool
     * Holds if the item emits signals related to mouse interaction.
     *TODO: remove
     * The default value is false.
     */
    property bool supportsMouseEvents: hoverEnabled

    /**
     * containsMouse: bool
     * True when the user hovers the mouse over the list item
     * NOTE: on mobile touch devices this will be true only when pressed is also true
     * TODO: remove?
     */
    property alias containsMouse: listItem.hovered

    /**
     * sectionDelegate: bool
     * If true the item will be a delegate for a section, so will look like a
     * "title" for the items under it.
     */
    property bool sectionDelegate: false

    /**
     * separatorVisible: bool
     * True if the separator between items is visible
     * default: true
     */
    property bool separatorVisible: true

    /**
     * textColor: color
     * Color for the text in the item
     * It is advised to leave the default value (Theme.textColor)
     *
     * Note: if custom text elements are inserted in an AbstractListItem,
     * their color property will have to be manually bound with this property
     */
    property color textColor: Theme.textColor

    /**
     * backgroundColor: color
     * Color for the background of the item
     * It is advised to leave the default value (Theme.viewBackgroundColor)
     */
    property color backgroundColor: "transparent"

    /**
     * activeTextColor: color
     * Color for the text in the item when pressed or selected
     * It is advised to leave the default value (Theme.highlightedTextColor)
     *
     * Note: if custom text elements are inserted in an AbstractListItem,
     * their color property will have to be manually bound with this property
     */
    property color activeTextColor: Theme.highlightedTextColor

    /**
     * activeBackgroundColor: color
     * Color for the background of the item when pressed or selected
     * It is advised to leave the default value (Theme.highlightColor)
     */
    property color activeBackgroundColor: Theme.highlightColor

    default property alias _default: listItem.contentItem

    Theme.colorGroup: internal.indicateActiveFocus ? Theme.Active : Theme.Inactive

    leftPadding: LayoutMirroring.enabled && internal.view && internal.view.T2.ScrollBar.vertical ? internal.view.T2.ScrollBar.vertical.width : Units.largeSpacing
    topPadding: Units.largeSpacing
    rightPadding: !LayoutMirroring.enabled && internal.view && internal.view.T2.ScrollBar.vertical ? internal.view.T2.ScrollBar.vertical.width : Units.largeSpacing
    bottomPadding: Units.largeSpacing

    implicitWidth: contentItem ? contentItem.implicitWidth : Units.gridUnit * 12

    implicitHeight: contentItem.implicitHeight + Units.smallSpacing * 5

    width: parent ? parent.width : implicitWidth
    Layout.fillWidth: true

    opacity: enabled ? 1 : 0.6

    height: visible ? implicitHeight : 0

    hoverEnabled: true

    QtObject {
        id: internal
        property Flickable view: listItem.ListView.view || (listItem.parent ? listItem.parent.ListView.view : null)
        property bool indicateActiveFocus: listItem.pressed || Settings.tabletMode || listItem.activeFocus || (view ? view.activeFocus : false)
    }

    Accessible.role: Accessible.ListItem
}
