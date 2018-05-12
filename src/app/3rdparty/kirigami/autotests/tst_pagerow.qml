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

TestCase {
    id: testCase
//     when: mainWindow.visible
    width: 400
    height: 400
//     visible: true
    when: mainWindow.visible
    name: "GoBack"

    function applicationWindow() { return mainWindow; }

    Kirigami.ApplicationWindow {
        id: mainWindow
        width: 480
        height: 360
        visible: true
        pageStack.initialPage: Kirigami.Page {
            Rectangle {
                anchors.fill: parent
                color: "green"
            }
        }
    }

    Component {
        id: randomPage
        Kirigami.Page {
            Rectangle {
                anchors.fill: parent
                color: "red"
            }
        }
    }

    SignalSpy {
        id: spyCurrentIndex
        target: mainWindow.pageStack
        signalName: "currentIndexChanged"
    }

    SignalSpy {
        id: spyActive
        target: mainWindow
        signalName: "activeChanged"
    }

    function init() {
        mainWindow.pageStack.clear()
        spyActive.clear()
        spyCurrentIndex.clear()
    }

    function test_pop() {
        compare(mainWindow.pageStack.depth, 0)
        mainWindow.pageStack.push(randomPage)
        compare(mainWindow.pageStack.depth, 1)
        mainWindow.pageStack.pop()
        compare(mainWindow.pageStack.depth, 0)
    }

    function test_goBack() {
        compare(mainWindow.pageStack.depth, 0)
        mainWindow.pageStack.push(randomPage)
        mainWindow.pageStack.push(randomPage)
        compare(mainWindow.pageStack.depth, 2)
        compare(mainWindow.pageStack.currentIndex, 1)
        compare(spyCurrentIndex.count, 2)
        spyActive.clear()
        mainWindow.requestActivate()
        spyCurrentIndex.clear()
        if (!mainWindow.active)
            spyActive.wait()
        verify(mainWindow.active)
        keyClick(Qt.Key_Left, Qt.AltModifier)

        spyCurrentIndex.wait()

        compare(mainWindow.pageStack.depth, 2)
        compare(mainWindow.pageStack.currentIndex, 0)
        compare(spyCurrentIndex.count, 1)
        mainWindow.pageStack.pop()
        compare(mainWindow.pageStack.depth, 1)
    }

    property int destructions: 0
    Component {
        id: destroyedPage
        Kirigami.Page {
            id: page
            Rectangle {
                anchors.fill: parent
                color: "blue"
                Component.onDestruction: {
                    console.log("ko", page)
                    testCase.destructions++
                }
            }
        }
    }
    SignalSpy {
        id: spyDestructions
        target: testCase
        signalName: "destructionsChanged"
    }
    function test_clearPages() {
        mainWindow.pageStack.push(destroyedPage)
        mainWindow.pageStack.push(destroyedPage)
        mainWindow.pageStack.push(destroyedPage)
        compare(mainWindow.pageStack.depth, 3)
        mainWindow.pageStack.clear()

        compare(mainWindow.pageStack.depth, 0)
        console.log(spyDestructions.wait())
        compare(testCase.destructions, 3)
    }
}
