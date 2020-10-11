import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15
import ".."
import eu.skribisto.documenthandler 1.0
import eu.skribisto.highlighter 1.0
import eu.skribisto.spellchecker 1.0
import eu.skribisto.projectdicthub 1.0

WritingZoneForm {
    id: root
    stretch: true
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


    property int paperId: -1
    property int projectId: -1

    function clear(){
        textArea.clear()
        paperId = -1
        projectId = -1
    }



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

    function determineTextCursorUnblinkingSetting(){
        if(SkrSettings.ePaperSettings.textCursorUnblinking){
            textArea.cursorDelegate = unblinkingCursorComponent
        }
        else {
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
        if(event.buttons === Qt.RightButton){


            // deselect if outside selection :
            var selectStart = textArea.selectionStart
            var selectEnd = textArea.selectionEnd
            var eventCursorPosition = textArea.positionAt(event.x, event.y)

            if(textArea.selectedText != "" & (eventCursorPosition < selectStart | eventCursorPosition > selectEnd)){
                textArea.cursorPosition = textArea.positionAt(event.x, event.y)
                menu.popup(textArea, event.x, event.y)
                console.log("deselect")
                return
            }
            else{
                menu.popup(textArea, event.x, event.y)
            }




        }
    }

    textArea.onActiveFocusChanged: {
        if(textArea.activeFocus){
            console.log("activeFocus = true")


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

    function cut(){

        console.log("cut action text", textArea.selectedText)
        textArea.forceActiveFocus()
        textArea.cut()

    }
    function copy(){

        console.log("copy action text", textArea.selectedText)
        textArea.forceActiveFocus()
        textArea.copy()

    }
    function paste(){

        console.log("paste action text")
        textArea.forceActiveFocus()
        textArea.paste()

    }

    function italic(checked){

        console.log("italic action text")
        textArea.forceActiveFocus()

        documentHandler.italic = checked

    }


    function bold(checked){

        console.log("bold action text", checked)
        textArea.forceActiveFocus()

        documentHandler.bold = checked
    }
    function strike(checked){

        console.log("strike action text")
        textArea.forceActiveFocus()

        documentHandler.strikeout = checked
    }
    function underline(checked){

        console.log("underline action text")
        textArea.forceActiveFocus()

        documentHandler.underline = checked
    }
    //menu :
    Menu {
        id: menu
        objectName: "editMenu"


        MenuItem{
            id: italicItem
            action: italicAction
            objectName: "italicItem"
        }

        MenuItem {
            id: boldItem
            action: boldAction
            objectName: "boldItem"

        }

        MenuItem {
            id: strikeItem
            action: strikeAction
            objectName: "strikeItem"


        }

        MenuItem {
            id: underlineItem
            action: underlineAction
            objectName: "underlineItem"


        }

        MenuSeparator {}

        MenuItem{
            id: cutItem
            action: cutAction
            objectName: "cutItem"
            enabled: textArea.selectedText !== ""
        }

        MenuItem {
            id: copyItem
            action: copyAction
            objectName: "copyItem"
            enabled: textArea.selectedText !== ""

        }
        MenuItem {
            id: pasteItem
            action: pasteAction
            objectName: "pasteItem"

        }
        MenuSeparator {}



        Component.onCompleted:{
            skrEditMenuSignalHub.subscribe(menu.objectName)
            skrEditMenuSignalHub.subscribe(italicItem.objectName)
            skrEditMenuSignalHub.subscribe(boldItem.objectName)
            skrEditMenuSignalHub.subscribe(strikeItem.objectName)
            skrEditMenuSignalHub.subscribe(underlineItem.objectName)
            skrEditMenuSignalHub.subscribe(cutItem.objectName)
            skrEditMenuSignalHub.subscribe(copyItem.objectName)
            skrEditMenuSignalHub.subscribe(pasteItem.objectName)


            determineTextCursorUnblinkingSetting()
        }
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

    //property int cursorPosition: textArea.cursorPosition


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

    textArea.onCursorPositionChanged: {
        if(textArea.activeFocus){

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

        Component.onCompleted: {

            // activate
            SkrSettings.spellCheckingSettings.onSpellCheckingActivationChanged.connect(
                   function() {
                       highlighter.spellChecker.activate(SkrSettings.spellCheckingSettings.spellCheckingActivation)
                       highlighter.rehighlight()
            })
            highlighter.spellChecker.active = SkrSettings.spellCheckingSettings.spellCheckingActivation

            //lang
            SkrSettings.spellCheckingSettings.onSpellCheckingLangCodeChanged.connect(determineSpellCheckerLanguageCode)
            determineSpellCheckerLanguageCode()


        }

    }

    Connections{
        target: plmData.projectDictHub()
        function onProjectDictWordAdded(projectId, newWord){
            if(root.projectId === projectId){
                highlighter.spellChecker.addWordToUserDict(newWord)
            }
        }
    }

    Connections{
        target: plmData.projectDictHub()
        function onProjectDictWordRemoved(projectId, removedWord){
            if(root.projectId === projectId){
                highlighter.spellChecker.removeWordFromUserDict(removedWord)
            }
        }
    }

    Connections{
        target: plmData.projectHub()
        function onLangCodeChanged(projectId, langCode){
            if(root.projectId === projectId){
                determineSpellCheckerLanguageCode(projectId)
            }
        }
    }


    function determineSpellCheckerLanguageCode(){
        var langCode  = ""

        //if project has a lang defined :
        if(plmData.projectHub().getLangCode(projectId) !== "" && projectId !== -1){
            langCode = plmData.projectHub().getLangCode(projectId)
        }
        else{ // use default lang from settings
            langCode = SkrSettings.spellCheckingSettings.spellCheckingLangCode
        }

        highlighter.spellChecker.langCode = langCode

        setProjectDictInSpellChecker(projectId)


        highlighter.rehighlight()
        console.log("langCode :", langCode)
    }

    function setProjectDictInSpellChecker(projectId){

        highlighter.spellChecker.clearUserDict()
        var projectDictList = plmData.projectDictHub().getProjectDictList(projectId)
        highlighter.spellChecker.setUserDict(projectDictList)
    }

    onProjectIdChanged: {
        determineSpellCheckerLanguageCode()
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
