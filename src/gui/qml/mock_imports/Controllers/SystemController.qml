// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    id: controller


    function createNewAtelier(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.system().createNewAtelierChanged(reply_dto);
            })
        }

        return task
    }


    function loadAtelier(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.system().loadAtelierChanged(reply_dto);
            })
        }

        return task
    }


    function saveAtelier(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.system().saveAtelierChanged(reply_dto);
            })
        }

        return task
    }


    function closeAtelier(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.system().closeAtelierChanged(reply_dto);
            })
        }

        return task
    }


    function getCurrentTime(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.system().getCurrentTimeReplied(reply_dto);
            })
        }

        return task
    }


}