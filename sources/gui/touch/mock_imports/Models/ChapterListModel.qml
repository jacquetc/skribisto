pragma Singleton

import QtQuick

ListModel {
    ListElement {
        title: "Chapter 1"
    }
    ListElement {
        title: "Chapter 2"
    }
    ListElement {
        title: "Chapter 3"
    }
    ListElement {
        title: "Chapter 4"
    }

    function create(createChapterDTO) {
        append(createChapterDTO.title)
    }
}
