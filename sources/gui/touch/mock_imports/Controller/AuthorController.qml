pragma Singleton

import QtQuick

QtObject {


    function getCreateAuthorDTO() {
        return {
            "title": "Author 1"
        }
    }
    function getUpdateDTO() {
        return {
            "id": 0,
            "name": ""
        }
    }
    function get(id) {
        EventDispatcher.author().getReplied(id)
    }

    function getWithDetails(id) {
        EventDispatcher.author().getWithDetailsReplied(id)
    }

    function create(dto) {
        dto["id"] = 1
        EventDispatcher.author().created(dto)
    }
    
    function update(dto) {
        EventDispatcher.author().updated(dto)
        EventDispatcher.author().detailsUpdated(dto)
    }

    function remove(id) {
        EventDispatcher.author().removed(id)
    }

}
