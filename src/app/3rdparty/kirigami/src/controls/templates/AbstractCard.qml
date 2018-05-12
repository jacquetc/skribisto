/*
 *   Copyright 2018 Marco Martin <mart@kde.org>
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

import QtQuick 2.6
import QtQuick.Layouts 1.2
import QtQuick.Templates 2.0 as T
import org.kde.kirigami 2.4 as Kirigami

/**
 * A AbstractCard is the base for cards. A Card is a visual object that serves
 * as an entry point for more detailed information. An abstractCard is empty,
 * providing just the look and the base properties and signals for an ItemDelegate.
 * It can be filled with any custom layout of items, its content is organized
 * in 3 properties: header, contentItem and footer.
 * Use this only when you need particular custom contents, for a standard layout
 * for cards, use the Card component.
 *
 * @see Card
 * @inherits QtQuick.Templates.ItemDelegate
 * @since 2.4
 */
T.ItemDelegate {
    id: root

    /**
     * header: Item
     * This item serves as header, it will be put either on top if headerOrientation
     * is Qt.Vertical(default) or on the left if it's Qt.Horizontal
     */
    property Item header

    /**
     * headerOrientation: Qt.Orientation
     * If Qt.Vertical the header will be positioned on top(default),
     * if Qt.Horizontal will be positioned on the left (or right if an RTL layout is used)
     */
    property int headerOrientation: Qt.Vertical

    /**
     * footer: Item
     * This item serves as footer, and it will be positioned at the bottom of the card.
     */
    property Item footer

    /**
     * showClickFeedback: bool
     * if true, when clicking or tapping on the card area, the card will be colored
     * to show a visual click feedback.
     * Use this if you want to do an action in the onClicked signal handler of the card.
     */
    property bool showClickFeedback: false

    Layout.fillWidth: true

    implicitWidth: Math.max(background.implicitWidth, mainLayout.implicitWidth) + leftPadding + rightPadding
    
    implicitHeight: mainLayout.implicitHeight + topPadding + bottomPadding

    hoverEnabled: !Kirigami.Settings.tabletMode && showClickFeedback
    //if it's in a CardLayout, try to expand horizontal cards to both columns
    Layout.columnSpan: headerOrientation == Qt.Horizontal && parent.hasOwnProperty("columns") ? parent.columns : 1

    Kirigami.Theme.inherit: false
    Kirigami.Theme.colorSet: Kirigami.Theme.View

    topPadding: Kirigami.Units.largeSpacing
    leftPadding: Kirigami.Units.largeSpacing
    bottomPadding: Kirigami.Units.largeSpacing
    rightPadding: Kirigami.Units.largeSpacing

    GridLayout {
        id: mainLayout
        rowSpacing: root.topPadding
        columnSpacing: root.leftPadding
        anchors {
            top: parent.top
            left: parent.left
            right: parent.right
            leftMargin: root.leftPadding
            topMargin: root.topPadding
            rightMargin: root.rightPadding
            //never anchor bottom, to not have binding loops
        }
        columns: headerOrientation == Qt.Vertical ? 1 : 2
        function preferredHeight(item) {
            if (!item) {
                return 0;
            }
            if (item.Layout.preferredHeight > 0) {
                return item.Layout.preferredHeight;
            }
            return item.implicitHeight
        }
        Item {
            id: headerParent
            Layout.fillWidth: true
            Layout.fillHeight: root.headerOrientation == Qt.Horizontal
            Layout.rowSpan: root.headerOrientation == Qt.Vertical ? 1 : 2
            Layout.preferredWidth: header ? header.implicitWidth : 0
            Layout.preferredHeight: root.headerOrientation == Qt.Vertical ? mainLayout.preferredHeight(header) : -1
        }
        Item {
            id: contentItemParent
            Layout.fillWidth: true
            Layout.preferredWidth: contentItem ? contentItem.implicitWidth : 0
            Layout.preferredHeight: mainLayout.preferredHeight(contentItem)
        }
        Item {
            id: footerParent
            Layout.fillWidth: true
            Layout.preferredWidth: footer ? footer.implicitWidth : 0
            Layout.preferredHeight: mainLayout.preferredHeight(footer)
        }
    }

//BEGIN signal handlers
    onContentItemChanged: {
        if (!contentItem) {
            return;
        }

        contentItem.parent = contentItemParent;
        contentItem.anchors.fill = contentItemParent;
    }
    onHeaderChanged: {
        if (!header) {
            return;
        }

        header.parent = headerParent;
        header.anchors.fill = headerParent;
    }
    onFooterChanged: {
        if (!footer) {
            return;
        }

        //make the footer always looking it's at the bottom of the card
        footer.parent = footerParent;
        footer.anchors.left = footerParent.left;
        footer.anchors.top = footerParent.top;
        footer.anchors.right = footerParent.right;
        footer.anchors.topMargin = Qt.binding(function() {return (root.height - root.bottomPadding - root.topPadding)  - (footerParent.y + footerParent.height)});
    }
    Component.onCompleted: {
        contentItemChanged();
    }
//END signal handlers
}
