pragma Singleton

import QtQuick

QtObject {


    function getCreateSceneDTO() {
        return {
            "title": ""
        }
    }
    function getUpdateDTO() {
        return {
            "id": 0,
            "name": ""
        }
    }
    function get(id) {
        EventDispatcher.scene().getReplied(id)
    }

    function getWithDetails(id) {
        EventDispatcher.scene().getWithDetailsReplied(id)
    }

    function create(dto) {
        dto["id"] = 1
        EventDispatcher.scene().created(dto)
    }
    
    function update(dto) {
        EventDispatcher.scene().updated(dto)
        EventDispatcher.scene().detailsUpdated(dto)
    }

    function remove(id) {
        EventDispatcher.scene().removed(id)
    }

}
