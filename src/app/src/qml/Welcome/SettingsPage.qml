import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.platform 1.1 as LabPlatform
import eu.skribisto.projecthub 1.0
import Qt.labs.settings 1.1
import ".."

SettingsPageForm {

    Component.onCompleted: {
        checkOnBackUpEveryCheckBox()
        populateBackupPathListView()

        checkOnSaveEveryCheckBox()
        backUpOnceADayIfNeeded()

        langComboBox.currentIndex = langComboBox.indexOfValue(skrRootItem.getLanguageFromSettings())

    }


    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            accessibilityGroupBox.forceActiveFocus()
        }
    }



    //    // scrollview :
    //    Component.onCompleted: scroll.contentItem = contener
    //	function scrollChange(){
    //                if ((width > 600) && (height > 600)){
    //                        if(scroll.contentItem != itemNull) scroll.contentItem = itemNull
    //                        contener.width = width
    //                        contener.height = height
    //                } else if ((width < 600) && (height < 600)){
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.width = 600
    //                        contener.height = 600
    //                } else if ((width < 600)){
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.width = 600
    //                        contener.height = height
    //                } else {
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.height = 600
    //                        contener.width = width
    //                }
    //        }

    //	onWidthChanged: scrollChange()
    //	onHeightChanged: scrollChange()



    // --------------------------------------------
    // ---- appareance --------------------------------
    // --------------------------------------------


    menuButtonsInStatusBarSwitch.checked: SkrSettings.interfaceSettings.menuButtonsInStatusBar
    Binding {
        target: SkrSettings.interfaceSettings
        property: "menuButtonsInStatusBar"
        value: menuButtonsInStatusBarSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    // interface languages :


    ListModel {
        id: langModel
        ListElement { text: "english (US)"; langCode: "en_US" }
        ListElement { text: "français (France)"; langCode: "fr_FR" }
        ListElement { text: "deutsch"; langCode: "de_DE" }
    }


    langComboBox.model: langModel
    langComboBox.textRole: "text"
    langComboBox.valueRole: "langCode"
    langComboBox.onCurrentValueChanged: {
        if(langComboBox.activeFocus){
            skrRootItem.currentTranslationLanguageCode = langComboBox.currentValue
        }
    }


    Connections {
        target: skrRootItem
        function onCurrentTranslationLanguageCodeChanged(langCode){
            langComboBox.currentIndex = langComboBox.indexOfValue(langCode)
        }
    }



    // --------------------------------------------
    // ---- accessibility --------------------------------
    // --------------------------------------------

    allowSwipeBetweenTabsCheckBox.checked: SkrSettings.accessibilitySettings.allowSwipeBetweenTabs
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "allowSwipeBetweenTabs"
        value: allowSwipeBetweenTabsCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    showMenuBarCheckBox.checked: SkrSettings.accessibilitySettings.showMenuBar
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "showMenuBar"
        value: showMenuBarCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    // --------------------------------------------
    // ---- backup --------------------------------
    // --------------------------------------------
    // backup paths :

    ListModel{
        id: backupPathListModel
    }
    backupPathListView.model: backupPathListModel

    function populateBackupPathListView(){


        var backupPaths = SkrSettings.backupSettings.paths
        var backupPathList = backupPaths.split(";")

        //no backup path set
        if (backupPaths === ""){
            //TODO: send notification, backup not configured

            return
        }

        var j;
        for (j = 0; j < backupPathList.length ; j++ ){
            var path = backupPathList[j]
            if(path === ""){
                break
            }

            backupPathListModel.append({"path": Qt.resolvedUrl(path), "localPath": skrQMLTools.translateURLToLocalFile(Qt.resolvedUrl(path))})

        }

    }



    function addBackupPath(pathUrl){

        backupPathListModel.append({"path": pathUrl, "localPath": skrQMLTools.translateURLToLocalFile(pathUrl)})
        saveBackupPathsToSettings()

    }

    function removeBackupPath(index){

        backupPathListModel.remove(index)
        saveBackupPathsToSettings()

    }

    function saveBackupPathsToSettings(){

        var backupPathList = [];
        var j;
        for (j = 0; j < backupPathListModel.count ; j++ ){

            backupPathList.push(backupPathListModel.get(j).localPath)
        }

        var backupPaths = backupPathList.join(";")
        SkrSettings.backupSettings.paths = backupPaths
    }

    LabPlatform.FolderDialog{
        id: addBackupFolderDialog
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
        onAccepted: {
            //                    var path = folderDialog.folder.toString()
            //                    path = path.replace(/^(file:\/{2})/,"");
            //                    model.path = path
            var path = addBackupFolderDialog.folder

            addBackupPath(path)
            backupPathListView.currentIndex = backupPathListView.count -1

        }
        onRejected: {

        }



    }

    Action {
        id: addBackupPathAction
        text: qsTr("Add backup path")
        icon.name: "list-add"
        onTriggered: {


            addBackupFolderDialog.open()
        }
    }
    addBackupPathButton.action: addBackupPathAction



    Action {
        id: removeBackupPathAction
        text: qsTr("Remove backup path")
        icon.name: "list-remove"
        onTriggered: {

            removeBackupPath(backupPathListView.currentIndex)
        }
    }
    removeBackupPathButton.action: removeBackupPathAction




    backupPathListView.delegate: Component {
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
                    backupPathListView.currentIndex = model.index
                    delegateRoot.forceActiveFocus()
                    eventPoint.accepted = true
                }
                onDoubleTapped: {

                    // path editor :
                    //backupPathListView.currentIndex = model.index
                    editName()


                    eventPoint.accepted = true
                }
            }

            function editName() {
                state = "edit_path"
                editPathTextField.forceActiveFocus()
                editPathTextField.selectAll()
            }



            LabPlatform.FolderDialog{
                id: backupFolderDialog

                onAccepted: {

                    model.path = folderDialog.folder
                    model.localPath = skrQMLTools.translateURLToLocalFile(folderDialog.folder)
                }
                onRejected: {

                }



            }

            RowLayout{
                anchors.fill: parent

                Label {
                    id: pathLabel
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    text: model.localPath
                    horizontalAlignment: Qt.AlignLeft
                    verticalAlignment: Qt.AlignVCenter
                }
                Button {
                    id: selectFolderButton
                    Layout.fillHeight: true
                    Layout.preferredWidth: 30
                    flat: true
                    icon.name: "document-open-folder"

                    onClicked: {
                        backupFolderDialog.open()
                        backupFolderDialog.currentFolder = model.path
                    }


                }

                TextField {
                    id: editPathTextField
                    visible: false
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    text: model.path
                    horizontalAlignment: Qt.AlignLeft
                    verticalAlignment: Qt.AlignVCenter


                    placeholderText: qsTr("Enter a path to back up to")


                    onEditingFinished: {
                        //if (!activeFocus) {
                        //accepted()
                        //}
                        console.log("editing finished")

                        model.path = text

                        saveBackupPathsToSettings()


                        delegateRoot.state = ""
                    }

                    //Keys.priority: Keys.AfterItem

                    Keys.onPressed: {
                        if (event.key === Qt.Key_Return){
                            console.log("Return key pressed path")
                            editingFinished()
                            event.accepted = true
                        }
                        if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                            console.log("Ctrl Return key pressed title")

                            backupFolderDialog.open()
                            backupFolderDialog.currentFolder = model.path
                            event.accepted = true
                        }
                    }

                }
            }


            states: [
                State {
                    name: "edit_path"
                    PropertyChanges {
                        target: pathLabel
                        visible: false
                    }
                    PropertyChanges {
                        target: editPathTextField
                        visible: true
                    }
                }
            ]


        }
    }

    backupPathListView.highlight:  Component {
        id: highlight
        Rectangle {

            radius: 5
            border.color:  "lightsteelblue"
            border.width: 2
            visible: backupPathListView.activeFocus
            Behavior on y {
                SpringAnimation {
                    spring: 3
                    damping: 0.2
                }
            }
        }
    }





    // backup every :

    backUpEveryCheckBox.checked: SkrSettings.backupSettings.backUpEveryCheckBoxChecked
    Binding {
        target: SkrSettings.backupSettings
        property: "backUpEveryCheckBoxChecked"
        value: backUpEveryCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    backUpEveryCheckBox.onCheckedChanged:{
        checkOnBackUpEveryCheckBox()
    }

    function checkOnBackUpEveryCheckBox(){
        backupHoursDial.enabled = backUpEveryCheckBox.checked
        backupHoursSpinBox.enabled = backUpEveryCheckBox.checked

    }

    //backUp Once A Day CheckBox
    backUpOnceADayCheckBox.checked: SkrSettings.backupSettings.backUpOnceADay
    Binding {
        target: SkrSettings.backupSettings
        property: "backUpOnceADay"
        value: backUpOnceADayCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    backUpOnceADayCheckBox.onCheckedChanged:{
        if(backUpOnceADayCheckBox.checked){
            backUpOnceADayIfNeeded()
        }
    }

    //dials :

    backupHoursDial.onMoved: backupHoursSpinBox.value = backupHoursDial.value
    backupHoursSpinBox.onValueModified: backupHoursDial.value = backupHoursSpinBox.value


    backupHoursDial.value: SkrSettings.backupSettings.backUpEveryHours
    backupHoursSpinBox.value: SkrSettings.backupSettings.backUpEveryHours
    Binding {
        delayed: true
        target: SkrSettings.backupSettings
        property: "backUpEveryHours"
        value: backupHoursDial.value
        restoreMode: Binding.RestoreBindingOrValue

    }

    // -------------------------
    // backup action


    Timer{
        id: backupTimer
        repeat: true
        running: backUpEveryCheckBox.checked
        interval: backupHoursSpinBox.value * 60 * 60 * 1000
        onTriggered: {
            backUpAction.trigger()
        }
    }

    Connections {
        target: plmData.projectHub()
        function onProjectLoaded(projectId){
                backUpOnceADayIfNeeded()

        }
    }


    // once a day :
    function backUpOnceADayIfNeeded(){
        if(!backUpOnceADayCheckBox.checked){
            return
        }
        var backupPaths = SkrSettings.backupSettings.paths
        var backupPathList = backupPaths.split(";")

        //no backup path set
        if (backupPaths === ""){
            //TODO: send notification, backup not configured

            return
        }

        var projectIdList = plmData.projectHub().getProjectIdList()
        var projectCount = plmData.projectHub().getProjectCount()


        // all projects :
        var i;
        for (i = 0; i < projectCount ; i++ ){
            var projectId = projectIdList[i]


            //no project path
            if (plmData.projectHub().getPath(projectId) === ""){
                //TODO: send notification, project not yet saved once

                break
            }

            // in all backup paths :
            var j;
            for (j = 0; j < backupPathList.length ; j++ ){
                var path = backupPathList[j]


                if (path === ""){
                    //TODO: send notification
                    continue
                }



                // check if wanted backup exists already at paths
                var isBackupThere = plmData.projectHub().doesBackupOfTheDayExistAtPath(projectId, skrQMLTools.getURLFromLocalFile(path))

                if(isBackupThere){
                    break
                }

                // back up :

                var error = plmData.projectHub().backupAProject(projectId, "skrib", skrQMLTools.getURLFromLocalFile(path))

                if (error.getErrorCode() === "E_PROJECT_path_is_readonly"){

                }

            }
        }
    }



    // --------------------------------------------
    // ---- save --------------------------------
    // --------------------------------------------


    saveEveryCheckBox.checked: SkrSettings.saveSettings.saveEveryCheckBoxChecked
    Binding {
        target: SkrSettings.saveSettings
        property: "saveEveryCheckBoxChecked"
        value: saveEveryCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    saveEveryCheckBox.onCheckedChanged:{
        checkOnSaveEveryCheckBox()
    }

    function checkOnSaveEveryCheckBox(){
        saveDial.enabled = saveEveryCheckBox.checked
        saveSpinBox.enabled = saveEveryCheckBox.checked

    }

    //dials :

    saveDial.onMoved: saveSpinBox.value = saveDial.value
    saveSpinBox.onValueModified: saveDial.value = saveSpinBox.value


    saveDial.value: SkrSettings.saveSettings.saveEveryMinutes
    saveSpinBox.value: SkrSettings.saveSettings.saveEveryMinutes
    Binding {
        delayed: true
        target: SkrSettings.saveSettings
        property: "saveEveryMinutes"
        value: saveDial.value
        restoreMode: Binding.RestoreBindingOrValue

    }


    // -------------------------
    // save action


    Timer{
        id: saveTimer
        repeat: true
        running: saveEveryCheckBox.checked
        interval: saveSpinBox.value * 60 * 1000
        onTriggered: {
            saveAction.trigger()
        }
    }

    // --------------------------------------------
    // ---- special E-Paper --------------------------------
    // --------------------------------------------

    setTextCursorUnblinkingCheckBox.checked: SkrSettings.ePaperSettings.textCursorUnblinking
    Binding {
        target: SkrSettings.ePaperSettings
        property: "textCursorUnblinking"
        value: setTextCursorUnblinkingCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }
    // --------------------------------------------
    // ---- advanced --------------------------------
    // --------------------------------------------

    Settings {
        id: leftDockSettings
        category : "writeLeftDock"
        property var dockSplitView
        property bool navigationFrameFolded
        property int width
    }


    Settings {
        id: rightDockSettings
        category: "writeRightDock"
        property var dockSplitView
        property bool editFrameFolded
        property bool notePadFrameFolded
        property bool tagPadFrameFolded
        property int width
        //        property bool documentFrameFolded: documentFrame.folded ? true : false
    }


    resetDockConfButton.onClicked: {

        Globals.resetDockConfCalled()

    }


    // --------------------------------------------
    // ---- spell checking --------------------------------
    // --------------------------------------------


    checkSpellingCheckBox.checked: SkrSettings.spellCheckingSettings.spellCheckingActivation
    Binding {
        target: SkrSettings.spellCheckingSettings
        property: "spellCheckingActivation"
        value: checkSpellingCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


}
