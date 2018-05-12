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

import QtQuick 2.7
import QtQuick.Controls.Material 2.0
import org.kde.kirigami 2.4 as Kirigami

pragma Singleton


QtObject {
    id: theme
    //NOTE: this is useless per se, but it forces the Material attached property to be created
    Material.elevation:2

    property color textColor: theme.Material.foreground
    onTextColorChanged: theme.Material.foreground = textColor
    property color disabledTextColor: "#9931363b"

    property color highlightColor: theme.Material.accent
    onHighlightColorChanged: theme.Material.accent = highlightColor
    //FIXME: something better?
    property color highlightedTextColor: theme.Material.background
    property color backgroundColor: theme.Material.background
    property color activeTextColor: theme.Material.primary
    property color linkColor: "#2980B9"
    property color visitedLinkColor: "#7F8C8D"
    property color hoverColor: theme.Material.highlightedButtonColor
    property color focusColor: theme.Material.highlightedButtonColor
    property color negativeTextColor: "#DA4453"
    property color neutralTextColor: "#F67400"
    property color positiveTextColor: "#27AE60"

    property color buttonTextColor: theme.Material.foreground
    property color buttonBackgroundColor: theme.Material.buttonColor
    property color buttonHoverColor: theme.Material.highlightedButtonColor
    property color buttonFocusColor: theme.Material.highlightedButtonColor

    property color viewTextColor: theme.Material.foreground
    property color viewBackgroundColor: theme.Material.dialogColor
    property color viewHoverColor: theme.Material.listHighlightColor
    property color viewFocusColor: theme.Material.listHighlightColor

    property color selectionTextColor: theme.Material.primaryHighlightedTextColor
    property color selectionBackgroundColor: theme.Material.textSelectionColor
    property color selectionHoverColor: theme.Material.highlightedButtonColor
    property color selectionFocusColor: theme.Material.highlightedButtonColor

    property color tooltipTextColor: fontMetrics.Material.foreground
    property color tooltipBackgroundColor: fontMetrics.Material.tooltipColor
    property color tooltipHoverColor: fontMetrics.Material.highlightedButtonColor
    property color tooltipFocusColor: fontMetrics.Material.highlightedButtonColor

    property color complementaryTextColor: fontMetrics.Material.foreground
    property color complementaryBackgroundColor: fontMetrics.Material.background
    property color complementaryHoverColor: theme.Material.highlightedButtonColor
    property color complementaryFocusColor: theme.Material.highlightedButtonColor

    property font defaultFont: fontMetrics.font

    property list<QtObject> children: [
        TextMetrics {
            id: fontMetrics
            //this is to get a source of dark colors
            Material.theme: Material.Dark
        }
    ]
    //for internal use
    function __propagateColorSet(object, context) {
        //TODO: actually check if it's a dark or light color
        if (context == Kirigami.Theme.Complementary) {
            object.Material.theme = Material.Dark;
        } else {
            object.Material.theme = Material.Light;
        }
    }
}
