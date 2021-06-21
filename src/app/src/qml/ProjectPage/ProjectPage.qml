import QtQuick 2.15
import QtQuick.Controls 2.15
import eu.skribisto.spellchecker 1.0
import eu.skribisto.stathub 1.0
import eu.skribisto.skr 1.0
import "../Items"
import "../Commons"
import ".."

ProjectPageForm {
    id: root

    pageType: "PROJECT"



    Component.onCompleted: {
        editTitleTextFieldLoader.sourceComponent = editTitleTextFieldComponent

        titleLabel.text = skrData.projectHub().getProjectName(projectId)
        locationLabel.text = skrData.projectHub().getPath(projectId)

        dictNotFoundLabel.visible = false
        populateDictComboBox()
        determineCurrentDictComboBoxValue()
        populateStatistics()

        populateNoteFolderComboBox()
    }


    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------

    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
        skrWindowManager.addWindowForItemId(projectId, treeItemId)
        rootWindow.setNavigationTreeItemIdCalled(projectId, treeItemId)
    }

    viewButtons.onSplitCalled: function(position){
        viewManager.loadTreeItemAt(projectId, treeItemId, position)
    }


    //---------------------------------------------------------------
    //----Change Title---------------------------------------------------
    //---------------------------------------------------------------

    Connections {
        target: skrData.projectHub()
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

                skrData.projectHub().setProjectName(projectId, editTitleTextField.text)

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
        dictComboBoxModel.clear()

        var dictList = spellChecker.dictList()

        var i;
        for(i = 0 ; i < dictList.length ; i++){
            dictComboBoxModel.append({"text": dictList[i], "dictCode": dictList[i]})
        }

        var projectDictCode = skrData.projectHub().getLangCode(projectId)
        if(dictComboBox.indexOfValue(projectDictCode) === -1){
            dictComboBoxModel.append({"text": projectDictCode, "dictCode": projectDictCode})
            privateObject.originalLangCode = projectDictCode
            privateObject.showDictNotFoundLabel = true
        }
    }

    function determineCurrentDictComboBoxValue(){

        var langCode = skrData.projectHub().getLangCode(projectId)


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
            skrData.projectHub().setLangCode(projectId, dictComboBox.currentValue)
        }
        if(privateObject.showDictNotFoundLabel && privateObject.originalLangCode === dictComboBox.currentValue){
            dictNotFoundLabel.visible = true
        }
        else {

            dictNotFoundLabel.visible = false
        }
    }


    Connections {
        target: skrData.projectHub()
        function onLangCodeChanged(projectId, newLangCode){
            if(projectId === root.projectId){
                dictComboBox.currentIndex = dictComboBox.indexOfValue(newLangCode)
            }
        }
    }
    //------------------------------------------------
    // install dict

    Component {
        id: component_newDictWizard
        NewDictWizard {
            id: newDictWizard

            onClosed: loader_newDictWizard.active = false
        }
    }
    Loader {
        id: loader_newDictWizard
        active: false
        sourceComponent: component_newDictWizard
    }

    installDictButton.onClicked: {
        loader_newDictWizard.active = true
    }

    Connections {
        target: Globals
        function onNewDictInstalled(dictName){
            populateDictComboBox()
            determineCurrentDictComboBoxValue()

        }
    }
    //---------------------------------------------------------------
    //----Stats---------------------------------------------------
    //---------------------------------------------------------------

    function populateStatistics(){
        var countString = skrRootItem.toLocaleIntString(skrData.statHub().getTreeItemTotalCount(SKRStatHub.Character, root.projectId))

        charCountLabel.text = qsTr("Character count : %1").arg(countString)
        countString = skrRootItem.toLocaleIntString(skrData.statHub().getTreeItemTotalCount(SKRStatHub.Word, root.projectId))
        wordCountLabel.text = qsTr("Word count : %1").arg(countString)
    }


    Connections {
        target: skrData.statHub()
        function onStatsChanged(statType, projectId, count){
            if(projectId !== root.projectId){
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


    //---------------------------------------------------------------
    //----Note ---------------------------------------------------
    //---------------------------------------------------------------

    ListModel {
        id: noteFolderModel
    }

    noteFolderComboBox.model: noteFolderModel
    noteFolderComboBox.textRole: "text"
    noteFolderComboBox.valueRole: "treeItemId"

    function populateNoteFolderComboBox(){
        noteFolderModel.clear()

        var idList = skrData.treeHub().getAllIds(projectId)
        for(var i in idList){
            var treeItemId = idList[i]

            if(skrData.treeHub().getType(projectId, treeItemId) === "FOLDER" &&
                    !skrData.treeHub().getTrashed(projectId, treeItemId)){

                var title = skrData.treeHub().getTitle(projectId, treeItemId)
                noteFolderModel.append({"text": title, "treeItemId": idList[i]})
            }
        }


        var foldersList = skrData.treeHub().getIdsWithInternalTitle(projectId, "note_folder")
        for(var j in foldersList){
            noteFolderComboBox.currentIndex = noteFolderComboBox.indexOfValue(foldersList[j])
            break
        }
    }





}
