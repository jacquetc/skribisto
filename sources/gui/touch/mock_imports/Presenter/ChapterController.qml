pragma Singleton

import QtQuick

QtObject {


    function getCreateChapterDTO() {
        return {
            "title": "Chapter 1"
        }
    }
    signal chapterCreated(var dto)
    function create(dto) {
        dto["id"] = 1
        chapterCreated(dto)
    }

    signal getReplied(var dto)
    function get(id) {
        getReplied(id)
    }

    signal getAllReplied(var dtos)
    function getAll() {
        getAllReplied()
    }

    signal chapterUpdated(var dto)
    function update(dto) {
        chapterUpdated(dto)
    }

        signal moveChapterReplied(var dto)
        function moveChapter(dto) {
            moveChapterReplied(dto)
        }
    
}