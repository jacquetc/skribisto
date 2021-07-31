import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material 2.15
import ".."
import "../Items"
import "../Commons"

SettingsPageForm {
    signal closeCalled()

    Component.onCompleted: {
        determineAvailableTranslations()
        determineCurrentTranslation()
        checkOnBackUpEveryCheckBox()
        populateBackupPathListView()

        checkOnSaveEveryCheckBox()
        backUpOnceADayIfNeeded()


        populateCheckSpellingComboBox()
        checkSpellingComboBox.currentIndex = checkSpellingComboBox.indexOfValue(SkrSettings.spellCheckingSettings.spellCheckingLangCode)

        loadFontFamily()
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



    // interface languages :

    function determineAvailableTranslations(){
        langModel.clear()

        var translationsMap = skrRootItem.findAvailableTranslationsMap()

        for(var translation in translationsMap){
            langModel.append({"text": translationsMap[translation] + " (" + translation + ")", "langCode": translation})
        }
    }
    function determineCurrentTranslation(){
        langComboBox.currentIndex = langComboBox.indexOfValue(skrRootItem.getLanguageFromSettings())
    }

    ListModel {
        id: langModel
    }


    langComboBox.model: langModel
    langComboBox.textRole: "text"
    langComboBox.valueRole: "langCode"
    langComboBox.onCurrentValueChanged: {
        if(langComboBox.activeFocus){
            skrRootItem.currentTranslationLanguageCode = langComboBox.currentValue
            skrWindowManager.retranslate()

        }
    }


    Connections {
        target: skrRootItem
        function onCurrentTranslationLanguageCodeChanged(langCode){
            langComboBox.currentIndex = langComboBox.indexOfValue(langCode)
        }
    }

    openThemePageButton.onClicked: {
        rootWindow.protectedSignals.openThemePageCalled()
        closeCalled()
    }



    // --------------------------------------------
    // ---- Behavior --------------------------------
    // --------------------------------------------


    createEmpyProjectAtStartSwitch.checked: SkrSettings.behaviorSettings.createEmptyProjectAtStart
    Binding {
        target: SkrSettings.behaviorSettings
        property: "createEmptyProjectAtStart"
        value: createEmpyProjectAtStartSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }



    centerTextCursorSwitch.checked: SkrSettings.behaviorSettings.centerTextCursor
    Binding {
        target: SkrSettings.behaviorSettings
        property: "centerTextCursor"
        value: centerTextCursorSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


    // plugins:


    pluginPageButton.onClicked: {
        loader_pluginPagePopup.active = true
    }

    Component {
        id: component_pluginPagePopup
        SkrPopup {
            property alias closeButton: inner_closeButton


            id: pluginPagePopup
            parent: Overlay.overlay
            width: Overlay.overlay.width >= 1000 ? 1000 : Overlay.overlay.width
            height: Overlay.overlay.height >= 1000 ? 1000 : Overlay.overlay.height
            anchors.centerIn: Overlay.overlay

            modal: true
            visible: true

            closePolicy: Popup.CloseOnEscape
            onClosed: loader_pluginPagePopup.active = false

            onOpened: {
                pluginPage.forceActiveFocus()
            }

            background: Rectangle {

                radius: 10
                color: SkrTheme.pageBackground

            }
            contentItem: SkrPane {
                anchors.fill: parent
                clip: true
                ColumnLayout {
                    anchors.fill: parent

                    SkrToolButton {
                        id: inner_closeButton
                        text: qsTr("Close")
                        focusPolicy: Qt.NoFocus
                        display: AbstractButton.IconOnly
                        Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                        icon {
                            source: "qrc:///icons/backup/arrow-down.svg"
                        }

                        onClicked: pluginPagePopup.close()
                    }

                    PluginPage {
                        id: pluginPage
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                    }

                    SkrLabel{
                        Layout.alignment: Qt.AlignHCenter
                        text: qsTr("Please restart to apply changes.")
                    }
                }
            }
        }
    }
    Loader {
        id: loader_pluginPagePopup
        sourceComponent: component_pluginPagePopup
        active: false
    }


    firstStepsButton.action: firstStepsAction

    // --------------------------------------------
    // ---- accessibility --------------------------------
    // --------------------------------------------

    accessibilityCheckBox.checked: SkrSettings.accessibilitySettings.accessibilityEnabled
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "accessibilityEnabled"
        value: accessibilityCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    accessibilityCheckBox.onCheckedChanged: {
        showMenuButtonCheckBox.checked = accessibilityCheckBox.checked
    }

    showMenuButtonCheckBox.checked: SkrSettings.accessibilitySettings.showMenuButton
    Binding {
        target: SkrSettings.accessibilitySettings
        property: "showMenuButton"
        value: showMenuButtonCheckBox.checked
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
                continue
            }

            backupPathListModel.append({"path": skrQMLTools.getURLFromLocalFile(path), "localPath": path})

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
        icon.source: "qrc:///icons/backup/list-add.svg"
        onTriggered: {


            addBackupFolderDialog.open()
        }
    }
    addBackupPathButton.action: addBackupPathAction



    Action {
        id: removeBackupPathAction
        text: qsTr("Remove backup path")
        icon.source: "qrc:///icons/backup/list-remove.svg"
        onTriggered: {

            removeBackupPath(backupPathListView.currentIndex)
        }
    }
    removeBackupPathButton.action: removeBackupPathAction




    backupPathListView.delegate: Component {
        id: itemDelegate

        Item {
            id: delegateRoot
            height: Material.delegateHeight



            anchors {
                left: Qt.isQtObject(parent) ? parent.left : undefined
                right: Qt.isQtObject(parent) ? parent.right : undefined
                leftMargin: 5
                rightMargin: 5
            }

            TapHandler {
                id: tapHandler
                onSingleTapped: function(eventPoint) {
                    backupPathListView.currentIndex = model.index
                    delegateRoot.forceActiveFocus()
                    eventPoint.accepted = true
                }
                onDoubleTapped: function(eventPoint) {

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

                    model.path = backupFolderDialog.folder
                    model.localPath = skrQMLTools.translateURLToLocalFile(backupFolderDialog.folder)
                }
                onRejected: {

                }



            }

            RowLayout{
                anchors.fill: parent

                SkrLabel {
                    id: pathLabel
                    activeFocusOnTab: false
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    text: model.localPath
                    horizontalAlignment: Qt.AlignLeft
                    verticalAlignment: Qt.AlignVCenter
                }
                SkrButton {
                    id: selectFolderButton
                    Layout.fillHeight: true
                    Layout.preferredWidth: 30
                    flat: true
                    icon.source: "qrc:///icons/backup/document-open.svg"

                    onClicked: {
                        backupFolderDialog.open()
                        backupFolderDialog.currentFolder = model.path
                    }


                }

                SkrTextField {
                    id: editPathTextField
                    visible: false
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    text: model.localPath
                    horizontalAlignment: Qt.AlignLeft
                    verticalAlignment: Qt.AlignVCenter


                    placeholderText: qsTr("Enter a path to back up to")


                    onEditingFinished: {
                        //if (!activeFocus) {
                        //accepted()
                        //}
                        //console.log("editing finished")

                        model.path = text

                        saveBackupPathsToSettings()


                        delegateRoot.state = ""
                    }

                    //Keys.priority: Keys.AfterItem

                    Keys.onPressed: function(event) {
                        if (event.key === Qt.Key_Return){
                            //console.log("Return key pressed path")
                            editingFinished()
                            event.accepted = true
                        }
                        if ((event.modifiers & Qt.CtrlModifier) && event.key === Qt.Key_Return){
                            //console.log("Ctrl Return key pressed title")

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
                enabled: SkrSettings.ePaperSettings.animationEnabled
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
        target: skrData.projectHub()
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

        var projectIdList = skrData.projectHub().getProjectIdList()
        var projectCount = skrData.projectHub().getProjectCount()


        // all projects :
        var i;
        for (i = 0; i < projectCount ; i++ ){
            var projectId = projectIdList[i]


            //no project path
            if (skrData.projectHub().getPath(projectId) === ""){
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
                var isBackupThere = skrData.projectHub().doesBackupOfTheDayExistAtPath(projectId, skrQMLTools.getURLFromLocalFile(path))

                if(isBackupThere){
                    break
                }

                // back up :

                var result = skrData.projectHub().backupAProject(projectId, "skrib", skrQMLTools.getURLFromLocalFile(path))

                if (result.containsErrorCodeDetail("path_is_readonly")){
                    //TODO: notification
                }

            }
        }
    }


    // -------------------------------------------------------
    // ---- minimap scrollbar --------------------------------
    // --------------------------------------------------------



    showMinimapCheckBox.checked: SkrSettings.minimapSettings.visible
    Binding {
        target: SkrSettings.minimapSettings
        property: "visible"
        value: showMinimapCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    //dials :

    minimapDividerDial.onMoved: minimapDividerSpinBox.value = minimapDividerDial.value
    minimapDividerSpinBox.onValueModified: minimapDividerDial.value = minimapDividerSpinBox.value


    minimapDividerDial.value: SkrSettings.minimapSettings.divider
    minimapDividerSpinBox.value: SkrSettings.minimapSettings.divider
    Binding {
        delayed: true
        target: SkrSettings.minimapSettings
        property: "divider"
        value: minimapDividerDial.value
        restoreMode: Binding.RestoreBindingOrValue

    }

    // ------------------------------------------------------
    // ---- save -------------------------------------------
    // -------------------------------------------------------


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
    // ---- Quick print --------------------------------
    // --------------------------------------------



    // textPointSizeSlider :

    textPointSizeSlider.value: SkrSettings.quickPrintSettings.textPointSize


    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textPointSize"
        value: textPointSizeSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Font family combo :
    fontFamilyComboBox.model: skrFonts.fontFamilies()

    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textFontFamily"
        value: fontFamilyComboBox.currentText
        when:  fontFamilyLoaded
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    property bool fontFamilyLoaded: false

    function loadFontFamily(){
        var fontFamily = SkrSettings.quickPrintSettings.textFontFamily
        //console.log("fontFamily", fontFamily)
        //console.log("application fontFamily", Qt.application.font.family)

        var index = fontFamilyComboBox.find(fontFamily, Qt.MatchFixedString)
        //console.log("index", index)
        if(index === -1){
            index = fontFamilyComboBox.find("Liberation Serif", Qt.MatchFixedString)
        }
        if(index === -1){
            index = fontFamilyComboBox.find(skrRootItem.defaultFontFamily(), Qt.MatchContains)
        }
        //console.log("index", index)

        fontFamilyComboBox.currentIndex = index
        fontFamilyLoaded = true
    }

    // Indent :
    textIndentSlider.value: SkrSettings.quickPrintSettings.textIndent

    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textIndent"
        value: textIndentSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Margins :
    textTopMarginSlider.value: SkrSettings.quickPrintSettings.textTopMargin

    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textTopMargin"
        value: textTopMarginSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
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


    animationEnabledCheckBox.checked: SkrSettings.ePaperSettings.animationEnabled
    Binding {
        target: SkrSettings.ePaperSettings
        property: "animationEnabled"
        value: animationEnabledCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


    // --------------------------------------------
    // ---- advanced --------------------------------
    // --------------------------------------------

    //    Settings {
    //        id: leftDockSettings
    //        category : "writeLeftDock"
    //        property var dockSplitView
    //        property bool navigationFrameFolded
    //        property int width
    //    }


    //    Settings {
    //        id: rightDockSettings
    //        category: "writeRightDock"
    //        property var dockSplitView
    //        property bool editFrameFolded
    //        property bool notePadFrameFolded
    //        property bool tagPadFrameFolded
    //        property int width
    //        //        property bool documentFrameFolded: documentFrame.folded ? true : false
    //    }


    devModeCheckBox.checked: SkrSettings.devSettings.devModeEnabled
    Binding {
        target: SkrSettings.devSettings
        property: "devModeEnabled"
        value: devModeCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


    // --------------------------------------------
    // ---- spell checking --------------------------------
    // --------------------------------------------

    checkSpellingCheckBox.action: checkSpellingAction


    // combo box :

    SKRSpellChecker {
        id : spellChecker
    }

    ListModel {
        id: checkSpellingComboBoxModel
    }

    checkSpellingComboBox.model: checkSpellingComboBoxModel
    checkSpellingComboBox.textRole: "text"
    checkSpellingComboBox.valueRole: "dictCode"

    function populateCheckSpellingComboBox(){

        var dictList = spellChecker.dictList()

        var i;
        for(i = 0 ; i < dictList.length ; i++){
            checkSpellingComboBoxModel.append({"text": dictList[i], "dictCode": dictList[i]})
        }

    }


    Connections {
        target: SkrSettings.spellCheckingSettings
        function onSpellCheckingLangCodeChanged(){
            var value = SkrSettings.spellCheckingSettings.spellCheckingLangCode
            checkSpellingComboBox.currentIndex = checkSpellingComboBox.indexOfValue(value)
        }
    }
    checkSpellingComboBox.onCurrentValueChanged: {
        if(checkSpellingComboBox.activeFocus){
            SkrSettings.spellCheckingSettings.spellCheckingLangCode = langComboBox.currentValue
        }
    }



}
