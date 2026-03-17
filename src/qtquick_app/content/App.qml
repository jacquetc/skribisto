/*
 * Copyright (C) 2026 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QtQml
import QtQuick.Controls.Basic
import Skr
import Skr.Desktop
import Skr.Mobile
import Skr.Controllers
import Skr.Singles

ApplicationWindow {
    id: applicationWindow

    readonly property bool isMobileMode: {
        if (Qt.platform.os === "linux" && Screen.count > 1)
            return false;
        return width < 800 || Qt.platform.os === "android";
    }

    property int undoRedoStackId: 1

    height: 600
    title: "Skribisto"
    visible: true
    width: 1200

    // --- UI loader: Desktop or Mobile ---
    Loader {
        id: uiLoader

        anchors.fill: parent
        asynchronous: false
        sourceComponent: applicationWindow.isMobileMode ? mobileComponent : desktopComponent

        Behavior on opacity {
            NumberAnimation { duration: 150 }
        }

        onLoaded: opacity = 1
        onSourceComponentChanged: opacity = 0
    }

    Component {
        id: desktopComponent

        MainWindow {}
    }

    Component {
        id: mobileComponent

        MainView {}
    }

}
