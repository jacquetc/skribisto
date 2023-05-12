// Copyright (C) 2018 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR GPL-3.0

import QtQuick 6.4
import QtQuick.Studio.EventSimulator 1.0
import QtQuick.Studio.EventSystem 1.0

QtObject {
    id: simulator
    property bool active: true

    property Timer __timer: Timer {
        id: timer
        interval: 100
        onTriggered: {
            EventSimulator.show()
        }
    }

    Component.onCompleted: {
        EventSystem.init(Qt.resolvedUrl("EventListModel.qml"))
        if (simulator.active)
            timer.start()
    }
}
