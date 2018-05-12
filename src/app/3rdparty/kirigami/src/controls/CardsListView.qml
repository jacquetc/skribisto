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
 * CardsListView is a ListView which can have AbstractCard as its delegete: it will
 * automatically assign the proper spacings and margins around the cards adhering
 * to the design guidelines.
 * CardsListView should be used only with cards which can look good at any
 * horizontal size, so It is recommended to use directly AbstractCard with an 
 * appropriate layout inside, because they are stretching for the whole list width.
 * Therefore is discouraged to use it with the Card type, unless it has
 * Horizontal as headerOrientation.
 * The choice between using this view with AbstractCard or a normal ListView
 * with AbstractListItem/BasicListItem is purely a choice based on aestetics alone.
 * It is discouraged to tweak the properties of this ListView.
 * @inherits ListView
 * @since 2.4
 */
CardsListViewPrivate {
    id: root
    spacing: Kirigami.Units.largeSpacing * 2
    topMargin: headerPositioning != ListView.InlineHeader ? spacing : 0

    property alias delegate: root._delegateComponent
    headerPositioning: ListView.OverlayHeader

    onContentHeightChanged: {
        var item = contentItem.children[0];
        if (item && !item.hasOwnProperty("header") && !item.hasOwnProperty("_contentItem")) {
            print("Warning: only AbstractCard items are supported in CardsListView")
        }
    }
}
