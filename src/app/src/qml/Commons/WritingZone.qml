import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import eu.skribisto.documenthandler 1.0

WritingZoneForm {
    id: root
    stretch: true
    minimapVisibility: true
    leftScrollItemVisible: true
    property string name: ""
    readonly property int wantedCenteredTextAreaLeftPos: rootPageBase.width / 2 - textAreaWidth / 2
    readonly property int wantedCenteredTextAreaRightPos: rootPageBase.width / 2 + textAreaWidth / 2

    readonly property int wantedCenteredWritingZoneLeftPos: rootPageBase.width / 2 - root.width / 2
    readonly property int wantedCenteredWritingZoneRightPos: rootPageBase.width / 2 + root.width / 2

    textArea.persistentSelection: true

    property int textPointSize: 12
    onTextPointSizeChanged: {
        textArea.font.pointSize = textPointSize
    }
    property string textFontFamily: ""
    onTextFontFamilyChanged: {
        textArea.font.family = textFontFamily
    }

    property real textIndent: 0
    onTextIndentChanged: {
        //console.log("eeee")
        documentHandler.indentEverywhere = textIndent
    }

    property real textTopMargin: 0
    onTextTopMarginChanged: {
        //console.log("bbbb")
        documentHandler.topMarginEverywhere = textTopMargin
    }



    //quit fullscreen :

//    Shortcut {
//        enabled: textArea.activeFocus
//        sequence: "Esc"
//        onActivatedAmbiguously: {
//            console.log("activated 2")
//            if(fullscreenAction.checked === true){
//            fullscreenAction.checked = false

//            }

//        }
//    }


//    Keys.onShortcutOverride: event.accepted = (event.key === Qt.Key_Escape)

//        console.log("escape in WritingZone z")
//        if (event.key === Qt.Key_Escape){
//            console.log("escape in WritingZone a")
//            if (textArea.activeFocus){
//                     if(fullscreenAction.checked === true){
//                         console.log("escape in WritingZone b")
//                            fullscreenAction.checked = false
//                         event.accepted = true
//                     }
//                     else
//                     {
//                         event.accepted = false
//                     }
//            }
//            event.accepted = false


//        }
//        if ((event.modifiers & Qt.ShiftModifier) && event.key === Qt.Ke){

//        }

//    }



    // context menu :


    textArea.onPressed: {
        if(event.buttons === Qt.RightButton){


            // deselect if outside selection :
            var startY = textArea.positionToRectangle(textArea.selectionStart).y
            var endY = textArea.positionToRectangle(textArea.selectionEnd).y
            var pointY = event.y

            if(textArea.selectedText != "" & (pointY < startY | pointY > endY)){
                textArea.cursorPosition = textArea.positionAt(event.x, event.y)
                menu.popup(textArea, event.x, event.y)
                return
            }
            else{
                menu.popup(textArea, event.x, event.y)
            }





        }
    }



    //menu :
    Menu {
        id: menu

        onOpened: {
        }




        MenuSeparator {}
        Action {

            text: qsTr("Copy")
            shortcut: StandardKey.Copy
            icon {
                name: "edit-copy"
            }
            enabled: textArea.selectedText !== ""

            onTriggered: {
                console.log("copy action text", textArea.selectedText)
                textArea.copy()
            }
        }
        Action {

            text: qsTr("Cut")
            shortcut: StandardKey.Cut
            icon {
                name: "edit-cut"
            }
            enabled: textArea.selectedText !== ""

            onTriggered: {
                console.log("cut action text", textArea.selectedText)
                textArea.cut()

            }
        }
        Action {

            text: qsTr("Paste")
            shortcut: StandardKey.Cut
            icon {
                name: "edit-paste"
            }

            onTriggered: {
                console.log("paste action text")
                textArea.paste()
            }
        }
        MenuSeparator {}
    }

    //property int visibleAreaY: 0

    //    Binding {
    //        target: flickable
    //        property: "contentY"
    //        value: visibleAreaY
    //        when: !flickable.moving
    //    }
    //    Binding {
    //        target: root
    //        property: "visibleAreaY"
    //        value: flickable.contentY
    //        when: flickable.moving
    //    }
    //    property int cursorPosition: textArea.cursorPosition
    //    onCursorPositionChanged: {
    //        if(!textArea.activeFocus){
    //                    if (documentHandler.maxCursorPosition() >= cursorPosition) {
    //                        textArea.cursorPosition = cursorPosition
    //                        console.log("textArea.cursorPosition =", cursorPosition)

    //                    } else {
    //                        textArea.cursorPosition = documentHandler.maxCursorPosition()
    //                    }
    //        }
    //    }

    property int cursorPosition: textArea.cursorPosition


    function setCursorPosition(cursorPosition){
        if (documentHandler.maxCursorPosition() >= cursorPosition) {
            textArea.cursorPosition = cursorPosition
            console.log("textArea.cursorPosition =", cursorPosition)

        } else {
            textArea.cursorPosition = documentHandler.maxCursorPosition()
        }

    }


    //    property int selectionStart: textArea.cursorPosition
    //    property int selectionEnd: textArea.cursorPosition
    //    textArea.onSelectionStartChanged: {
    //        console.log("selection start changed")
    //        selectionStart = textArea.selectionStart
    //    }
    //    textArea.onSelectionEndChanged: {
    //        selectionEnd = textArea.selectionEnd
    //    }

    //    textArea.onCursorPositionChanged: {
    //        if(textArea.activeFocus){
    //        cursorPosition = textArea.cursorPosition
    //        documentHandler.cursorPosition = textArea.cursorPosition
    //        }
    //    }

    //    property int cursorPosition: 0
    //    onCursorPositionChanged: {
    //        if (documentHandler.maxCursorPosition() >= cursorPosition) {
    //            textArea.cursorPosition = cursorPosition
    //            cursorPositionBindingBool = true

    //            //console.log(">=", cursorPosition)
    //        } else {
    //            cursorPositionBindingBool = false
    //            textArea.cursorPosition = documentHandler.maxCursorPosition()
    //            //console.log("<", cursorPosition)
    //        }
    //    }

    //    Binding {
    //        id: cursorPositionBinding
    //        target: textArea
    //        property: "cursorPosition"
    //        value: cursorPosition
    //        delayed: true
    //        when: cursorPositionBindingBool
    //    }
    property alias documentHandler: documentHandler

    DocumentHandler {
        id: documentHandler
        textDocument: textArea.textDocument
        cursorPosition: textArea.cursorPosition
        //        onCursorPositionChanged:
        //            root.cursorPosition = documentHandler.cursorPosition


        selectionStart: textArea.selectionStart
        selectionEnd: textArea.selectionEnd


    }


    // left scroll area :

    //textArea.onCursorRectangleChanged: flickable.ensureVisible(textArea.cursorRectangle)
    leftScrollTouchArea.onUpdated: {
        var deltaY = touchPoints[0].y - touchPoints[0].previousY
        //        console.log("deltaY :", deltaY)

        if (flickable.atYBeginning && deltaY > 0) {
            flickable.returnToBounds()
            return
        }
        if (flickable.atYEnd && deltaY < 0) {
            flickable.returnToBounds()
            return
        }

        flickable.flick(0, deltaY * 50 )

        //        for (var touch in touchPoints)
        //            console.log("Multitouch updated touch", touchPoints[touch].pointId,
        //                        "at", touchPoints[touch].x, ",", touchPoints[touch].y,
        //                        ",", touchPoints[touch].previousY, ",",
        //                        touchPoints[touch].startY)
    }

    leftScrollMouseArea.onPressAndHold: {

    }
    leftScrollMouseArea.onWheel: {


        var deltaY = wheel.angleDelta.y *10

        flickable.flick(0, deltaY)

        if (flickable.atYBeginning && wheel.angleDelta.y > 0) {
            flickable.returnToBounds()
            return
        }
        if (flickable.atYEnd && wheel.angleDelta.y < 0) {
            flickable.returnToBounds()
            return
        }
    }


    // right scroll area :

    //textArea.onCursorRectangleChanged: flickable.ensureVisible(textArea.cursorRectangle)
    rightScrollTouchArea.onUpdated: {
        var deltaY = touchPoints[0].y - touchPoints[0].previousY
        //        console.log("deltaY :", deltaY)

        if (flickable.atYBeginning && deltaY > 0) {
            flickable.returnToBounds()
            return
        }
        if (flickable.atYEnd && deltaY < 0) {
            flickable.returnToBounds()
            return
        }

        flickable.flick(0, deltaY * 50)

        //        for (var touch in touchPoints)
        //            console.log("Multitouch updated touch", touchPoints[touch].pointId,
        //                        "at", touchPoints[touch].x, ",", touchPoints[touch].y,
        //                        ",", touchPoints[touch].previousY, ",",
        //                        touchPoints[touch].startY)
    }

    rightScrollMouseArea.onPressAndHold: {

    }
    rightScrollMouseArea.onWheel: {

        var deltaY = wheel.angleDelta.y *10

        flickable.flick(0, deltaY)

        if (flickable.atYBeginning && wheel.angleDelta.y > 0) {
            flickable.returnToBounds()
            return
        }
        if (flickable.atYEnd && wheel.angleDelta.y < 0) {
            flickable.returnToBounds()
            return
        }
    }

    
    // scrollView :
    
    
    
    //focus :

    onActiveFocusChanged: {
        if (activeFocus) {
            textArea.forceActiveFocus()
        }
        else{
        }
    }
}
