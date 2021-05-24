import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."


OutlinePadForm {
    id: root
    property int projectId: -2
    property int treeItemId: -2
    property int milestone: -2


//---------------------------------------------------------
    Component.onCompleted: {
        openDocument(projectId, treeItemId, milestone)

        if(outlineWritingZone.text.length === 0){
            addOutlineToolButton.visible = true
            outlineWritingZone.Layout.preferredHeight = 0
        }


    }

//---------------------------------------------------------

    Action {
        id: openOutlineAction
        text: qsTr("Open outline")
        icon.source: "qrc:///icons/backup/quickopen-file.svg"
        onTriggered: {
             saveContent()
             saveCurrentCursorPositionAndY()
             rootWindow.viewManager.insertAdditionalProperty("isSecondary", true)
             rootWindow.viewManager.loadTreeItemAtAnotherView(projectId, treeItemId)
        }
    }
    openOutlineToolButton.action: openOutlineAction

//---------------------------------------------------------

    Action {
        id: addOutlineAction
        text: qsTr("Add outline")
        icon.source: "qrc:///icons/backup/list-add.svg"
        onTriggered: {
            outlineWritingZone.visible = true
            outlineWritingZone.forceActiveFocus()
            outlineWritingZone.Layout.preferredHeight = 400
        }
    }
    addOutlineToolButton.action: addOutlineAction

    //---------------------------------------------------------


    //---------------------------------------------------------

    function clearWritingZone(){
        if(treeItemId !== -2 && projectId !== -2 && milestone === -2){
            contentSaveTimer.stop()
            saveContent()
            saveCurrentCursorPositionAndYTimer.stop()
            saveCurrentCursorPositionAndY()
            var uniqueDocumentReference = projectId + "_" + treeItemId + "_secondary"
            skrTextBridge.unsubscribeTextDocument(uniqueDocumentReference, outlineWritingZone.textArea.objectName, outlineWritingZone.textArea.textDocument)

            root.projectId = -2
            root.treeItemId = -2
            outlineWritingZone.treeItemId = -2
            outlineWritingZone.projectId = -2
        }

        outlineWritingZone.setCursorPosition(0)
        outlineWritingZone.clear()
    }
    //---------------------------------------------------------

    function runActionsBeforeDestruction() {
        clearWritingZone()
    }

    Component.onDestruction: {
        runActionsBeforeDestruction()
    }

    //---------------------------------------------------------

    //---------------------------------------------------------
    // modifiable :

    property bool isModifiable: true

    Connections{
        target: skrData.treePropertyHub()
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

        root.isModifiable = skrData.treePropertyHub().getProperty(projectId, treeItemId, "modifiable", "true") === "true"

        if(!root.isModifiable !== outlineWritingZone.textArea.readOnly){
            saveCurrentCursorPositionAndY()
            outlineWritingZone.textArea.readOnly = !root.isModifiable
            restoreCurrentPaperCursorPositionAndY()
        }

    }
    //---------------------------------------------------------

    //---------------------------------------------------------

    QtObject {
        id: documentPrivate
        property bool contentSaveTimerAllowedToStart: true
        property bool saveCurrentCursorPositionAndYTimerAllowedToStart: true
    }
    //---------------------------------------------------------


    function openDocument(_projectId, _treeItemId, milestone) {
        // save current
        if(projectId !== _projectId || treeItemId !== _treeItemId ){ //meaning it hasn't just used the constructor
            clearWritingZone()
        }

        documentPrivate.contentSaveTimerAllowedToStart = false
        documentPrivate.saveCurrentCursorPositionAndYTimerAllowedToStart = false

        treeItemId = _treeItemId
        projectId = _projectId
        outlineWritingZone.treeItemId = _treeItemId
        outlineWritingZone.projectId = _projectId

        //console.log("opening sheet :", _projectId, _paperId)
        outlineWritingZone.setCursorPosition(0)

        if(milestone === -2){

                outlineWritingZone.text = skrData.treeHub().getSecondaryContent(_projectId, _treeItemId)


        }
        else {
            //TODO: if milestone
        }


        if(milestone === -2){
            var uniqueDocumentReference = projectId + "_" + treeItemId + "_secondary"
            skrTextBridge.subscribeTextDocument(uniqueDocumentReference,
                                                outlineWritingZone.textArea.objectName,
                                                outlineWritingZone.textArea.textDocument)
        }

        outlineWritingZone.documentHandler.indentEverywhere = SkrSettings.outlineSettings.textIndent
        outlineWritingZone.documentHandler.topMarginEverywhere = SkrSettings.outlineSettings.textTopMargin

        restoreCurrentPaperCursorPositionAndY()

//        forceActiveFocusTimer.start()

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

//    Timer{
//        id: forceActiveFocusTimer
//        repeat: false
//        interval: 100
//        onTriggered:  outlineWritingZone.forceActiveFocus()
//    }

    function restoreCurrentPaperCursorPositionAndY(){

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    projectId, "outlineTextPositionHash", treeItemId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId, "outlineTextYHash", treeItemId, 0)
        //console.log("newCursorPosition", position)

        // set positions :
        outlineWritingZone.setCursorPosition(position)

        writingZoneFlickableContentYTimer.y = visibleAreaY
        writingZoneFlickableContentYTimer.start()

    }

    Timer{

        property int y: 0
        id: writingZoneFlickableContentYTimer
        repeat: false
        interval: 50
        onTriggered: {
            outlineWritingZone.flickable.contentY = y
        }
    }


    function saveCurrentCursorPositionAndY(){

        if(treeItemId !== -2 || projectId !== -2){
            //save cursor position of current document :

            var previousCursorPosition = outlineWritingZone.textArea.cursorPosition
            //console.log("previousCursorPosition", previousCursorPosition)
            var previousY = outlineWritingZone.flickable.contentY
            //console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId, "outlineTextPositionHash", treeItemId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId,
                                                       "outlineTextYHash",
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



    // save content once after writing:
    outlineWritingZone.textArea.onTextChanged: {

        //avoid first text change, when blank HTML is inserted
        if(outlineWritingZone.textArea.length === 0
                && skrData.projectHub().isProjectNotModifiedOnce(projectId)){
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
            result = skrData.treeHub().setSecondaryContent(projectId, treeItemId, outlineWritingZone.text)


        if (!result.success){
            console.log("saving outline text failed", projectId, treeItemId)
        }
        else {
            //console.log("saving text success", projectId, treeItemId)

        }
    }

}
