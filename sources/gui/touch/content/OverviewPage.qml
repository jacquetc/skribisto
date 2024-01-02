import QtQuick 
import QtQuick.Controls 
import Controller
import Models
import Writing

OverviewPageForm {
    clip: true
    BookChaptersListModel { id: bookChaptersListModel}
    listView.model: bookChaptersListModel

//    Connections {
//        target: EventDispatcher.chapter()
//        function onCreated(dto) {
//            var chapterDto = dto
//            console.log("dto", chapterDto)
//            console.log("dto.title", chapterDto.title)
//            button.text = chapterDto.title
//        }
//    }

//    Connections {
//        target: button
//        onClicked: {
//            var dto = ChapterController.getCreateChapterDTO()
//            dto.title = "test chapter"

//            ChapterController.create(dto)
//        }
//    }

//    DocumentHandler {
//        quickTextDocument: textArea.textDocument
//        uuid: 1
//    }

//    DocumentHandler {
//        quickTextDocument: textAreaBis.textDocument
//        uuid: 1
//    }

//    DocumentHandler {
//        quickTextDocument: textAreaOther.textDocument
//        uuid: 2
//    }
}
