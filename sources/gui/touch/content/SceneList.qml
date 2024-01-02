import QtQuick
import QtQuick.Controls
import Models

Item {
    id: root

    required property int chapterId

    signal chapterCalled(int chapterId)

    SceneListModelFromChapterScenes{
        id: chapterScenesListModel
        chapterId: root.chapterId
    }

    ListView
    {
        anchors.fill: parent
        boundsBehavior: Flickable.StopAtBounds
        model: chapterScenesListModel
        clip: true
        delegate: ItemDelegate {
                        text: title
                        width: ListView.width

                       onDoubleClicked: {
                           renameSceneLoader.visible = true

                       }
                       onClicked: {
                           root.chapterCalled(model.id)
                       }

                       Loader {
                           id: renameSceneLoader
                           anchors.fill: parent
                           sourceComponent: renameSceneComponent
                           visible: false
                       }

                }


    }

    Component {
        id: renameSceneComponent
        //rename popup:
        Rectangle {
            id: renameSceneRectangle
            width: 400
            height: 200
            color: "lightgrey"
            border.color: "black"
            border.width: 2
            radius: 10
            anchors.centerIn: parent
            visible: false
            Text {
                id: renameSceneText
                text: "Rename Scene"
                anchors.top: parent.top
                anchors.horizontalCenter: parent.horizontalCenter
                font.pixelSize: 20
            }
            TextField {
                id: renameSceneTextField
                anchors.top: renameSceneText.bottom
                anchors.horizontalCenter: parent.horizontalCenter
                width: 200
                placeholderText: "Enter new scene name"
            }
            Button {
                id: renameSceneButton
                anchors.top: renameSceneTextField.bottom
                anchors.horizontalCenter: parent.horizontalCenter
                text: "Rename"
                onClicked: {
                    renameSceneLoader.visible = false
                    renameSceneRectangle.visible = false
                    renameSceneTextField.text = ""
                }
            }
        }


    }



}
