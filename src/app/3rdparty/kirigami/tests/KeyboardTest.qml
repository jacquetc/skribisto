/*
 *   Copyright 2016 Aleix Pol Gonzalez <aleixpol@kde.org>
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
import QtQuick.Controls 2.0
import org.kde.kirigami 2.4 as Kirigami

Kirigami.ApplicationWindow
{
    id: main
    Component {
        id: keyPage
        Kirigami.Page {
            readonly property alias lastKey: see.text
            Text {
                id: see
                anchors.centerIn: parent
                color: parent.focus ? "black" : "red"
            }

            Keys.onPressed: {
                if (event.text)
                    see.text = event.text
                else
                    see.text = event.key
            }

            Keys.onEnterPressed: main.showPassiveNotification("page!")
        }
    }

    header: Text {
        text: "focus:" + activeFocusItem + " current: " + main.pageStack.currentIndex
    }

    Component.onCompleted: {
        main.pageStack.push(keyPage)
        main.pageStack.push(keyPage)
    }
}
