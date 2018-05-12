/*
*   Copyright (C) 2016 by Marco Martin <mart@kde.org>
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

import QtQuick 2.5
import QtQuick.Layouts 1.2
import QtQuick.Window 2.2
import org.kde.kirigami 2.4
import QtGraphicalEffects 1.0
import QtQuick.Templates 2.0 as T2
import "private"
import "../private"

/**
 * An overlay sheet that covers the current Page content.
 * Its contents can be scrolled up or down, scrolling all the way up or
 * all the way down, dismisses it.
 * Use this for big, modal dialogs or information display, that can't be
 * logically done as a new separate Page, even if potentially
 * are taller than the screen space.
 * @inherits: QtQuick.QtObject
 */
QtObject {
    id: root

    Theme.colorSet: Theme.View
    Theme.inherit: false

    /**
     * contentItem: Item
     * This property holds the visual content item.
     *
     * Note: The content item is automatically resized inside the
     * padding of the control.
     * Conversely, the Sheet will be sized based on the size hints
     * of the contentItem, so if you need a cusom size sheet,
     * redefine contentWidth and contentHeight of your contentItem
     */
    default property Item contentItem

    /**
     * sheetOpen: bool
     * If true the sheet is open showing the contents of the OverlaySheet
     * component.
     */
    property bool sheetOpen

    /**
     * leftPadding: int
     * default contents padding at left
     */
    property int leftPadding: Units.gridUnit

    /**
     * topPadding: int
     * default contents padding at top
     */
    property int topPadding: Units.gridUnit

    /**
     * rightPadding: int
     * default contents padding at right
     */
    property int rightPadding: Units.gridUnit

    /**
     * bottomPadding: int
     * default contents padding at bottom
     */
    property int bottomPadding: Units.gridUnit

    /**
     * header: Item
     * an optional item which will be used as the sheet's header,
     * always kept on screen
     * @since 5.43
     */
    property Item header

    /**
     * header: Item
     * an optional item which will be used as the sheet's footer,
     * always kept on screen
     * @since 5.43
     */
    property Item footer
    /**
     * background: Item
     * This property holds the background item.
     *
     * Note: If the background item has no explicit size specified,
     * it automatically follows the control's size.
     * In most cases, there is no need to specify width or
     * height for a background item.
     */
    property Item background

    /**
     * showCloseButton: bool
     * whether to show the close button in the top-right corner
     * @since 5.44
     */
    property alias showCloseButton: closeIcon.visible

    property Item parent


    function open() {
        openAnimation.from = -mainItem.height;
        openAnimation.to = openAnimation.topOpenPosition;
        openAnimation.running = true;
        root.sheetOpen = true;
        mainItem.visible = true;
    }

    function close() {
        if (scrollView.flickableItem.contentY < 0) {
            closeAnimation.to = -height;
        } else {
            closeAnimation.to = scrollView.flickableItem.contentHeight;
        }
        closeAnimation.running = true;
    }

    onBackgroundChanged: {
        background.parent = flickableContents;
        background.z = -1;
    }
    onContentItemChanged: {
        if (contentItem.hasOwnProperty("contentY") && // Check if flickable
            contentItem.hasOwnProperty("contentHeight")) {
            contentItem.parent = scrollView;
            scrollView.contentItem = contentItem;
        } else {
            contentItem.parent = contentItemParent;
            scrollView.contentItem = flickableContents;
            contentItem.anchors.left = contentItemParent.left;
            contentItem.anchors.right = contentItemParent.right;
        }
        scrollView.flickableItem.flickableDirection = Flickable.VerticalFlick;
    }
    onSheetOpenChanged: {
        if (sheetOpen) {
            open();
        } else {
            close();
            Qt.inputMethod.hide();
        }
    }
    onHeaderChanged: {
        header.parent = headerParent;
        header.anchors.fill = headerParent;

        //TODO: special case for actual ListViews
    }
    onFooterChanged: {
        footer.parent = footerParent;
        footer.anchors.fill = footerParent;
    }

    Component.onCompleted: {
        if (!root.parent && typeof applicationWindow !== "undefined") {
            root.parent = applicationWindow().overlay
        }
    }

    readonly property Item rootItem: MouseArea {
        id: mainItem
        Theme.colorSet: root.Theme.colorSet
        Theme.inherit: root.Theme.inherit
        //we want to be over any possible OverlayDrawers, including handles
        parent: root.parent
        anchors.fill: parent
        z: root.parent == typeof applicationWindow !== "undefined" && applicationWindow().overlay ? 0 : 2000000
        visible: false
        drag.filterChildren: true
        hoverEnabled: true

        onClicked: {
            var pos = mapToItem(flickableContents, mouse.x, mouse.y);
            if (!flickableContents.contains(pos)) {
                root.close();
            }
        }

        onWidthChanged: {
            if (!contentItem.contentItem)
                return

            var width = Math.max(mainItem.width/2, Math.min(mainItem.width, root.contentItem.implicitWidth));
            contentItem.contentItem.x = (mainItem.width - width)/2
            contentItem.contentItem.width = width;
        }
        onHeightChanged: {
            var focusItem;

            focusItem = Window.activeFocusItem;

            if (!focusItem) {
                return;
            }

            //NOTE: there is no function to know if an item is descended from another,
            //so we have to walk the parent hyerarchy by hand
            var isDescendent = false;
            var candidate = focusItem.parent;
            while (candidate) {
                if (candidate == root) {
                    isDescendent = true;
                    break;
                }
                candidate = candidate.parent;
            }
            if (!isDescendent) {
                return;
            }

            var cursorY = 0;
            if (focusItem.cursorPosition !== undefined) {
                cursorY = focusItem.positionToRectangle(focusItem.cursorPosition).y;
            }

            
            var pos = focusItem.mapToItem(flickableContents, 0, cursorY - Units.gridUnit*3);
            //focused item alreqady visible? add some margin for the space of the action buttons
            if (pos.y >= scrollView.flickableItem.contentY && pos.y <= scrollView.flickableItem.contentY + scrollView.flickableItem.height - Units.gridUnit * 8) {
                return;
            }
            scrollView.flickableItem.contentY = pos.y;
        }

        ParallelAnimation {
            id: openAnimation 
            property int margins: Units.gridUnit * 5
            property int topOpenPosition: Math.min(-mainItem.height*0.15, scrollView.flickableItem.contentHeight - mainItem.height + margins)
            property alias from: openAnimationInternal.from
            property alias to: openAnimationInternal.to
            NumberAnimation {
                id: openAnimationInternal
                target: scrollView.flickableItem
                properties: "contentY"
                from: -mainItem.height
                to: openAnimation.topOpenPosition
                duration: Units.longDuration
                easing.type: Easing.OutQuad
                onRunningChanged: {
                    //hack to center listviews
                    if (!running && contentItem.contentItem) {
                        var width = Math.max(mainItem.width/2, Math.min(mainItem.width, root.contentItem.implicitWidth));
                        contentItem.contentItem.x = (mainItem.width - width)/2
                        contentItem.contentItem.width = width;
                    }
                }
            }
            OpacityAnimator {
                target: mainItem
                from: 0
                to: 1
                duration: Units.longDuration
                easing.type: Easing.InQuad
            }
        }

        SequentialAnimation {
            id: closeAnimation
            property int to: -mainItem.height
            ParallelAnimation {
                NumberAnimation {
                    target: scrollView.flickableItem
                    properties: "contentY"
                    to: closeAnimation.to
                    duration: Units.longDuration
                    easing.type: Easing.InQuad
                }
                OpacityAnimator {
                    target: mainItem
                    from: 1
                    to: 0
                    duration: Units.longDuration
                    easing.type: Easing.InQuad
                }
            }
            ScriptAction {
                script: {
                    scrollView.flickableItem.contentY = -mainItem.height;
                    mainItem.visible = root.sheetOpen = false;
                }
            }
        }
        Rectangle {
            anchors.fill: parent
            color: Theme.textColor
            opacity: 0.6 * Math.min(
                (Math.min(scrollView.flickableItem.contentY + scrollView.flickableItem.height, scrollView.flickableItem.height) / scrollView.flickableItem.height),
                (2 + (scrollView.flickableItem.contentHeight - scrollView.flickableItem.contentY - scrollView.flickableItem.topMargin - scrollView.flickableItem.bottomMargin)/scrollView.flickableItem.height))
        }

        Icon {
            id: closeIcon
            anchors {
                right: headerItem.right
                margins: Units.smallSpacing
                top: headerItem.top
            }
            z: 3
            visible: !Settings.isMobile
            width: Units.iconSizes.smallMedium
            height: width
            source: closeMouseArea.containsMouse ? "window-close" : "window-close-symbolic"
            active: closeMouseArea.containsMouse
            MouseArea {
                id: closeMouseArea
                hoverEnabled: true
                anchors.fill: parent
                onClicked: root.close();
            }
        }
        Rectangle {
            id: headerItem
            width: flickableContents.width
            x: flickableContents.x
            visible: root.header
            height: Math.max(headerParent.implicitHeight, closeIcon.height) + Units.smallSpacing * 2
            color: Theme.backgroundColor
            //different y depending if we're a listview or a normal item
            y: Math.max(0, -scrollView.flickableItem.contentY - (scrollView.contentItem != flickableContents ? height : 0))
            z: 2
            Item {
                id: headerParent
                implicitHeight: header ? header.implicitHeight : 0
                anchors {
                    fill: parent
                    margins: Units.smallSpacing
                    rightMargin: closeIcon.width + Units.smallSpacing
                }
            }
            
            EdgeShadow {
                z: -2
                edge: Qt.TopEdge
                anchors {
                    right: parent.right
                    left: parent.left
                    top: parent.bottom
                }

                opacity: parent.y == 0 ? 1 : 0

                Behavior on opacity {
                    NumberAnimation {
                        duration: Units.longDuration
                        easing.type: Easing.InOutQuad
                    }
                }
            }
        }
        Rectangle {
            id: footerItem
            width: flickableContents.width
            x: flickableContents.x
            visible: root.footer
            height: footerParent.implicitHeight + Units.smallSpacing * 2 + extraMargin
            color: Theme.backgroundColor
            y: mainItem.mapFromItem(flickableContents, 0, flickableContents.height).y - height
            //Show an extra margin when:
            //* the application is in mobile mode (no toolbarapplicationheader)
            //* the bottom screen controls are visible
            //* the sheet is disaplayed *under* the controls
            property int extraMargin: (!root.parent ||
                applicationWindow === "undefined" ||
                (root.parent === applicationWindow().overlay && root.parent.action.main) ||
                !applicationWindow().controlsVisible ||
                !applicationWindow().header ||
                applicationWindow().header.toString().indexOf("ToolBarApplicationHeader") === 0)
                    ? 0 : Units.gridUnit * 3
            Connections {
                target: scrollView.flickableItem
                onContentYChanged: footerItem.y = Math.min(mainItem.height, mainItem.mapFromItem(flickableContents, 0, flickableContents.height).y) - footerItem.height;

                onHeightChanged: scrollView.flickableItem.contentYChanged()
            }
            z: 2
            Item {
                id: footerParent
                implicitHeight: footer ? footer.implicitHeight : 0
                anchors {
                    top: parent.top
                    left: parent.left
                    right: parent.right
                    margins: Units.smallSpacing
                }
            }

            EdgeShadow {
                z: -2
                edge: Qt.BottomEdge
                anchors {
                    right: parent.right
                    left: parent.left
                    bottom: parent.top
                }

                opacity: parent.y + parent.height < mainItem.height ? 0 : 1

                Behavior on opacity {
                    NumberAnimation {
                        duration: Units.longDuration
                        easing.type: Easing.InOutQuad
                    }
                }
            }
        }

        Item {
            id: flickableContents
            //anchors.horizontalCenter: parent.horizontalCenter
            x: (mainItem.width - width) / 2

            readonly property real listHeaderHeight: scrollView.flickableItem && root.contentItem.headerItem ? root.contentItem.headerItem.height : 0

            y: (scrollView.contentItem != flickableContents ? -scrollView.flickableItem.contentY - listHeaderHeight  - (headerItem.visible ? headerItem.height : 0): 0)

            width: root.contentItem.implicitWidth <= 0 ? mainItem.width : Math.max(mainItem.width/2, Math.min(mainItem.width, root.contentItem.implicitWidth))

            height: (scrollView.contentItem != flickableContents ? scrollView.flickableItem.contentHeight + listHeaderHeight : (root.contentItem.height + topPadding + bottomPadding)) + (headerItem.visible ? headerItem.height : 0) + (footerItem.visible ? footerItem.height : 0)

            Item {
                id: contentItemParent
                anchors {
                    fill: parent
                    leftMargin: leftPadding
                    topMargin: topPadding + (headerItem.visible ? headerItem.height : 0)
                    rightMargin: rightPadding
                    bottomMargin: bottomPadding + (footerItem.visible ? footerItem.height : 0)
                }
            }
        }
        Binding {
            when: scrollView.flickableItem != null
            target: scrollView.flickableItem
            property: "topMargin"
            //hack needed for smoother open anim
            value: openAnimation.running ? -scrollView.flickableItem.contentY : -openAnimation.topOpenPosition
        }
        Binding {
            when: scrollView.flickableItem != null
            target: scrollView.flickableItem
            property: "bottomMargin"
            value: openAnimation.margins
        }

        Binding {
            target: scrollView.verticalScrollBar ? scrollView.verticalScrollBar.anchors : null
            property: "topMargin"
            value: headerItem.y + headerItem.height
        }
        Binding {
            target: scrollView.verticalScrollBar
            property: "height"
            value: mainItem.height - (scrollView.verticalScrollBar ? scrollView.verticalScrollBar.anchors.topMargin : 0) - (mainItem.height - footerItem.y)
        }
        Binding {
            target: scrollView.verticalScrollBar ? scrollView.verticalScrollBar.anchors : null
            property: "rightMargin"
            value: mainItem.width - flickableContents.width - flickableContents.x
        }

        Connections {
            target: scrollView.flickableItem
            onContentHeightChanged: {
                if (openAnimation.running) {
                    openAnimation.running = false;
                    open();
                }
            }
            onDraggingChanged: {
                if (scrollView.flickableItem.dragging) {
                    return;
                }

                //close
                if ((mainItem.height + scrollView.flickableItem.contentY) < mainItem.height/2) {
                    closeAnimation.to = -mainItem.height;
                    closeAnimation.running = true;

                } else if ((mainItem.height*0.6 + scrollView.flickableItem.contentY) > scrollView.flickableItem.contentHeight) {
                    closeAnimation.to = scrollView.flickableItem.contentHeight;
                    closeAnimation.running = true;
                }
            }
        }

        Binding {
            target: scrollView.verticalScrollBar
            property: "visible"
            value: scrollView.flickableItem.contentHeight > mainItem.height*0.8
        }
        Connections {
            target: scrollView.verticalScrollBar
            onActiveChanged: {
                if (!scrollView.verticalScrollBar.active) {
                    scrollView.flickableItem.movementEnded();
                }
            }
        }
        ScrollView {
            id: scrollView
            anchors.fill: parent
            horizontalScrollBarPolicy: Qt.ScrollBarAlwaysOff
        }
    }
}
