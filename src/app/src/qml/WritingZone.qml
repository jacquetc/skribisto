import QtQuick 2.12
import eu.plumecreator.documenthandler 1.0

WritingZoneForm {
    textAreaWidth: 200
    stretch: true
    minimapVisibility: false
    readonly property int textAreaLeftPos: base.width / 2 - textAreaWidth / 2
    readonly property int textAreaRightPos: base.width / 2 + textAreaWidth / 2


    DocumentHandler {
        id: documentHandler
        textDocument: textArea.textDocument
        cursorPosition: textArea.cursorPosition
        selectionStart: textArea.selectionStart
        selectionEnd: textArea.selectionEnd
    }

    //textArea.onCursorRectangleChanged: flickable.ensureVisible(textArea.cursorRectangle)

    leftScrollTouchArea.onUpdated: {
        var deltaY = touchPoints[0].y - touchPoints[0].previousY
        console.log("deltaY :", deltaY)

        if(flickable.atYBeginning && deltaY > 0){
            flickable.returnToBounds();
            return;
        }
        if(flickable.atYEnd && deltaY < 0){
            flickable.returnToBounds();
            return;
        }

        flickable.contentY = flickable.contentY - deltaY



        for (var touch in touchPoints)
            console.log("Multitouch updated touch", touchPoints[touch].pointId, "at", touchPoints[touch].x, ",", touchPoints[touch].y ,",",  touchPoints[touch].previousY, ",",  touchPoints[touch].startY)
    }

    leftScrollMouseArea.onPressAndHold: {
    }
    leftScrollMouseArea.onWheel: {
//        if(wheel.pixelDelta.y !== 0){
//console.log("pixelDelta :", wheel.pixelDelta.y)
//            flickable.contentY = flickable.contentY - wheel.pixelDelta.y
//        }else
            flickable.contentY = flickable.contentY - wheel.angleDelta.y/8
        console.log("angleDelta :",wheel.angleDelta.y)
        console.log("flickable.contentY :",flickable.contentY)

        if(flickable.atYBeginning && wheel.angleDelta.y > 0){
            flickable.returnToBounds();
            return;
        }
        if(flickable.atYEnd && wheel.angleDelta.y < 0){
            flickable.returnToBounds();
            return;
        }

    }

}
