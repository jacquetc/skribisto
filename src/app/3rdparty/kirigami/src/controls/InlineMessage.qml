/*
 *   Copyright 2018 Eike Hein <hein@kde.org>
 *   Copyright 2018 Marco Martin <mart@kde.org>
 *   Copyright 2018 Kai Uwe Broulik <kde@privat.broulik.de>
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
import QtGraphicalEffects 1.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.0 as Controls
import org.kde.kirigami 2.4 as Kirigami
import "private"

import "templates" as T

/**
 * An inline message item with support for informational, positive,
 * warning and error types, and with support for associated actions.
 *
 * InlineMessage can be used to give information to the user or
 * interact with the user, without requiring the use of a dialog.
 *
 * The InlineMessage item is hidden by default. It also manages its
 * height (and implicitHeight) during an animated reveal when shown.
 * You should avoid setting height on an InlineMessage unless it is
 * already visible.
 *
 * Optionally an icon can be set, defaulting to an icon appropriate
 * to the message type otherwise.
 *
 * Optionally a close button can be shown.
 *
 * Actions are added from left to right. If more actions are set than
 * can fit, an overflow menu is provided.
 *
 * Example:
 * @code
 * InlineMessage {
 *     type: Kirigami.MessageType.Error
 *
 *     text: "My error message"
 *
 *     actions: [
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
 * }
 * @endcode
 *
 * @since 5.45
 */

