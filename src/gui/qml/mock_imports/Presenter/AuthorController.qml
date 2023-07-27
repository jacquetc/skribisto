pragma Singleton

import QtQuick

QtObject {


    function getCreateAuthorDTO() {
        return {
            "title": "Author 1"
        }
    }
    signal authorCreated(var dto)
    function create(dto) {
        dto["id"] = 1
        authorCreated(dto)
    }

    signal getReplied(var dto)
    function get(id) {
        getReplied(id)
    }

    signal getAllReplied(var dtos)
    function getAll() {
        getAllReplied()
    }

    signal authorUpdated(var dto)
    function update(dto) {
        authorUpdated(dto)
    }

}