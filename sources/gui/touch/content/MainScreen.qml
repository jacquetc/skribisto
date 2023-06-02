import QtQuick 2.15
import QtQuick.Controls 2.15
import Presenter
import Contracts.DTO.Chapter
import Models

MainScreenForm {

    listView.model: ChapterListModel

    Connections {
        target: ChapterController
        function onChapterCreatedPointer(dto) {
            var chapterDto = dto
            console.log("dto", chapterDto)
            console.log("dto.title", chapterDto.title)
            button.text = chapterDto.title
        }
    }

    CreateChapterDTO {
        id: createChapterDTO
    }

    Connections {
        target: button
        onClicked: {
            var dto = createChapterDTO
            dto.title = "test chapter"

            ChapterController.create(dto)
        }
    }
}
