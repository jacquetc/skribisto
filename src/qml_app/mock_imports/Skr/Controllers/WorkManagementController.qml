/*
 * Copyright (C) 2025 by Cyril Jacquet
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

QtObject {
    id: controller

    function loadWork(loadWorkDto) {


        // mocking QCoro::Task
        let task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(loadDto);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.root().created(loadDto);
            });
        }

        return task;

    }

    function getLoadWorkDto() {
        return {
            "id": 1,
            "title": "My work",
            "authorName": "Author 1",
            "createdAt": "2023-10-01T12:00:00Z",
            "updatedAt": "2023-10-01T12:00:00Z",
            "description": "This is a description of my work.",
            "items": [1, 2, 3],
            "recentItems": [1, 2],
            "tags": [1, 2],
            "binderItems": [1, 2, 3],
            "binderTags": [1, 2],
            "coverContentId": 1
        };
    }

    function saveWork(saveWorkDto) {
        // mocking QCoro::Task
        let task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(saveWorkDto);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.root().updated(saveWorkDto);
            });
        }

        return task;
    }

    function getSaveWorkDto() {

    }

}