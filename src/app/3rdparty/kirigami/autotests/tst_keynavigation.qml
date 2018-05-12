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
import QtQuick.Window 2.1
import org.kde.kirigami 2.4 as Kirigami
import QtTest 1.0
import "../tests"

TestCase {
    id: testCase
    width: 400
    height: 400
    when: mainWindow.visible
    name: "KeyboardNavigation"

    KeyboardTest {
        id: mainWindow
        width: 480
        height: 360
        visible: true
    }

    SignalSpy {
        id: spyActive
        target: mainWindow
        signalName: "activeChanged"
    }
    SignalSpy {
        id: spyLastKey
        target: mainWindow.pageStack.currentItem
        signalName: "lastKeyChanged"
    }

    function test_press() {
        compare(mainWindow.pageStack.depth, 2)
        compare(mainWindow.pageStack.currentIndex, 1)
        if (!mainWindow.active)
            spyActive.wait(5000)
        verify(mainWindow.active)
        keyClick("A")
        spyLastKey.wait()
        compare(mainWindow.pageStack.currentItem.lastKey, "A")
        keyClick(Qt.Key_Left, Qt.AltModifier)
        compare(mainWindow.pageStack.currentIndex, 0)
        compare(mainWindow.pageStack.currentItem.lastKey, "")
        keyClick("B")
        spyLastKey.wait()
        compare(mainWindow.pageStack.currentItem.lastKey, "B")
    }
}
