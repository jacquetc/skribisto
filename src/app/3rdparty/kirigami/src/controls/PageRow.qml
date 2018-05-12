/*
 *   Copyright 2016 Marco Martin <mart@kde.org>
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

import QtQuick 2.5
import QtQuick.Layouts 1.2
import QtQml.Models 2.2
import QtQuick.Templates 2.0 as T
import QtQuick.Controls 2.0 as QQC2
import org.kde.kirigami 2.4

/**
 * PageRow implements a row-based navigation model, which can be used
 * with a set of interlinked information pages. Items are pushed in the
 * back of the row and the view scrolls until that row is visualized.
 * A PageRowcan show a single page or a multiple set of columns, depending
 * on the window width: on a phone a single column should be fullscreen,
 * while on a tablet or a desktop more than one column should be visible.
 * @inherit QtQuick.Templates.Control
 */
T.Control {
    id: root

//BEGIN PROPERTIES
    /**
     * This property holds the number of items currently pushed onto the view
     */
    readonly property int depth: popScrollAnim.running && popScrollAnim.pendingDepth > -1 ? popScrollAnim.pendingDepth : pagesLogic.count

    /**
     * The last Page in the Row
     */
    readonly property Item lastItem: pagesLogic.count ? pagesLogic.get(pagesLogic.count - 1).page : null

    /**
     * The currently visible Item
     */
    readonly property Item currentItem: mainView.currentItem ? mainView.currentItem.page : null

    /**
     * the index of the currently visible Item
     */
    property alias currentIndex: mainView.currentIndex

    /**
     * The initial item when this PageRow is created
     */
    property variant initialPage

    /**
     * The main flickable of this Row
     */
    contentItem: mainView

    /**
     * The default width for a column
     * default is wide enough for 30 grid units.
     * Pages can override it with their Layout.fillWidth,
     * implicitWidth Layout.minimumWidth etc.
     */
    property int defaultColumnWidth: Units.gridUnit * 20

    /**
     * interactive: bool
     * If true it will be possible to go back/forward by dragging the
     * content themselves with a gesture.
     * Otherwise the only way to go back will be programmatically
     * default: true
     */
    property alias interactive: mainView.interactive

    /**
     * wideMode: bool
     * If true, the PageRow is wide enough that willshow more than one column at once
     * @since 5.37
     */
    readonly property bool wideMode: root.width >= root.defaultColumnWidth*2 && pagesLogic.count >= 2

    /**
     * separatorVisible: bool
     * True if the separator between pages should be visible
     * default: true
     * @since 5.38
     */
    property bool separatorVisible: true
//END PROPERTIES

//BEGIN FUNCTIONS
    /**
     * Pushes a page on the stack.
     * The page can be defined as a component, item or string.
     * If an item is used then the page will get re-parented.
     * If a string is used then it is interpreted as a url that is used to load a page 
     * component.
     *
     * @param page The page can also be given as an array of pages.
     *     In this case all those pages will
     *     be pushed onto the stack. The items in the stack can be components, items or
     *     strings just like for single pages.
     *     Additionally an object can be used, which specifies a page and an optional
     *     properties property.
     *     This can be used to push multiple pages while still giving each of
     *     them properties.
     *     When an array is used the transition animation will only be to the last page.
     *
     * @param properties The properties argument is optional and allows defining a
     * map of properties to set on the page.
     * @return The new created page
     */
    function push(page, properties) {
        //don't push again things already there
        if (page.createObject === undefined && typeof page != "string" && pagesLogic.containsPage(page)) {
            print("The item " + page + " is already in the PageRow");
            return;
        }

        if (popScrollAnim.running) {
            popScrollAnim.running = false;
            popScrollAnim.popPageCleanup(popScrollAnim.pendingPage);
        }

        popScrollAnim.popPageCleanup(currentItem);

        // figure out if more than one page is being pushed
        var pages;
        if (page instanceof Array) {
            pages = page;
            page = pages.pop();
            if (page.createObject === undefined && page.parent === undefined && typeof page != "string") {
                properties = properties || page.properties;
                page = page.page;
            }
        }

        // push any extra defined pages onto the stack
        if (pages) {
            var i;
            for (i = 0; i < pages.length; i++) {
                var tPage = pages[i];
                var tProps;
                if (tPage.createObject === undefined && tPage.parent === undefined && typeof tPage != "string") {
                    if (pagesLogic.containsPage(tPage)) {
                        print("The item " + page + " is already in the PageRow");
                        continue;
                    }
                    tProps = tPage.properties;
                    tPage = tPage.page;
                }

                var container = pagesLogic.initPage(tPage, tProps);
                pagesLogic.append(container);
            }
        }

        // initialize the page
        var container = pagesLogic.initPage(page, properties);
        pagesLogic.append(container);
        container.visible = container.page.visible = true;

        mainView.currentIndex = container.level;
        return container.page
    }

    /**
     * Pops a page off the stack.
     * @param page If page is specified then the stack is unwound to that page,
     * to unwind to the first page specify
     * page as null.
     * @return The page instance that was popped off the stack.
     */
    function pop(page) {
        if (depth == 0) {
            return;
        }

        //if a pop was animating, stop it
        if (popScrollAnim.running) {
            popScrollAnim.running = false;
            popScrollAnim.popPageCleanup(popScrollAnim.pendingPage);
        //if a push was animating, stop it
        } else {
            mainView.positionViewAtIndex(mainView.currentIndex, ListView.Beginning);
        }

        popScrollAnim.from = mainView.contentX

        if ((!page || !page.parent) && pagesLogic.count > 1) {
            page = pagesLogic.get(pagesLogic.count - 2).page;
        }
        popScrollAnim.to = page && page.parent ? page.parent.x : 0;
        popScrollAnim.pendingPage = page;
        popScrollAnim.pendingDepth = page && page.parent ? page.parent.level + 1 : 0;

        popScrollAnim.running = true;
    }

    SequentialAnimation {
        id: popScrollAnim
        property real from
        property real to
        property var pendingPage
        property int pendingDepth: -1
        function popPageCleanup(page) {
            if (pagesLogic.count == 0) {
                return;
            }
            if (popScrollAnim.running) {
                popScrollAnim.running = false;
            }

            var oldPage = pagesLogic.get(pagesLogic.count-1).page;
            if (page !== undefined) {
                // an unwind target has been specified - pop until we find it
                while (page != oldPage && pagesLogic.count > 1) {
                    pagesLogic.removePage(oldPage.parent.level);

                    oldPage = pagesLogic.get(pagesLogic.count-1).page;
                }
            } else {
                pagesLogic.removePage(pagesLogic.count-1);
            }
        }
        NumberAnimation {
            target: mainView
            properties: "contentX"
            duration: Units.shortDuration
            from: popScrollAnim.from
            to: popScrollAnim.to
        }
        ScriptAction {
            script: {
                //snap
                mainView.flick(100, 0)
                popScrollAnim.popPageCleanup(popScrollAnim.pendingPage);
            }
        }
    }
    /**
     * Replaces a page on the stack.
     * @param page The page can also be given as an array of pages.
     *     In this case all those pages will
     *     be pushed onto the stack. The items in the stack can be components, items or
     *     strings just like for single pages.
     *     Additionally an object can be used, which specifies a page and an optional
     *     properties property.
     *     This can be used to push multiple pages while still giving each of
     *     them properties.
     *     When an array is used the transition animation will only be to the last page.
     * @param properties The properties argument is optional and allows defining a
     * map of properties to set on the page.
     * @see push() for details.
     */
    function replace(page, properties) {
        if (currentIndex>=1)
            popScrollAnim.popPageCleanup(pagesLogic.get(currentIndex-1).page);
        else if (currentIndex==0)
            popScrollAnim.popPageCleanup();
        else
            console.warn("There's no page to replace");
        return push(page, properties);
    }

    /**
     * Clears the page stack.
     * Destroy (or reparent) all the pages contained.
     */
    function clear() {
        return pagesLogic.clearPages();
    }

    /**
     * @return the page at idx
     * @param idx the depth of the page we want
     */
    function get(idx) {
        return pagesLogic.get(idx).page;
    }

    /**
     * go back to the previous index and scroll to the left to show one more column
     */
    function flickBack() {
        if (depth > 1) {
            currentIndex = Math.max(0, currentIndex - 1);
        }

        if (LayoutMirroring.enabled) {
            if (!mainView.atEnd) {
                mainViewScrollAnim.from = mainView.contentX
                mainViewScrollAnim.to =  Math.min(mainView.contentWidth - mainView.width, mainView.contentX + defaultColumnWidth)
                mainViewScrollAnim.running = true;
            }
        } else {
            if (mainView.contentX - mainView.originX > 0) {
                mainViewScrollAnim.from = mainView.contentX
                mainViewScrollAnim.to =  Math.max(mainView.originX, mainView.contentX - defaultColumnWidth)
                mainViewScrollAnim.running = true;
            }
        }
    }

    /**
     * layers: QtQuick.Controls.PageStack
     * Access to the modal layers.
     * Sometimes an application needs a modal page that always covers all the rows.
     * For instance the full screen image of an image viewer or a settings page.
     * @since 5.38
     */
    property alias layers: layersStack
//END FUNCTIONS

    onInitialPageChanged: {
        clear();
        if (initialPage) {
            push(initialPage, null)
        }
    }

    Keys.forwardTo: [currentItem]

    SequentialAnimation {
        id: mainViewScrollAnim
        property real from
        property real to
        NumberAnimation {
            target: mainView
            properties: "contentX"
            duration: Units.longDuration
            from: mainViewScrollAnim.from
            to: mainViewScrollAnim.to
        }
        ScriptAction {
            script: mainView.flick(100, 0)
        }
    }
    QQC2.StackView {
        id: layersStack
        z: 99
        anchors.fill: parent
        initialItem: mainView
        function clear () {
            //don't let it kill the main page row
            var d = root.depth;
            for (var i = 1; i < d; ++i) {
                pop();
            } 
        }

        popEnter: Transition {
            YAnimator {
                from: -height
                to: 0
                duration: Units.longDuration
                easing.type: Easing.OutCubic
            }
        }
        popExit: Transition {
            YAnimator {
                from: 0
                to: height
                duration: Units.longDuration
                easing.type: Easing.OutCubic
            }
        }

        pushEnter: Transition {
            YAnimator {
                from: height
                to: 0
                duration: Units.longDuration
                easing.type: Easing.OutCubic 
            }
        }

        pushExit: Transition {
            YAnimator {
                from: 0
                to: -height
                duration: Units.longDuration
                easing.type: Easing.OutCubic 
            }
        }

        replaceEnter: Transition {
            YAnimator {
                from: height
                to: 0
                duration: Units.longDuration
                easing.type: Easing.OutCubic
            }
        }

        replaceExit: Transition {
            YAnimator {
                from: 0
                to: -height
                duration: Units.longDuration
                easing.type: Easing.OutCubic
            }
        }
    }

    ListView {
        id: mainView
        boundsBehavior: Flickable.StopAtBounds
        orientation: Qt.Horizontal
        snapMode: ListView.SnapToItem
        currentIndex: root.currentIndex
        property int marginForLast: count > 1 ? pagesLogic.get(count-1).page.width - pagesLogic.get(count-1).width : 0
        leftMargin: LayoutMirroring.enabled ? marginForLast : 0
        rightMargin: LayoutMirroring.enabled ? 0 : marginForLast
        preferredHighlightBegin: 0
        preferredHighlightEnd: 0
        highlightMoveDuration: Units.longDuration
        highlightFollowsCurrentItem: true
        onMovementEnded: currentIndex = Math.max(0, indexAt(contentX, 0))
        onFlickEnded: onMovementEnded();
        onCurrentIndexChanged: {
            if (currentItem) {
                currentItem.page.forceActiveFocus();
            }
        }
        model: ObjectModel {
            id: pagesLogic
            readonly property var componentCache: new Array()
            readonly property int roundedDefaultColumnWidth: root.width < root.defaultColumnWidth*2 ? root.width : root.defaultColumnWidth

            function removePage(id) {
                if (id < 0 || id >= count) {
                    print("Tried to remove an invalid page index:" + id);
                    return;
                }

                var item = pagesLogic.get(id);
                if (item.owner) {
                    item.page.visible = false;
                    item.page.parent = item.owner;
                }
                //FIXME: why reparent ing is necessary?
                //is destroy just an async deleteLater() that isn't executed immediately or it actually leaks?
                pagesLogic.remove(id);
                item.parent = root;
                if (item.page.parent==item) {
                    item.page.destroy(1)
                }
                item.destroy();
            }
            function clearPages () {
                popScrollAnim.running = false;
                popScrollAnim.pendingDepth = -1;
                while (count > 0) {
                    removePage(count-1);
                }
            }
            function initPage(page, properties) {
                var container = containerComponent.createObject(mainView, {
                    "level": pagesLogic.count,
                    "page": page
                });

                var pageComp;
                if (page.createObject) {
                    // page defined as component
                    pageComp = page;
                } else if (typeof page == "string") {
                    // page defined as string (a url)
                    pageComp = pagesLogic.componentCache[page];
                    if (!pageComp) {
                        pageComp = pagesLogic.componentCache[page] = Qt.createComponent(page);
                    }
                }
                if (pageComp) {
                    if (pageComp.status == Component.Error) {
                        throw new Error("Error while loading page: " + pageComp.errorString());
                    } else {
                        // instantiate page from component
                        page = pageComp.createObject(container.pageParent, properties || {});
                    }
                } else {
                    // copy properties to the page
                    for (var prop in properties) {
                        if (properties.hasOwnProperty(prop)) {
                            page[prop] = properties[prop];
                        }
                    }
                }

                container.page = page;
                if (page.parent == null || page.parent == container.pageParent) {
                    container.owner = null;
                }

                // the page has to be reparented
                if (page.parent != container) {
                    page.parent = container;
                }

                return container;
            }
            function containsPage(page) {
                for (var i = 0; i < pagesLogic.count; ++i) {
                    var candidate = pagesLogic.get(i);
                    if (candidate.page == page) {
                        print("The item " + page + " is already in the PageRow");
                        return;
                    }
                }
            }
        }
        T.ScrollIndicator.horizontal: T.ScrollIndicator {
            anchors {
                left: parent.left
                right: parent.right
                bottom: parent.bottom
            }
            height: Units.smallSpacing
            contentItem: Rectangle {
                height: Units.smallSpacing
                width: Units.smallSpacing
                color: Theme.textColor
                opacity: 0
                onXChanged: {
                    opacity = 0.3
                    scrollIndicatorTimer.restart();
                }
                Behavior on opacity {
                    OpacityAnimator {
                        duration: Units.longDuration
                        easing.type: Easing.InOutQuad
                    }
                }
                Timer {
                    id: scrollIndicatorTimer
                    interval: Units.longDuration * 4
                    onTriggered: parent.opacity = 0;
                }
            }
        }

        onContentWidthChanged: mainView.positionViewAtIndex(root.currentIndex, ListView.Contain)
    }

    Component {
        id: containerComponent

        MouseArea {
            id: container
            height: mainView.height
            width: root.width
            state: page ? (!root.wideMode ? "vertical" : (container.level >= pagesLogic.count - 1 ? "last" : "middle")) : "";
            acceptedButtons: Qt.LeftButton | Qt.BackButton | Qt.ForwardButton

            property int level

            readonly property int hint: page && page.implicitWidth ? page.implicitWidth : root.defaultColumnWidth
            readonly property int roundedHint: Math.floor(root.width/hint) > 0 ? root.width/Math.floor(root.width/hint) : root.width

            property Item page
            property Item owner
            onPageChanged: {
                if (page) {
                    owner = page.parent;
                    page.parent = container;
                    page.anchors.fill = container;
                }
            }
            drag.filterChildren: true
            onClicked: {
                switch (mouse.button) {
                case Qt.BackButton:
                    root.flickBack();
                    break;
                case Qt.ForwardButton:
                    root.currentIndex = Math.min(root.depth, root.currentIndex + 1);
                    break;
                default:
                    root.currentIndex = level;
                    break;
                }
            }
            onFocusChanged: {
                if (focus) {
                    root.currentIndex = level;
                }
            }

            Separator {
                z: 999
                anchors {
                    top: parent.top
                    bottom: parent.bottom
                    left: parent.left
                }
                visible: root.separatorVisible && container.level > 0
            }
            states: [
                State {
                    name: "vertical"
                    PropertyChanges {
                        target: container
                        width: root.width
                    }
                    PropertyChanges {
                        target: container.page ? container.page.anchors : null
                        rightMargin: 0
                    }
                },
                State {
                    name: "last"
                    PropertyChanges {
                        target: container
                        width: pagesLogic.roundedDefaultColumnWidth
                    }
                    PropertyChanges {
                        target: container.page.anchors
                        rightMargin: {
                            return -(root.width - pagesLogic.roundedDefaultColumnWidth*2);
                        }
                    }
                },
                State {
                    name: "middle"
                    PropertyChanges {
                        target: container
                        width: pagesLogic.roundedDefaultColumnWidth
                    }
                    PropertyChanges {
                        target: container.page.anchors
                        rightMargin: 0
                    }
                }
            ]
        }
    }
}
