// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

import QtQuick

QtObject {
    id: controller

    function create(dto) {
        // create random id
        var newId = Math.floor(Math.random() * 1000000);
        dto["id"] = newId;

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.work().created(dto);
            });
        }

        return task;
    }
    function get(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.work().getReplied(id);
            });
        }

        return task;
    }
    function getCreateDto() {
        return {
            "createdAt": "",
            "updatedAt": "",
            "name": "",
            "binders": []
        };
    }
    function remove(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.work().removed(id);
            });
        }

        return task;
    }
    function update(dto) {

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventRegistry.work().updated(dto);
                EventRegistry.work().allRelationsInvalidated(dto.id);
            });
        }

        return task;
    }

    //TODO: add relation methods here
}

