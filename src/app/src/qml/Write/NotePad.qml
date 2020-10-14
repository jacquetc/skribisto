import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.skrusersettings 1.0
import "../Commons"
import ".."

NotePadForm {
    id: root
    property int minimumHeight: 300

    
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
    
    
    ListModel {
        id: noteListModel
    }
    
    noteRepeater.model: noteListModel

    // force focus on first child
    noteFlow.activeFocusOnTab: true
    noteFlow.onActiveFocusChanged: {
        if(noteFlow.children.length > 1){ // means there is no children
            var first = noteFlow.children[0]
            first.forceActiveFocus()
            first.isSelected = true
            return
        }

    }

    Component {
        id: noteFlowComponent
        Rectangle {
            id: itemBase
            width: childrenRect.width + 10
            height: childrenRect.height + 10
            color: isOpened ? "cyan" : "lightskyblue"
            border.color: isSelected ? "blue" : "lightskyblue"
            border.width: 1
            radius : height / 2
            property int projectId: model.itemProjectId
            property int sheetId: model.itemSheetId
            property int noteId: model.itemNoteId
            property string noteTitle:  model.title
            property int isSynopsis: -2
            property bool isSelected: false
            property bool isOpened: false


            focus: true



            TapHandler {
                id: tapHandler
                onSingleTapped: {
                    //reset other notes :
                    var i;
                    for(i = 0; i < noteRepeater.count; i++) {
                        noteRepeater.itemAt(i).isOpened = false
                    }

                    itemBase.isOpened = true
                    //open in noteTextArea
                    openDocument(projectId, noteId)


                }
                onDoubleTapped: {
                    //reset other notes :
                    var i;
                    for(i = 0; i < noteRepeater.count; i++) {
                        noteRepeater.itemAt(i).isOpened = false
                    }

                    itemBase.isOpened = true
                    Globals.openNoteInNewTabCalled(itemBase.projectId, itemBase.noteId)


                }
            }


            TapHandler {
                id: rightClickHandler
                acceptedButtons: Qt.RightButton
                onSingleTapped: {
                    rightClickMenu.popup()


                }
            }

            Menu{
                id: rightClickMenu

                MenuItem {
                    id: renameMenuItem
                    text: qsTr("Rename")

                    onTriggered: {

                    }
                }

                MenuItem {
                    id: dissociateNoteMenu
                    text: qsTr("Dissociate")

                    onTriggered: {
                        plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, model.itemNoteId)
                    }
                }
                MenuItem {
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
                    itemBase.isOpened = true
                    Globals.openNoteInNewTabCalled(itemBase.projectId, itemBase.noteId)
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

                if (event.key === Qt.Key_Right || event.key === Qt.Key_Down ){

                    itemBase.isSelected = false
                    if(model.index === noteRepeater.count - 1){
                        noteRepeater.itemAt(0).forceActiveFocus()
                        noteRepeater.itemAt(0).isSelected = true

                    }
                    else{
                        noteRepeater.itemAt(model.index + 1).forceActiveFocus()
                        noteRepeater.itemAt(model.index + 1).isSelected = true
                    }

                }
                if (event.key === Qt.Key_Left || event.key === Qt.Key_Up ){

                    itemBase.isSelected = false
                    if(model.index === 0){
                        noteRepeater.itemAt(noteRepeater.count - 1).forceActiveFocus()
                        noteRepeater.itemAt(noteRepeater.count - 1).isSelected = true
                    }
                    else{
                        noteRepeater.itemAt(model.index - 1).forceActiveFocus()
                        noteRepeater.itemAt(model.index - 1).isSelected = true
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
                
                Label{
                    id: noteTitle
                    text: model.title
                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignHCenter
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }

                RoundButton {
                    id: removeRelationshipButton
                    Layout.preferredWidth: 0
                    Layout.maximumHeight: noteTitle.height
                    padding:1
                    icon.name: "list-remove"
                    onReleased:{
                        plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, model.itemNoteId)
                    }

                    activeFocusOnTab: false

                }
            }
            
            HoverHandler {
                id: hoverHandler

            }
            state: hoverHandler.hovered ? "visible_removeRelationshipButton": ""

            states:[
                State {
                    name: "visible_removeRelationshipButton"
                    PropertyChanges { target: removeRelationshipButton; Layout.preferredWidth: noteTitle.height}
                }
            ]

            transitions: [
                Transition {
                    from: ""
                    to: "visible_removeRelationshipButton"
                    NumberAnimation {target: removeRelationshipButton; property: "Layout.preferredWidth";duration: 300; easing.type: Easing.OutCubic }
                },
                Transition {
                    from: "visible_removeRelationshipButton"
                    to: ""
                    NumberAnimation { target: removeRelationshipButton; property: "Layout.preferredWidth";duration: 300; easing.type: Easing.OutCubic }

                }
            ]
        }

    }
    
    function populateNoteListModel(){
        if(projectId === -2 || sheetId === -2){
            noteListModel.clear()
            return
        }
        
        noteListModel.clear()
        
        var noteList = plmData.noteHub().getNotesFromSheetId(projectId, sheetId)
        
        var i;
        for (i = 0; i < noteList.length ; i++){
            var noteId = noteList[i]
            
            var title = plmData.noteHub().getTitle(projectId, noteId)
            
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



        // create if nothing opened
        if(currentNoteId === -2 && noteWritingZone.textArea.length === 1){

            //cancel if time too short between note creations
            if(notePadPrivateObject.creationProtectionEnabled){
                return
            }
            notePadPrivateObject.creationProtectionEnabled = true
            newOnTheFlyNoteCreationProtectionTimer.start()

            notePadPrivateObject.newText = noteWritingZone.text
            console.log("notped :", notePadPrivateObject.newText)

            //create basic note
            var error = plmData.noteHub().addNoteRelatedToSheet(projectId, sheetId)
            if (!error.success){
                //TODO: add notification
                return
            }

            var noteId = error.getDataList()[0]

            // set title
            var title = titleTextField.text
            plmData.noteHub().setTitle(projectId, noteId, qsTr("New note"))



            noteIdToOpen = noteId
            openDocumentAfterClosingPopupTimer.start()
            newOnTheFlyNotePositionAndTextTimer.start()
        }




        if(contentSaveTimer.running){
            contentSaveTimer.stop()
        }
        contentSaveTimer.start()
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
        if(projectId !== -2 && currentNoteId !== -2){
            console.log("saving note in notepad")
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

            if (projectId === this.projectId){
                // save
                saveContent()
                //TODO: saveCurrentPaperCursorPositionAndY()

            }
        }
    }


    SkrUserSettings {
        id: skrUserSettings
    }

    function openDocument(_projectId, _noteId) {
        // save current
        if(projectId !== -2 && currentNoteId !== -2 ){
            saveContent()
            saveCurrentPaperCursorPositionAndY()
            skrTextBridge.unsubscribeTextDocument(pageType, projectId, currentNoteId, noteWritingZone.textArea.objectName, noteWritingZone.textArea.textDocument)
        }


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
        console.log("newCursorPosition", position)

        // set positions :
        writingZone.setCursorPosition(position)
        writingZone.flickable.contentY = visibleAreaY

    }


    function saveCurrentPaperCursorPositionAndY(){

        if(paperId != -2 || currentNoteId != -2){
            //save cursor position of current document :

            var previousCursorPosition = noteWritingZone.cursorPosition
            console.log("previousCursorPosition", previousCursorPosition)
            var previousY = noteWritingZone.flickable.contentY
            console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId, "notePadPositionHash", currentNoteId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId,
                                                       "notePadYHash",
                                                       currentNoteId,
                                                       previousY)
        }
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
                    break
                }
                
            }
            
            
        }
        
    }
    //--------------------------------------------
    //---------- Add Note------------------------
    //--------------------------------------------
    
    Action {
        id: addNoteAction
        text: qsTr("Add note")
        icon.name: "list-add"
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
    Popup {
        id: titleEditPopup
        x: addNoteMenuToolButton.x - 200
        y: addNoteMenuToolButton.y + addNoteMenuToolButton.height
        width: 200
        height: 200
        modal: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
        padding: 0


        ColumnLayout {
            anchors.fill: parent
            TextField {
                id: titleTextField
                Layout.fillWidth: true
                
                selectByMouse: true
                
                
                onVisibleChanged: {
                    if (visible){
                        titleTextField.text = "test"
                        titleTextField.forceActiveFocus()
                        titleTextField.selectAll()
                    }
                }
                
                onAccepted: {
                    
                    //create basic note
                    var error = plmData.noteHub().addNoteRelatedToSheet(projectId, sheetId)
                    if (!error.success){
                        //TODO: add notification
                        return
                    }
                    
                    var noteId = error.getDataList()[0]
                    
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
                    anchors.fill: parent
                    clip: true
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
                                    var error = plmData.noteHub().setSheetNoteRelationship(model.projectId, sheetId, noteId )

                                    if (!error.success){
                                        //TODO: add notification
                                        return
                                    }

                                    noteIdToOpen = noteId
                                    openDocumentAfterClosingPopupTimer.start()
                                    titleEditPopup.close()

                                }
                            }
                            
                            //                        Shortcut {
                            //                            sequences: ["Return", "Space"]
                            //                            onActivated: {
                            
                            //                                //create relationship with note
                            
                            //                                var noteId = model.paperId
                            //                                var error = plmData.noteHub().setSheetNoteRelationship(model.projectId, sheetId, noteId )
                            
                            //                                if (!error.success){
                            //                                    //TODO: add notification
                            //                                    return
                            //                                }
                            
                            //                                titleEditPopup.close()
                            
                            //                            }
                            
                            //                            //enabled: listView.activeFocus
                            //                        }
                            
                            //Keys.shortcutOverride: event.accepted = (event.key === Qt.Key_Return || event.key === Qt.Key_Space)
                            Keys.priority: Keys.AfterItem
                            
                            Keys.onPressed: {
                                if (event.key === Qt.Key_Return || event.key === Qt.Key_Space){
                                    console.log("Return key pressed title")
                                    
                                    //create relationship with note
                                    
                                    var noteId = model.paperId
                                    var error = plmData.noteHub().setSheetNoteRelationship(model.projectId, sheetId, noteId )
                                    
                                    if (!error.success){
                                        //TODO: add notification
                                        return
                                    }
                                    
                                    noteIdToOpen = noteId
                                    openDocumentAfterClosingPopupTimer.start()
                                    titleEditPopup.close()


                                    
                                }




                            }




                            Label {
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
                            border.color:  "lightsteelblue"
                            border.width: 2
                            visible: searchResultList.activeFocus
                            Behavior on y {
                                SpringAnimation {
                                    spring: 3
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
                            color: "lightsteelblue"
                            
                            required property string section
                            
                            Label {
                                text: qsTr("Existing notes")
                                font.bold: true
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
            console.log("removing")
            var i;
            for(i=0; i < noteListModel.count; i++){
                var item = noteListModel.get(i)
                
                if (item.itemNoteId === noteId){
                    console.log("removing " + i)
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
            openSynopsisToolButton.forceActiveFocus()
        }
    }
}









