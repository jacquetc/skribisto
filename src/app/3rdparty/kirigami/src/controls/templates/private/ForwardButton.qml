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

import QtQuick 2.1
import QtQuick.Layouts 1.2
import QtQuick.Controls 2.0 as Controls

import org.kde.kirigami 2.4

Controls.ToolButton {
    id: button

    property Flickable headerFlickable
    //visible: headerFlickable.internalHeaderStyle == ApplicationHeaderStyle.Titles && !applicationWindow().pageStack.contentItem.atXEnd && applicationWindow().pageStack.layers.depth < 2
    enabled: __appWindow.pageStack.currentIndex < __appWindow.pageStack.depth-1 || !applicationWindow().pageStack.contentItem.atXEnd
    visible: applicationWindow().pageStack.contentItem.contentWidth > applicationWindow().pageStack.width
    width: height
    height: parent.height

    onClicked: applicationWindow().pageStack.goForward();

    Icon {
        anchors.centerIn: parent
        width: Math.min(parent.width, Units.iconSizes.smallMedium)
        height: width
        opacity: parent.enabled ? 1 : 0.6
        selected: header.background && header.background.color && header.background.color == Theme.highlightColor
        source: (LayoutMirroring.enabled ? "go-next-symbolic-rtl" : "go-next-symbolic")
    }
    Controls.ToolTip {
        visible: button.hovered
        text: qsTr("Navigate Forward")
        delay: 1000
        timeout: 5000
        y: button.height
    }
}