T.InlineMessage {
    id: root

    implicitHeight: visible ? contentLayout.implicitHeight + (2 * (background.border.width + Kirigami.Units.smallSpacing)) : 0

    property bool _animating: false

    Behavior on implicitHeight {
        enabled: !root.visible

        SequentialAnimation {
            PropertyAction { targets: root; property: "_animating"; value: true }
            NumberAnimation { duration: Kirigami.Units.longDuration }
        }
    }

    onVisibleChanged: {
        if (!visible) {
            contentLayout.opacity = 0.0;
        }
    }

    opacity: visible ? 1.0 : 0.0

    Behavior on opacity {
        enabled: !root.visible

        NumberAnimation { duration: Kirigami.Units.shortDuration }
    }

    onOpacityChanged: {
        if (opacity == 0.0) {
            contentLayout.opacity = 0.0;
        } else if (opacity == 1.0) {
            contentLayout.opacity = 1.0;
        }
    }

    onImplicitHeightChanged: {
        height = implicitHeight;
    }

    background: Rectangle {
        id: bgBorderRect

        color: {
            if (root.type == Kirigami.MessageType.Positive) {
                return Kirigami.Theme.positiveTextColor;
            } else if (root.type == Kirigami.MessageType.Warning) {
                return Kirigami.Theme.neutralTextColor;
            } else if (root.type == Kirigami.MessageType.Error) {
                return Kirigami.Theme.negativeTextColor;
            }

            return Kirigami.Theme.activeTextColor;
        }

        radius: Kirigami.Units.smallSpacing / 2

        Rectangle {
            id: bgFillRect

            anchors.fill: parent
            anchors.margins: Kirigami.Units.devicePixelRatio

            color: Kirigami.Theme.backgroundColor

            radius: bgBorderRect.radius * 0.60
        }

        Rectangle {
            anchors.fill: bgFillRect

            color: bgBorderRect.color

            opacity: 0.20

            radius: bgFillRect.radius
        }

        layer.enabled: true
        layer.effect: DropShadow {
            horizontalOffset: 0
            verticalOffset: 1
            radius: 12
            samples: 32
            color: Qt.rgba(0, 0, 0, 0.5)
        }
    }

    GridLayout {
        id: contentLayout

        x: background.border.width + Kirigami.Units.smallSpacing
        y: background.border.width + Kirigami.Units.smallSpacing
        width: parent.width - (2 * (background.border.width + Kirigami.Units.smallSpacing))

        // Used to defer opacity animation until we know if InlineMessage was
        // initialized visible.
        property bool complete: false

        Behavior on opacity {
            enabled: root.visible && contentLayout.complete

            SequentialAnimation {
                NumberAnimation { duration: Kirigami.Units.shortDuration * 2 }
                PropertyAction { targets: root; property: "_animating"; value: false }
            }
        }

        rowSpacing: Kirigami.Units.largeSpacing
        columnSpacing: Kirigami.Units.smallSpacing

        Kirigami.Icon {
            id: icon

            width: Kirigami.Units.iconSizes.smallMedium
            height: width

            Layout.alignment: text.lineCount > 1 ? Qt.AlignTop : Qt.AlignVCenter

            Layout.minimumWidth: width
            Layout.minimumHeight: height

            source: {
                if (root.icon.source) {
                    return root.icon.source;
                }

                if (root.type == Kirigami.MessageType.Positive) {
                    return "dialog-positive";
                } else if (root.type == Kirigami.MessageType.Warning) {
                    return "dialog-warning";
                } else if (root.type == Kirigami.MessageType.Error) {
                    return "dialog-error";
                }

                return "dialog-information";
            }

            color: root.icon.color
        }

        MouseArea {
            implicitHeight: text.implicitHeight

            Layout.fillWidth: true
            Layout.alignment: text.lineCount > 1 ? Qt.AlignTop : Qt.AlignVCenter
            Layout.row: 0
            Layout.column: 1

            cursorShape: text.hoveredLink ? Qt.PointingHandCursor : Qt.ArrowCursor

            Controls.Label {
                id: text

                width: parent.width

                color: Kirigami.Theme.textColor
                wrapMode: Text.WordWrap
                elide: Text.ElideRight

                text: root.text

                onLinkHovered: root.linkHovered(link)
                onLinkActivated: root.linkActivated(link)
            }
        }

        RowLayout {
            id: actionsLayout

            visible: root.actions.length

            Layout.alignment: Qt.AlignRight
            Layout.fillWidth: true

            Layout.row: {
                if (messageTextMetrics.width + Kirigami.Units.smallSpacing >
                    (contentLayout.width - icon.width - actionsLayout.width
                    - closeButton.width - (3 * contentLayout.columnSpacing))) {
                    return 1;
                }

                return 0;
            }
            Layout.column: Layout.row ? 0 : 2
            Layout.columnSpan: Layout.row ? (closeButton.visible ? 3 : 2) : 1

            property var overflowSet: []

            spacing: Kirigami.Units.smallSpacing

            // TODO Use Array.findIndex once we depend on Qt 5.9+.
            function findIndex(array, cb) {
                for (var i = 0, length = array.length; i < length; ++i) {
                    if (cb(array[i])) {
                        return i;
                    }
                }
                return -1;
            }

            TextMetrics {
                id: messageTextMetrics

                font: text.font
                text: text.text
            }

            Repeater {
                model: root.actions

                //TODO: use a normal button when we can depend from Qt 5.10
                delegate: PrivateActionToolButton {
                    id: actionButton

                    flat: false
                    kirigamiAction: modelData
                    visible: modelData.visible && fits

                    Layout.alignment: Qt.AlignVCenter
                    Layout.minimumWidth: implicitWidth

                    readonly property bool fits: {
                        var minX = 0;

                        for (var i = 0; i < index; ++i) {
                            if (actionsLayout.children[i].visible) {
                                minX += actionsLayout.children[i].implicitWidth + actionsLayout.spacing;
                            }
                        }

                        return minX + implicitWidth < contentLayout.width - moreButton.width;
                    }

                    onFitsChanged: updateOverflowSet()

                    function updateOverflowSet() {
                        var index = actionsLayout.findIndex(actionsLayout.overflowSet, function(act){
                            return act == modelData});

                        if ((fits || !modelData.visible) && index > -1) {
                            actionsLayout.overflowSet.splice(index, 1);
                        } else if (!fits && modelData.visible && index == -1) {
                            actionsLayout.overflowSet.push(modelData);
                        }

                        actionsLayout.overflowSetChanged();
                    }

                    Connections {
                        target: modelData

                        onVisibleChanged: actionButton.updateOverflowSet();
                    }

                    Component.onCompleted: updateOverflowSet();
                }
            }

            Controls.ToolButton {
                id: moreButton

                visible: actionsLayout.overflowSet.length > 0

                checked: menu.visible

                onClicked: menu.visible ? menu.close() : menu.open()

                Kirigami.Icon {
                    anchors.fill: parent
                    source: "overflow-menu"
                    anchors.margins: 4
                }

                Controls.Menu {
                    id: menu
                    y: -height
                    x: -width + moreButton.width

                    Repeater {
                        model: root.actions

                        delegate: BasicListItem {
                            enabled: modelData.enabled

                            checkable: modelData.checkable
                            checked: modelData.checked

                            text: modelData ? modelData.text : ""
                            icon: modelData.icon

                            onClicked: {
                                modelData.trigger();
                                menu.visible = false;
                            }

                            separatorVisible: false

                            backgroundColor: "transparent"

                            visible: actionsLayout.findIndex(actionsLayout.overflowSet, function(act) {
                                return act == modelData}) > -1 && modelData.visible
                        }
                    }
                }
            }
        }

        Controls.ToolButton {
            id: closeButton

            visible: root.showCloseButton

            Layout.alignment: text.lineCount > 1 || actionsLayout.Layout.row ? Qt.AlignTop : Qt.AlignVCenter
            Layout.row: 0
            Layout.column: actionsLayout.Layout.row ? 2 : 3

            //TODO: use toolbuttons icons when we can depend from Qt 5.10
            Kirigami.Icon {
                anchors.centerIn: parent
                source: "dialog-close"
                width: Kirigami.Units.iconSizes.smallMedium
                height: width
            }

            onClicked: root.visible = false
        }

        Component.onCompleted: complete = true
    }
}
