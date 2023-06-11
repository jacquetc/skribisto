pragma Singleton

import QtQuick

QtObject {


    function getCreateSceneDTO() {
        return {
            "title": "Scene 1"
        }
    }
    signal sceneCreated(var dto)
    function create(dto) {
        dto["id"] = 1
        sceneCreated(dto)
    }

    signal getReplied(var dto)
    function get(id) {
        getReplied(id)
    }

    signal getAllReplied(var dtos)
    function getAll() {
        getAllReplied()
    }

    signal sceneUpdated(var dto)
    function update(dto) {
        sceneUpdated(dto)
    }

        signal moveSceneReplied(var dto)
        function moveScene(dto) {
            moveSceneReplied(dto)
        }
    
}