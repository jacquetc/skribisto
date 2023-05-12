pragma Singleton
import QtQuick
import Contracts.DTO.Chapter


QtObject {

    function create(chapterDTO){



        chapterCreated(chapterDTO)
    }

    signal chapterCreated(var dto)
}
