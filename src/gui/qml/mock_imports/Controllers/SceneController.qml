// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    id: controller


    function get(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){EventDispatcher.scene().getReplied(id)})
        }

        return task
    }

    function getWithDetails(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){EventDispatcher.scene().getWithDetailsReplied(id)})
        }

        return task
    }

    function getCreateDTO() {
        return {
            "content": "Scene 1"
        }
    }

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
            task.setSignalFn(function(){EventDispatcher.scene().created(dto)})
        }

        return task
    }

    function getUpdateDTO() {
        return {
            "id": 0,
            "content": ""
        }
    }

    function update(dto) {

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.scene().updated(dto);
                EventDispatcher.scene().allRelationsInvalidated(dto.id);
            })
        }

        return task
    }

    signal sceneRemoved(int id)
    function remove(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.scene().removed(id);
            })
        }

        return task
    }


}