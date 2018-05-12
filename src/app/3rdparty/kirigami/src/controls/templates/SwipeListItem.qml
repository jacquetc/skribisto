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

import QtQuick 2.7
import QtQuick.Layouts 1.2
import QtQuick.Controls 2.0 as Controls
import org.kde.kirigami 2.4
import "../private"
import QtQuick.Templates 2.0 as T2

/**
 * An item delegate Intended to support extra actions obtainable
 * by uncovering them by dragging away the item with the handle
 * This acts as a container for normal list items.
 * Any subclass of AbstractListItem can be assigned as the contentItem property.
 * @code
 * ListView {
 *     model: myModel
 *     delegate: SwipeListItem {
 *         QQC2.Label {
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
 * @inherit QtQuick.Templates.ItemDelegate
 */
T2.ItemDelegate {
    id: listItem

//BEGIN properties
    /**
     * supportsMouseEvents: bool
     * Holds if the item emits signals related to mouse interaction.
     *TODO: remove
     * The default value is false.
     */
    property alias supportsMouseEvents: listItem.hoverEnabled

    /**
     * containsMouse: bool
     * True when the user hover the mouse over the list item
     * NOTE: on mobile touch devices this will be true only when pressed is also true
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
     * actions: list<Action>
     * Defines the actions for the list item: at most 4 buttons will
     * contain the actions for the item, that can be revealed by
     * sliding away the list item.
     */
    property list<Action> actions

    /**
     * textColor: color
     * Color for the text in the item
     *
     * Note: if custom text elements are inserted in an AbstractListItem,
     * their color property will have to be manually bound with this property
     */
    property color textColor: Theme.textColor

    /**
     * backgroundColor: color
     * Color for the background of the item
     */
    property color backgroundColor: Theme.backgroundColor

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

    Theme.colorGroup: behindItem.indicateActiveFocus ? Theme.Active : Theme.Inactive

    hoverEnabled: true
    implicitWidth: contentItem ? contentItem.implicitWidth : Units.gridUnit * 12
    width: parent ? parent.width : implicitWidth
    implicitHeight: contentItem.implicitHeight + Units.smallSpacing * 5

    leftPadding: Units.smallSpacing * 2 + (LayoutMirroring.enabled ? (handleMouse.visible ? handleMouse.width : 0) + handleMouse.anchors.rightMargin : 0)
    topPadding: Units.smallSpacing * 2
    rightPadding: Units.smallSpacing * 2 + (LayoutMirroring.enabled ?  0 : (handleMouse.visible ? handleMouse.width : 0) + handleMouse.anchors.rightMargin)
    bottomPadding: Units.smallSpacing * 2

//END properties

    Item {
        id: behindItem
        parent: listItem
        z: -1
        //TODO: a global "open" state
        enabled: background.x !== 0
        property bool indicateActiveFocus: listItem.pressed || Settings.tabletMode || listItem.activeFocus || (view ? view.activeFocus : false)
        property Flickable view: listItem.ListView.view || listItem.parent.ListView.view
        anchors {
            fill: parent
        }
        Rectangle {
            id: shadowHolder
            color: Qt.darker(Theme.backgroundColor, 1.05);
            anchors.fill: parent
        }
        EdgeShadow {
            edge: Qt.TopEdge
            anchors {
                right: parent.right
                left: parent.left
                top: parent.top
            }
        }
        EdgeShadow {
            edge: LayoutMirroring.enabled ? Qt.RightEdge : Qt.LeftEdge
            x: LayoutMirroring.enabled ? listItem.background.x - width : (listItem.background.x + listItem.background.width)
            anchors {
                top: parent.top
                bottom: parent.bottom
            }
        }
        MouseArea {
            anchors.fill: parent
            preventStealing: true
            enabled: background.x != 0
            onClicked: {
                positionAnimation.from = background.x;
                positionAnimation.to = 0;
                positionAnimation.running = true;
            }
        }
        Row {
            id: actionsLayout
            z: 1
            parent: Settings.tabletMode ? behindItem : listItem
            opacity: Settings.tabletMode ? 1 : (listItem.hovered ? 1 : 0)
            Behavior on opacity {
                OpacityAnimator {
                    duration: Units.longDuration
                    easing.type: Easing.InOutQuad
                }
            }
            anchors {
                right: parent.right
                verticalCenter: parent.verticalCenter
                rightMargin: LayoutMirroring.enabled ? listItem.leftPadding : listItem.rightPadding
            }
            height: Math.min( parent.height / 1.5, Units.iconSizes.medium)
            width: childrenRect.width
            property bool exclusive: false
            property Item checkedButton
            spacing: Units.largeSpacing
            Repeater {
                model: {
                    if (listItem.actions.length == 0) {
                        return null;
                    } else {
                        return listItem.actions[0].text !== undefined &&
                            listItem.actions[0].trigger !== undefined ?
                                listItem.actions :
                                listItem.actions[0];
                    }
                }
                delegate: Icon {
                    height: actionsLayout.height
                    width: height
                    source: modelData.iconName
                    enabled: (modelData && modelData.enabled !== undefined) ? modelData.enabled : true;
                    visible: (modelData && modelData.visible !== undefined) ? modelData.visible : true;
                    MouseArea {
                        id: actionMouse
                        anchors {
                            fill: parent;
                            margins: Settings.tabletMode ? -Units.smallSpacing : 0;
                        }
                        enabled: (modelData && modelData.enabled !== undefined) ? modelData.enabled : true;
                        hoverEnabled: !Settings.tabletMode
                        onClicked: {
                            if (modelData && modelData.trigger !== undefined) {
                                modelData.trigger();
                            }
                            positionAnimation.from = background.x;
                            positionAnimation.to = 0;
                            positionAnimation.running = true;
                        }
                        Controls.ToolTip.delay: 1000
                        Controls.ToolTip.timeout: 5000
                        Controls.ToolTip.visible: (Settings.tabletMode ? actionMouse.pressed : actionMouse.containsMouse) && Controls.ToolTip.text.length > 0
                        Controls.ToolTip.text: modelData.tooltip || modelData.text
                    }
                    
                }
            }
        }
    }

    MouseArea {
        id: handleMouse
        parent: listItem.background
        visible: Settings.tabletMode
        z: 99
        anchors {
            right: parent.right
            top: parent.top
            bottom: parent.bottom
            rightMargin: behindItem.view && behindItem.view.T2.ScrollBar && behindItem.view.T2.ScrollBar.vertical && behindItem.view.T2.ScrollBar.vertical.interactive ? behindItem.view.T2.ScrollBar.vertical.width : Units.smallSpacing
        }
        preventStealing: true
        width: height
        property var downTimestamp;
        property int startX
        property int startMouseX

        onClicked: {
            positionAnimation.from = background.x;
            if (listItem.background.x > -listItem.background.width/2) {
                positionAnimation.to = (LayoutMirroring.enabled ? -1 : +1) * (-listItem.width + height + handleMouse.anchors.rightMargin);
            } else {
                positionAnimation.to = 0;
            }
            positionAnimation.running = true;
        }
        onPressed: {
            downTimestamp = (new Date()).getTime();
            startX = listItem.background.x;
            startMouseX = mouse.x;
        }
        onPositionChanged: {
            if (LayoutMirroring.enabled) {
                listItem.background.x = Math.max(0, Math.min(listItem.width - height, listItem.background.x - (startMouseX - mouse.x)));
            } else {
                listItem.background.x = Math.min(0, Math.max(-listItem.width + height, listItem.background.x - (startMouseX - mouse.x)));
            }
        }
        onReleased: {
            var speed = ((startX - listItem.background.x) / ((new Date()).getTime() - downTimestamp) * 1000);
            if (LayoutMirroring.enabled) {
                speed = -speed;
            }

            if (Math.abs(speed) < Units.gridUnit) {
                return;
            }
            if (speed > listItem.width/2) {
                positionAnimation.to = (LayoutMirroring.enabled ? -1 : +1) * (-listItem.width + height + handleMouse.anchors.rightMargin);
            } else {
                positionAnimation.to = 0;
            }
            positionAnimation.from = background.x;
            positionAnimation.running = true;
        }
        Icon {
            id: handleIcon
            anchors.verticalCenter: parent.verticalCenter
            selected: listItem.checked || (listItem.pressed && !listItem.checked && !listItem.sectionDelegate)
            width: Units.iconSizes.smallMedium
            height: width
            x: y
            source: (LayoutMirroring.enabled ? (listItem.background.x < listItem.background.width/2 ? "handle-right" : "handle-left") : (listItem.background.x < -listItem.background.width/2 ? "handle-right" : "handle-left"))
        }
    }

    NumberAnimation {
        id: positionAnimation
        property: "x"
        target: background
        duration: Units.longDuration
        easing.type: Easing.InOutQuad
    }

//BEGIN signal handlers
    onContentItemChanged: {
        if (!contentItem) {
            return;
        }
        contentItem.parent = background;
        contentItem.anchors.top = background.top;
        contentItem.anchors.left = background.left;
        contentItem.anchors.right = background.right;
        contentItem.anchors.leftMargin = Qt.binding(function() {return listItem.leftPadding});
        contentItem.anchors.rightMargin = Qt.binding(function() {return listItem.rightPadding});
        contentItem.anchors.topMargin = Qt.binding(function() {return listItem.topPadding});
        contentItem.z = 0;
    }
    Component.onCompleted: {
        //this will happen only once
        if (Settings.tabletMode && !swipeFilterConnection.swipeFilterItem) {
            var component = Qt.createComponent(Qt.resolvedUrl("../private/SwipeItemEventFilter.qml"));
            behindItem.view.parent.parent._swipeFilter = component.createObject(behindItem.view.parent.parent);
        }
        listItem.contentItemChanged();
    }
    Connections {
        target: Settings
        onTabletModeChanged: {
            if (Settings.tabletMode) {
                if (!swipeFilterConnection.swipeFilterItem) {
                    var component = Qt.createComponent(Qt.resolvedUrl("../private/SwipeItemEventFilter.qml"));
                    listItem.ListView.view.parent.parent._swipeFilter = component.createObject(listItem.ListView.view.parent.parent);
                }
            } else {
                if (listItem.ListView.view.parent.parent._swipeFilter) {
                    listItem.ListView.view.parent.parent._swipeFilter.destroy();
                    positionAnimation.to = 0;
                    positionAnimation.from = background.x;
                    positionAnimation.running = true;
                }
            }
        }
    }
    Connections {
        id: swipeFilterConnection
        readonly property QtObject swipeFilterItem: (behindItem.view && behindItem.view && behindItem.view.parent && behindItem.view.parent.parent) ? behindItem.view.parent.parent._swipeFilter : null
        readonly property bool enabled: swipeFilterItem ? swipeFilterItem.currentItem === listItem : false
        target: enabled ? swipeFilterItem : null
        onPeekChanged: listItem.background.x = -(listItem.background.width - listItem.background.height) * swipeFilterItem.peek
        onCurrentItemChanged: {
            if (!enabled) {
                positionAnimation.to = 0;
                positionAnimation.from = background.x;
                positionAnimation.running = true;
            }
        }
    }
//END signal handlers

    Accessible.role: Accessible.ListItem
}
