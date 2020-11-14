import QtQuick 2.15
import QtQuick.Controls 2.15
import eu.skribisto.spellchecker 1.0
import eu.skribisto.stathub 1.0
import eu.skribisto.skr 1.0
import "../Items"

ProjectSupPageForm {
    id: root
    property int projectId: -1



    Component.onCompleted: {
        editTitleTextFieldLoader.sourceComponent = editTitleTextFieldComponent

        titleLabel.text = plmData.projectHub().getProjectName(projectId)
        locationLabel.text = plmData.projectHub().getPath(projectId)

        dictNotFoundLabel.visible = false
        populateDictComboBox()
        determineCurrentDictComboBoxValue()
    }

    //---------------------------------------------------------------
    //----Change Title---------------------------------------------------
    //---------------------------------------------------------------

    Connections {
        target: plmData.projectHub()
        function onProjectNameChanged(projectId, newName){
            if(projectId === root.projectId){
                titleLabel.text = newName
            }
        }
    }


    editTitlebutton.icon {
        source: "qrc:///icons/backup/edit-rename.svg"
    }

    editTitlebutton.onClicked: {
        editTitleTextFieldLoader.item.visible = true
        titleLabel.visible = false
        editTitlebutton.visible = false

        editTitleTextFieldLoader.item.selectAll()
        editTitleTextFieldLoader.item.forceActiveFocus()

    }



    Component {
        id: editTitleTextFieldComponent
        SkrTextField {
            id: editTitleTextField
            visible: false
            placeholderText: qsTr("Write this project's new name")

            text: titleLabel.text

            font.bold: true
            font.pointSize: 20

            onEditingFinished: {

                plmData.projectHub().setProjectName(projectId, editTitleTextField.text)

                // reset
                editTitleTextField.visible = false
                titleLabel.visible = true
                editTitlebutton.visible = true
            }

            Keys.onShortcutOverride: event.accepted = (event.key === Qt.Key_Escape)
            Keys.onPressed: {
                if (event.key === Qt.Key_Return){
                    console.log("Return key pressed title")
                    editTitleTextField.editingFinished()
                    event.accepted = true
                }
                if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                    console.log("Ctrl Return key pressed title")
                    editTitleTextField.editingFinished()
                    event.accepted = true
                }
                if (event.key === Qt.Key_Escape){
                    console.log("Escape key pressed title")
                    // reset
                    editTitleTextField.text = titleLabel.text
                    editTitleTextField.visible = false
                    titleLabel.visible = true
                    editTitlebutton.visible = true
                }




            }
        }
    }

    //---------------------------------------------------------------
    //----Language---------------------------------------------------
    //---------------------------------------------------------------

    // dict combo box :

    SKRSpellChecker {
        id : spellChecker
    }

    ListModel {
        id: dictComboBoxModel
    }

    dictComboBox.model: dictComboBoxModel
    dictComboBox.textRole: "text"
    dictComboBox.valueRole: "dictCode"

    function populateDictComboBox(){

        var dictList = spellChecker.dictList()

        var i;
        for(i = 0 ; i < dictList.length ; i++){
            dictComboBoxModel.append({"text": dictList[i], "dictCode": dictList[i]})
        }

        var projectDictCode = plmData.projectHub().getLangCode(projectId)
        if(dictComboBox.indexOfValue(projectDictCode) === -1){
            dictComboBoxModel.append({"text": projectDictCode, "dictCode": projectDictCode})
            privateObject.originalLangCode = projectDictCode
            privateObject.showDictNotFoundLabel = true
        }
    }

    function determineCurrentDictComboBoxValue(){

        var langCode = plmData.projectHub().getLangCode(projectId)


        if(langCode === ""){
            langCode = skrRootItem.getLanguageFromSettings()
        }

        dictComboBox.currentIndex = dictComboBox.indexOfValue(langCode)

    }


    QtObject {
        id: privateObject
        property bool showDictNotFoundLabel: false
        property string originalLangCode: ""
    }

    dictComboBox.onCurrentValueChanged: {
        if(dictComboBox.activeFocus){
            plmData.projectHub().setLangCode(projectId, dictComboBox.currentValue)
        }
        if(privateObject.showDictNotFoundLabel && privateObject.originalLangCode === dictComboBox.currentValue){
            dictNotFoundLabel.visible = true
        }
        else {

            dictNotFoundLabel.visible = false
        }
    }


    Connections {
        target: plmData.projectHub()
        function onLangCodeChanged(projectId, newLangCode){
            if(projectId === root.projectId){
                dictComboBox.currentIndex = dictComboBox.indexOfValue(newLangCode)
            }
        }
    }



    //---------------------------------------------------------------
    //----Stats---------------------------------------------------
    //---------------------------------------------------------------

    Connections {
        target: plmData.statHub()
        function onStatsChanged(statType, paperType, projectId, count){
            if(projectId !== root.projectId){
                return
            }

            if(paperType === SKR.Note){
                return
            }
            var countString = skrRootItem.toLocaleIntString(count)
            if(statType === SKRStatHub.Character){
                charCountLabel.text = qsTr("Character count : %1").arg(countString)
            }
            if(statType === SKRStatHub.Word){
                wordCountLabel.text = qsTr("Word count : %1").arg(countString)
            }

        }
    }






}
