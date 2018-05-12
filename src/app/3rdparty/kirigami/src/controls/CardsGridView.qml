/*
 *   Copyright 2018 Marco Martin <mart@kde.org>
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
import QtQuick.Controls 2.0 as Controls
import QtQuick.Layouts 1.2
import org.kde.kirigami 2.4 as Kirigami
import "private"

/**
 * CardsGridView is used to display a grid of Cards generated from any model.
 * The behavior is same as CardsLayout, and it allowes cards to be put in one or two
 * columns depending from the available width.
 * GridView has the limitation that every Card must have the same exact height,
 * so cellHeight must be manually set to a value in which the content fits
 * for every item.
 * If possible use cards only when you don't need to instantiate a lot
 * and use CardsLayout intead.
 * @inherits GridView
 * @see CardsLayout
 * @since 2.4
 */
CardsGridViewPrivate {
    id: root

    /**
     * maximumColumnWidth: int
     * The maximum width the columns may have. the cards will never
     * get wider than this size, when the GridView is wider than
     * maximumColumnWidth, it will switch from one to two columns.
     * If the default needs to be overridden for some reason,
     * it is advised to express this unit as a multiple
     * of Kirigami.Units.gridUnit
     */
    property int maximumColumnWidth: Kirigami.Units.gridUnit * 20
    cellWidth: width > maximumColumnWidth ? width/2 : width
    cellHeight: Math.max(Kirigami.Units.gridUnit * 15, Math.min(cellWidth, maximumColumnWidth) / 1.2)

    default property alias delegate: root._delegateComponent

    topMargin: Kirigami.Units.largeSpacing * 2
}
