import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.searchnotelistproxymodel 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

NotePadForm {
    id: root
    
    
    property int projectId: -2
    property int sheetId: -2
    property int currentNoteId: -2

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
        if(noteFlow.children.length !== 0){
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
            property int projectId: itemProjectId
            property int sheetId: itemSheetId
            property int noteId: itemNoteId
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
                if (event.key === Qt.Key_Delete){
                    console.log("Delete key pressed ")
                    // remove
                    plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, model.itemNoteId)
                    
                }
                if ((event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_Delete){
                    console.log("Shift delete key pressed ")
                    // remove completely the note
                    
                    //TODO: ask confirmation before erasing
                    
                    //plmData.noteHub().removeSheetNoteRelationship(projectId, sheetId, model.itemNoteId)
                    
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
                
                Text{
                    id: noteTitle
                    text: model.title
                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignHCenter
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }
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
        if(contentSaveTimer.running){
            contentSaveTimer.stop()
        }
        contentSaveTimer.start()
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
        }


        noteWritingZone.text = plmData.noteHub().getContent(_projectId, _noteId)

        // apply format
        noteWritingZone.documentHandler.indentEverywhere = SkrSettings.notePadSettings.textIndent
        noteWritingZone.documentHandler.topMarginEverywhere = SkrSettings.notePadSettings.textTopMargin


        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    _projectId, "notePadPositionHash", _noteId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    _projectId, "notePadYHash", _noteId, 0)
        // set positions :
        noteWritingZone.setCursorPosition(position)
        noteWritingZone.flickable.contentY = visibleAreaY
        currentNoteId = _noteId
        projectId = _projectId


        noteWritingZone.forceActiveFocus()
        //save :
        //skrUserSettings.setProjectSetting(projectId, "writeCurrentPaperId", _noteId)

        // start the timer for automatic position saving
        if(!saveCurrentPaperCursorPositionAndYTimer.running){
            saveCurrentPaperCursorPositionAndYTimer.start()
        }

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
        showDeletedFilter: false
        showNotDeletedFilter: true
        projectIdFilter: projectId
    }
    
    
    
    
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
                        console.log("visible !")
                        titleTextField.text = "test"
                        titleTextField.forceActiveFocus()
                        titleTextField.selectAll()
                    }
                }
                
                onAccepted: {
                    
                    //create basic note
                    var error = plmData.noteHub().addNoteRelatedToSheet(projectId, sheetId)
                    console.log("A")
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
                    
                    titleEditPopup.close()
                }
                
                onTextChanged: {
                    searchProxyModel.textFilter = text
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
                            
                            
                            anchors {
                                left: Qt.isQtObject(parent) ? parent.left : undefined
                                right: Qt.isQtObject(parent) ? parent.right : undefined
                                leftMargin: 5
                                rightMargin: 5
                            }
                            
                            TapHandler {
                                id: tapHandler
                                onSingleTapped: {
                                    listView.currentIndex = model.index
                                    delegateRoot.forceActiveFocus()
                                    eventPoint.accepted = true
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
                                    
                                    titleEditPopup.close()
                                    
                                    
                                }
                                
                            }
                            Text {
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
                            visible: searchResultList.focus
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
                            
                            Text {
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
                
                console.log("r " + i)
                if (item.itemNoteId === noteId){
                    console.log("removing " + i)
                    noteListModel.remove(i)
                    break
                }
                
            }
            
        }
        
    }
}









