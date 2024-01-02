

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    property int chapterId: 0
    property string chapterTitle: ""
    property string chapterSynopsis: ""

    id: root
    height: 200 + chapterTextEdit.contentHeight
    x: 5
    width: ListView.view.width - 10
    //color: "#707070"
    radius: 5
    clip: true
    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: 3
        anchors.topMargin: 3
        anchors.bottomMargin: 3
        spacing: 4
        ColumnLayout {
            id: columnLayout
            Layout.fillHeight: true
            Layout.fillWidth: true

            Text {
                Layout.fillWidth: true
                text: chapterTitle
                font.bold: true
                styleColor: "#000000"
            }
            Flickable {
                id: chapterTextEditFlick
                Layout.fillHeight: true
                Layout.fillWidth: true
                contentWidth: chapterTextEdit.contentWidth
                boundsBehavior: Flickable.StopAtBounds
                contentHeight: chapterTextEdit.contentHeight
                TextEdit {
                    id: chapterTextEdit
                    readOnly: true
                    cursorVisible: false
                    selectByKeyboard: false
                    selectByMouse: true
                    wrapMode: TextEdit.Wrap
                    width: chapterTextEditFlick.width

                    text: chapterSynopsis

                    TapHandler {
                        gesturePolicy: TapHandler.ReleaseWithinBounds
                        onTapped: {
                            // Trigger the animation when clicked
                            overlayTextEdit.text = originalTextEdit.text
                            overlayAnimation.start()
                        }
                    }
                }
            }
        }

        SceneList {
            id: sceneList
            chapterId: chapterId
            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
            Layout.fillHeight: true
            Layout.minimumWidth: 100
        }
    }

    // Overlay TextEdit for animation
    Flickable {
        id: overlayFlick
        width: overlayFlick.width
        height: overlayFlick.height
        x: chapterTextEdit.x
        y: chapterTextEdit.y
        boundsBehavior: Flickable.StopAtBounds

        contentWidth: overlayTextEdit.contentWidth
        contentHeight: overlayTextEdit.contentHeight
        clip: true

        function ensureVisible(r) {
            if (contentX >= r.x)
                contentX = r.x
            else if (contentX + width <= r.x + r.width)
                contentX = r.x + r.width - width
            if (contentY >= r.y)
                contentY = r.y
            else if (contentY + height <= r.y + r.height)
                contentY = r.y + r.height - height
        }
        TextEdit {
            id: overlayTextEdit
            visible: false
            z: 1 // To ensure it's on top
            readOnly: true
            onCursorRectangleChanged: overlayFlick.ensureVisible(cursorRectangle)
        }
    }
    // Animation to expand overlay TextEdit
    NumberAnimation {
        id: overlayAnimation
        target: overlayTextEdit
        properties: "x, y, width, height"
        to: "0, 0, " + parent.width + ", " + parent.height
        duration: 500
        easing.type: Easing.InOutQuad

        onStarted: {
            overlayTextEdit.visible = true
        }

        onStopped: {
            overlayTextEdit.visible = false
            // Reset the overlay TextEdit to original size and position
            overlayTextEdit.width = originalTextEdit.width
            overlayTextEdit.height = originalTextEdit.height
            overlayTextEdit.x = originalTextEdit.x
            overlayTextEdit.y = originalTextEdit.y
        }
    }
}
