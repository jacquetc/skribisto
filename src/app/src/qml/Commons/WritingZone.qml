import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15
import ".."
import eu.skribisto.documenthandler 1.0
import eu.skribisto.highlighter 1.0
import eu.skribisto.spellchecker 1.0
import eu.skribisto.projectdicthub 1.0
import eu.skribisto.clipboard 1.0
import "../Items"

WritingZoneForm {
    id: root
    stretch: true
    leftScrollItemVisible: true

    textArea.persistentSelection: true
    textArea.textFormat: TextEdit.RichText

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
    property string placeholderText: ""
    onPlaceholderTextChanged: {
                textArea.placeholderText = placeholderText
    }
    // clipboard :
    SKRClipboard {
        id: clipboard
        documentHandler: documentHandler
        fontPointSize: root.textPointSize
        fontFamily: root.textFontFamily
        textIndent: root.textIndent
        textTopMargin: root.textTopMargin
    }

    //-------------------------------------------------
    Component.onCompleted: {
        determineTextCursorUnblinkingSetting()
    }

    //-------------------------------------------------

    //style :
    property bool textAreaStyleElevation: textArea.styleElevation
    onTextAreaStyleElevationChanged: {
        textArea.styleElevation = textAreaStyleElevation
    }
    property string textAreaStyleBackgroundColor: textArea.styleBackgroundColor
    onTextAreaStyleBackgroundColorChanged: {
        textArea.styleBackgroundColor = textAreaStyleBackgroundColor
    }
    property string textAreaStyleForegroundColor: textArea.styleForegroundColor
    onTextAreaStyleForegroundColorChanged: {
        textArea.styleForegroundColor = textAreaStyleForegroundColor
    }
    property string textAreaStyleAccentColor: textArea.styleAccentColor
    onTextAreaStyleAccentColorChanged: {
        textArea.styleAccentColor = textAreaStyleAccentColor
    }

    //-------------------------------------------------
    property int projectId: -2
    onProjectIdChanged: {
        documentHandler.setId(projectId, treeItemId)

        if (!spellCheckerKilled) {
            determineSpellCheckerLanguageCode()
        }
    }
    property int treeItemId: -2
    onTreeItemIdChanged: {
        documentHandler.setId(projectId, treeItemId)
    }

    function clear() {
        textArea.clear()
        treeItemId = -2
        projectId = -2
    }

    //-------------------------------------------------
    property bool spellCheckerKilled: false

    property bool textCenteringEnabled: false

    // style :

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

    //-----------------------------------------------------------
    //----- special non-blinking cursor : -----------------------
    //-----------------------------------------------------------
    Connections {
        target: SkrSettings.ePaperSettings
        function onTextCursorUnblinkingChanged() {
            determineTextCursorUnblinkingSetting()
        }
    }

    //-----------------------------------------------------------
    function determineTextCursorUnblinkingSetting() {
        if (SkrSettings.ePaperSettings.textCursorUnblinking) {
            textArea.cursorDelegate = unblinkingCursorComponent
        } else {
            textArea.cursorDelegate = null
        }
    }

    //-----------------------------------------------------------
    Component {
        id: unblinkingCursorComponent
        RowLayout {
            width: 4

            Rectangle {
                id: rectangle
                color: textArea.activeFocus ? "#000000" : "transparent"
                Layout.preferredWidth: 4
                Layout.fillHeight: true
            }
        }
    }

    //-----------------------------------------------------------
    // ---------context menu :----------------------------------
    //-----------------------------------------------------------
    textArea.onPressed: {
        if (event.buttons === Qt.RightButton) {
            callTextAreaContextMenu(event.x, event.y)
        }
    }

    function callTextAreaContextMenu(posX, posY) {
        if (textContextMenu.visible) {
            textContextMenu.close()
            return
        }

        // deselect if outside selection :
        var selectStart = textArea.selectionStart
        var selectEnd = textArea.selectionEnd
        var eventCursorPosition = textArea.positionAt(posX, posY)

        if (textArea.selectedText.length != 0
                && (selectStart <= eventCursorPosition
                    && eventCursorPosition <= selectEnd)) {

            //prepare context menu
            var point = mapFromItem(textArea, posX, posY)
            textContextMenu.x = point.x
            textContextMenu.y = point.y

            if (documentHandler.isWordMisspelled(eventCursorPosition)) {
                textContextMenu.currentIndex = 1
                documentHandler.listAndSendSpellSuggestions(eventCursorPosition)
            }

            textContextMenu.open()
            //console.log("deselect")
            return
        } else {

            // no text selected OR text selected but clicked outside selection
            textArea.cursorPosition = eventCursorPosition

            var pointB = mapFromItem(textArea, posX, posY)
            textContextMenu.x = pointB.x
            textContextMenu.y = pointB.y

            //prepare context menu
            if (documentHandler.isWordMisspelled(eventCursorPosition)) {
                textContextMenu.currentIndex = 1
                documentHandler.listAndSendSpellSuggestions(eventCursorPosition)
            }

            textContextMenu.open()
        }
    }

    textArea.onActiveFocusChanged: {
        if (textArea.activeFocus) {
            //console.log("activeFocus = true")

            //console.log("disconnect", skrEditMenuSignalHub.clearCutConnections())
            skrEditMenuSignalHub.clearCutConnections()
            skrEditMenuSignalHub.clearCopyConnections()
            skrEditMenuSignalHub.clearPasteConnections()
            skrEditMenuSignalHub.clearItalicConnections()
            skrEditMenuSignalHub.clearBoldConnections()
            skrEditMenuSignalHub.clearStrikeConnections()
            skrEditMenuSignalHub.clearUnderlineConnections()

            skrEditMenuSignalHub.subscribe(textArea.objectName)

            skrEditMenuSignalHub.cutActionTriggered.connect(cut)
            skrEditMenuSignalHub.copyActionTriggered.connect(copy)
            skrEditMenuSignalHub.pasteActionTriggered.connect(paste)

            skrEditMenuSignalHub.italicActionTriggered.connect(italic)
            skrEditMenuSignalHub.boldActionTriggered.connect(bold)
            skrEditMenuSignalHub.strikeActionTriggered.connect(strike)
            skrEditMenuSignalHub.underlineActionTriggered.connect(underline)

            italicAction.preventTrigger = true
            italicAction.checked = documentHandler.italic
            italicAction.preventTrigger = false

            boldAction.preventTrigger = true
            boldAction.checked = documentHandler.bold
            boldAction.preventTrigger = false

            strikeAction.preventTrigger = true
            strikeAction.checked = documentHandler.strikeout
            strikeAction.preventTrigger = false

            underlineAction.preventTrigger = true
            underlineAction.checked = documentHandler.underline
            underlineAction.preventTrigger = false
        }
    }

    //    Binding{
    //        target: cutAction
    //        property: "enabled"
    //        value: textArea.selectedText !== "" ? true : false
    //        when: textArea.activeFocus
    //        restoreMode: Binding.RestoreBindingOrValue
    //    }
    Component.onDestruction: {
        skrEditMenuSignalHub.unsubscribe(textArea.objectName)
    }

    function cut() {

        console.log("cut action text", textArea.selectedText)
        textArea.forceActiveFocus()
        textArea.cut()
        textContextMenu.close()
    }
    function copy() {

        console.log("copy action text", textArea.selectedText)
        textArea.forceActiveFocus()
        textArea.copy()
        textContextMenu.close()
    }
    function paste() {

        console.log("paste action text")
        textArea.forceActiveFocus()
        var originalLength = textArea.length
        clipboard.prepareAndSendLastClipboardText()
        var newLength = textArea.length
        textArea.deselect()
        //setCursorPosition(textArea.cursorPosition + (newLength - originalLength))
        //textArea.paste()
        textContextMenu.close()
    }

    function italic(checked) {

        console.log("italic action text")
        textArea.forceActiveFocus()

        documentHandler.italic = checked
    }

    function bold(checked) {

        console.log("bold action text", checked)
        textArea.forceActiveFocus()

        documentHandler.bold = checked
    }
    function strike(checked) {

        console.log("strike action text")
        textArea.forceActiveFocus()

        documentHandler.strikeout = checked
    }
    function underline(checked) {

        console.log("underline action text")
        textArea.forceActiveFocus()

        documentHandler.underline = checked
    }

    //menu :
    TextContextMenu {
        id: textContextMenu
        objectName: "editMenu"

        Component.onCompleted: {
            textContextMenu.suggestionChosen.connect(
                        documentHandler.replaceWord)
        }

        Connections {
            target: textContextMenu
            function onSuggestionToBeLearned(word) {
                skrData.projectDictHub().addWordToProjectDict(root.projectId,
                                                              word)
            }
        }
    }

    //    SkrMenu {

    //        SkrMenuItem{
    //            id: italicItem
    //            action: italicAction
    //            objectName: "italicItem"
    //        }

    //        SkrMenuItem {
    //            id: boldItem
    //            action: boldAction
    //            objectName: "boldItem"

    //        }

    //        SkrMenuItem {
    //            id: strikeItem
    //            action: strikeAction
    //            objectName: "strikeItem"

    //        }

    //        SkrMenuItem {
    //            id: underlineItem
    //            action: underlineAction
    //            objectName: "underlineItem"

    //        }

    //        MenuSeparator {}

    //        SkrMenuItem{
    //            id: cutItem
    //            action: cutAction
    //            objectName: "cutItem"
    //            enabled: textArea.selectedText !== ""
    //        }

    //        SkrMenuItem {
    //            id: copyItem
    //            action: copyAction
    //            objectName: "copyItem"
    //            enabled: textArea.selectedText !== ""

    //        }
    //        SkrMenuItem {
    //            id: pasteItem
    //            action: pasteAction
    //            objectName: "pasteItem"

    //        }
    //        MenuSeparator {}

    //    }

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

    //property int cursorPosition: textArea.cursorPosition
    function setCursorPosition(cursorPosition) {

        if (documentHandler.maxCursorPosition() >= cursorPosition) {
            textArea.cursorPosition = cursorPosition
            //console.log("textArea.cursorPosition =", cursorPosition)
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
    textArea.onCursorPositionChanged: {

        formatActionTimer.start()
    }
    Timer {
        id: formatActionTimer
        interval: 20
        onTriggered: {
            if (textArea.activeFocus) {

                italicAction.preventTrigger = true
                italicAction.checked = documentHandler.italic
                italicAction.preventTrigger = false

                boldAction.preventTrigger = true
                boldAction.checked = documentHandler.bold
                boldAction.preventTrigger = false

                strikeAction.preventTrigger = true
                strikeAction.checked = documentHandler.strikeout
                strikeAction.preventTrigger = false

                underlineAction.preventTrigger = true
                underlineAction.checked = documentHandler.underline
                underlineAction.preventTrigger = false
            }
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

    //-----------------------------------------------------------------------------
    property alias documentHandler: documentHandler
    property alias highlighter: documentHandler.highlighter

    DocumentHandler {
        id: documentHandler
        textDocument: textArea.textDocument
        cursorPosition: textArea.cursorPosition

        //        onCursorPositionChanged:
        //            root.cursorPosition = documentHandler.cursorPosition
        selectionStart: textArea.selectionStart
        selectionEnd: textArea.selectionEnd

        // needed because bindings are not working between DocumentHandler and TextContextMenu
        onSuggestionListChanged: {
            textContextMenu.suggestionList = suggestionList
        }
        onSuggestionOriginalWordChanged: {
            textContextMenu.suggestionOriginalWord = suggestionOriginalWord
        }

        Component.onCompleted: {

            if (!spellCheckerKilled) {
                // activate
                SkrSettings.spellCheckingSettings.onSpellCheckingActivationChanged.connect(
                            determineSpellCheckerActivation)
                determineSpellCheckerActivation()
//                paintUnderlineForSpellcheckCalled.connect(
//                            paintUnderlineForSpellcheck)

                //lang
                SkrSettings.spellCheckingSettings.onSpellCheckingLangCodeChanged.connect(
                            determineSpellCheckerLanguageCode)
                determineSpellCheckerLanguageCode()
            }
        }
        Component.onDestruction: {
            SkrSettings.spellCheckingSettings.onSpellCheckingActivationChanged.disconnect(
                        determineSpellCheckerActivation)
            SkrSettings.spellCheckingSettings.onSpellCheckingLangCodeChanged.disconnect(
                        determineSpellCheckerLanguageCode)
        }
    }

    Connections {
        target: skrData.projectDictHub()
        enabled: !spellCheckerKilled
        function onProjectDictWordAdded(projectId, newWord) {
            if (root.projectId === projectId) {
                highlighter.spellChecker.addWordToUserDict(newWord)
            }
        }
    }

    Connections {
        target: skrData.projectDictHub()
        enabled: !spellCheckerKilled
        function onProjectDictWordRemoved(projectId, removedWord) {
            if (root.projectId === projectId) {
                highlighter.spellChecker.removeWordFromUserDict(removedWord)
            }
        }
    }

    Connections {

        target: skrData.projectHub()
        enabled: !spellCheckerKilled
        function onLangCodeChanged(projectId, langCode) {
            if (root.projectId === projectId) {
                determineSpellCheckerLanguageCode(projectId)
            }
        }
    }

    function determineSpellCheckerActivation() {
        var value = SkrSettings.spellCheckingSettings.spellCheckingActivation
        highlighter.spellChecker.activate(
                    SkrSettings.spellCheckingSettings.spellCheckingActivation)
        highlighter.rehighlight()

        // needed to "shake" the highlighter
        textArea.enabled = false
        textArea.enabled = true
    }

    function determineSpellCheckerLanguageCode() {

        if (!highlighter) {
            console.log("no valid highlighter loaded")
            return
        }

        var langCode = ""

        //if project has a lang defined :
        if (projectId === -2) {
            // use default lang from settings
            langCode = SkrSettings.spellCheckingSettings.spellCheckingLangCode
        } else if (skrData.projectHub().getLangCode(projectId) !== "") {
            langCode = skrData.projectHub().getLangCode(projectId)
        } else {
            // use default lang from settings
            langCode = SkrSettings.spellCheckingSettings.spellCheckingLangCode
        }

        highlighter.spellChecker.langCode = langCode

        if (projectId !== -2) {
            setProjectDictInSpellChecker(projectId)
        }

        highlighter.rehighlight()
        //console.log("langCode :", langCode)
    }

    function setProjectDictInSpellChecker(projectId) {

        highlighter.spellChecker.clearUserDict()
        var projectDictList = skrData.projectDictHub().getProjectDictList(
                    projectId)
        highlighter.spellChecker.setUserDict(projectDictList)
    }

    Connections {
        target: documentHandler
        function onShakeTextSoHighlightsTakeEffectCalled() {
            if (!shakeTextTimer.running) {
                shakeTextTimer.start()
            }
        }
    }

    Timer {
        id: shakeTextTimer
        interval: 50
        onTriggered: {
            textArea.width += 1
            textArea.width -= 1
        }
    }

    //-----------------------------------------------------------------------------
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

        flickable.flick(0, deltaY * 50)

        //        for (var touch in touchPoints)
        //            console.log("Multitouch updated touch", touchPoints[touch].pointId,
        //                        "at", touchPoints[touch].x, ",", touchPoints[touch].y,
        //                        ",", touchPoints[touch].previousY, ",",
        //                        touchPoints[touch].startY)
    }

    //    leftScrollMouseArea.onPressAndHold: {

    //    }
    //    leftScrollMouseArea.onWheel: {

    //        var deltaY = wheel.angleDelta.y *10

    //        flickable.flick(0, deltaY)

    //        if (flickable.atYBeginning && wheel.angleDelta.y > 0) {
    //            flickable.returnToBounds()
    //            return
    //        }
    //        if (flickable.atYEnd && wheel.angleDelta.y < 0) {
    //            flickable.returnToBounds()
    //            return
    //        }
    //    }

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

    //    rightScrollMouseArea.onPressAndHold: {

    //    }
    //    rightScrollMouseArea.onWheel: {

    //        var deltaY = wheel.angleDelta.y *10

    //        flickable.flick(0, deltaY)

    //        if (flickable.atYBeginning && wheel.angleDelta.y > 0) {
    //            flickable.returnToBounds()
    //            return
    //        }
    //        if (flickable.atYEnd && wheel.angleDelta.y < 0) {
    //            flickable.returnToBounds()
    //            return
    //        }
    //    }

    // scrollView :

    //--------------------------------------------------------------------------------
    //--------Page Up/Down-------------------------------------------------------------
    //--------Text centering----------------------------------------------------------
    //--------------------------------------------------------------------------------
    textArea.viewHeight: flickable.height - textArea.topPadding - textArea.bottomPadding

    Connections {
        target: textArea
        function onMoveViewYCalled(height, animationEnabled) {
            var value = height - textArea.topPadding - textArea.bottomPadding

            contentYBehavior.enabled = animationEnabled
            //top bound
            if (flickable.contentY + value < 0) {
                flickable.contentY = 0
                contentYBehavior.enabled = true
                return
            }

            // bottom bound
            if (textCenteringEnabled && flickable.contentY + value + 20
                    > flickable.contentHeight + textArea.viewHeight / 2) {
                flickable.contentY = flickable.contentHeight
                        - textArea.viewHeight + textArea.viewHeight / 2
                contentYBehavior.enabled = true
                flickable.returnToBounds()
                contentYBehavior.enabled = true
                return
            } else if (!textCenteringEnabled
                       && flickable.contentY + value + 20 > flickable.contentHeight) {
                flickable.contentY = flickable.contentHeight - textArea.viewHeight
                contentYBehavior.enabled = true
                return
            }
            else if(flickable.contentHeight < textArea.height){
                return
            }



            // normal move
            flickable.contentY += value
            contentYBehavior.enabled = true
        }
    }

    textArea.textCenteringEnabled: textCenteringEnabled
    textArea.viewContentY: flickable.contentY

    Behavior on flickable.contentY {
        id: contentYBehavior
        enabled: SkrSettings.ePaperSettings.animationEnabled
        NumberAnimation {
            duration: 200
            easing.type: Easing.OutQuad
        }
    }

    // wheel :
    WheelHandler {
        id: leftWheelHandler
        target: leftScrollItem
        onWheel: {
            textArea.moveViewYCalled(-event.angleDelta.y / 2, false)
        }
    }

    WheelHandler {
        id: rightWheelHandler
        target: rightScrollItem
        onWheel: {
            textArea.moveViewYCalled(-event.angleDelta.y / 2, false)
        }
    }

    //--------------------------------------------------------------------------------

    //--------------------------------------------------------------
    //--------Highlighter---------------------------------------------
    //--------------------------------------------------------------
//    property rect visibleRect: Qt.rect(0, 0, 0, 0)
//    //--------------------------------------------------------------------------------
//    flickable.onContentYChanged: determineVisibleRect()

//    function determineVisibleRect() {
//        visibleRect = Qt.rect(flickable.contentX, flickable.contentY,
//                              flickable.contentWidth, flickable.contentHeight)
//    }

//    function paintUnderlineForSpellcheck(positionList, blockBegin, blockEnd, uniqueBlock) {
//        if (paintUnderlineForSpellcheckTimer.running) {
//            paintUnderlineForSpellcheckTimer.stop()
//        }

//        paintUnderlineForSpellcheckTimer.positionList = positionList
//        paintUnderlineForSpellcheckTimer.blockBegin = blockBegin
//        paintUnderlineForSpellcheckTimer.blockEnd = blockEnd
//        paintUnderlineForSpellcheckTimer.uniqueBlock = uniqueBlock
//        paintUnderlineForSpellcheckTimer.start()
//    }
//    Timer {
//        id: paintUnderlineForSpellcheckTimer
//        property var positionList
//        property int blockBegin
//        property int blockEnd
//        property bool uniqueBlock

//        interval: 20
//        onTriggered: {
//            determineVisibleRect()
//            canvas.positionList = positionList
//            canvas.blockBegin = blockBegin
//            canvas.blockEnd = blockEnd
//            canvas.uniqueBlock = uniqueBlock
//            canvas.requestPaint()
//        }
//    }

//    Connections {
//        target: SkrSettings.spellCheckingSettings
//        function onSpellCheckingActivationChanged() {
//            canvas.spellcheckEnabled = SkrSettings.spellCheckingSettings.spellCheckingActivation
//            if (canvas.available) {
//                canvas.requestPaint()
//            }
//        }
//    }

//    onVisibleRectChanged: {
//        canvas.spellcheckEnabled = SkrSettings.spellCheckingSettings.spellCheckingActivation
//        if (canvas.available) {
//            canvas.requestPaint()
//        }
//    }

//    Canvas {
//        id: canvas
//        parent: scrollView
//        anchors.fill: parent

//        //renderStrategy: Canvas.Immediate
//        //renderTarget: Canvas.FramebufferObject
//        property var positionList: []
//        property int blockBegin: -1
//        property int blockEnd: -1
//        property bool uniqueBlock: false
//        property bool spellcheckEnabled: true

//        onPaint: {

//            var pointList = []
//            var charWidthList = []
//            var visiblePositionList = []

//            //console.log(positionList)
//            for (var i = 0; i < positionList.length; i++) {
//                var position = positionList[i]
//                var rectangle = textArea.positionToRectangle(position)

//                if (rectangle.y + rectangle.height > visibleRect.y
//                        && rectangle.y + rectangle.height < visibleRect.y + visibleRect.height) {
//                    visiblePositionList.push(position)
//                }

//                //            if(rectangle.height < font.pointSize){
//                //               return
//            }
//            for (var j = 0; j < visiblePositionList.length; j++) {
//                var position2 = visiblePositionList[j]
//                var rectangle2 = textArea.positionToRectangle(position2)
//                var nextRectangle = textArea.positionToRectangle(position2 + 1)
//                pointList.push(
//                            Qt.point(
//                                rectangle2.x,
//                                rectangle2.y + rectangle2.height - visibleRect.y))
//                var charWidth = nextRectangle.x - rectangle2.x
//                if (charWidth < 0) {
//                    charWidthList.push(charWidthList[charWidthList.lenght - 1])
//                } else {
//                    charWidthList.push(charWidth)
//                }
//            }

//            if (uniqueBlock) {
//                var blockBeginRectangle = textArea.positionToRectangle(
//                            blockBegin)
//                var blockEndRectangle = textArea.positionToRectangle(blockEnd)
//                var blockRectangle = Qt.rect(
//                            0, blockBeginRectangle.y - visibleRect.y,
//                            canvas.width,
//                            blockEndRectangle.y - blockBeginRectangle.y
//                            + blockBeginRectangle.height)
//            }

//            var ctx = getContext("2d")
//            ctx.setLineDash([2, 4])
//            ctx.strokeStyle = SkrTheme.spellcheck
//            ctx.beginPath()

//            if (uniqueBlock) {
//                ctx.clearRect(blockRectangle.x, blockRectangle.y,
//                              blockRectangle.width, blockRectangle.height)
//            } else {
//                ctx.clearRect(0, 0, canvas.width, canvas.height)
//            }
//            if (spellcheckEnabled) {
//                for (var k = 0; k < pointList.length; k++) {
//                    var spellcheckPoint = pointList[k]

//                    ctx.moveTo(spellcheckPoint.x, spellcheckPoint.y)
//                    ctx.lineTo(spellcheckPoint.x + charWidthList[k],
//                               spellcheckPoint.y)
//                    ctx.moveTo(spellcheckPoint.x, spellcheckPoint.y + 1)
//                    ctx.lineTo(spellcheckPoint.x + charWidthList[k],
//                               spellcheckPoint.y + 1)
//                }

//                ctx.stroke()
//            }
//        }
//    }
    //focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            textArea.forceActiveFocus()
        } else {

        }
    }

    //----------------------------------------------------------------------------
    //-----Find Panel------------------------------------------------------------
    //----------------------------------------------------------------------------
    findPanel.documentHandler: documentHandler
    findPanel.highlighter: documentHandler.highlighter
    findPanel.textArea: textArea

    Action {
        id: findAction
        text: qsTr("Find")
        shortcut: "Ctrl+F"
        icon.source: "qrc:///icons/backup/edit-find.svg"
        onTriggered: {
            findPanel.visible = true
        }
    }
}
