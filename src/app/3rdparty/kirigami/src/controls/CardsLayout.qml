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

/**
 * A GridLayout optimized for showing one or two columns of cards,
 * depending on the available space.
 * It Should be used when the cards are not instantiated by a model or by a
 * model which has always very few items (In the case of a big model
 * CardsListView or CardsGridview should be used instead).
 * They are presented as a grid of two columns which will remain
 * centered if the application is really wide, or become a single
 * column if there is not enough space for two columns,
 * such as a mobile phone screen.
 * A CardsLayout should always be contained within a ColumnLayout.
 * @inherits GridLayout
 * @since 2.4
 */
GridLayout {
    /**
     * maximumColumnWidth: int
     * The maximum width the columns may have. the cards will never
     * get wider than this size, when the GridLayout is wider than
     * maximumColumnWidth, it will switch from one to two columns.
     * If the default needs to be overridden for some reason,
     * it is advised to express this unit as a multiple
     * of Kirigami.Units.gridUnit
     */
    property int maximumColumnWidth: Kirigami.Units.gridUnit * 20

    columns: width > maximumColumnWidth ? 2 : 1
    rowSpacing: Kirigami.Units.largeSpacing * 2
    columnSpacing: Kirigami.Units.largeSpacing * 2

    Layout.preferredWidth: maximumColumnWidth * 2 + (columns > 1 ? Kirigami.Units.largeSpacing * 2 : 0)
    Layout.maximumWidth: Layout.preferredWidth
    Layout.alignment: Qt.AlignHCenter

    Component.onCompleted: childrenChanged()
    onChildrenChanged: {
        for (var i = 0; i < children.length; ++i) {
            children[i].Layout.fillHeight = true;
        }
    }
}
