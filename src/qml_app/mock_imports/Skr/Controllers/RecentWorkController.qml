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

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

import QtQuick

QtObject {
    id: controller

    function create(dtos) {
        for (var i = 0; i < dtos.length; i++) {
            const dto = dtos[i];
            dto["id"] = Math.floor(Math.random() * 1000000);
        }

        var task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(dtos);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.recentWork().created(dtos);
            });
        }

        return task;
    }
    function get(ids) {
        let dtos = [];
        for (var i = 0; i < ids.length; i++) {
            const id = ids[i];
            let dto = {};
            dto["id"] = id;
            dto["createdAt"] = "2023-10-01T12:00:00Z";
            dto["updatedAt"] = "2023-10-01T12:00:00Z";
            dto["title"] = "Recent " + id;
            dto["lastOpenedAt"] = "2023-10-02T12:00:00Z";
            dto["absolutePath"] = "/path/to/work_" + id;
            dtos.push(dto);
        }
        var task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(dtos);
            task.setDelay(50);
        }
        return task;
    }
    function getCreateDto() {
        return {
            "createdAt": "",
            "updatedAt": "",
            "title": "",
            "lastOpenedAt": "",
            "absolutePath": ""
        };
    }
    function remove(ids) {
        var task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(ids);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.recentWork().removed(ids);
            });
        }
        return task;
    }
    function update(dtos) {
        var task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(dtos);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.recentWork().updated(dtos);
            });
        }
        return task;
    }
}