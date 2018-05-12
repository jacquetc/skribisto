/*
 *   Copyright 2012 Marco Martin <mart@kde.org>
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
import QtQuick.Templates 2.0 as T2
import org.kde.kirigami 2.4
import "private"

/**
 * Overlay Drawers are used to expose additional UI elements needed for
 * small secondary tasks for which the main UI elements are not needed.
 * For example in Okular Active, an Overlay Drawer is used to display
 * thumbnails of all pages within a document along with a search field.
 * This is used for the distinct task of navigating to another page.
 * @inherits: QtQuick.Templates.Drawer
 */
T2.Drawer {
    id: root

    z: modal ? (Math.round((position * 100) / 10) ): 0

//BEGIN Properties
    /**
     * drawerOpen: bool
     * true when the drawer is open and visible
     */
    property bool drawerOpen: false

    /**
     * enabled: bool
     * This property holds whether the item receives mouse and keyboard events. By default this is true.
     */
    property bool enabled: true

    /**
     * peeking: true
     * When true the drawer is in a state between open and closed. the drawer is visible but not completely open.
     * This is usually the case when the user is dragging the drawer from a screen 
     * edge, so the user is "peeking" what's in the drawer
     */
    property bool peeking: false

    /**
     * animating: Bool
     * True during an animation of a drawer either opening or closing
     */
    readonly property bool animating : enterAnimation.animating || exitAnimation.animating || positionResetAnim.running

    /**
     * A grouped property describing an optional icon.
     * * source: The source of the icon, a freedesktop-compatible icon name is recommended.
     * * color: An optional tint color for the icon.
     *
     * If no custom icon is set, a menu icon is shown for the application  globalDrawer
     * and an overflow menu icon is shown for the contextDrawer.
     * That's the default for the GlobalDrawer and ContextDrawer components respectively.
     * 
     * For OverlayDrawer the default is view-right-close or view-left-close depending on the drawer location
     * @since 2.5
     */
    readonly property QtObject handleOpenIcon: IconPropertiesGroup {source: root.edge == Qt.RightEdge ? "view-right-close" : "view-left-close"}

    /**
     * A grouped property describing an optional icon.
     * * source: The source of the icon, a freedesktop-compatible icon name is recommended.
     * * color: An optional tint color for the icon.
     *
     * If no custom icon is set, an X icon is shown,
     * which will morph into the Menu or overflow icons
     * 
     * For OverlayDrawer the default is view-right-new or view-left-new depending on the drawer location
     * @since 2.5
     */
    readonly property QtObject handleClosedIcon: IconPropertiesGroup {source: root.edge == Qt.RightEdge ? "view-right-new" : "view-left-new"}

    /**
     * handleVisible: bool
     * If true, a little handle will be visible to make opening the drawer easier
     * Currently supported only on left and right drawers
     */
    property bool handleVisible: typeof(applicationWindow)===typeof(Function) && applicationWindow() ? applicationWindow().controlsVisible : true

    /**
     * handle: Item
     * Readonly property that points to the item that will act as a physical
     * handle for the Drawer
     **/
    readonly property Item handle: MouseArea {
        id: drawerHandle
        z: root.modal ? applicationWindow().overlay.z + (root.position > 0 ? +1 : -1) : root.background.parent.z + 1
        preventStealing: true
        hoverEnabled: desktopMode
        parent: applicationWindow().overlay.parent

        property int startX
        property int mappedStartX

        property bool desktopMode: applicationWindow() && applicationWindow().header.toString().indexOf("ToolBarApplicationHeader") !== -1

        onPressed: {
            root.peeking = true;
            startX = mouse.x;
            mappedStartX = mapToItem(parent, startX, 0).x
        }
        onPositionChanged: {
            if (!pressed) {
                return;
            }
            var pos = mapToItem(parent, mouse.x - startX, mouse.y);
            switch(root.edge) {
            case Qt.LeftEdge:
                root.position = pos.x/root.contentItem.width;
                break;
            case Qt.RightEdge:
                root.position = (root.parent.width - pos.x - width)/root.contentItem.width;
                break;
            default:
            }
        }
        onReleased: {
            root.peeking = false;
            
            if (Math.abs(mapToItem(parent, mouse.x, 0).x - mappedStartX) < Qt.styleHints.startDragDistance) {
                if (!root.drawerOpen) {
                    root.close();
                }
                root.drawerOpen = !root.drawerOpen;
            }
        }
        onCanceled: {
            root.peeking = false
        }
        x: {
            switch(root.edge) {
            case Qt.LeftEdge:
                return root.background.width * root.position;
            case Qt.RightEdge:
                return drawerHandle.parent.width - (root.background.width * root.position) - width;
            default:
                return 0;
            }
        }

        anchors {
            top: drawerHandle.desktopMode ? parent.top : undefined

            bottom: drawerHandle.desktopMode ? undefined : parent.bottom

            bottomMargin: {
                if (!applicationWindow()) {
                    return;
                }

                var margin = 0;
                if (applicationWindow().footer) {
                    margin = applicationWindow().footer.height;
                }

                if (!applicationWindow() || !applicationWindow().pageStack ||
                    !applicationWindow().pageStack.contentItem ||
                    !applicationWindow().pageStack.contentItem.itemAt) {
                    return margin;
                }

                var item;
                if (applicationWindow().pageStack.layers.depth > 1) {
                    item = applicationWindow().pageStack.layers.currentItem;
                } else {
                    item = applicationWindow().pageStack.contentItem.itemAt(applicationWindow().pageStack.contentItem.contentX + drawerHandle.x, 0);
                }

                //try to take the last item
                if (!item) {
                    item = applicationWindow().pageStack.lastItem;
                }

                var pageFooter = item && item.page ? item.page.footer : (item ? item.footer : undefined);
                if (pageFooter) {
                    margin += pageFooter.height;
                }

                return margin;
            }
            Behavior on bottomMargin {
                NumberAnimation {
                    duration: Units.shortDuration
                    easing.type: Easing.InOutQuad
                }
            }
        }

        visible: root.enabled && (root.edge == Qt.LeftEdge || root.edge == Qt.RightEdge)
        width: Units.iconSizes.medium + Units.smallSpacing*2
        height: width
        opacity: root.handleVisible ? 1 : 0
        Behavior on opacity {
            NumberAnimation {
                duration: Units.longDuration
                easing.type: Easing.InOutQuad
            }
        }
        transform: Translate {
            id: translateTransform
            x: root.handleVisible ? 0 : (root.edge == Qt.LeftEdge ? -drawerHandle.width : drawerHandle.width)
            Behavior on x {
                NumberAnimation {
                    duration: Units.longDuration
                    easing.type: !root.handleVisible ? Easing.OutQuad : Easing.InQuad
                }
            }
        }
    }

    Theme.colorSet: Theme.View
    Theme.onColorSetChanged: {
        contentItem.Theme.colorSet = Theme.colorSet
        background.Theme.colorSet = Theme.colorSet
    }
//END Properties


//BEGIN reassign properties
    //default paddings
    leftPadding: Units.smallSpacing
    topPadding: Units.smallSpacing
    rightPadding: Units.smallSpacing
    bottomPadding: Units.smallSpacing

    parent: modal ? T2.ApplicationWindow.overlay : T2.ApplicationWindow.contentItem
    height: edge == Qt.LeftEdge || edge == Qt.RightEdge ? applicationWindow().height : Math.min(contentItem.implicitHeight, Math.round(applicationWindow().height*0.8))
    width:  edge == Qt.TopEdge || edge == Qt.BottomEdge ? applicationWindow().width : Math.min(contentItem.implicitWidth, Math.round(applicationWindow().width*0.8))
    edge: Qt.LeftEdge
    modal: true

    dragMargin: enabled && (edge == Qt.LeftEdge || edge == Qt.RightEdge) ? Qt.styleHints.startDragDistance : 0
    
    implicitWidth: Math.max(background ? background.implicitWidth : 0, contentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(background ? background.implicitHeight : 0, contentHeight + topPadding + bottomPadding)

    contentWidth: contentItem.implicitWidth || (contentChildren.length === 1 ? contentChildren[0].implicitWidth : 0)
    contentHeight: contentItem.implicitHeight || (contentChildren.length === 1 ? contentChildren[0].implicitHeight : 0)

    enter: Transition {
        SequentialAnimation {
            id: enterAnimation
            /*NOTE: why this? the running status of the enter transition is not relaible and
             * the SmoothedAnimation is always marked as non running,
             * so the only way to get to a reliable animating status is with this
             */
            property bool animating
            ScriptAction {
                script: {
                    enterAnimation.animating = true
                    //on non modal dialog we don't want drawers in the overlay
                    if (!root.modal) {
                        root.background.parent.parent = applicationWindow().overlay.parent
                    }
                }
            }
            SmoothedAnimation {
                velocity: 5
            }
            ScriptAction {
                script: enterAnimation.animating = false
            }
        }
    }

    exit: Transition {
        SequentialAnimation {
            id: exitAnimation
            property bool animating
            ScriptAction {
                script: exitAnimation.animating = true
            }
            SmoothedAnimation {
                velocity: 5 
            }
            ScriptAction {
                script: exitAnimation.animating = false
            }
        }
    }
//END reassign properties


//BEGIN signal handlers
    onPositionChanged: {
        if (peeking) {
            visible = true
        }
    }
    onVisibleChanged: {
        if (peeking) {
            visible = true
        } else {
            drawerOpen = visible;
        }
    }
    onPeekingChanged:  {
        if (peeking) {
            root.enter.enabled = false;
            root.exit.enabled = false;
        } else {
            positionResetAnim.to = position > 0.5 ? 1 : 0;
            positionResetAnim.running = true
            root.enter.enabled = true;
            root.exit.enabled = true;
        }
    }
    onDrawerOpenChanged: {
        //sync this property only when the component is properly loaded
        if (!__internal.completed) {
            return;
        }
        positionResetAnim.running = false;
        if (drawerOpen) {
            open();
        } else {
            close();
        }
    }

    Component.onCompleted: {
        //if defined as drawerOpen by default in QML, don't animate
        if (root.drawerOpen) {
            root.enter.enabled = false;
            root.visible = true;
            root.position = 1;
            root.enter.enabled = true;
        }
        __internal.completed = true;
        contentItem.Theme.colorSet = Theme.colorSet;
        background.Theme.colorSet = Theme.colorSet;
    }
//END signal handlers

    //this is as hidden as it can get here
    property QtObject __internal: QtObject {
        //here in order to not be accessible from outside
        property bool completed: false
        property NumberAnimation positionResetAnim: NumberAnimation {
            id: positionResetAnim
            target: root
            to: 0
            property: "position"
            duration: (root.position)*Units.longDuration
        }
    }
}
