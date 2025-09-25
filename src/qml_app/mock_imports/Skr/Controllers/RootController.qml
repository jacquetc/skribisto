// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

import QtQuick

QtObject {
    id: controller

    function create(dtos) {
        for (var i = 0; i < dtos.length; i++) {
            const dto = dtos[i];
            // create random id
            dto["id"] = Math.floor(Math.random() * 1000000);
        }

        // mocking QCoro::Task
        let task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(dtos);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.root().created(dtos);
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
            dto["authorName"] = "Author " + id;
            dto["works"] = [1];
            dto["recentWorks"] = [1];
            dtos.push(dto);
        }
        // mocking QCoro::Task
        let task;
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
            "authorName": "",
            "works": [],
            "recentWorks": []
        };
    }
    function remove(EventRegistry) {
        // mocking QCoro::Task
        let task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(ids);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.root().removed(id);
            });
        }

        return task;
    }
    function update(dtos) {

        // mocking QCoro::Task
        let task;
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            task = component.createObject(controller);
            task.setValue(dtos);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.root().updated(dtos);
            });
        }

        return task;
    }

    //TODO: add relation methods here
}

