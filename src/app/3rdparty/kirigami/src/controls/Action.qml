/*
 *   Copyright 2016 Marco Martin <mart@kde.org>
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

import QtQuick 2.7
import "private"

/**
 * An item that represents an abstract Action
 *
 * @inherit QtObject
 */
QtObject {
    id: root

    /**
     * Emitted whenever a action's checked property changes.
     * This usually happens at the same time as triggered.
     * @param checked
     */
    signal toggled(bool checked)

    /**
     * Emitted when either the menu item or its bound action have been activated. Includes the object that triggered the event if relevant (e.g. a Button).
     * You shouldn't need to emit this signal, use trigger() instead.
     * @param source Object that triggered the event if relevant, often null
     */
    signal triggered(QtObject source)

    /**
     * visible: bool
     * True (default) when the graphic representation of the action
     * is supposed to be visible.
     * It's up to the action representation to honor this property.
     */
    property bool visible: true

    /**
     * checkable: bool
     * Whether action can be checked, or toggled. Defaults to false.
     */
    property bool checkable: false

    /**
     * checked: bool
     * Whether the action is checked. Defaults to false.
     */
    property bool checked: false

    /**
     * enabled: bool
     * Whether the action is enabled, and can be triggered. Defaults to true.
     */
    property bool enabled: true

    /**
     * iconName: string
     * Sets the icon name for the action. This will pick the icon with the given name from the current theme.
     */
    property alias iconName: iconGroup.name

    /**
     * iconSource: string
     * Sets the icon file or resource url for the action. Defaults to the empty URL. Use this if you want a specific file rather than an icon from the theme
     */
    property alias iconSource: iconGroup.source

    /**
     * metadata for the icon, such as width/height.name and source
     * * name	This property holds the name of the icon to use.
     *       The icon will be loaded from the platform theme.
     *       If the icon is found in the theme, it will always be used;
     *       even if icon.source is also set. If the icon is not found,
     *       icon.source will be used instead.
     *       For more information on theme icons, see QIcon::fromTheme().
     *
     * * source	This property holds the name of the icon to use.
     *       The icon will be loaded as a regular image.
     *       If icon.name is set and refers to a valid theme icon,
     *       it will always be used instead of this property.
     *
     * * width	This property holds the width of the icon.
     *   The icon's width will never exceed this value,
     *   though it will shrink when necessary.
     *   height	This property holds the height of the icon.
     *   The icon's height will never exceed this value,
     *   though it will shrink when necessary.
     *
     * *color	This property holds the color of the icon.
     *   The icon is tinted with the specified color, unless the color is set to "transparent".
     */
    property ActionIconGroup icon: ActionIconGroup {
        id: iconGroup
    }

    /**
     * shortcut : keysequence
     * Shortcut bound to the action. The keysequence can be a string or a Qt standard key.
     */
    property alias shortcut: shortcutItem.sequence

    /**
     * Text for the action. This text will show as the button text, or as title in a menu item, depending from the way the developer will choose to represent it
     */
    property string text

    /**
     * A tooltip text to be shown when hovering the control bound to this action. Not all controls support tooltips on all platforms
     */
    property string tooltip

    /**
     * children: list<Action>
     * A list of children actions.
     * Useful for tree-like menus
     * @code
     * Action {
     *    text: "Tools"
     *    Action {
     *        text: "Action1"
     *    }
     *    Action {
     *        text: "Action2"
     *    }
     * }
     * @endcode
     */
    default property alias children: root.__children
    property list<QtObject> __children
    property Shortcut __shortcut: Shortcut {
        property bool checked: false
        id: shortcutItem
        enabled: root.enabled
        onActivated: root.trigger();
    }
    function trigger(source) {
        if (!enabled) {
            return;
        }
        root.triggered(source);
        if (root.checkable) {
            root.checked = !root.checked;
            root.toggled(root.checked);
        }
    }

    onCheckedChanged: root.toggled(root.checked);
}
