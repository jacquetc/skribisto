import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.spellchecker 1.0
import eu.skribisto.stathub 1.0
import eu.skribisto.skr 1.0

import "../../Items"
import "../../Commons"
import "../.."

ProjectPageForm {
    id: root

    pageType: "PROJECT"

    property string title: {
        return getTitle()
    }

    function getTitle() {
        return skrData.treeHub().getTitle(projectId, treeItemId)

    }

    Connections {
        target: skrData.treeHub()
        function onTitleChanged(_projectId, _treeItemId, newTitle) {
            if (projectId === _projectId && treeItemId === _treeItemId) {
                title = getTitle()
            }
        }
    }

    titleLabel.text: title

    //--------------------------------------------------------
    additionalPropertiesForSavingView: {
        return {}
    }

    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------
    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
//        skrWindowManager.insertAdditionalPropertyForViewManager("isSecondary",
//                                                                isSecondary)
        skrWindowManager.addWindowForItemId(projectId, treeItemId)
        rootWindow.setNavigationTreeItemIdCalled(projectId, treeItemId)
    }

    viewButtons.onSplitCalled: function (position) {
//        viewManager.insertAdditionalProperty("isSecondary", isSecondary)
        viewManager.loadTreeItemAt(projectId, treeItemId, position)
    }

    //--------------------------------------------------------
    //---Tool bar-----------------------------------------
    //--------------------------------------------------------
    Connections {
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value) {
            if (projectId === root.projectId
                    && treeItemId === root.treeItemId) {

                if (name === "word_count_with_children") {
                    countPriv.wordCount = value
                    updateCountLabel()
                }
            }
        }
    }

    Connections {
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value) {
            if (projectId === root.projectId
                    && treeItemId === root.treeItemId) {

                if (name === "char_count_with_children") {
                    countPriv.characterCount = value
                    updateCountLabel()
                }
            }
        }
    }

    QtObject {
        id: countPriv
        property string wordCount: ""
        property string characterCount: ""
    }

    function updateCountLabel() {
        var wordCountString = skrRootItem.toLocaleIntString(countPriv.wordCount)
        var characterCountString = skrRootItem.toLocaleIntString(
                    countPriv.characterCount)

        countLabel.text = qsTr("%1 words, %2 characters (all)").arg(
                    wordCountString).arg(characterCountString)
    }

    //-----------------------------------------------------------------
    //----- Page menu ------------------------------------------------
    //-----------------------------------------------------------------
    pageMenuToolButton.onClicked: {
        if (pageMenu.visible) {
            pageMenu.close()
            return
        }
        pageMenu.open()
    }

    SkrMenu {
        id: pageMenu
        y: pageMenuToolButton.height
        x: pageMenuToolButton.x

        SkrMenuItem {
            action: newIdenticalPageAction
        }

        SkrMenuItem {
            action: Action {
                id: showRelationshipPanelAction
                text: skrShortcutManager.description("show-relationship-panel")
                icon.source: "qrc:///icons/backup/link.svg"
                onTriggered: {

                    relationshipPanel.visible = true
                    relationshipPanel.forceActiveFocus()
                }
            }
        }

        SkrMenuItem {
            action: Action {
                id: addQuickNoteAction
                text: skrShortcutManager.description("add-quick-note")
                icon.source: "qrc:///icons/backup/list-add.svg"
                onTriggered: {
                    relationshipPanel.visible = true
                    var result = skrData.treeHub().addQuickNote(projectId,
                                                                treeItemId,
                                                                "TEXT",
                                                                qsTr("Note"))
                    if (result.success) {
                        var newId = result.getData("treeItemId", -2)

                        relationshipPanel.openTreeItemInPanel(projectId, newId)
                        relationshipPanel.forceActiveFocus()
                    }
                }
            }
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("add-quick-note")
        enabled: root.activeFocus
        onActivated: {
            addQuickNoteAction.trigger()
        }
    }
    Shortcut {
        sequences: skrShortcutManager.shortcuts("show-relationship-panel")
        enabled: root.activeFocus
        onActivated: {
            showRelationshipPanelAction.trigger()
        }
    }


    //-----------------------------------------------------------------
    //----- Related Panel------------------------------------------------
    //-----------------------------------------------------------------
    relationshipPanel.projectId: root.projectId
    relationshipPanel.treeItemId: root.treeItemId

    relationshipPanel.viewManager: viewManager

    relationshipPanel.onExtendedChanged: {
        if (relationshipPanel.extended) {
            relationshipPanelPreferredHeight = root.height / 2
        } else {
            relationshipPanelPreferredHeight = 200
        }
    }

    Behavior on relationshipPanelPreferredHeight {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        SpringAnimation {
            spring: 5
            mass: 0.2
            damping: 0.2
        }
    }


    //-----------------------------------------------------------------
    //----- Project ------------------------------------------------
    //-----------------------------------------------------------------
    property bool built: false

    Component.onCompleted: {
        editTitleTextFieldLoader.sourceComponent = editTitleTextFieldComponent

        titleLabel2.text = skrData.projectHub().getProjectName(projectId)
        locationLabel.text = skrData.projectHub().getPath(projectId)

        dictNotFoundLabel.visible = false
        populateDictComboBox()
        determineCurrentDictComboBoxValue()
        populateStatistics()

        populateNoteFolderComboBox()

        built =true
    }


    //---------------------------------------------------------------
    //----Change Title---------------------------------------------------
    //---------------------------------------------------------------
    titleLabel2.text: root.title

    Connections {
        target: skrData.projectHub()
        function onProjectNameChanged(projectId, newName){
            if(projectId === root.projectId){
                titleLabel2.text = newName
            }
        }
    }


    editTitlebutton.icon {
        source: "qrc:///icons/backup/edit-rename.svg"
    }

    editTitlebutton.onClicked: {
        editTitleTextFieldLoader.item.visible = true
        titleLabel2.visible = false
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

            text: titleLabel2.text

            font.bold: true
            font.pointSize: 20

            onEditingFinished: {

                skrData.projectHub().setProjectName(projectId, editTitleTextField.text)

                // reset
                editTitleTextField.visible = false
                titleLabel2.visible = true
                editTitlebutton.visible = true
            }

            Keys.onShortcutOverride: function(event) { event.accepted = (event.key === Qt.Key_Escape) }
            Keys.onPressed: function(event) {
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
                    editTitleTextField.text = titleLabel2.text
                    editTitleTextField.visible = false
                    titleLabel2.visible = true
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

    QtObject{
        id: priv
        property bool internalTitleConnectionEnabled: true
    }

    noteFolderComboBox.onCurrentValueChanged: {
        if(built){
            // clear other folders:
            var foldersList = skrData.treeHub().getIdsWithInternalTitle(projectId, "note_folder")

            priv.internalTitleConnectionEnabled = false
            for(var j in foldersList){
                if(skrData.treeHub().getInternalTitle(projectId, foldersList[j]) === "note_folder"){
                    skrData.treeHub().setInternalTitle(root.projectId, foldersList[j], "")
                }
            }

            //set new folder
            skrData.treeHub().setInternalTitle(root.projectId, noteFolderComboBox.currentValue, "note_folder")
            priv.internalTitleConnectionEnabled = true
        }
    }


    Connections{
        target: skrData.treeHub()
        function onTreeItemAdded(projectId, treeItemId) {
            populateNoteFolderComboBox()
        }
    }
    Connections{
        target: skrData.treeHub()
        function onTitleChanged(projectId, treeItemId, newTitle) {
            populateNoteFolderComboBox()
        }
    }
    Connections{
        target: skrData.treeHub()
        function onInternalTitleChanged(projectId, treeItemId, newTitle) {
            if(priv.internalTitleConnectionEnabled){
                populateNoteFolderComboBox()
            }
        }
    }
    Connections{
        target: skrData.treeHub()
        function onTrashedChanged(projectId, treeItemId, newTrashedState) {
            populateNoteFolderComboBox()
        }
    }

    addNewNoteFolderButton.onClicked: {
        skrData.treeHub().addChildTreeItem(root.projectId, 0, "FOLDER")
        var folderId = skrData.treeHub().getLastAddedId()

        skrData.treeHub().setTitle(root.projectId, folderId, qsTr("Notes"))
        skrData.treeHub().setInternalTitle(root.projectId, folderId, "note_folder")
        populateNoteFolderComboBox()
    }

}
