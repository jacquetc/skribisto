import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

TextArea {
    id: root
    property bool styleElevation: false
    property string styleBackgroundColor: "#FFFFFF"
    property string styleForegroundColor: "#000000"
    property string styleAccentColor: "#000000"
    objectName: "SKRTextArea-" + Qt.formatDateTime(new Date(), "yyyyMMddhhmmsszzz")
    //color: styleForegroundColor

    //    Keys.priority: Keys.BeforeItem

    //    Keys.onReleased: {
    //        if (event.key === Qt.Key_Alt){
    //            console.log("alt not accepted")
    //            Globals.showMenuBarCalled()
    //            event.accepted = true
    //        }

    //    }
    Material.accent: styleAccentColor
    Material.foreground: styleForegroundColor

    background:
        Pane {
            Material.background: styleBackgroundColor
            Material.elevation: styleElevation ? 6 : 0
        }



    property int viewHeight: 200
    signal moveViewY(int height)

    Keys.priority: Keys.BeforeItem

    property int initialCursorPositionX: -1
    property int initialCursorPosition: -1
    Keys.onPressed: {
        if((event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_PageUp){

            if(initialCursorPositionX === -1){
                initialCursorPositionX = root.cursorRectangle.x
            }
            if(initialCursorPosition === -1){
                initialCursorPosition = root.cursorPosition
            }

            var newPosition = root.positionAt(initialCursorPositionX, root.cursorRectangle.y - viewHeight)
            moveViewY(-viewHeight)
            root.cursorPosition = newPosition

            root.select(initialCursorPosition, root.cursorPosition)

            event.accepted = true
            return
        }
        if((event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_PageDown){

            if(initialCursorPositionX === -1){
                initialCursorPositionX = root.cursorRectangle.x
            }
            if(initialCursorPosition === -1){
                initialCursorPosition = root.cursorPosition
            }

            var newPosition = root.positionAt(initialCursorPositionX, root.cursorRectangle.y + viewHeight)
            moveViewY(viewHeight)
            root.cursorPosition = newPosition

            root.select(initialCursorPosition, root.cursorPosition)

            event.accepted = true
            return
        }

        if(event.key === Qt.Key_PageUp){
            root.deselect()

            if(initialCursorPositionX === -1){
                initialCursorPositionX = root.cursorRectangle.x
            }

            var newPosition = root.positionAt(initialCursorPositionX, root.cursorRectangle.y - viewHeight)
            moveViewY(-viewHeight)
            root.cursorPosition = newPosition

            event.accepted = true
            return
        }
        if(event.key === Qt.Key_PageDown){
            root.deselect()

            if(initialCursorPositionX === -1){
                initialCursorPositionX = root.cursorRectangle.x
            }

            var newPosition = root.positionAt(initialCursorPositionX, root.cursorRectangle.y + viewHeight)
            moveViewY(viewHeight)
            root.cursorPosition = newPosition

            event.accepted = true
            return
        }

        initialCursorPositionX = -1
        initialCursorPosition = -1

        event.accepted = false

    }

}
