import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15

import "../../Items"
import "../../Commons"
import "../.."

TextPageForm {
    id: root

    pageType: "TEXT"


    property string title: {return getTitle()}

    function getTitle(){
       var fetchedTitle =  plmData.treeHub().getTitle(projectId, treeItemId)

        if(isSecondary){
            return qsTr("Plan of %1").arg(fetchedTitle)
        }
        else {
            return fetchedTitle
        }




    }

    Connections {
        target: plmData.treeHub()
        function onTitleChanged(_projectId, _treeItemId, newTitle){
            if(projectId === _projectId && treeItemId === _treeItemId){
                title = getTitle()
            }

        }

    }

    titleLabel.text: title

    //--------------------------------------------------------

    additionalPropertiesForSavingView: {
        return {"isSecondary": isSecondary, "milestone": milestone}
    }

    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------

    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
        saveContent()
        saveCurrentCursorPositionAndY()
        skrWindowManager.insertAdditionalPropertyForViewManager("isSecondary", isSecondary)
        skrWindowManager.addWindowForItemId(projectId, treeItemId)
        rootWindow.setNavigationTreeItemIdCalled(projectId, treeItemId)
    }

    viewButtons.onSplitCalled: function(position){
        saveContent()
        saveCurrentCursorPositionAndY()
        viewManager.insertAdditionalProperty("isSecondary", isSecondary)
        viewManager.loadTreeItemAt(projectId, treeItemId, position)
    }



    //--------------------------------------------------------
    //---Tool bar-----------------------------------------
    //--------------------------------------------------------

    Connections{
        target: plmData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value){
            if(projectId === root.projectId && treeItemId === root.treeItemId){

                if(name === "word_count"){
                    countPriv.wordCount = value
                    updateCountLabel()
                }
            }
        }
    }

    Connections{
        target: plmData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value){
            if(projectId === root.projectId && treeItemId === root.treeItemId){

                if(name === "char_count"){
                    countPriv.characterCount = value
                    updateCountLabel()

                }
            }
        }
    }

    QtObject{
        id: countPriv
        property string wordCount: ""
        property string characterCount: ""
    }

    function updateCountLabel(){
        var wordCountString = skrRootItem.toLocaleIntString(countPriv.wordCount)
        var characterCountString = skrRootItem.toLocaleIntString(countPriv.characterCount)

        countLabel.text = qsTr("%1 words, %2 characters").arg(wordCountString).arg(characterCountString)
    }

    //--------------------------------------------------------
    //---Writing Zone-----------------------------------------
    //--------------------------------------------------------

    property bool isSecondary: false
    property int milestone: -2

    writingZone.maximumTextAreaWidth: SkrSettings.textSettings.textWidth
    writingZone.textPointSize: SkrSettings.textSettings.textPointSize
    writingZone.textFontFamily: SkrSettings.textSettings.textFontFamily
    writingZone.textIndent: SkrSettings.textSettings.textIndent
    writingZone.textTopMargin: SkrSettings.textSettings.textTopMargin

    writingZone.stretch: root.width < 300 ? true : viewManager.rootWindow.compactMode

    previousWritingZone.maximumTextAreaWidth: SkrSettings.textSettings.textWidth
    previousWritingZone.textPointSize: SkrSettings.textSettings.textPointSize
    previousWritingZone.textFontFamily: SkrSettings.textSettings.textFontFamily
    previousWritingZone.textIndent: SkrSettings.textSettings.textIndent
    previousWritingZone.textTopMargin: SkrSettings.textSettings.textTopMargin

    previousWritingZone.stretch: root.width < 300 ? true : viewManager.rootWindow.compactMode
    // focus
    Connections {
        enabled: viewManager.focusedPosition === position
        target: viewManager.rootWindow
        function onForceFocusOnEscapePressed(){
            writingZone.forceActiveFocus()
        }
    }


    //---------------------------------------------------------

    Component.onCompleted: {
        openDocument(projectId, treeItemId, isSecondary, milestone)
    }

    //---------------------------------------------------------

    function clearWritingZone(){
        if(root.treeItemId !== -2 && root.projectId !== -2 && milestone === -2){
            contentSaveTimer.stop()
            saveContent()
            saveCurrentCursorPositionAndYTimer.stop()
            saveCurrentCursorPositionAndY()
            var uniqueDocumentReference = projectId + "_" + treeItemId + "_" + (isSecondary ? "secondary" : "primary")
            skrTextBridge.unsubscribeTextDocument(uniqueDocumentReference, writingZone.textArea.objectName, writingZone.textArea.textDocument)

            root.projectId = -2
            root.treeItemId = -2
        }

        writingZone.setCursorPosition(0)
        writingZone.clear()
    }
    //---------------------------------------------------------

    function runActionsBeforeDestruction() {
        clearWritingZone()
    }

    Component.onDestruction: {
        runActionsBeforeDestruction()
    }

    //---------------------------------------------------------
    // modifiable :

    property bool isModifiable: true

    Connections{
        target: plmData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value){
            if(projectId === root.projectId && treeItemId === root.treeItemId){

                if(name === "modifiable"){
                    determineModifiable()
                }
            }
        }
    }

    Timer{
        id: determineModifiableTimer
        repeat: false
        interval: 200
        onTriggered: {
            determineModifiable()
        }
    }




    function determineModifiable(){

        root.isModifiable = plmData.treePropertyHub().getProperty(projectId, treeItemId, "modifiable", "true") === "true"

        if(!root.isModifiable !== writingZone.textArea.readOnly){
            saveCurrentCursorPositionAndY()
            writingZone.textArea.readOnly = !root.isModifiable
            restoreCurrentPaperCursorPositionAndY()
        }

    }



    //--------------------------------------------------------
    //--- stretching if too small-----------------------------------------
    //--------------------------------------------------------

    QtObject {
        id: priv
        property int tempWritingZoneWidth: 0
    }

    onWidthChanged: {
        if(!rootWindow.compactMode && !writingZone.stretch && root.width  < 450){
            writingZone.stretch = true
            priv.tempWritingZoneWidth = root.width
        }
        if(!rootWindow.compactMode && writingZone.stretch && root.width > priv.tempWritingZoneWidth + 50){
            writingZone.stretch = false
        }
    }

    //--------------------------------------------------------
    //---Left Scroll Area-----------------------------------------
    //--------------------------------------------------------



    leftPaneScrollTouchArea.onUpdated: {
        var deltaY = touchPoints[0].y - touchPoints[0].previousY
        //        console.log("deltaY :", deltaY)

        if (writingZone.flickable.atYBeginning && deltaY > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && deltaY < 0) {
            writingZone.flickable.returnToBounds()
            return
        }

        writingZone.flickable.flick(0, deltaY * 50)

    }

    leftPaneScrollMouseArea.onWheel: {

        var deltaY = wheel.angleDelta.y *10

        writingZone.flickable.flick(0, deltaY)

        if (writingZone.flickable.atYBeginning && wheel.angleDelta.y > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && wheel.angleDelta.y < 0) {
            writingZone.flickable.returnToBounds()
            return
        }
    }

    //--------------------------------------------------------
    //---Right Scroll Area-----------------------------------------
    //--------------------------------------------------------




    rightPaneScrollTouchArea.onUpdated: {
        var deltaY = touchPoints[0].y - touchPoints[0].previousY
        //        console.log("deltaY :", deltaY)

        if (writingZone.flickable.atYBeginning && deltaY > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && deltaY < 0) {
            writingZone.flickable.returnToBounds()
            return
        }

        writingZone.flickable.flick(0, deltaY * 50)

    }

    rightPaneScrollMouseArea.onWheel: {

        var deltaY = wheel.angleDelta.y *10

        writingZone.flickable.flick(0, deltaY)

        if (writingZone.flickable.atYBeginning && wheel.angleDelta.y > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && wheel.angleDelta.y < 0) {
            writingZone.flickable.returnToBounds()
            return
        }
    }

    //---------------------------------------------------------
    //------Actions----------------------------------------
    //---------------------------------------------------------




    Connections {
        target: italicAction
        function onTriggered() {closeRightDrawer()}
    }
    Connections {
        target: boldAction
        function onTriggered() {closeRightDrawer()}
    }
    Connections {
        target: strikeAction
        function onTriggered() {closeRightDrawer()}
    }
    Connections {
        target: underlineAction
        function onTriggered() {closeRightDrawer()}
    }


    function closeRightDrawer(){
        if(viewManager.rootWindow.compactMode){
            viewManager.rootWindow.closeRightDrawerCalled()
        }
    }
    //---------------------------------------------------------

    QtObject {
        id: documentPrivate
        property bool contentSaveTimerAllowedToStart: true
        property bool saveCurrentCursorPositionAndYTimerAllowedToStart: true
    }
    //---------------------------------------------------------


    function openDocument(_projectId, _treeItemId, isSecondary, milestone) {
        // save current
        if(projectId !== _projectId && treeItemId !== _treeItemId ){ //meaning it hasn't just used the constructor
            clearWritingZone()
        }

        documentPrivate.contentSaveTimerAllowedToStart = false
        documentPrivate.saveCurrentCursorPositionAndYTimerAllowedToStart = false

        treeItemId = _treeItemId
        projectId = _projectId
        writingZone.treeItemId = _treeItemId
        writingZone.projectId = _projectId

        //console.log("opening sheet :", _projectId, _paperId)
        writingZone.setCursorPosition(0)

        if(milestone === -2){

            if(isSecondary){
                writingZone.text = skrRootItem.cleanUpHtml(plmData.treeHub().getSecondaryContent(_projectId, _treeItemId))
            }
            else {
                writingZone.text = skrRootItem.cleanUpHtml(plmData.treeHub().getPrimaryContent(_projectId, _treeItemId))
            }

        }
        else {
            //TODO: if milestone
        }


        title = getTitle()

        if(milestone === -2){
            var uniqueDocumentReference = projectId + "_" + treeItemId + "_" + (isSecondary ? "secondary" : "primary")
            skrTextBridge.subscribeTextDocument(uniqueDocumentReference,
                                                writingZone.textArea.objectName,
                                                writingZone.textArea.textDocument)
        }

        writingZone.documentHandler.indentEverywhere = SkrSettings.textSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.textSettings.textTopMargin

        restoreCurrentPaperCursorPositionAndY()

        forceActiveFocusTimer.start()

        // start the timer for automatic position saving
        if(milestone === -2){
            documentPrivate.saveCurrentCursorPositionAndYTimerAllowedToStart = true
            if(!saveCurrentCursorPositionAndYTimer.running){
                saveCurrentCursorPositionAndYTimer.start()
            }

            documentPrivate.contentSaveTimerAllowedToStart = true
        }

        //        leftDock.setCurrentPaperId(projectId, paperId)
        //        leftDock.setOpenedPaperId(projectId, paperId)

        determineModifiableTimer.start()

    }

    Timer{
        id: forceActiveFocusTimer
        repeat: false
        interval: 100
        onTriggered:  writingZone.forceActiveFocus()
    }

    function restoreCurrentPaperCursorPositionAndY(){

        var positionKey
        var yKey

        if(isSecondary){
            positionKey = "outlineTextPositionHash"
            yKey = "outlineTextYHash"
        }
        else {
            positionKey = "textPositionHash"
            yKey = "textYHash"
        }

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    projectId, positionKey, treeItemId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId, yKey, treeItemId, 0)
        //console.log("restoredCursorPosition", position)

        // set positions :
        writingZone.setCursorPosition(position)

        writingZoneFlickableContentYTimer.y = visibleAreaY
        writingZoneFlickableContentYTimer.start()

    }

    Timer{

        property int y: 0
        id: writingZoneFlickableContentYTimer
        repeat: false
        interval: 50
        onTriggered: {
            writingZone.flickable.contentY = y
        }
    }


    function saveCurrentCursorPositionAndY(){

        if(root.treeItemId !== -2 || root.projectId !== -2){
            var positionKey
            var yKey

            if(isSecondary){
                positionKey = "outlineTextPositionHash"
                yKey = "outlineTextYHash"
            }
            else {
                positionKey = "textPositionHash"
                yKey = "textYHash"
            }

            //save cursor position of current document :

            var previousCursorPosition = writingZone.textArea.cursorPosition
            //console.log("savedCursorPosition", previousCursorPosition)
            var previousY = writingZone.flickable.contentY
            //console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId, positionKey, treeItemId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId,
                                                       yKey,
                                                       treeItemId,
                                                       previousY)
        }
    }

    Timer{
        id: saveCurrentCursorPositionAndYTimer
        repeat: true
        interval: 10000
        onTriggered: saveCurrentCursorPositionAndY()
    }

    //needed to adapt width to a shrinking window
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactMode && middleBase.width - 40 < writingZone.maximumTextAreaWidth
        value: middleBase.width - 40
        restoreMode: Binding.RestoreBindingOrValue

    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactMode && middleBase.width - 40 >= writingZone.maximumTextAreaWidth
        value: writingZone.maximumTextAreaWidth
        restoreMode: Binding.RestoreBindingOrValue

    }

    //needed to adapt width to a shrinking window
    Binding on previousWritingZone.textAreaWidth {
        when: !Globals.compactMode && middleBase.width - 40 < previousWritingZone.maximumTextAreaWidth
        value: previousWritingZone.width - 40
        restoreMode: Binding.RestoreBindingOrValue

    }
    Binding on previousWritingZone.textAreaWidth {
        when: !Globals.compactMode && middleBase.width - 40 >= previousWritingZone.maximumTextAreaWidth
        value: previousWritingZone.maximumTextAreaWidth
        restoreMode: Binding.RestoreBindingOrValue

    }
    // save content once after writing:
    writingZone.textArea.onTextChanged: {

        //avoid first text change, when blank HTML is inserted
        if(writingZone.textArea.length === 0
                && plmData.projectHub().isProjectNotModifiedOnce(projectId)){
            return
        }

        if(contentSaveTimer.running){
            contentSaveTimer.stop()
        }
        if(documentPrivate.contentSaveTimerAllowedToStart){
            contentSaveTimer.start()
        }
    }
    Timer{
        id: contentSaveTimer
        repeat: false
        interval: 200
        onTriggered: saveContent()
    }

    function saveContent(){
        //console.log("saving text")
        var result

        var text = skrRootItem.cleanUpHtml(writingZone.text)


        if(isSecondary){
            result = plmData.treeHub().setSecondaryContent(projectId, treeItemId, text)
        }
        else {
            result = plmData.treeHub().setPrimaryContent(projectId, treeItemId, text)
            if(!contentSaveTimer.running)
                skrTreeManager.updateCharAndWordCount(projectId, treeItemId, root.pageType, true)

        }

        if (!result.success){
            console.log("saving text failed", projectId, treeItemId)
        }
        else {
            //console.log("saving text success", projectId, treeItemId)

        }
    }

    //------------------------------------------------------------------------
    //-----minimap------------------------------------------------------------
    //------------------------------------------------------------------------

    property bool minimapVisibility: false
    minimap.visible: minimapVisibility

    Binding on minimap.text {
        when: minimapVisibility
        value: writingZone.textArea.text
        restoreMode: Binding.RestoreBindingOrValue
        delayed: true
    }

    //Scrolling minimap
    Binding on writingZone.internalScrollBar.position {
        when: minimapVisibility
        value: minimap.position
        restoreMode: Binding.RestoreBindingOrValue
        delayed: true
    }
    Binding on  minimap.position {
        when: minimapVisibility
        value: writingZone.internalScrollBar.position
        restoreMode: Binding.RestoreBindingOrValue
        delayed: true
    }

    // ScrollBar invisible while minimap is on
    writingZone.scrollBarVerticalPolicy: minimapVisibility ? ScrollBar.AlwaysOff : ScrollBar.AsNeeded

    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            writingZone.forceActiveFocus()
        }
    }





    //------------------------------------------------------------------------
    //-----tool boxes------------------------------------------------------------
    //-----------------------------------------------------------------------



    toolBoxes: [editViewComponent, propertyPadComponent, outlinePadComponent, tagPadComponent]

    Component {
        id: editViewComponent

        SkrToolBox{

            showButtonText: qsTr( "Show edit tool box")
            iconSource: "qrc:///icons/backup/format-text-italic.svg"
            EditView{
                id: editView
                anchors.fill: parent
                skrSettingsGroup: SkrSettings.textSettings
            }
        }
    }

    Component {
        id: propertyPadComponent

        SkrToolBox{

            showButtonText: qsTr( "Show properties tool box")
            iconSource: "qrc:///icons/backup/configure.svg"
            PropertyPad{
                id: propertyPad
                anchors.fill: parent

                projectId: root.projectId
                treeItemId: root.treeItemId
            }
        }
    }

    Component {
        id: outlinePadComponent

        SkrToolBox{

            showButtonText: qsTr( "Show outline tool box")
            iconSource: "qrc:///icons/backup/story-editor.svg"
            OutlinePad{
                id: outlinePad
                anchors.fill: parent

                projectId: root.projectId
                treeItemId: root.treeItemId


            }

        }
    }

    Component {
        id: tagPadComponent

        SkrToolBox{

            showButtonText: qsTr( "Show tags tool box")
            iconSource: "qrc:///icons/backup/tag.svg"
            TagPad{
                id: tagPad
                anchors.fill: parent

                projectId: root.projectId
                treeItemId: root.treeItemId


            }

        }
    }
}
