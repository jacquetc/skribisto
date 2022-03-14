import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Controls.Material
import ".."

TextArea {
    id: root

    antialiasing: true
    property bool styleElevation: false
    property string styleBackgroundColor: "#FFFFFF"
    property string styleForegroundColor: "#000000"
    property string styleAccentColor: "#000000"


    //needed for skrtextbridge
    objectName: "SKRTextArea-" + Qt.formatDateTime(new Date(),
                                                   "yyyyMMddhhmmsszzz")
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

    background: Pane {
        Material.background: styleBackgroundColor
        Material.elevation: styleElevation ? 6 : 0
    }
    //-------------------------------------------------
    property bool active: false

    Timer{
        id: deactivateTimer
        interval: 100
        onTriggered: {
            root.active = false
        }
    }
    //-------------------------------------------------


    property int viewHeight: 200
    signal moveViewYCalled(int height, bool animationEnabled)

    Keys.priority: Keys.BeforeItem

    property int initialCursorPositionX: -1
    property int initialCursorPosition: -1
    Keys.onPressed: function(event) {
        root.active = true
        if(deactivateTimer.running){
            deactivateTimer.stop()
        }
        deactivateTimer.start()

        // paste :
        if (((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_V)
                || event.key === Qt.Key_Paste) {
            pasteTextAction.trigger()
            event.accepted = true
            return
        }

        // page Up :
        if ((event.modifiers & Qt.ShiftModifier)
                && event.key === Qt.Key_PageUp) {

            if (initialCursorPositionX === -1) {
                initialCursorPositionX = root.cursorRectangle.x
            }
            if (initialCursorPosition === -1) {
                initialCursorPosition = root.cursorPosition
            }

            var newPosition = root.positionAt(
                        initialCursorPositionX,
                        root.cursorRectangle.y - viewHeight)
            moveViewYCalled(-viewHeight, true)
            root.cursorPosition = newPosition

            root.select(initialCursorPosition, root.cursorPosition)

            event.accepted = true
            return
        }
        if ((event.modifiers & Qt.ShiftModifier)
                && event.key === Qt.Key_PageDown) {

            if (initialCursorPositionX === -1) {
                initialCursorPositionX = root.cursorRectangle.x
            }
            if (initialCursorPosition === -1) {
                initialCursorPosition = root.cursorPosition
            }

            var newPosition = root.positionAt(
                        initialCursorPositionX,
                        root.cursorRectangle.y + viewHeight)
            moveViewYCalled(viewHeight, true)
            root.cursorPosition = newPosition

            root.select(initialCursorPosition, root.cursorPosition)

            event.accepted = true
            return
        }

        if (event.key === Qt.Key_PageUp) {
            root.deselect()

            if (initialCursorPositionX === -1) {
                initialCursorPositionX = root.cursorRectangle.x
            }

            var newPosition = root.positionAt(
                        initialCursorPositionX,
                        root.cursorRectangle.y - viewHeight)
            moveViewYCalled(-viewHeight, true)
            root.cursorPosition = newPosition

            event.accepted = true
            return
        }
        if (event.key === Qt.Key_PageDown) {
            root.deselect()

            if (initialCursorPositionX === -1) {
                initialCursorPositionX = root.cursorRectangle.x
            }

            var newPosition = root.positionAt(
                        initialCursorPositionX,
                        root.cursorRectangle.y + viewHeight)
            moveViewYCalled(viewHeight, true)
            root.cursorPosition = newPosition

            event.accepted = true
            return
        }

        initialCursorPositionX = -1
        initialCursorPosition = -1

        for (var i in keysPriv.additionalKeyFunctions) {
            var keyFunction = keysPriv.additionalKeyFunctions[i]
            var returnCode = keyFunction(event)
            if (returnCode) {
                return
            }
        }

        event.accepted = false
    }

    QtObject {
        id: keysPriv
        property var additionalKeyFunctions: []
    }

    function addAdditionalKeyFunction(_function) {
        keysPriv.additionalKeyFunctions.push(_function)
    }

    function removeAdditionalKeyFunction(_function) {
        keysPriv.additionalKeyFunctions = keysPriv.additionalKeyFunctions.filter(
                    item => item !== _function)
    }

    //--------------------------------------------------------------------------------
    //--------Text centering----------------------------------------------------------
    //--------------------------------------------------------------------------------
    property bool textCenteringEnabled: false
    property int viewContentY

    onLengthChanged: {
        if (!root.textCenteringEnabled || root.selectedText) {
            return
        }

        var cursorY = root.cursorRectangle.y

        var wantedCursorY = root.viewContentY + root.viewHeight / 2

        moveViewYCalled(-(wantedCursorY - cursorY), false)
    }

    //--------------------------------------------------------------------------------
    //--------press and hold to move text----------------------------------------------------------
    //--------------------------------------------------------------------------------
    TapHandler {
        id: textDragTapHandler
        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
        acceptedButtons: Qt.LeftButton

        onGrabChanged: function(transition, point) {

            //console.log("tapHandler", transition)
        }

        gesturePolicy: TapHandler.DragThreshold

        function dist(pointA, pointB) {
            var x1 = pointA.x
            var y1 = pointA.y
            var x2 = pointB.x
            var y2 = pointB.y

            var dist = Math.sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))
            return (dist)
        }

        property point pressPosition: textDragTapHandler.point.pressPosition
        property point position: textDragTapHandler.point.position
        property bool startedWithSelectedText: false

        onPressedChanged: {
            priv.touchDetected = false

            if (pressed) {
                //                console.log("pressed")
                startedWithSelectedText = selectedText.length !== 0
                priv.selectionStart = selectionStart
                priv.selectionEnd = selectionEnd
            }
        }

        onCanceled: {

            // means dragThreshold was reached
            if (selectedText.length === 0) {

                return
            }
            if (startedWithSelectedText && isMouseIsInSelectedRect(
                        pressPosition.x, pressPosition.y)) {

                //                    console.log("dragging")
                startedWithSelectedText = false

                priv.selectedText = getText(priv.selectionStart,
                                            priv.selectionEnd)
                priv.selectedText = documentHandler.getHtmlAtSelection(
                            priv.selectionStart, priv.selectionEnd)

                //                    console.log("tedText", priv.selectedText)
                //                    console.log("priv.selectedText", priv.selectedText)
                //                    console.log("  textL", priv.selectedText.length)
                //                    console.log("selectL", selectionEnd - selectionEnd)
                root.Drag.active = true

                point.accepted = true
            }
            // }
        }

        function isMouseIsInSelectedRect(eventX, eventY) {
            return selectionStart <= positionAt(eventX, eventY) && positionAt(
                        eventX, eventY) <= selectionEnd
        }
    }

    Drag.dragType: Drag.Automatic

    Drag.supportedActions: Qt.MoveAction

    Drag.mimeData: {
        "text/html": priv.selectedText
    }

    DropArea {
        anchors.fill: parent

        keys: ["text/html"]

        onPositionChanged: {

            cursorPosition = positionAt(drag.x, drag.y)
            //must select falsly by highlighter
        }

        onDropped: {
            if (!containsDrag) {
                priv.selectionStart = 0
                priv.selectionEnd = 0
                drop.accept()
                return
            }
            console.log("dropped")

            if (priv.selectionStart <= cursorPosition
                    && cursorPosition <= priv.selectionEnd) {
                priv.selectionStart = 0
                priv.selectionEnd = 0
                drop.accept()
                return
            }

            if (drop.proposedAction === Qt.MoveAction) {

                var isBefore = false
                if (cursorPosition < priv.selectionStart) {
                    isBefore = true
                }

                var originalCursorPosition = cursorPosition

                if (drop.hasHtml) {

                    documentHandler.insertHtml(cursorPosition,
                                               skrRootItem.cleanUpHtml(
                                                   drop.html))
                } else if (drop.hasText) {
                    var st = drop.text
                    console.log("text:", st)
                    insert(cursorPosition, st)
                }

                console.log("originalCursorPosition", originalCursorPosition)
                console.log("priv.selectionStart", priv.selectionStart)
                console.log("priv.selectionEnd", priv.selectionEnd)

                if (isBefore) {
                    priv.selectionStart += cursorPosition - originalCursorPosition
                    priv.selectionEnd += cursorPosition - originalCursorPosition
                }
                console.log("cursorPosition", cursorPosition)
                console.log("priv.selectionStart", priv.selectionStart)
                console.log("priv.selectionEnd", priv.selectionEnd)

                remove(priv.selectionStart, priv.selectionEnd)
                priv.selectionStart = 0
                priv.selectionEnd = 0
                drop.accept()
            }
        }

        onExited: {

        }
        onEntered: {

        }
    }

    QtObject {
        id: priv
        //property bool dragging: false
        property string selectedText: ""
        property int selectionStart: 0
        property int selectionEnd: 0
        property bool touchDetected: false
    }

    //----------------------------------------------------------------------
    //-----------touch handler--------------------------------------------------
    //----------------------------------------------------------------------
    TapHandler {
        id: touchHandler
        acceptedDevices: PointerDevice.TouchScreen
        acceptedPointerTypes: PointerDevice.Finger

        onSingleTapped: function(eventPoint) {
            console.log("tapped")
            forceActiveFocus()
            priv.touchDetected = false
            priv.touchDetected = true
            cursorPosition = positionAt(eventPoint.position.x,
                                        eventPoint.position.y)
        }

        onDoubleTapped: function(eventPoint) {
            console.log("double tapped")
            cursorPosition = positionAt(eventPoint.position.x,
                                        eventPoint.position.y)
            selectWord()
        }

        onLongPressed: {
            callTextAreaContextMenu(point.position.x, point.position.y)
        }
    }

    //    PointHandler{
    //        id: touchFlickHandler
    //        acceptedDevices: PointerDevice.TouchScreen
    //        acceptedPointerTypes: PointerDevice.Finger

    //        //        onGrabChanged: function(transition, point) {
    //        //            console.log("grab txtarea grabbed")

    //        //        }

    //        property real tempY: 0
    //        onPointChanged: {

    //            if(dragThreshold < dist(point.pressPosition, point.position)){
    //                console.log("flicking")

    //                //textAreaFlickable.contentY = textAreaFlickable.contentY - (point.position.y - tempY)
    //                //                temPos = point.position
    //                if(!textAreaFlickable.flickingVertically){
    //                    textAreaFlickable.flick(0, (point.position.y - tempY) * 50)
    //                }

    //            }

    //            //                textAreaFlickable.flick(0, point.velocity.y * dist(temPos, point.position))

    //            tempY = point.position.y

    //        }

    //        function dist (pointA, pointB) {
    //            var x1 = pointA.x
    //            var y1 = pointA.y
    //            var x2 = pointB.x
    //            var y2 = pointB.y

    //            var dist = Math.sqrt((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1));
    //            return (dist);
    //        }

    //        onCanceled: { // means dragThreshold was reached

    //            console.log("onCanceled")
    //        }

    //    }

    //-----------------------------------------------------------------------
    //---------Touch selection handles ----------------------------------------------
    //-----------------------------------------------------------------------
    QtObject {
        id: p_selectionHandle

        property bool blocked: false
        property int rectHeight: root.font.pointSize * 3 / 4 < 15 ? 15 : root.font.pointSize * 3 / 4
    }
    Item {
        id: leftSelectionHandle
        visible: priv.touchDetected && selectedText.length !== 0

        width: p_selectionHandle.rectHeight
        height: p_selectionHandle.rectHeight * 3 + root.font.pointSize

        onVisibleChanged: {
            if (visible) {
                p_selectionHandle.blocked = true
                leftSelectionHandle.x = positionToRectangle(selectionStart).x
                leftSelectionHandle.y = positionToRectangle(
                            selectionStart).y - p_selectionHandle.rectHeight
                select(positionAt(
                           leftSelectionHandle.x,
                           leftSelectionHandle.y + leftSelectionHandle.height / 2),
                       selectionEnd)
                p_selectionHandle.blocked = false
            }
        }

        Binding {
            target: leftSelectionHandle
            property: "visible"
            id: leftSelectionHandleVisibleBinding
            value: leftSelectionHandleHandler_up.active
                   || leftSelectionHandleHandler_down.active
                   || (priv.touchDetected && selectedText.length !== 0)
            delayed: true
            restoreMode: Binding.RestoreNone
        }

        property int xPlusY: leftSelectionHandle.x + leftSelectionHandle.y
        onXPlusYChanged: {
            if (p_selectionHandle.blocked) {
                return
            }
            select(positionAt(
                       leftSelectionHandle.x,
                       leftSelectionHandle.y + leftSelectionHandle.height / 2),
                   selectionEnd)
        }

        Column {

            anchors.fill: parent
            anchors.leftMargin: -p_selectionHandle.rectHeight / 2
            anchors.rightMargin: -p_selectionHandle.rectHeight / 2
            padding: 0

            Rectangle {
                color: SkrTheme.accent
                border.width: 2
                border.color: SkrTheme.buttonBackground

                width: p_selectionHandle.rectHeight
                height: p_selectionHandle.rectHeight
                radius: p_selectionHandle.rectHeight / 2

                DragHandler {
                    id: leftSelectionHandleHandler_up
                    acceptedDevices: PointerDevice.TouchScreen
                    acceptedPointerTypes: PointerDevice.Finger
                    margin: 10

                    target: leftSelectionHandle

                    onActiveChanged: {
                        if (!active) {
                            p_selectionHandle.blocked = true
                            leftSelectionHandle.x = positionToRectangle(
                                        selectionStart).x
                            leftSelectionHandle.y = positionToRectangle(
                                        selectionStart).y - p_selectionHandle.rectHeight
                            p_selectionHandle.blocked = false
                        }
                    }
                }
            }

            Item {
                width: p_selectionHandle.rectHeight
                height: root.font.pointSize + p_selectionHandle.rectHeight
            }
            Rectangle {
                color: SkrTheme.accent
                border.width: 2
                border.color: SkrTheme.buttonBackground
                width: p_selectionHandle.rectHeight
                height: p_selectionHandle.rectHeight
                radius: p_selectionHandle.rectHeight / 2

                DragHandler {
                    id: leftSelectionHandleHandler_down
                    acceptedDevices: PointerDevice.TouchScreen
                    acceptedPointerTypes: PointerDevice.Finger
                    margin: 10

                    target: leftSelectionHandle

                    onActiveChanged: {
                        if (!active) {
                            p_selectionHandle.blocked = true
                            leftSelectionHandle.x = positionToRectangle(
                                        selectionStart).x
                            leftSelectionHandle.y = positionToRectangle(
                                        selectionStart).y - leftSelectionHandle.height / 3
                            p_selectionHandle.blocked = false
                        }
                    }
                }
            }
        }
    }

    //-----------------------------------------------------------------------
    Item {
        id: rightSelectionHandle
        width: p_selectionHandle.rectHeight
        height: p_selectionHandle.rectHeight * 3 + root.font.pointSize

        onVisibleChanged: {
            if (visible) {
                p_selectionHandle.blocked = true
                rightSelectionHandle.x = positionToRectangle(selectionEnd).x
                rightSelectionHandle.y = positionToRectangle(
                            selectionEnd).y - p_selectionHandle.rectHeight
                select(selectionStart, positionAt(
                           rightSelectionHandle.x,
                           rightSelectionHandle.y + rightSelectionHandle.height / 2))
                p_selectionHandle.blocked = false
            }
        }

        Binding {
            target: rightSelectionHandle
            property: "visible"
            id: rightSelectionHandleVisibleBinding
            value: rightSelectionHandleHandler_up.active
                   || rightSelectionHandleHandler_down.active
                   || (priv.touchDetected && selectedText.length !== 0)
            delayed: true
            restoreMode: Binding.RestoreNone
        }

        property int xPlusY: rightSelectionHandle.x + rightSelectionHandle.y
        onXPlusYChanged: {
            if (p_selectionHandle.blocked) {
                return
            }
            select(selectionStart, positionAt(
                       rightSelectionHandle.x,
                       rightSelectionHandle.y + rightSelectionHandle.height / 2))
        }

        Column {

            anchors.fill: parent
            anchors.leftMargin: -p_selectionHandle.rectHeight / 2
            anchors.rightMargin: -p_selectionHandle.rectHeight / 2
            padding: 0

            Rectangle {
                color: SkrTheme.accent
                border.width: 2
                border.color: SkrTheme.buttonBackground

                width: p_selectionHandle.rectHeight
                height: p_selectionHandle.rectHeight
                radius: p_selectionHandle.rectHeight / 2

                DragHandler {
                    id: rightSelectionHandleHandler_up
                    acceptedDevices: PointerDevice.TouchScreen
                    acceptedPointerTypes: PointerDevice.Finger
                    margin: 10

                    target: rightSelectionHandle

                    onActiveChanged: {
                        if (!active) {
                            p_selectionHandle.blocked = true
                            rightSelectionHandle.x = positionToRectangle(
                                        selectionEnd).x
                            rightSelectionHandle.y = positionToRectangle(
                                        selectionEnd).y - p_selectionHandle.rectHeight
                            p_selectionHandle.blocked = false
                        }
                    }
                }
            }

            Item {
                width: p_selectionHandle.rectHeight
                height: root.font.pointSize + p_selectionHandle.rectHeight
            }
            Rectangle {
                color: SkrTheme.accent
                border.width: 2
                border.color: SkrTheme.buttonBackground
                width: p_selectionHandle.rectHeight
                height: p_selectionHandle.rectHeight
                radius: p_selectionHandle.rectHeight / 2

                DragHandler {
                    id: rightSelectionHandleHandler_down
                    acceptedDevices: PointerDevice.TouchScreen
                    acceptedPointerTypes: PointerDevice.Finger
                    margin: 10

                    target: rightSelectionHandle

                    onActiveChanged: {
                        if (!active) {
                            p_selectionHandle.blocked = true
                            rightSelectionHandle.x = positionToRectangle(
                                        selectionEnd).x
                            rightSelectionHandle.y = positionToRectangle(
                                        selectionEnd).y - rightSelectionHandle.height / 3
                            p_selectionHandle.blocked = false
                        }
                    }
                }
            }
        }
    }

    //--------------------------------------------------------------
    //--------Wheel---------------------------------------------
    //--------------------------------------------------------------
    WheelHandler {
        acceptedModifiers: Qt.NoModifier
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        onWheel: function(event) {
            moveViewYCalled(-event.angleDelta.y / 2, false)
        }
    }
}
