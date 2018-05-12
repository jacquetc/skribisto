/*
 *   Copyright 2017 Marco Martin <mart@kde.org>
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

import QtQuick 2.6
import QtQuick.Layouts 1.2
import QtQuick.Controls 2.2
import org.kde.kirigami 2.4 as Kirigami

/**
 * This is the base class for Form layouts conforming to the
 * Kirigami Human interface guidelines. The layout will
 * be divided in two columns: on the right there will be a column
 * of fields, on the left their labels specified in the FormData attached
 * property.
 *
 * Example:
 * @code
 * import org.kde.kirigami 2.3 as Kirigami
 * Kirigami.FormLayout {
 *    TextField {
 *       Kirigami.FormData.label: "Label:"
 *    }
 *    Kirigami.Separator {
 *        Kirigami.FormData.label: "Section Title"
 *        Kirigami.FormData.isSection: true
 *    }
 *    TextField {
 *       Kirigami.FormData.label: "Label:"
 *    }
 *    TextField {
 *    }
 * }
 * @endcode
 * @inherits QtQuick.Controls.Control
 * @since 2.3
 */
Control {
    id: root

    /**
     * wideMode: bool
     * If true the layout will be optimized for a wide screen, such as
     * a desktop machine (the labels will be on a left column,
     * the fields on a right column beside it), if false (such as on a phone)
     * everything is laid out in a single column.
     * by default this will be based on wether the application is
     * wide enough for the layout of being in such mode.
     * It can be overridden by reasigning the property
     */
    property bool wideMode: width >= lay.wideImplicitWidth

    implicitWidth: lay.implicitWidth
    implicitHeight: lay.implicitHeight
    Layout.preferredHeight: lay.implicitHeight

    GridLayout {
        id: lay
        property int wideImplicitWidth
        columns: root.wideMode ? 2 : 1
        rowSpacing: Kirigami.Units.smallSpacing
        columnSpacing: Kirigami.Units.smallSpacing
        property var knownItems: []
        anchors {
            left: parent.left
            top: parent.top
            right: parent.right
        }

        Timer {
            id: hintCompression
            onTriggered: {
                if (root.wideMode) {
                    lay.wideImplicitWidth = lay.implicitWidth;
                }
            }
        }
        onImplicitWidthChanged: hintCompression.restart();
        Component.onCompleted:  wideImplicitWidth = lay.implicitWidth;
    }

    Item {
        id: temp
    }

    Timer {
        id: relayoutTimer
        interval: 0
        onTriggered: {
            var __items = children;
            //exclude the layout and temp
            for (var i = 2; i < __items.length; ++i) {
                var item = __items[i];

                //skip items that are already there
                if (lay.knownItems.indexOf(item) != -1 ||
                    //exclude Repeaters
                    //NOTE: this is an heuristic but there are't better ways
                    (item.hasOwnProperty("model") && item.model !== undefined && item.children.length == 0)) {
                    continue;
                }
                lay.knownItems.push(item);

                var itemContainer = itemComponent.createObject(temp, {"item": item})

                //if section, label goes after the separator
                if (item.Kirigami.FormData.isSection) {
                    //put an extra spacer
                    var placeHolder = placeHolderComponent.createObject(lay, {"item": item});
                    itemContainer.parent = lay;
                }

                if (item.Kirigami.FormData.checkable) {
                    var buddy = checkableBuddyComponent.createObject(lay, {"item": item})
                } else {
                    var buddy = buddyComponent.createObject(lay, {"item": item})
                }
                
                itemContainer.parent = lay;
            }
        }
    }

    onChildrenChanged: relayoutTimer.restart();

    Component.onCompleted: relayoutTimer.triggered();
    Component {
        id: itemComponent
        Item {
            id: container
            property var item
            enabled: item.enabled
            visible: item.visible
            implicitWidth: item.implicitWidth
            Layout.preferredWidth: item.Layout.preferredWidth > 0 ? item.Layout.preferredWidth : item.implicitWidth
            Layout.preferredHeight: item.Layout.preferredHeight > 0 ? item.Layout.preferredHeight : item.implicitHeight

            Layout.alignment: (root.wideMode ? Qt.AlignLeft | Qt.AlignVCenter : Qt.AlignHCenter | Qt.AlignTop)
            Layout.fillWidth: item.Kirigami.FormData.isSection
            Layout.columnSpan: item.Kirigami.FormData.isSection ? lay.columns : 1
            onItemChanged: {
                if (!item) {
                    container.destroy();
                }
            }
            onXChanged: item.x = x;
            onYChanged: item.y = y;
            onWidthChanged: item.width = width;
        }
    }
    Component {
        id: placeHolderComponent
        Item {
            property var item
            enabled: item.enabled
            visible: item.visible
            width: Kirigami.Units.smallSpacing
            height: Kirigami.Units.smallSpacing
            onItemChanged: {
                if (!item) {
                    labelItem.destroy();
                }
            }
        }
    }
    Component {
        id: buddyComponent
        Kirigami.Heading {
            id: labelItem

            property var item
            enabled: item.enabled
            visible: item.visible
            Kirigami.MnemonicData.enabled: item.Kirigami.FormData.buddyFor && item.Kirigami.FormData.buddyFor.activeFocusOnTab
            Kirigami.MnemonicData.controlType: Kirigami.MnemonicData.FormLabel
            Kirigami.MnemonicData.label: item.Kirigami.FormData.label
            text: Kirigami.MnemonicData.richTextLabel

            level: item.Kirigami.FormData.isSection ? 3 : 5

            Layout.columnSpan: item.Kirigami.FormData.isSection ? lay.columns : 1
            Layout.preferredHeight: item.Kirigami.FormData.label.length > 0 ? implicitHeight : Kirigami.Units.smallSpacing

            Layout.alignment: item.Kirigami.FormData.isSection
                             ? Qt.AlignLeft
                             : (root.wideMode
                                ? (Qt.AlignRight | (item.Kirigami.FormData.buddyFor.height > height * 2 ? Qt.AlignTop : Qt.AlignVCenter))
                                : (Qt.AlignLeft | Qt.AlignBottom))
            verticalAlignment: root.wideMode ? Text.AlignVCenter : Text.AlignBottom

            Layout.topMargin: item.Kirigami.FormData.buddyFor.height > implicitHeight * 2 ? Kirigami.Units.smallSpacing/2 : 0
            onItemChanged: {
                if (!item) {
                    labelItem.destroy();
                }
            }
            Shortcut {
                sequence: labelItem.Kirigami.MnemonicData.sequence
                onActivated: item.Kirigami.FormData.buddyFor.forceActiveFocus()
            }
        }
    }
    Component {
        id: checkableBuddyComponent
        CheckBox {
            id: labelItem
            property var item
            visible: item.visible
            Kirigami.MnemonicData.enabled: item.Kirigami.FormData.buddyFor && item.Kirigami.FormData.buddyFor.activeFocusOnTab
            Kirigami.MnemonicData.controlType: Kirigami.MnemonicData.FormLabel
            Kirigami.MnemonicData.label: item.Kirigami.FormData.label

            Layout.columnSpan: item.Kirigami.FormData.isSection ? lay.columns : 1
            Layout.preferredHeight: item.Kirigami.FormData.label.length > 0 ? implicitHeight : Kirigami.Units.smallSpacing

            Layout.alignment: item.Kirigami.FormData.isSection
                             ? Qt.AlignLeft
                             : (root.wideMode
                                ? (Qt.AlignRight | (item.Kirigami.FormData.buddyFor.height > height * 2 ? Qt.AlignTop : Qt.AlignVCenter))
                                : (Qt.AlignLeft | Qt.AlignBottom))
            Layout.topMargin: item.Kirigami.FormData.buddyFor.height > implicitHeight * 2 ? Kirigami.Units.smallSpacing/2 : 0
            
            activeFocusOnTab: indicator.visible && indicator.enabled
            text: labelItem.Kirigami.MnemonicData.richTextLabel
            enabled: labelItem.item.Kirigami.FormData.enabled
            checked: labelItem.item.Kirigami.FormData.checked

            onItemChanged: {
                if (!item) {
                    labelItem.destroy();
                }
            }
            Shortcut {
                sequence: labelItem.Kirigami.MnemonicData.sequence
                onActivated: {
                    checked = !checked
                    item.Kirigami.FormData.buddyFor.forceActiveFocus()
                }
            }
            onCheckedChanged: {
                item.Kirigami.FormData.checked = checked
            }
            contentItem: Kirigami.Heading {
                id: labelItemHeading
                level: labelItem.item.Kirigami.FormData.isSection ? 3 : 5
                text: labelItem.text
                verticalAlignment: root.wideMode ? Text.AlignVCenter : Text.AlignBottom
                enabled: labelItem.item.Kirigami.FormData.enabled
                leftPadding: parent.indicator.width
            }
            Rectangle {
                enabled: labelItem.indicator.enabled
                anchors.left: labelItemHeading.left
                anchors.right: labelItemHeading.right
                anchors.top: labelItemHeading.bottom
                anchors.leftMargin: labelItemHeading.leftPadding
                height: 1 * Kirigami.Units.devicePixelRatio
                color: Kirigami.Theme.highlightColor
                visible: labelItem.activeFocus && labelItem.indicator.visible
            }
        }
    }
}
