import QtQuick 2.15
import QtQuick.Controls 2.15
import Presenter
import Models

MainScreenForm {

    listView.model: ChapterListModel

    Connections {
        target: ChapterController
        function onChapterCreated(dto) {
            var chapterDto = dto
            console.log("dto", chapterDto)
            console.log("dto.title", chapterDto.title)
            button.text = chapterDto.title
        }
    }

    Connections {
        target: button
        onClicked: {
            var dto = ChapterController.getCreateChapterDTO()
            dto.title = "test chapter"

            ChapterController.create(dto)
        }
    }
}
