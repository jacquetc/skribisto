import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

NotePadForm {
    id: root


    property int projectId: -2
    property int sheetId: -2

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

    Component {
        id: noteFlowComponent
        Rectangle {
            id: itemBase
            height: 40
            width: 40
            property int projectId: itemProjectId
            property int sheetId: itemSheetId
            property int noteId: itemNoteId
            property int isSynopsis: -2



            RowLayout{
                anchors.fill: parent


                Text{
                    id: noteTitle
                    text: model.title
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

                    if (!error.success){
                        //TODO: add notification
                        return
                    }

                    var noteId = error.getDataList()[0]

                    // set title
                    var title = titleTextField.text
                    plmData.noteHub().setTitle(projectId, noteId, title)


                    // add to model
                    noteListModel.append({title: title, itemProjectId: projectId, itemSheetId: sheetId, itemNoteId: noteId})

                    titleEditPopup.close()
                }

                onTextChanged: {

                }


            }

            ListView {
                id: searchResultList


            }
        }

    }








}
