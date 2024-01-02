import QtQuick
import Models
import Singles

SceneSynopsisPageForm {
    required property int chapterId
    sceneSynopsisListView.model: SceneListModelFromChapterScenes{chapterId: chapterId}

    Chapter{
        id: chapter
        itemId: chapterId

    }
    chapterTitleLabel.text: chapter.title

    sceneSynopsisListView.delegate:     Item {
        id: componentRoot
        width: ListView.view.width
        height: textEdit.height + 10
        onActiveFocusChanged: {
            if(activeFocus)
                textEdit.forceActiveFocus()
        }


    Rectangle{
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: textEdit.height + 10
        Column {
            anchors.fill: parent
            spacing: 4
            TextEdit {
                id: textEdit
                height: contentHeight
                width: componentRoot.width
                text: model.synopsis
                wrapMode: TextEdit.Wrap
                // has focus if it is the current item

                //if shift enter is pressed, pass to the next item
                Keys.onReturnPressed: function(event) {
                    if (event.modifiers & Qt.ShiftModifier) {
                        console.log("Shift Enter pressed")
                        componentRoot.ListView.view.currentIndex = model.index + 1
                        event.accepted = true

                    }
                    else{
                        event.accepted = false
                    }
                }
                Keys.onShortcutOverride:  function(event) {
                    event.accepted = (event.key ===  Qt.Key_Return && event.modifiers & Qt.ShiftModifier )
                }


            }

            Canvas {
                id: canvas
                visible: model.index < componentRoot.ListView.view.model.count - 1
                width: componentRoot.width - 20
                height: 1
                onPaint: {
                            var ctx = getContext("2d")
                            ctx.beginPath()
                            ctx.setLineDash([1, 4]) // Dot size, space size
                            ctx.moveTo(0, height / 2)
                            ctx.lineTo(width, height / 2)
                            ctx.stroke()
                        }
            }
        }
    }
    }
}
