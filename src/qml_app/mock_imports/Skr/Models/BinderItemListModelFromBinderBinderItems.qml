import QtQuick

ListModel {
    property int binderId: 1
    property int parentId: 1

    ListElement {
        binderTags: [1, 2, 3]
        createdAt: "2020-01-01T00:00:00"
        itemId: 1
        role: "text"
        subTitle: "subtitle 1"
        title: "example 1"
        updatedAt: "2020-01-01T00:00:00"
    }
    ListElement {
        binderTags: [1, 2, 3]
        createdAt: "2020-01-01T00:00:00"
        itemId: 2
        role: "text"
        subTitle: "subtitle 2"
        title: "example 2"
        updatedAt: "2020-01-01T00:00:00"
    }
}

