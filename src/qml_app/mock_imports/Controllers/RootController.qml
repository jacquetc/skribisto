// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    id: controller

    signal brandRemoved(int id)

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
                EventDispatcher.brand().created(dto);
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
                EventDispatcher.brand().getReplied(id);
            });
        }

        return task;
    }
    function getAll() {
        // fill it with whatever you want to return
        var dtos = [];

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dtos);
            task.setDelay(50);
            task.setSignalFn(function () {
                EventDispatcher.brand().getAllReplied(dtos);
            });
        }

        return task;
    }
    function getCreateDto() {
        return {
            "createdAt": "",
            "updatedAt": "",
            "authorName": "",
            "works": "",
            "recentWorks": ""
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
                EventDispatcher.brand().removed(id);
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
                EventDispatcher.brand().updated(dto);
                EventDispatcher.brand().allRelationsInvalidated(dto.id);
            });
        }

        return task;
    }

    //TODO: add relation methods here
}

