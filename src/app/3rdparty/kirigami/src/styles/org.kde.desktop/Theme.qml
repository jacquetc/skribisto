/*
 *   Copyright 2015 Marco Martin <mart@kde.org>
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

import QtQuick 2.4

pragma Singleton


QtObject {
    id: theme

    property color textColor: palette.windowText
    property color disabledTextColor: disabledPalette.windowText

    property color highlightColor: palette.highlight
    property color highlightedTextColor: palette.highlightedText
    property color backgroundColor: palette.window
    property color activeTextColor: palette.highlight
    property color linkColor: "#2980B9"
    property color visitedLinkColor: "#7F8C8D"
    property color hoverColor: palette.highlight
    property color focusColor: palette.highlight
    property color negativeTextColor: "#DA4453"
    property color neutralTextColor: "#F67400"
    property color positiveTextColor: "#27AE60"

    property color buttonTextColor: palette.buttonText
    property color buttonBackgroundColor: palette.button
    property color buttonHoverColor: palette.highlight
    property color buttonFocusColor: palette.highlight

    property color viewTextColor: palette.text
    property color viewBackgroundColor: palette.base
    property color viewHoverColor: palette.highlight
    property color viewFocusColor: palette.highlight

    property color selectionTextColor: palette.highlightedText
    property color selectionBackgroundColor: palette.highlight
    property color selectionHoverColor: palette.highlight
    property color selectionFocusColor: palette.highlight

    property color tooltipTextColor: palette.base
    property color tooltipBackgroundColor: palette.text
    property color tooltipHoverColor: palette.highlight
    property color tooltipFocusColor: palette.highlight

    property color complementaryTextColor: palette.base
    property color complementaryBackgroundColor: palette.text
    property color complementaryHoverColor: palette.highlight
    property color complementaryFocusColor: palette.highlight

    property font defaultFont: fontMetrics.font

    property list<QtObject> children: [
        TextMetrics {
            id: fontMetrics
        },
        SystemPalette {
            id: palette
            colorGroup: SystemPalette.Active
        },
        SystemPalette {
            id: disabledPalette
            colorGroup: SystemPalette.Disabled
        }
    ]

    function __propagateColorSet(object, context) {}
}
