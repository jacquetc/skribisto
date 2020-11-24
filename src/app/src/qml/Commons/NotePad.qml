import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.usersettings 1.0
import eu.skribisto.propertyhub 1.0
import "../Commons"
import "../Items"
import ".."

NotePadForm {
    id: root

    property int projectId: -2
    property int sheetId: -2
    property int currentNoteId: -2
    property string pageType: "note"

    onProjectIdChanged: {
        populateNoteListModel()
    }
    onSheetIdChanged: {
        populateNoteListModel()
    }

    function openSynopsis(){
        openSynopsisTimer.start()
    }

    Timer{
        id: openSynopsisTimer
        repeat: false
        interval: 0
        onTriggered:  openSynopsisAction.trigger()

    }

    //-----------------------------------------------------------------------------


    property int focusedIndex: -2
    property var selectedList: []
    signal selectedListModified(var list)
    signal escapeKeyPressed()

    function determineWhichItemIsSelected() {

        var count = noteFlow.children.length - 1
        var i
        for (i = 0 ; i < count ; i++ ){
            noteFlow.children[i].determineIfSelected()
        }

        selectedListModified(selectedList)

    }


    // options :
    property bool selectionEnabled: false
    property bool multipleSelectionsEnabled: false

    function clearSelection(){
        selectedList = []
        determineWhichItemIsSelected()
    }

    noteFlowFocusScope.onActiveFocusChanged: {
        if(!noteFlowFocusScope.activeFocus){
            focusedIndex = -2
        }
    }
    //-----------------------------------------------------------------------------

    ListModel {
        id: noteListModel
    }
    DelegateModel {
        id: visualModel
        model: noteListModel
        delegate: noteFlowComponent
    }
    noteRepeater.model: visualModel

    // force focus on first child
    noteFlow.activeFocusOnTab: true
    noteFlow.onActiveFocusChanged: {
        if(noteFlow.children.length > 1){ // means there is no children
            var first = noteFlow.children[0]
            first.forceActiveFocus()
            first.setFocused()
            return
        }

    }

    Component {
        id: noteFlowComponent
        Rectangle {
            id: itemBase
            width: childrenRect.width + 10
            height: childrenRect.height + 10
            color: isOpened && !minimalMode? SkrTheme.accent : "lightskyblue"
            border.color: isSelected ? SkrTheme.accent : "lightskyblue"
            border.width: 2
            radius : height / 2
            property int projectId: model.itemProjectId
            property int sheetId: model.itemSheetId
            property int noteId: model.itemNoteId
            property string noteTitle:  model.title
            property int isSynopsis: -2
            property bool isSelected: false
            property bool isFocused: root.focusedIndex === model.index
            property bool isOpened: false



            focus: true


            function setFocused(){
                root.focusedIndex = model.index
            }

            function setSelected(){
                if(!multipleSelectionsEnabled && selectionEnabled){
                    root.selectedList = []
                }

                if(selectionEnabled){

                    var here = false
                    for(var i = 0; i < root.selectedList.length ; i ++){

                        if(root.selectedList[i] === itemBase.itemNoteId){
                            here = true
                        }
                    }

                    if(!here){
                        root.selectedList.push(itemBase.itemNoteId)
                        //console.log(" root.selectedList",  root.selectedList)
                        root.determineWhichItemIsSelected()
                    }
                }

            }

            function setDeselected(){


                var index = root.selectedList.indexOf(itemBase.itemNoteId)
                if(index === -1){
                    return
                }

                root.selectedList.splice(index, 1)
                root.determineWhichItemIsSelected()


            }


            function toggleSelected(){
                if(itemBase.isSelected){
                    itemBase.setDeselected()

                }
                else{
                    itemBase.setSelected()
                }

            }


            function determineIfSelected(){

                var here = false
                for(var i = 0; i < root.selectedList.length ; i++){

                    if(root.selectedList[i] === itemBase.itemNoteId){
                        here = true
                    }
                }

                itemBase.isSelected = here
            }

            TapHandler {
                id: tapHandler
                acceptedButtons: Qt.LeftButton
                onSingleTapped: {
                    //reset other notes :
                    var i;
                    for(i = 0; i < noteRepeater.count; i++) {
                        noteRepeater.itemAt(i).isOpened = false
                    }

                    itemBase.setFocused()
                    itemBase.toggleSelected()
                    itemBase.forceActiveFocus()

                    itemBase.isOpened = true
                    //open in noteTextArea
                    openDocument(projectId, noteId)

                    eventPoint.accepted = true

                }
                onDoubleTapped: {
                    //reset other notes :
                    var i;
                    for(i = 0; i < noteRepeater.count; i++) {
                        noteRepeater.itemAt(i).isOpened = false
                    }

                    itemBase.setFocused()
                    itemBase.toggleSelected()
                    itemBase.forceActiveFocus()

                    itemBase.isOpened = true
                    Globals.openNoteInNewTabCalled(itemBase.projectId, itemBase.noteId)

                    eventPoint.accepted = true

                }

                onLongPressed: {

                    itemBase.setFocused()
                    itemBase.forceActiveFocus()


                    if(rightClickMenu.visible){
                        rightClickMenu.close()
                        eventPoint.accepted = true
                        return
                    }

                    rightClickMenu.popup(itemBase, 0, itemBase.height)
                    eventPoint.accepted = true
                }
            }


            TapHandler {
                id: rightClickHandler
                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                acceptedButtons: Qt.RightButton
                onSingleTapped: {
                    var i;
                    for(i = 0; i < noteRepeater.count; i++) {
                        noteRepeater.itemAt(i).isSelected = false
                    }

                    itemBase.setFocused()
                    itemBase.forceActiveFocus()

                    if(rightClickMenu.visible){
                        rightClickMenu.close()
                        eventPoint.accepted = true
                        return
                    }

                    rightClickMenu.popup()

                    eventPoint.accepted = true

                }
            }

            SkrMenu{
                id: rightClickMenu

                SkrMenuItem {
                    id: renameMenuItem
                    text: qsTr("Rename")

                    onTriggered: {

                    }
                }

                SkrMenuItem {
                    id: addNoteMenuItem
                    text: qsTr("Add")
                    action: addNoteAction

                }

                SkrMenuItem {
                    id: dissociateNoteMenu
                    text: qsTr("Dissociate")

                    onTriggered: {
                        plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, model.itemNoteId)
                    }
                }
                SkrMenuItem {
                    id: moveNoteToTrashMenuItem
                    text: qsTr("Send to trash")

                    onTriggered: {
                        moveToTrashOrNotDialog.projectId = projectId
                        moveToTrashOrNotDialog.noteId = model.itemNoteId
                        moveToTrashOrNotDialog.sheetId = itemBase.sheetId
                        moveToTrashOrNotDialog.noteName = model.title
                        moveToTrashOrNotDialog.open()
                        moveToTrashOrNotDialog.forceActiveFocus()
                    }
                }

            }


            SimpleDialog {
                property int projectId: -2
                property int noteId: -2
                property int sheetId: -2
                property string noteName: ""

                id: moveToTrashOrNotDialog
                title: qsTr("Warning")
                text: qsTr("Do you want to move the note \"%1\" to the trash ?").arg(noteName)
                standardButtons: Dialog.Yes  | Dialog.Cancel

                onRejected: {

                }

                onAccepted: {

                    plmData.noteHub().setTrashedWithChildren(projectId, noteId, true)
                    plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, noteId)

                }
            }

            TapHandler {
                id: shiftTapHandler
                acceptedModifiers: Qt.ShiftModifier
                onSingleTapped: {
                    //reset other notes :
                    var i;
                    for(i = 0; i < noteRepeater.count; i++) {
                        noteRepeater.itemAt(i).isOpened = false
                    }
                    itemBase.setFocused()
                    itemBase.isOpened = true
                    Globals.openNoteInNewTabCalled(itemBase.projectId, itemBase.noteId)
                }
            }

            Keys.onShortcutOverride: {
                if( event.key === Qt.Key_Escape){
                    event.accepted = true
                }
            }


            Keys.onPressed: {
                if ((event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_Delete){
                    console.log("Shift delete key pressed ")
                    // move the note to trash

                    moveToTrashOrNotDialog.projectId = projectId
                    moveToTrashOrNotDialog.noteId = itemBase.noteId
                    moveToTrashOrNotDialog.sheetId = itemBase.sheetId
                    moveToTrashOrNotDialog.noteName = itemBase.noteTitle
                    moveToTrashOrNotDialog.open()
                    moveToTrashOrNotDialog.forceActiveFocus()
                    //TODO: fix that and Del Shortcut from Navigation


                }
                else if (event.key === Qt.Key_Delete){
                    console.log("Delete key pressed: dissociate ")
                    // dissociate
                    plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, model.itemNoteId)

                }

                if (event.key === Qt.Key_Space){
                    itemBase.toggleSelected()
                }

                if (event.key === Qt.Key_Escape){
                    escapeKeyPressed()
                }

                if (event.key === Qt.Key_Right || event.key === Qt.Key_Down ){

                    if(model.index === noteRepeater.count - 1){
                        noteRepeater.itemAt(0).forceActiveFocus()
                        noteRepeater.itemAt(0).setFocused()

                    }
                    else{
                        noteRepeater.itemAt(model.index + 1).forceActiveFocus()
                        noteRepeater.itemAt(model.index + 1).setFocused()
                    }

                }
                if (event.key === Qt.Key_Left || event.key === Qt.Key_Up ){

                    if(model.index === 0){
                        noteRepeater.itemAt(noteRepeater.count - 1).forceActiveFocus()
                        noteRepeater.itemAt(noteRepeater.count - 1).setFocused()
                    }
                    else{
                        noteRepeater.itemAt(model.index - 1).forceActiveFocus()
                        noteRepeater.itemAt(model.index - 1).setFocused()
                    }

                }
            }

            Accessible.role: Accessible.ListItem
            Accessible.name: model.title
            Accessible.description: qsTr("note related to the current sheet")

            RowLayout{
                id: noteLayout
                anchors.left: parent.left
                anchors.top: parent.top

                anchors.margins : 5

                SkrLabel{
                    id: noteTitle
                    text: model.title
                    font.bold: itemBase.isFocused

                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignHCenter
                    Layout.minimumWidth: 20
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    color: SkrTheme.buttonForeground


                    SkrRoundButton {
                        id: removeRelationshipButton
                        width: 0
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        anchors.right: parent.right
                        padding: 0
                        topInset: 1
                        bottomInset: 1
                        leftInset: 1
                        rightInset: 1
                        opacity: 0
                        icon.source: "qrc:///icons/backup/list-remove.svg"
                        onReleased:{
                            plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, model.itemNoteId)
                        }

                        activeFocusOnTab: false
                        focusPolicy: Qt.NoFocus

                    }
                }


                HoverHandler {
                    id: hoverHandler

                    onHoveredChanged: {
                        if(hovered){
                            showRemoveRelationshipButtonTimer.start()
                        }
                    }

                }

                Timer{
                    id: showRemoveRelationshipButtonTimer
                    repeat: false
                    interval: 1000

                }

                state: hoverHandler.hovered && !showRemoveRelationshipButtonTimer.running ? "visible_removeRelationshipButton": ""

                states:[
                    State {

                        name: "visible_removeRelationshipButton"
                        PropertyChanges { target: removeRelationshipButton; width: noteTitle.height}
                        PropertyChanges { target: removeRelationshipButton; opacity: 1.0}
                    }
                ]

                transitions: [
                    Transition {
                        from: ""
                        to: "visible_removeRelationshipButton"
                        reversible: true
                        ParallelAnimation {
                            NumberAnimation {target: removeRelationshipButton; property: "opacity";duration: 250; easing.type: Easing.OutCubic }
                            NumberAnimation {target: removeRelationshipButton; property: "width";duration: 250; easing.type: Easing.OutCubic }
                        }
                    }
                ]
            }
        }
    }

    function populateNoteListModel(){
        if(projectId === -2 || sheetId === -2){
            noteListModel.clear()
            return
        }

        noteListModel.clear()

        var noteList = plmData.noteHub().getNotesFromSheetId(projectId, sheetId)
        var synopsisId = plmData.noteHub().getSynopsisNoteId(projectId, sheetId)

        var i;
        for (i = 0; i < noteList.length ; i++){
            var noteId = noteList[i]

            var title = plmData.noteHub().getTitle(projectId, noteId)


            // ignore synopsis
            if (synopsisId === noteId){
                continue
            }

            noteListModel.append({title: title, itemProjectId: projectId, itemSheetId: sheetId, itemNoteId: noteId})
        }

    }

    //--------------------------------------------
    //---------- note writing zone------------------------
    //--------------------------------------------

    noteWritingZone.textPointSize: SkrSettings.notePadSettings.textPointSize
    noteWritingZone.textFontFamily: SkrSettings.notePadSettings.textFontFamily
    noteWritingZone.textIndent: SkrSettings.notePadSettings.textIndent
    noteWritingZone.textTopMargin: SkrSettings.notePadSettings.textTopMargin
    // save content once after writing:
    noteWritingZone.textArea.onTextChanged: {

        if(projectId === -2 || sheetId === -2){
            return
        }

        if(minimalMode){
            return
        }

        // create if nothing opened
        if(currentNoteId === -2 && noteWritingZone.textArea.length === 1){

            //cancel if time too short between note creations
            if(notePadPrivateObject.creationProtectionEnabled){
                return
            }
            notePadPrivateObject.creationProtectionEnabled = true
            newOnTheFlyNoteCreationProtectionTimer.start()

            notePadPrivateObject.newText = noteWritingZone.text
            //console.log("notped :", notePadPrivateObject.newText)

            //create basic note
            var result = plmData.noteHub().addNoteRelatedToSheet(projectId, sheetId)
            if (!result.success){
                //TODO: add notification
                return
            }

            var noteId = result.getData("noteId", -2)

            // set title
            var title = titleTextField.text
            plmData.noteHub().setTitle(projectId, noteId, qsTr("New note"))



            noteIdToOpen = noteId
            openDocumentAfterClosingPopupTimer.start()
            newOnTheFlyNotePositionAndTextTimer.start()
        }


        // save content :
        if(contentSaveTimer.running){
            contentSaveTimer.stop()
        }

        if(notePadPrivateObject.contentSaveTimerEnabled){
            contentSaveTimer.start()
        }

    }

    Timer{
        id: newOnTheFlyNoteCreationProtectionTimer
        repeat: false
        interval: 500
        onTriggered: {
            notePadPrivateObject.creationProtectionEnabled = false
        }
    }

    QtObject{
        id: notePadPrivateObject
        property string newText: ""
        property bool creationProtectionEnabled: false
        property bool contentSaveTimerEnabled: true

    }

    Timer{
        id: newOnTheFlyNotePositionAndTextTimer
        repeat: false
        interval: 0
        onTriggered: {
            noteWritingZone.text = notePadPrivateObject.newText
            plmData.noteHub().setContent(projectId, noteIdToOpen, notePadPrivateObject.newText)
            noteWritingZone.textArea.cursorPosition = 1
        }
    }





    Timer{
        id: contentSaveTimer
        repeat: false
        interval: 1000
        onTriggered: saveContent()
    }

    function saveContent(){
        if(projectId !== -2 && currentNoteId !== -2 && sheetId !== -2 && !minimalMode){
            //console.log("saving note in notepad", "projectId", projectId,  "currentNoteId", currentNoteId, "sheetId", sheetId)
            plmData.noteHub().setContent(projectId, currentNoteId, noteWritingZone.text)
        }
    }


    //    noteWritingZone.onActiveFocusChanged: {
    //            noteWritingZone.text = plmData.sheetHub().getContent(projectId, currentNoteId)
    //            restoreCurrentPaperCursorPositionAndY()
    //}


    // project to be closed :
    Connections{
        target: plmData.projectHub()
        function onProjectToBeClosed(projectId) {

            if (projectId === root.projectId){
                // save
                clearNoteWritingZone()
            }
        }
    }

    function clearNoteWritingZone(){
        if(currentNoteId !== -2 && projectId !== -2 && !minimalMode){
            notePadPrivateObject.contentSaveTimerEnabled = false
            contentSaveTimer.stop()
            saveContent()
            saveCurrentPaperCursorPositionAndYTimer.stop()
            saveCurrentPaperCursorPositionAndY()
            skrTextBridge.unsubscribeTextDocument(pageType, projectId, currentNoteId, noteWritingZone.textArea.objectName, noteWritingZone.textArea.textDocument)

            root.projectId = -2
            root.currentNoteId = -2
        }

        noteWritingZone.clear()
    }

    Component.onDestruction:  {
        clearNoteWritingZone()

    }

    SKRUserSettings {
        id: skrUserSettings
    }

    function openDocument(_projectId, _noteId) {
        if(minimalMode){
            return
        }

        clearNoteWritingZone()


        noteWritingZone.text = plmData.noteHub().getContent(_projectId, _noteId)

        // apply format
        noteWritingZone.documentHandler.indentEverywhere = SkrSettings.notePadSettings.textIndent
        noteWritingZone.documentHandler.topMarginEverywhere = SkrSettings.notePadSettings.textTopMargin



        currentNoteId = _noteId
        projectId = _projectId
        noteWritingZone.paperId = _noteId
        noteWritingZone.projectId = _projectId

        skrTextBridge.subscribeTextDocument(pageType, projectId, currentNoteId, noteWritingZone.textArea.objectName, noteWritingZone.textArea.textDocument)


        restoreCurrentPaperCursorPositionAndY()

        noteWritingZone.forceActiveFocus()
        //save :
        //skrUserSettings.setProjectSetting(projectId, "writeCurrentPaperId", _noteId)
        notePadPrivateObject.contentSaveTimerEnabled = true


        var synopsisId = plmData.noteHub().getSynopsisNoteId(projectId, sheetId)
        if (synopsisId === currentNoteId){
            currentNoteTitleLabel.text = qsTr("Outline")
        }
        else{
            currentNoteTitleLabel.text = plmData.noteHub().getTitle(_projectId, _noteId)
        }


        // start the timer for automatic position saving
        if(!saveCurrentPaperCursorPositionAndYTimer.running){
            saveCurrentPaperCursorPositionAndYTimer.start()
        }

    }

    function restoreCurrentPaperCursorPositionAndY(){

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    projectId, "notePadPositionHash", currentNoteId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId, "notePadYHash", currentNoteId, 0)
        //console.log("newCursorPosition", position)

        // set positions :
        noteWritingZone.setCursorPosition(position)
        noteWritingZone.flickable.contentY = visibleAreaY

    }


    function saveCurrentPaperCursorPositionAndY(){

        if(sheetId === -2 || currentNoteId === -2 ||  sheetId === -2 || minimalMode){
            return
        }
            //save cursor position of current document :

            var previousCursorPosition = noteWritingZone.textArea.cursorPosition
            //console.log("previousCursorPosition", previousCursorPosition)
            var previousY = noteWritingZone.flickable.contentY
            //console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId, "notePadPositionHash", currentNoteId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId,
                                                       "notePadYHash",
                                                       currentNoteId,
                                                       previousY)

    }

    Timer{
        id: saveCurrentPaperCursorPositionAndYTimer
        repeat: true
        interval: 10000
        onTriggered: saveCurrentPaperCursorPositionAndY()
    }



    //--------------------------------------------
    //---------- set title------------------------
    //--------------------------------------------

    Connections{
        target: plmData.noteHub()
        function onTitleChanged(projectId, noteId, newTitle){
            if(projectId !== root.projectId){
                return
            }

            var i;
            for(i=0; i < noteListModel.count; i++){
                var item = noteListModel.get(i)

                if (item.itemNoteId === noteId){

                    noteListModel.setProperty(i, "title", newTitle)

                    var synopsisId = plmData.noteHub().getSynopsisNoteId(projectId, sheetId)
                    if (item.itemNoteId === currentNoteId && synopsisId === currentNoteId){
                        currentNoteTitleLabel.text = qsTr("Outline")
                    }
                    else if (item.itemNoteId === currentNoteId){
                        currentNoteTitleLabel.text = newTitle
                    }

                    break
                }

            }


        }

    }
    //--------------------------------------------
    //---------- Open synopsis------------------------
    //--------------------------------------------

    Action {
        id: openSynopsisAction
        text: qsTr("Show outline")
        icon.source: "qrc:///icons/backup/story-editor.svg"
        onTriggered: {
            if(projectId  === -2|| sheetId === -2){
                return
            }

            var synopsisId = plmData.noteHub().getSynopsisNoteId(projectId, sheetId)
            if(synopsisId === currentNoteId){
                return;
            }

            openDocument(projectId, synopsisId)
            root.focusedIndex = -2
            var i;
            for(i = 0; i < noteRepeater.count; i++) {
                noteRepeater.itemAt(i).isOpened = false
            }

            noteWritingZone.forceActiveFocus();

        }
    }
    openSynopsisToolButton.action: openSynopsisAction


    //--------------------------------------------
    //---------- Open note in new tab------------------------
    //--------------------------------------------

    Action {
        id: openNoteInNewTabAction
        text: qsTr("Open current note in a new tab")
        icon.source: "qrc:///icons/backup/quickopen-file.svg"
        onTriggered: {
            Globals.openNoteInNewTabCalled(projectId, currentNoteId)
        }
    }
    openNoteInNewTabToolButton.action: openNoteInNewTabAction

    //--------------------------------------------
    //---------- Modify Note------------------------
    //--------------------------------------------

    Connections{
        target: plmData.noteHub()
        function sheetNoteRelationshipChanged(projectId, sheetId, noteId){
            if(sheetId !== root.sheetId){
                return
            }
            // ignore synopsis
            var synopsisId = plmData.noteHub().getSynopsisNoteId(projectId, sheetId)
            if (synopsisId === noteId){
                return
            }

            populateNoteListModel()

        }

    }
    //--------------------------------------------
    //---------- Add Note------------------------
    //--------------------------------------------

    Action {
        id: addNoteAction
        text: qsTr("Add note")
        icon.source: "qrc:///icons/backup/list-add.svg"
        onTriggered: {

            titleEditPopup.open()
        }
    }
    addNoteMenuToolButton.action: addNoteAction


    Connections{
        target: plmData.noteHub()
        function onSheetNoteRelationshipAdded(projectId, sheetId, noteId){
            if(sheetId !== root.sheetId){
                return
            }
            // ignore synopsis
            var synopsisId = plmData.noteHub().getSynopsisNoteId(projectId, sheetId)
            if (synopsisId === noteId){
                return
            }

            var title = plmData.noteHub().getTitle(projectId, noteId)
            noteListModel.append({title: title, itemProjectId: projectId, itemSheetId: sheetId, itemNoteId: noteId})

        }

    }

    //proxy model for search :

    SKRSearchNoteListProxyModel {
        id: searchProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        projectIdFilter: projectId
    }



    //---------------------------------------------
    property int noteIdToOpen: -2
    Timer{
        id: openDocumentAfterClosingPopupTimer
        repeat: false
        interval: 0
        onTriggered: {
            if(minimalMode){
                return
            }


            //reset other notes :
            var i;
            for(i = 0; i < noteRepeater.count; i++) {
                var item = noteRepeater.itemAt(i)
                if(item.noteId === noteIdToOpen){

                    item.isOpened = true
                }
                else{

                    item.isOpened = false
                }
            }

            root.openDocument(item.projectId, noteIdToOpen)
        }

    }
    //---------------------------------------------
    SkrPopup {
        id: titleEditPopup
        x: addNoteMenuToolButton.x - 200
        y: addNoteMenuToolButton.y + addNoteMenuToolButton.height
        width: 200
        height: 400
        modal: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
        padding: 0

        onOpened: {
            titleTextField.clear()
        }

        ColumnLayout {
            anchors.fill: parent
            SkrTextField {
                id: titleTextField
                Layout.fillWidth: true

                selectByMouse: true
                placeholderText: qsTr("Note name")


                onVisibleChanged: {
                    if (visible){
                        titleTextField.forceActiveFocus()
                        titleTextField.selectAll()
                    }
                }

                onAccepted: {

                    //create basic note
                    var result = plmData.noteHub().addNoteRelatedToSheet(projectId, sheetId)
                    if (!result.success){
                        //TODO: add notification
                        return
                    }

                    var noteId = result.getData("noteId", -2)

                    // set title
                    var title = titleTextField.text
                    plmData.noteHub().setTitle(projectId, noteId, title)

                    // add to model
                    //noteListModel.append({title: title, itemProjectId: projectId, itemSheetId: sheetId, itemNoteId: noteId})

                    noteIdToOpen = noteId
                    openDocumentAfterClosingPopupTimer.start()


                    titleEditPopup.close()
                }

                onTextChanged: {
                    searchProxyModel.textFilter = text
                }

                Keys.priority: Keys.BeforeItem
                Keys.onPressed: {
                    if (event.key === Qt.Key_Down){
                        if(searchResultList.count > 0){
                            searchResultList.itemAtIndex(0).forceActiveFocus()
                        }
                    }

                }
            }

            ScrollView {
                id: searchListScrollView
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                ListView {
                    id: searchResultList
                    smooth: true
                    focus: true
                    boundsBehavior: Flickable.StopAtBounds

                    model: searchProxyModel
                    interactive: true
                    spacing: 1
                    delegate: Component {
                        id: itemDelegate

                        Item {
                            id: delegateRoot
                            height: 30
                            focus: true


                            anchors {
                                left: Qt.isQtObject(parent) ? parent.left : undefined
                                right: Qt.isQtObject(parent) ? parent.right : undefined
                                leftMargin: 5
                                rightMargin: 5
                            }

                            TapHandler {
                                id: tapHandler
                                onSingleTapped: {
                                    searchResultList.currentIndex = model.index
                                    delegateRoot.forceActiveFocus()
                                    eventPoint.accepted = true
                                }
                                onDoubleTapped: {
                                    //create relationship with note

                                    var noteId = model.paperId
                                    var result = plmData.noteHub().setSheetNoteRelationship(model.projectId, sheetId, noteId )

                                    if (!result.success){
                                        //TODO: add notification
                                        eventPoint.accepted = true
                                        return
                                    }

                                    noteIdToOpen = noteId
                                    openDocumentAfterClosingPopupTimer.start()
                                    titleEditPopup.close()

                                    eventPoint.accepted = true
                                }
                            }

                            //                        Shortcut {
                            //                            sequences: ["Return", "Space"]
                            //                            onActivated: {

                            //                                //create relationship with note

                            //                                var noteId = model.paperId
                            //                                var result = plmData.noteHub().setSheetNoteRelationship(model.projectId, sheetId, noteId )

                            //                                if (!result.success){
                            //                                    //TODO: add notification
                            //                                    return
                            //                                }

                            //                                titleEditPopup.close()

                            //                            }

                            //                            //enabled: listView.activeFocus
                            //                        }

                            //Keys.shortcutOverride: event.accepted = (event.key === Qt.Key_Return || event.key === Qt.Key_Space)

                            Keys.onPressed: {
                                if (event.key === Qt.Key_Return || event.key === Qt.Key_Space){
                                    console.log("Return key pressed title")

                                    //create relationship with note

                                    var noteId = model.paperId
                                    var result = plmData.noteHub().setSheetNoteRelationship(model.projectId, sheetId, noteId )

                                    if (!result.success){
                                        //TODO: add notification
                                        event.accepted = true
                                        return
                                    }

                                    noteIdToOpen = noteId
                                    openDocumentAfterClosingPopupTimer.start()
                                    titleEditPopup.close()

                                    event.accepted = true

                                }




                            }




                            SkrLabel {
                                text: model.name
                                anchors.fill: parent
                                horizontalAlignment: Qt.AlignLeft
                                verticalAlignment: Qt.AlignVCenter
                            }
                        }
                    }

                    highlight:  Component {
                        id: highlight
                        Rectangle {
                            //                            x: 0
                            //                            y: searchResultList.currentItem.y + 1
                            //                            width: searchResultList.width
                            //                            height: searchResultList.currentItem.height - 1
                            //                            color: "transparent"
                            radius: 5
                            border.color:   SkrTheme.accent
                            border.width: 2
                            visible: searchResultList.activeFocus
                            Behavior on y {
                                SpringAnimation {
                                    spring: 5
                                    mass: 0.2
                                    damping: 0.2
                                }
                            }
                        }
                    }


                    section.property: "projectId"
                    section.criteria: ViewSection.FullString
                    section.labelPositioning: ViewSection.CurrentLabelAtStart |
                                              ViewSection.InlineLabels
                    section.delegate: sectionHeading

                    // The delegate for each section header
                    Component {
                        id: sectionHeading
                        Rectangle {
                            width: searchResultList.width
                            height: childrenRect.height
                            color: SkrTheme.buttonBackground

                            required property string section

                            SkrLabel {
                                text: qsTr("Existing notes")
                                font.bold: true
                                color: SkrTheme.buttonForeground
                                //font.pixelSize: 20
                            }
                        }
                    }
                }
            }
        }
    }


    //--------------------------------------------
    //------- Remove Note relationship with sheet
    //--------------------------------------------

    Connections{
        target: plmData.noteHub()
        function onSheetNoteRelationshipRemoved(projectId, sheetId, noteId){
            if(sheetId !== root.sheetId){
                return
            }
            //console.log("removing")
            var i;
            for(i=0; i < noteListModel.count; i++){
                var item = noteListModel.get(i)

                if (item.itemNoteId === noteId){
                    //console.log("removing " + i)
                    noteListModel.remove(i)

                    // clear:
                    if(currentNoteId === noteId){

                        contentSaveTimer.stop()
                        currentNoteId = -2
                        noteWritingZone.clear()
                    }

                    break
                }

            }

        }

    }



    onActiveFocusChanged: {
        if (activeFocus) {

            if(minimalMode){
                noteFlow.forceActiveFocus()
            }
            else{
                openSynopsisToolButton.forceActiveFocus()
            }
        }
    }
}









