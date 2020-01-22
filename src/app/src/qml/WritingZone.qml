import QtQuick 2.12
import eu.skribisto.documenthandler 1.0

WritingZoneForm {
    id: root
    stretch: true
    minimapVisibility: false
    readonly property int textAreaLeftPos: base.width / 2 - textAreaWidth / 2
    readonly property int textAreaRightPos: base.width / 2 + textAreaWidth / 2

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
    property bool cursorPositionBindingBool: false
    property int cursorPosition: 0
    onCursorPositionChanged: {
        if(textArea.activeFocus){
                    if (documentHandler.maxCursorPosition() >= cursorPosition) {
                        textArea.cursorPosition = cursorPosition
                        console.log("textArea.cursorPosition =", cursorPosition)

                    } else {
                        textArea.cursorPosition = documentHandler.maxCursorPosition()
                    }
        }
    }


    textArea.onCursorPositionChanged: {
        if(textArea.activeFocus){
        cursorPosition = textArea.cursorPosition
        documentHandler.cursorPosition = textArea.cursorPosition
        }
    }

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

        flickable.contentY = flickable.contentY - deltaY

//        for (var touch in touchPoints)
//            console.log("Multitouch updated touch", touchPoints[touch].pointId,
//                        "at", touchPoints[touch].x, ",", touchPoints[touch].y,
//                        ",", touchPoints[touch].previousY, ",",
//                        touchPoints[touch].startY)
    }

    leftScrollMouseArea.onPressAndHold: {

    }
    leftScrollMouseArea.onWheel: {
        //        if(wheel.pixelDelta.y !== 0){
        //console.log("pixelDelta :", wheel.pixelDelta.y)
        //            flickable.contentY = flickable.contentY - wheel.pixelDelta.y
        //        }else
        flickable.contentY = flickable.contentY - wheel.angleDelta.y / 8
//        console.log("angleDelta :", wheel.angleDelta.y)
//        console.log("flickable.contentY :", flickable.contentY)

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

        flickable.contentY = flickable.contentY - deltaY

//        for (var touch in touchPoints)
//            console.log("Multitouch updated touch", touchPoints[touch].pointId,
//                        "at", touchPoints[touch].x, ",", touchPoints[touch].y,
//                        ",", touchPoints[touch].previousY, ",",
//                        touchPoints[touch].startY)
    }

    rightScrollMouseArea.onPressAndHold: {

    }
    rightScrollMouseArea.onWheel: {
        //        if(wheel.pixelDelta.y !== 0){
        //console.log("pixelDelta :", wheel.pixelDelta.y)
        //            flickable.contentY = flickable.contentY - wheel.pixelDelta.y
        //        }else
        flickable.contentY = flickable.contentY - wheel.angleDelta.y / 8
//        console.log("angleDelta :", wheel.angleDelta.y)
//        console.log("flickable.contentY :", flickable.contentY)

        if (flickable.atYBeginning && wheel.angleDelta.y > 0) {
            flickable.returnToBounds()
            return
        }
        if (flickable.atYEnd && wheel.angleDelta.y < 0) {
            flickable.returnToBounds()
            return
        }
    }

    //focus :

    onActiveFocusChanged: {
        if (activeFocus) {
            textArea.forceActiveFocus()
        }
    }
}
