import QtQuick

ListModel {
    property int workId: 1

    ListElement {
        binderTags: [1, 2, 3]
        createdAt: "2020-01-01T00:00:00"
        itemId: 1
        role: "text"
        subTitle: "subtitle 1"
        title: "example 1"
        updatedAt: "2020-01-01T00:00:00"
    }
}

