import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import eu.skribisto.projectdicthub 1.0
import eu.skribisto.spellchecker 1.0
import "../../Items"
import "../../Commons"
import "../.."


UserDictPageForm {
    id: root

    required property int projectId

    Component.onCompleted: {
        populateList()
    }

    Connections {
        target: skrData.projectDictHub()
        function onProjectDictFullyChanged(projectId, projectDictList) {
            if(root.projectId === projectId){
                populateList()
            }
        }
    }



    Connections {
        target: skrData.projectDictHub()
        function onProjectDictWordAdded(projectId, newWord) {
            if(root.projectId === projectId){
                populateList()
            }
        }
    }

    Connections {
        target: skrData.projectDictHub()
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

        var dictList = skrData.projectDictHub().getProjectDictList(root.projectId)

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
                //                                onSingleTapped: function(eventPoint) {
                //                                    searchResultList.currentIndex = model.index
                //                                    delegateRoot.forceActiveFocus()
                //                                    eventPoint.accepted = true
                //                                }
                onSingleTapped: function(eventPoint) {
                    //create relationship with note
                    listView.currentIndex = model.index
                    delegateRoot.forceActiveFocus()
                    priv.selectedWord = model.word

                    eventPoint.accepted = true
                }


                onGrabChanged: function(transition, point) {
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
                enabled: SkrSettings.ePaperSettings.animationEnabled
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

        }
    }
    addWordButton.action: addWordAction

    SKRSpellChecker {
        id : spellChecker

        Component.onCompleted: {
            var lang = skrData.projectHub().getLangCode(root.projectId)
            if(lang){
                spellChecker.setLangCode(skrData.projectHub().getLangCode(root.projectId))
                spellChecker.setUserDict(skrData.projectDictHub().getProjectDictList(root.projectId))


        }
    }
    }


    Connections{
        target: skrData.projectHub()
        function onLangCodeChanged(projectId, newLang){
            if(projectId === root.projectId){
                spellChecker.setLangCode(newLang)
                spellChecker.setUserDict(skrData.projectDictHub().getProjectDictList(root.projectId))
            }
        }
    }

    SimpleDialog {
        id: addWordToDProjectDictDialog
        title: qsTr("Add a word to the project dictionary")
        width: 400
        contentItem: ColumnLayout {
            SkrTextField {
                id: addWordTextField
                Layout.alignment: Qt.AlignCenter
                Layout.fillWidth: true
                text: ""

                onAccepted: {
                    if(spellChecker.spell(addWordTextField.text)){
                        addWordToDProjectDictDialog.accept()
                    }
                }

                onTextChanged: {
                    if(!spellChecker.active){
                        label.text = ""

                    }
                    else if(spellChecker.spell(addWordTextField.text)){
                        label.text = qsTr("Word already in dictionary")
                    }
                    else {
                        label.text = ""
                    }
                }
            }

            SkrLabel {
                id: label
                Layout.alignment: Qt.AlignCenter
                Layout.fillWidth: true


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

            skrData.projectDictHub().addWordToProjectDict(root.projectId, addWordTextField.text)
            addWordTextField.text = ""
        }

        onActiveFocusChanged: {
            if(activeFocus){
                addWordTextField.forceActiveFocus()
            }

        }

        onOpened: {
            addWordTextField.text = ""
            addWordTextField.forceActiveFocus()
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
            skrData.projectDictHub().removeWordFromProjectDict(root.projectId, priv.selectedWord)
        }
    }
    removeWordButton.action: removeWordAction





}
