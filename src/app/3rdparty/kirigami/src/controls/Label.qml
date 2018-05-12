/*
*   Copyright (C) 2011 by Marco Martin <mart@kde.org>
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

import QtQuick 2.1
import QtQuick.Window 2.2
import org.kde.kirigami 2.4
import QtQuick.Controls 2.0 as Controls

/**
 * This is a label which uses the current Theme.
 *
 * The characteristics of the text will be automatically set according to the
 * current Theme. If you need a more customized text item use the Text component
 * from QtQuick.
 *
 * You can use all elements of the QML Text component, in particular the "text"
 * property to define the label text.
 *
 * @inherit QtQuick.Templates.Label
 * @deprecated use QtQuick.Templates.Label directly, it will be styled appropriately
 */
Controls.Label {
    height: Math.round(Math.max(paintedHeight, Units.gridUnit * 1.6))
    verticalAlignment: lineCount > 1 ? Text.AlignTop : Text.AlignVCenter

    activeFocusOnTab: false

    Component.onCompleted: {
        console.warn("Kirigami.Label is deprecated. Use QtQuickControls2.Label instead")
    }
}
