/*
 *   Copyright 2016 Aleix Pol Gonzalez <aleixpol@kde.org>
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

import QtQuick 2.7
import QtQuick.Controls 2.3 as QQC2
import org.kde.kirigami 2.4 as Kirigami

Kirigami.ApplicationWindow
{
    id: main

    header: Kirigami.ToolBarApplicationHeader {}

    pageStack.initialPage: Kirigami.Page {
        QQC2.Button {
            text: "button"
            onClicked: menu.popup()
            QQC2.Menu {
                id: menu

                QQC2.MenuItem { text: "xxx" }
                QQC2.MenuItem { text: "xxx" }
                QQC2.Menu {
                    title: "yyy"
                    QQC2.MenuItem { text: "yyy" }
                    QQC2.MenuItem { text: "yyy" }
                }
            }
        }

        title: "aaaaaaaaaaaaaaa aaaaaaaaaaaaaaaaa aaaaaaaaaaaaaaaaa"
        actions {
            main:  Kirigami.Action { icon.name: "kate"; text: "BonDia" }
            left : Kirigami.Action { icon.name: "kate"; text: "BonDia" }
            right: Kirigami.Action { icon.name: "kate"; text: "BonDia" }
        }

        QQC2.ActionGroup {
            id: group
        }

        contextualActions: [
            Kirigami.Action {
                text: "submenus"
                icon.name: "kalgebra"

                Kirigami.Action { text: "xxx"; onTriggered: console.log("xxx") }
                Kirigami.Action { text: "xxx"; onTriggered: console.log("xxx") }
                Kirigami.Action { text: "xxx"; onTriggered: console.log("xxx") }
                Kirigami.Action {
                    text: "yyy"
                    Kirigami.Action { text: "yyy" }
                    Kirigami.Action { text: "yyy" }
                    Kirigami.Action { text: "yyy" }
                    Kirigami.Action { text: "yyy" }
                }
            },
            Kirigami.Action {
                id: optionsAction
                text: "Options"
                icon.name: "kate"

                Kirigami.Action {
                    QQC2.ActionGroup.group: group
                    text: "A"
                    checkable: true
                    checked: true
                }
                Kirigami.Action {
                    QQC2.ActionGroup.group: group
                    text: "B"
                    checkable: true
                }
                Kirigami.Action {
                    QQC2.ActionGroup.group: group
                    text: "C"
                    checkable: true
                }
            },
            Kirigami.Action { text: "stuffing..." },
            Kirigami.Action { text: "stuffing..." },
            Kirigami.Action { text: "stuffing..." },
            Kirigami.Action { text: "stuffing..." },
            Kirigami.Action { text: "stuffing..." }
        ]
    }
}
