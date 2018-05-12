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
import org.kde.kirigami 2.4 as Kirigami
import "private"
import QtQuick.Templates 2.0 as T2

/**
 * Page is a container for all the app pages: everything pushed to the
 * ApplicationWindow stackView should be a Page instabnce (or a subclass,
 * such as ScrollablePage)
 * @see ScrollablePage
 * @inherit QtQuick.Templates.Page
 */
T2.Page {
    id: root

    /**
     * leftPadding: int
     * default contents padding at left
     */
    leftPadding: Kirigami.Units.gridUnit

    /**
     * topPadding: int
     * default contents padding at top
     */
    topPadding: Kirigami.Units.gridUnit

    /**
     * rightPadding: int
     * default contents padding at right
     */
    rightPadding: Kirigami.Units.gridUnit

    /**
     * bottomPadding: int
     * default contents padding at bottom
     */
    bottomPadding: actionButtons.item ? actionButtons.height : Kirigami.Units.gridUnit

    /**
     * flickable: Flickable
     * if the central element of the page is a Flickable
     * (ListView and Gridview as well) you can set it there.
     * normally, you wouldn't need to do that, but just use the
     * ScrollablePage element instead
     * @see ScrollablePage
     * Use this if your flickable has some non standard properties, such as not covering the whole Page
     */
    property Flickable flickable

    /**
     * actions.contextualActions: list<QtObject>
     * Defines the contextual actions for the page:
     * an easy way to assign actions in the right sliding panel
     *
     * Example usage:
     * @code
     * import org.kde.kirigami 2.4 as Kirigami
     *
     * Kirigami.ApplicationWindow {
     *  [...]
     *     contextDrawer: Kirigami.ContextDrawer {
     *         id: contextDrawer
     *     }
     *  [...]
     * }
     * @endcode
     *
     * @code
     * import org.kde.kirigami 2.4 as Kirigami
     *
     * Kirigami.Page {
     *   [...]
     *     actions.contextualActions: [
     *         Kirigami.Action {
     *             iconName: "edit"
     *             text: "Action text"
     *             onTriggered: {
     *                 // do stuff
     *             }
     *         },
     *         Kirigami.Action {
     *             iconName: "edit"
     *             text: "Action text"
     *             onTriggered: {
     *                 // do stuff
     *             }
     *         }
     *     ]
     *   [...]
     * }
     * @endcode
     */
    //TODO: remove
    property alias contextualActions: actionsGroup.contextualActions

    /**
     * actions.main: Action
     * An optional single action for the action button.
     * it can be a Kirigami.Action or a QAction
     *
     * Example usage:
     *
     * @code
     * import org.kde.kirigami 2.4 as Kirigami
     * Kirigami.Page {
     *     actions.main: Kirigami.Action {
     *         iconName: "edit"
     *         onTriggered: {
     *             // do stuff
     *         }
     *     }
     * }
     * @endcode
     */
    //TODO: remove
    property alias mainAction: actionsGroup.main

    /**
     * actions.left: Action
     * An optional extra action at the left of the main action button.
     * it can be a Kirigami.Action or a QAction
     *
     * Example usage:
     *
     * @code
     * import org.kde.kirigami 2.4 as Kirigami
     * Kirigami.Page {
     *     actions.left: Kirigami.Action {
     *         iconName: "edit"
     *         onTriggered: {
     *             // do stuff
     *         }
     *     }
     * }
     * @endcode
     */
    //TODO: remove
    property alias leftAction: actionsGroup.left

    /**
     * actions.right: Action
     * An optional extra action at the right of the main action button.
     * it can be a Kirigami.Action or a QAction
     *
     * Example usage:
     *
     * @code
     * import org.kde.kirigami 2.4 as Kirigami
     * Kirigami.Page {
     *     actions.right: Kirigami.Action {
     *         iconName: "edit"
     *         onTriggered: {
     *             // do stuff
     *         }
     *     }
     * }
     * @endcode
     */
    //TODO: remove
    property alias rightAction: actionsGroup.right

    /**
     * Actions properties are grouped.
     *
     * @code
     * import org.kde.kirigami 2.4 as Kirigami
     * Kirigami.Page {
     *     actions {
     *         main: Kirigami.Action {...}
     *         left: Kirigami.Action {...}
     *         right: Kirigami.Action {...}
     *         contextualActions: [
     *             Kirigami.Action {...},
     *             Kirigami.Action {...}
     *         ]
     *     }
     * }
     * @endcode
     */
    readonly property alias actions: actionsGroup

    /**
     * isCurrentPage: bool
     *
     * Specifies if it's the currently selected page in the window's pages row.
     *
     * @since 2.1
     */
    readonly property bool isCurrentPage: typeof applicationWindow === "undefined" || !applicationWindow().pageStack
                ? true
                : (applicationWindow().pageStack.layers.depth > 1
                    ? applicationWindow().pageStack.layers.currentItem == root
                    : applicationWindow().pageStack.currentItem == root)

    PageActionPropertyGroup {
        id: actionsGroup
    }

    /**
     * emitted When the application requests a Back action
     * For instance a global "back" shortcut or the Android
     * Back button has been pressed.
     * The page can manage the back event by itself,
     * and if it set event.accepted = true, it will stop the main
     * application to manage the back event.
     */
    signal backRequested(var event);

    //NOTE: This exists just because control instances require it
    contentItem: Item {
        onChildrenChanged: {
            //NOTE: make sure OverlaySheets are directly under the root
            //so they are over all the contents and don't have margins
            //search for an OverlaySheet, unfortunately have to blind test properties
            //as there is no way to get the classname from qml objects
            //TODO: OverlaySheets should be Popup instead?
            for (var i = children.length -1; i >= 0; --i) {
                var child = children[i];
                if (child.toString().indexOf("OverlaySheet") === 0 ||
                    (child.sheetOpen !== undefined && child.open !== undefined && child.close !== undefined)) {
                    child.parent = root;
                    child.z = 9997
                }
            }
        }
    }

    //on material the shadow would bleed over
    clip: header !== undefined
    Loader {
        id: actionButtons
        z: 9999
        parent: root
        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
        }
        //It should be T2.Page, Qt 5.7 doesn't like it
        property Item page: root
        height: item ? item.height : 0
        source: typeof applicationWindow !== "undefined" && ((applicationWindow().header && applicationWindow().header.toString().indexOf("ToolBarApplicationHeader") === 0) ||
                (applicationWindow().footer && applicationWindow().footer.visible && applicationWindow().footer.toString().indexOf("ToolBarApplicationHeader") === 0))
                ? "" : Qt.resolvedUrl("./private/ActionButton.qml")
    }

    Layout.fillWidth: true
}
