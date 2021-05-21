import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml.Models 2.15
import eu.skribisto.projectdicthub 1.0
import "../Items"
import "../Commons"
import ".."


UserDictPageForm {
    id: root

    required property int projectId

    Component.onCompleted: {
        populateList()
    }

    Connections {
        target: plmData.projectDictHub()
        function onProjectDictFullyChanged(projectId, projectDictList) {
            if(root.projectId === projectId){
                populateList()
            }
        }
    }



    Connections {
        target: plmData.projectDictHub()
        function onProjectDictWordAdded(projectId, newWord) {
            if(root.projectId === projectId){
                populateList()
            }
        }
    }

    Connections {
        target: plmData.projectDictHub()
        function onProjectDictWordRemoved(projectId, wordToRemove) {
            if(root.projectId === projectId){
                populateList()
            }
        }
    }

    ListModel {
        id: listModel
    }

    listView.model: listModel

    function populateList(){
        listModel.clear()

        var dictList = plmData.projectDictHub().getProjectDictList(root.projectId)

        dictList.forEach(word => listModel.append({"word": word}))
    }

    listView.delegate: Component {
        id: itemDelegate
        SkrListItemPane {
            id: delegateRoot
            height: 30
            focus: true




            Accessible.name: model.word
            Accessible.role: Accessible.ListItem

            anchors {
                left: Qt.isQtObject(parent) ? parent.left : undefined
                right: Qt.isQtObject(parent) ? parent.right : undefined
                leftMargin: 5
                rightMargin: 5
            }

            TapHandler {
                id: tapHandler
                //                                onSingleTapped: {
                //                                    searchResultList.currentIndex = model.index
                //                                    delegateRoot.forceActiveFocus()
                //                                    eventPoint.accepted = true
                //                                }
                onSingleTapped: {
                    //create relationship with note
                    listView.currentIndex = model.index
                    delegateRoot.forceActiveFocus()
                    priv.selectedWord = model.word

                    eventPoint.accepted = true
                }


                onGrabChanged: {
                    point.accepted = false
                }


            }





            SkrLabel {
                text: model.word
                anchors.fill: parent
                anchors.leftMargin: 3
                anchors.rightMargin: 3
                horizontalAlignment: Qt.AlignLeft
                verticalAlignment: Qt.AlignVCenter

            }
        }
    }

    listView.highlight:  Component {
        id: highlight
        Rectangle {
            radius: 5
            border.color:   SkrTheme.accent
            border.width: 2
            visible: listView.activeFocus
            Behavior on y {
                SpringAnimation {
                    spring: 5
                    mass: 0.2
                    damping: 0.2
                }
            }
        }
    }


    listView.onActiveFocusChanged: {
        if(activeFocus){
            if(listView.count > 0){
                listView.itemAtIndex(0).forceActiveFocus()
            }
        }

    }



    Action {
        id: addWordAction
        text: qsTr("Add word to dictionary")
        icon.source: "qrc:///icons/backup/list-add.svg"
        onTriggered: {
            addWordToDProjectDictDialog.open()
            //TODO : popup to add word

        }
    }
    addWordButton.action: addWordAction

    SimpleDialog {
        id: addWordToDProjectDictDialog
        title: qsTr("Add a word to the project dictionary")
        contentItem: SkrTextField {
            id: addWordTextField
            text: ""

            onAccepted: {
                addWordToDProjectDictDialog.accept()
            }

        }

        standardButtons: Dialog.Ok  | Dialog.Cancel

        onRejected: {
            addWordTextField.text = ""

        }

        onDiscarded: {


            addWordTextField.text = ""

        }

        onAccepted: {

            plmData.projectDictHub().addWordToProjectDict(root.projectId, addWordTextField.text)
            addWordTextField.text = ""
        }

        onActiveFocusChanged: {
            if(activeFocus){
                contentItem.forceActiveFocus()
            }

        }

        onOpened: {
            addWordTextField.text = ""
            contentItem.forceActiveFocus()
        }

    }

    QtObject {
        id: priv
        property string selectedWord: ""
    }


    Action {
        id: removeWordAction
        text: qsTr("Remove word from dictionary")
        icon.source: "qrc:///icons/backup/list-remove.svg"
        onTriggered: {
            plmData.projectDictHub().removeWordFromProjectDict(root.projectId, priv.selectedWord)
        }
    }
    removeWordButton.action: removeWordAction





}
