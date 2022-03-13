import QtQuick
import QtQml
import QtQuick.Controls
import eu.skribisto.documenthandler 1.0
import "../.."

MinimapForm {
    id: root


    // The properties that define the scrollbar's state.
    // position and pageSize are in the range 0.0 - 1.0.  They are relative to the
    // height of the page, i.e. a pageSize of 0.5 means that you can see 50%
    // of the height of the view.
    // orientation can be either Qt.Vertical or Qt.Horizontal
    property int value: 0
    readonly property double position: priv.position
    property bool active: false

    property double sourceViewHeight
    property double sourceViewWidth
    onSourceViewWidthChanged: {
        adaptWidth()
    }

    property int sourceTextPointSize : 12
    onSourceTextPointSizeChanged: {
        textArea.font.pointSize = sourceTextPointSize
    }
    property string sourceTextFontFamily : ""
    onSourceTextFontFamilyChanged: {
        textArea.font.family = sourceTextFontFamily
    }
    property real sourceTextIndent: 0
    onSourceTextIndentChanged: {
        documentHandler.indentEverywhere = sourceTextIndent
    }
    property real sourceTextTopMargin: 0
    onSourceTextTopMarginChanged: {
        documentHandler.topMarginEverywhere = sourceTextTopMargin
    }

    readonly property double scaleValue: preferredWidth / sourceViewWidth

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

    property int milestone: -2
    property bool isSecondary: false

    //--------------------------------------------------------------------------------------------
    // size:

    property int preferredWidth: 200
    property int maximumWidth: 300
    property int minimumWidth: 50
    property int divider: 5
    onDividerChanged: {
        adaptWidth()
    }

    textArea.width: sourceViewWidth

    function adaptWidth(){

        var newValue = sourceViewWidth / divider

        if(newValue > maximumWidth){
            preferredWidth = maximumWidth
        }
        if(newValue < minimumWidth){
            preferredWidth = minimumWidth
        }
        else {
            preferredWidth = newValue
        }

        textArea.width = sourceViewWidth

    }


    textArea.transform: Scale {
        origin.x: 0
        origin.y: 0
        xScale: scaleValue
        yScale: scaleValue
    }
    //--------------------------------------------------------------------------------------------

    QtObject{
        id: priv
        property string objectName: "TextArea-" + Qt.formatDateTime(new Date(),
                                                                    "yyyyMMddhhmmsszzz")
        property double position: 0
    }

    //text:
    //required property var textDoc

    //implicitWidth: sourceWidth * scaleValue



    Component.onCompleted: {
        openDocument()
    }

    Component.onDestruction: {
        var uniqueDocumentReference = projectId + "_" + treeItemId + "_"
                + (isSecondary ? "secondary" : "primary")
        skrTextBridge.unsubscribeTextDocument(
                    uniqueDocumentReference,
                    priv.objectName,
                    textArea.textDocument)
    }

    wheelHandler.onWheel: function(event) {

        root.active = true
        if(deactivateTimer.running){
            deactivateTimer.stop()
        }
        deactivateTimer.start()


        var newValue = root.value - event.angleDelta.y * 4
        if(newValue <= - handle.height / 2){
            root.value = - handle.height / 2
        }
        else if(newValue >= textArea.height  - sourceViewHeight  / 2){
            root.value = textArea.height - sourceViewHeight / 2
        }
        else{
            root.value = newValue
        }

        handle.y = root.value

            priv.position = handle.y / (textArea.height - handle.height)

            if(textArea.height * scaleValue > minimapFlickable.height){
                minimapFlickable.contentY = (textArea.height * scaleValue - minimapFlickable.height) * priv.position
            }
            else{
                minimapFlickable.contentY = 0
            }



    }
    wheelHandler.grabPermissions: PointerHandler.TakeOverForbidden


    Timer{
        id: deactivateTimer
        interval: 100
        onTriggered: {
            root.active = false
        }
    }

    onValueChanged:{
        if(active || dragHandler.active){
            return
        }

        priv.position = handle.y / (textArea.height - handle.height)

        if(textArea.height * scaleValue > minimapFlickable.height){
            minimapFlickable.contentY = (textArea.height * scaleValue - minimapFlickable.height) * priv.position
        }
        else{
            minimapFlickable.contentY = 0
        }

    }


    Binding on value {
        when: active
        value: handle.y
        restoreMode: Binding.RestoreNone
//delayed: true
    }

    Binding on handle.y {
        when:  !active
        value: root.value
        restoreMode: Binding.RestoreNone
    }


    Behavior on handle.y{
        enabled: SkrSettings.ePaperSettings.animationEnabled && !active

            SpringAnimation {
                duration: 200
                spring: 2
                mass: 0.2
                damping: 0.2
            }



    }

    dragHandler.onActiveChanged: {
        if(dragHandler.active){
            root.active = true
        }
        else {
            deactivateTimer.start()
        }
    }

    dragHandler.grabPermissions: PointerHandler.TakeOverForbidden


    //property bool tapping: false

    tapHandler.onSingleTapped: function(eventPoint){

        root.active = true
        if(deactivateTimer.running){
            deactivateTimer.stop()
        }
        deactivateTimer.start()


//        tapping = true
        var newValue = eventPoint.position.y

        if(newValue < 0){
            handle.y = - handle.height / 2
        }
        else if(newValue >= textArea.height){
            handle.y =  textArea.height - handle.height / 2
        }
        else{
            handle.y = newValue - handle.height / 2
        }

        //tapping = false
//        tappingTimer.start()

    }


    tapHandler.grabPermissions: PointerHandler.TakeOverForbidden


//    Timer{
//        id: tappingTimer
//        interval: 100
//        onTriggered: {
//            tapping = false

//        }
//    }


    //--------------------------------------------------------------------
    //--------------------------------------------------------------------
    //--------------------------------------------------------------------

    property bool spellCheckerKilled: false
    property alias documentHandler: documentHandler
    property alias highlighter: documentHandler.highlighter

    DocumentHandler{
        id: documentHandler
        textDocument: textArea.textDocument
        cursorPosition: textArea.cursorPosition
        selectionStart: textArea.selectionStart
        selectionEnd: textArea.selectionEnd

        highlighter.isForMinimap: true
        highlighter.spellCheckHighlightColor: SkrTheme.minimapSpellCheckHighlight
        highlighter.findHighlightColor: SkrTheme.minimapFindHighlight
        highlighter.otherHighlightColor_1: SkrTheme.minimapOtherHighlight_1
        highlighter.otherHighlightColor_2: SkrTheme.minimapOtherHighlight_2
        highlighter.otherHighlightColor_3: SkrTheme.minimapOtherHighlight_3

        Component.onCompleted: {

            //lang
            SkrSettings.spellCheckingSettings.onSpellCheckingLangCodeChanged.connect(
                        determineSpellCheckerLanguageCode)
            determineSpellCheckerLanguageCode()
            // activate
            SkrSettings.spellCheckingSettings.onSpellCheckingActivationChanged.connect(
                        determineSpellCheckerActivation)
            determineSpellCheckerActivation()

            documentHandler.indentEverywhere = sourceTextIndent
            documentHandler.topMarginEverywhere = sourceTextTopMargin

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
        highlighter.spellChecker.activate(value)
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




    //---------------------------------------------------------
    function openDocument() {

        if (milestone === -2) {


            if (isSecondary) {
                textArea.text = skrRootItem.cleanUpHtml(
                            skrData.treeHub().getSecondaryContent(projectId,
                                                                  treeItemId))
            } else {
                textArea.text = skrRootItem.cleanUpHtml(
                            skrData.treeHub().getPrimaryContent(projectId,
                                                                treeItemId))
            }
        } else {

            //TODO: if milestone
        }

        if (milestone === -2) {
            var uniqueDocumentReference = projectId + "_" + treeItemId + "_"
                    + (isSecondary ? "secondary" : "primary")

            skrTextBridge.subscribeTextDocument(
                        uniqueDocumentReference,
                        priv.objectName,
                        textArea.textDocument)
        }


        writingZone.documentHandler.indentEverywhere = SkrSettings.textSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.textSettings.textTopMargin


    }








}
