import QtQuick

QtObject {
    property var binderItems: [1, 2, 3]
    property var binderTags: [1, 2, 3]
    property var contents: [1, 2]
    property date createdAt: new Date("2023-10-01T12:00:00Z")
    property int itemId: 1
    property int parentItem: 0
    property string role: "text"
    property string subTitle: "My subtitle"
    property var tags: ["1, 2, 3"]
    property string title: "My item"
    property date updatedAt: new Date("2023-10-01T12:00:00Z")
}
