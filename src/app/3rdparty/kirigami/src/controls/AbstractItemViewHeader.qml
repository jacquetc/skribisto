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

import QtQuick 2.5
import QtQuick.Templates 2.0 as T2
import org.kde.kirigami 2.4 as Kirigami

/**
 * An item that can be used as an header for a ListView.
 * It will play nice with the margin policies of ScrollablePage and can
 * automatically shrink when the list is scrolled, like the behavior
 * of list headers in many mobile applications.
 * @since 2.1
 * @inherit QtQuick.Controls.Control
 */
T2.Control {
    property int minimumHeight: Kirigami.Units.gridUnit * 2 + Kirigami.Units.smallSpacing * 2
    property int maximumHeight: Kirigami.Units.gridUnit * 6

    property ListView view: ListView.view

    width: view.width

    implicitHeight: topPadding + bottomPadding + (view.headerPositioning == ListView.InlineHeader
                                                    ? maximumHeight
                                                    : Math.min(maximumHeight, Math.max(minimumHeight, maximumHeight - Math.max(0, view.contentY))))


    z: 9
    topPadding: applicationWindow() && !applicationWindow().wideScreen && applicationWindow().header ? applicationWindow().header.paintedHeight : 0
    rightPadding: Kirigami.Units.gridUnit

}
