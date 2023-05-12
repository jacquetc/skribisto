import QtQuick 2.15
import QtQuick.Controls 2.15
import Presenter
import Contracts.DTO.Chapter

MainScreenForm {


    Connections {
        target: ChapterController
        onChapterCreated: {
            button.text = dto.title
            chapterListModel.append({
                                                      "name": "ee"
                                                  })
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
            dto.comment = "test comment"
            ChapterController.create(dto)

        }
    }
}
