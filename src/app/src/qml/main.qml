import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQml 2.15
//import QtQuick.Dialogs 1.3
import Qt.labs.settings 1.1
import Qt.labs.platform 1.1 as LabPlatform
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import QtQuick.Controls.Material 2.15
import "Commons"

ApplicationWindow {

    id: rootWindow
    objectName: "rootWindow"
    //visible: true
    minimumHeight: 500
    minimumWidth: 600

    onHeightChanged: Globals.height = height
    onWidthChanged: Globals.width = width

    x: settings.x
    y: settings.y
    height: settings.height
    width: settings.width


    Material.background: SkrTheme.pageBackground


    visibility: settings.visibility
    Settings {
        id: settings
        category: "window"
        property int x: 0
        property int y: 0
        property int height: Screen.height
        property int width: Screen.width
        property int visibility: Window.Maximized
    }

    Component.onCompleted: {
        SkrTheme // instanciate singleton

        if(rootWindow.visibility === Window.FullScreen){
            fullscreenAction.checked = true
        }
    }



    //------------------------------------------------------------------
    //---------window title---------
    //------------------------------------------------------------------

    Connections{
        target: plmData.projectHub()
        function onActiveProjectChanged(projectId){

            rootWindow.title = "Skribisto - %1".arg(plmData.projectHub().getProjectName(projectId))
        }
    }


    //------------------------------------------------------------------
    //---------Fullscreen---------
    //------------------------------------------------------------------


    Action {

        id: fullscreenAction
        text: qsTr("Fullscreen")
        icon {
            source: "qrc:///icons/backup/view-fullscreen"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.FullScreen
        checkable: true
        onCheckedChanged: {
            Globals.fullScreen = fullscreenAction.checked
            Globals.fullScreenCalled(fullscreenAction.checked)
        }
    }

    Shortcut {
        sequences: [StandardKey.FullScreen, "F11"]
        context: Qt.ApplicationShortcut
        onActivated: fullscreenAction.trigger()
    }

    //------------------------------------------------------------------
    //---------Center vertically text cursor---------------------------
    //------------------------------------------------------------------

    Action {

        id: centerTextCursorAction
        text: qsTr("Center vertically the text cursor")
        icon {
            source: "qrc:///icons/backup/format-align-vertical-center.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.FullScreen
        checkable: true
        onCheckedChanged: {
            SkrSettings.behaviorSettings.centerTextCursor = centerTextCursorAction.checked

        }
    }

    Shortcut {
        sequence: "Alt+C"
        context: Qt.ApplicationShortcut
        onActivated: centerTextCursorAction.trigger()
    }

    //------------------------------------------------------------------
    //---------Help Content---------------------------------------------
    //------------------------------------------------------------------

    Action {

        id: showHelpContentAction
        text: qsTr("&Contents")
        icon {
            source: "qrc:///icons/backup/system-help.svg"
            height: 50
            width: 50
        }

        onTriggered: {
            console.log("show help content")
            Globals.showWelcomePageCalled()
            Globals.showHelpPageCalled()
            Globals.showHelpContentsCalled()
        }


    }
    Shortcut {
        sequence:  StandardKey.HelpContents
        context: Qt.ApplicationShortcut
        onActivated: showHelpAction.trigger()
    }

    //------------------------------------------------------------------
    //---------FAQ---------------------------------------------
    //------------------------------------------------------------------

    Action {

        id: showFaqAction
        text: qsTr("&FAQ")
//        icon {
//            source: "qrc:///icons/backup/system-help.svg"
//            height: 50
//            width: 50
//        }

        onTriggered: {
            console.log("show FAQ")
            Globals.showWelcomePageCalled()
            Globals.showHelpPageCalled()
            Globals.showFaqCalled()
        }

    }
    //------------------------------------------------------------------
    //--------- About---------------------------------------------
    //------------------------------------------------------------------

    Action {

        id: showAboutAction
        text: qsTr("&About")
        icon {
            source: "qrc:///icons/backup/help-about.svg"
            height: 50
            width: 50
        }

        onTriggered: {
            console.log("show about")
            Globals.showWelcomePageCalled()
            Globals.showHelpPageCalled()
            Globals.showAboutCalled()
        }

    }

    //------------------------------------------------------------------
    //--------- About Qt---------------------------------------------
    //------------------------------------------------------------------

    Action {

        id: showAboutQtAction
        text: qsTr("About &Qt")
//        icon {
//            source: "qrc:///icons/backup/system-help.svg"
//            height: 50
//            width: 50
//        }

        onTriggered: {
            console.log("show about Qt")
            Globals.showWelcomePageCalled()
            Globals.showHelpPageCalled()
            Globals.showAboutQtCalled()
        }

    }

    //------------------------------------------------------------------
    //---------New project---------------------------------------------
    //------------------------------------------------------------------

    Action {

        id: newProjectAction
        text: qsTr("&New Project")
        icon {
            source: "qrc:///icons/backup/document-new.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.New
        onTriggered: {
            console.log("New Project")
            Globals.showWelcomePageCalled()
            Globals.showWelcomeProjectPageCalled()
            Globals.showNewProjectWizardCalled()
        }


    }
    Shortcut {
        sequence:  StandardKey.New
        context: Qt.ApplicationShortcut
        onActivated: newProjectAction.trigger()
    }
    //------------------------------------------------------------------
    //--------- Check Spelling ---------
    //------------------------------------------------------------------

    Action {

        id: checkSpellingAction
        text: qsTr("&Check spelling")
        icon {
            source: "qrc:///icons/backup/tools-check-spelling.svg"
            height: 50
            width: 50
        }
        checkable: true
        checked:     SkrSettings.spellCheckingSettings.spellCheckingActivation



        onCheckedChanged: {
            console.log("check spelling", checkSpellingAction.checked)

            SkrSettings.spellCheckingSettings.spellCheckingActivation = checkSpellingAction.checked
        }
    }


    Shortcut {
        sequence:  "Shift+F7"
        context: Qt.ApplicationShortcut
        onActivated: checkSpellingAction.trigger()
    }

    //------------------------------------------------------------------
    //---------Open project---------
    //------------------------------------------------------------------

    Action {

        id: openProjectAction
        text: qsTr("&Open Project")
        icon {
            source: "qrc:///icons/backup/document-open.svg"
            height: 50
            width: 50
        }

        shortcut: StandardKey.Open
        onTriggered: {
            console.log("Open Project")
            openFileDialog.open()

        }



    }

    LabPlatform.FileDialog{


        id: openFileDialog
        title: qsTr("Open an existing project")
        modality: Qt.ApplicationModal
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.OpenFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = openFileDialog.file


            if(plmData.projectHub().isURLAlreadyLoaded(file)){

            }
            else {

                var result = plmData.projectHub().loadProject(file)
            }

        }
        onRejected: {

        }
    }
    //------------------------------------------------------------------
    //---------Save---------
    //------------------------------------------------------------------

    Action {

        id: saveAction
        text: qsTr("Save")
        icon {
            source: "qrc:///icons/backup/document-save.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.Save
        onTriggered: {
            var projectId = plmData.projectHub().getActiveProject()
            var result = plmData.projectHub().saveProject(projectId)

            if (result.containsErrorCodeDetail("no_path")){
                saveAsFileDialog.open()
            }



        }
    }
    Shortcut {
        sequence: StandardKey.Save
        context: Qt.ApplicationShortcut
        onActivated: saveAction.trigger()

    }

    Connections {
        target: plmData.projectHub()
        function onActiveProjectChanged(projectId){
            if (!plmData.projectHub().isProjectNotModifiedOnce(projectId)){
                saveAction.enabled = true
            }
            else{
                saveAction.enabled = false
            }
        }

    }

    Connections {
        target: plmData.projectHub()
        function onProjectNotSavedAnymore(projectId){
            if (projectId === plmData.projectHub().getActiveProject()
                    && !plmData.projectHub().isProjectNotModifiedOnce(projectId)){
                saveAction.enabled = true
            }
        }

    }

    Connections {
        target: plmData.projectHub()
        function onProjectSaved(projectId){
            if (projectId === plmData.projectHub().getActiveProject()){
                saveAction.enabled = false
            }
        }

    }



    //------------------------------------------------------------------
    //---------Save All---------
    //------------------------------------------------------------------

    Action {

        id: saveAllAction
        text: qsTr("Save All")
        icon {
            source: "qrc:///icons/backup/document-save-all.svg"
            height: 50
            width: 50
        }

        //shortcut: "Ctrl+Shift+S"
        onTriggered: {
            var projectIdList = plmData.projectHub().getProjectIdList()
            var projectCount = plmData.projectHub().getProjectCount()

            var i;
            for (i = 0; i < projectCount ; i++ ){
                var projectId = projectIdList[i]
                var result = plmData.projectHub().saveProject(projectId)

                if (result.containsErrorCodeDetail("no_path")){
                    var errorProjectId = result.getData("projectId", -2);
                    saveAsFileDialog.projectId = errorProjectId
                    saveAsFileDialog.open()
                }
            }
        }
    }
    Shortcut {
        sequence: qsTr("Ctrl+Shift+S")
        context: Qt.ApplicationShortcut
        onActivated: saveAllAction.trigger()

    }

    Connections {
        target: plmData.projectHub()
        function onIsThereAnyLoadedProjectChanged(value){
            saveAction.enabled = value
            saveAsAction.enabled = value
            saveAllAction.enabled = value

        }

    }


    //------------------------------------------------------------------
    //---------Save As---------
    //------------------------------------------------------------------

    Action {

        id: saveAsAction
        text: qsTr("Save As...")
        icon {
            source: "qrc:///icons/backup/document-save-as.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.SaveAs
        onTriggered: {
            var projectId = plmData.projectHub().getActiveProject()
            saveACopyFileDialog.projectId = projectId
            saveACopyFileDialog.projectName = plmData.projectHub().getProjectName(projectId)
            saveAsFileDialog.open()


            if(!plmData.projectHub().getPath(projectId) && skrQMLTools.isURLSchemeQRC(plmData.projectHub().getPath(projectId))){
                saveAsFileDialog.folder = LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
            }
            else {
                saveAsFileDialog.currentFile = skrQMLTools.translateURLToLocalFile(plmData.projectHub().getPath(projectId))
            }

        }
    }

    Shortcut {
        sequence: StandardKey.SaveAs
        context: Qt.ApplicationShortcut
        onActivated: saveAsAction.trigger()

    }

    LabPlatform.FileDialog{
        property int projectId: -2
        property string projectName: ""

        id: saveAsFileDialog
        title: qsTr("Save the \"%1\" project as ...").arg(projectName)
        modality: Qt.ApplicationModal
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveAsFileDialog.file.toString()

            if(file.indexOf(".skrib") === -1){ // not found
                file = file + ".skrib"
            }
            if(projectId == -2){
                projectId = plmData.projectHub().getActiveProject()
            }


            if(projectName == ""){
                projectName = plmData.projectHub().getProjectName(plmData.projectHub().getActiveProject())
            }

            var result = plmData.projectHub().saveProjectAs(projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")){
                // Dialog:
                pathIsReadOnlydialog.open()

            }

        }
        onRejected: {

        }
    }
    SimpleDialog {
        id: pathIsReadOnlydialog
        title: "Error"
        text: qsTr("This path is read-only, please choose another path.")
        onAccepted: saveAsFileDialog.open()
    }


    //------------------------------------------------------------------
    //---------Save A Copy---------
    //------------------------------------------------------------------


    Action {

        id: saveACopyAction
        text: qsTr("Save a Copy")
        icon {
            source: "qrc:///icons/backup/document-save-as-template.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.SaveAs
        onTriggered: {
            var projectId = plmData.projectHub().getActiveProject()
            saveACopyFileDialog.projectId = projectId
            saveACopyFileDialog.projectName = plmData.projectHub().getProjectName(projectId)
            saveACopyFileDialog.open()








        }
    }

    LabPlatform.FileDialog{
        property int projectId: -2
        property string projectName: ""

        id: saveACopyFileDialog
        title: qsTr("Save a copy of the \"%1\" project as ...").arg(projectName)
        modality: Qt.ApplicationModal
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveACopyFileDialog.file.toString()


            if(file.indexOf(".skrib") === -1){ // not found
                file = file + ".skrib"
            }
            if(projectId == -2){
                projectId = plmData.projectHub().getActiveProject()
            }
            console.log("FileDialog :" , projectId)

            if(projectName == ""){
                projectName = plmData.projectHub().getProjectName(plmData.projectHub().getActiveProject())
            }

            var result = plmData.projectHub().saveAProjectCopy(projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")){
                // Dialog:
                saveACopyFileDialog.open()

            }

        }
        onRejected: {

        }
    }
    SimpleDialog {
        id: pathIsReadOnlySaveACopydialog
        title: "Error"
        text: qsTr("This path is read-only, please choose another path.")
        onAccepted: saveACopyFileDialog.open()
    }
    //    Shortcut {
    //        sequence: StandardKey.Save
    //        context: Qt.ApplicationShortcut
    //        onActivated: saveAction.trigger()

    //    }

    //    Shortcut {
    //        sequence: StandardKey.Quit
    //        context: Qt.ApplicationShortcut
    //        onActivated: Qt.quit()

    //    }

    // style :
    //palette.window: "white"

    //Splash screen
    Item {
        id: splash
        parent: Overlay.overlay
        property int timeoutInterval: 1000
        signal timeout
        x: (Overlay.overlay.width - splashImage.width) / 2
        y: (Overlay.overlay.height - splashImage.height) / 2
        width: splashImage.width
        height: splashImage.height

        Image {
            id: splashImage
            source: "qrc:/pics/skribisto.svg"



            //                layer.enabled: true
            //                layer.samplerName: "maskSource"
            //                layer.effect: ShaderEffect {
            //                    property variant source: splashImage
            //                    fragmentShader: "
            //                            varying highp vec2 qt_TexCoord0;
            //                            uniform highp float qt_Opacity;
            //                            uniform lowp sampler2D source;
            //                            uniform lowp sampler2D maskSource;
            //                            void main(void) {
            //                                gl_FragColor = texture2D(source, qt_TexCoord0.st) * (1.0-texture2D(maskSource, qt_TexCoord0.st).a) * qt_Opacity;
            //                            }
            //                        "
            //                }

        }
        Timer {
            interval: splash.timeoutInterval; running: true; repeat: false
            onTriggered: {
                splash.visible = false
                splash.timeout()
            }
        }
        Component.onCompleted: splash.visible = true
    }



    RootPage {
        anchors.fill: parent
    }



    Connections {
        target: Globals
        function onFullScreenCalled(value) {
            console.log("fullscreen")
            if(value){
                visibility = Window.FullScreen
            }
            else {
                visibility = Window.AutomaticVisibility
            }

        }
    }



    //------------------------------------------------------------
    //------------Back up-----------------------------------
    //------------------------------------------------------------

    Action {

        id: backUpAction
        text: qsTr("Back up")
        icon {
            source: "qrc:///icons/backup/tools-media-optical-burn-image.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.SaveAs
        onTriggered: {


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
                if (!plmData.projectHub().getPath(projectId)){
                    //TODO: send notification, project not yet saved once

                    break
                }

                // in all backup paths :
                var j;
                for (j = 0; j < backupPathList.length ; j++ ){
                    var path = Qt.resolvedUrl(backupPathList[j])


                    if (!path){
                        //TODO: send notification
                        console.log("backup path empty")
                        continue
                    }



                    var result = plmData.projectHub().backupAProject(projectId, "skrib", path)

                    if (result.containsErrorCodeDetail("path_is_readonly")){

                    }

                }
            }

        }
    }

    //------------------------------------------------------------
    //------------Print project-----------------------------------
    //------------------------------------------------------------
    Action {
        id: printAction
        text: qsTr("&Print")
        icon {
            source: "qrc:///icons/backup/document-print.svg"
            height: 50
            width: 50
        }
        enabled: skrRootItem.hasPrintSupport()

        onTriggered: {
            Globals.showWelcomePageCalled()
            Globals.showWelcomeProjectPageCalled()
            Globals.showPrintWizardCalled()

        }
    }

    Shortcut {
        enabled: skrRootItem.hasPrintSupport()
        sequence: StandardKey.Print
        context: Qt.ApplicationShortcut
        onActivated: printAction.trigger()
    }
    //------------------------------------------------------------
    //------------Import project-----------------------------------
    //------------------------------------------------------------
    Action {
        id: importAction
        text: qsTr("&Import")
        icon {
            source: "qrc:///icons/backup/document-import.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey
        onTriggered: {
            Globals.showWelcomePageCalled()
            Globals.showWelcomeProjectPageCalled()
            Globals.showImportWizardCalled()

        }
    }
    //------------------------------------------------------------
    //------------Export project-----------------------------------
    //------------------------------------------------------------
    Action {
        id: exportAction
        text: qsTr("&Export")
        icon {
            source: "qrc:///icons/backup/document-export.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.New
        onTriggered: {
            Globals.showWelcomePageCalled()
            Globals.showWelcomeProjectPageCalled()
            Globals.showExportWizardCalled()

        }
    }

    //------------------------------------------------------------
    //------------Close project-----------------------------------
    //------------------------------------------------------------


    Connections{
        target: Globals
        function onCloseProjectCalled(projectId){
            closeProject(projectId)
        }
    }

    function closeProject(projectId){

        var savedBool = plmData.projectHub().isProjectSaved(projectId)
        if(savedBool || plmData.projectHub().isProjectNotModifiedOnce(projectId)){
            plmData.projectHub().closeProject(projectId)
        }
        else{
            saveOrNotBeforeClosingProjectDialog.projectId = projectId
            saveOrNotBeforeClosingProjectDialog.projectName = plmData.projectHub().getProjectName(projectId)
            saveOrNotBeforeClosingProjectDialog.open()
        }
    }


    SimpleDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveOrNotBeforeClosingProjectDialog
        title: "Warning"
        text: qsTr("The project %1 is not saved. Do you want to save it before quiting ?").arg(projectName)
        standardButtons: Dialog.Save  | Dialog.Discard | Dialog.Cancel

        onRejected: {
            saveOrNotBeforeClosingProjectDialog.close()

        }

        onDiscarded: {
            plmData.projectHub().closeProject(projectId)
            saveOrNotBeforeClosingProjectDialog.close()

        }

        onAccepted: {


            var result = plmData.projectHub().saveProject(projectId)
            if (result.containsErrorCodeDetail("no_path")){
                var errorProjectId = result.getData("projectId", -2);
                saveAsBeforeClosingProjectFileDialog.projectId = errorProjectId
                saveAsBeforeClosingProjectFileDialog.projectName = plmData.projectHub().getProjectName(projectId)
                saveAsBeforeClosingProjectFileDialog.open()
                saveAsBeforeClosingProjectFileDialog.currentFile = LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)[0]
            }
            else {
                plmData.projectHub().closeProject(projectId)
            }
            saveOrNotBeforeClosingProjectDialog.close()


        }


    }

    LabPlatform.FileDialog{
        property int projectId: -2
        property string projectName: ""

        id: saveAsBeforeClosingProjectFileDialog
        title: qsTr("Save the %1 project as ...").arg(projectName)
        modality: Qt.ApplicationModal
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveAsFileDialog.file.toString()

            if(file.indexOf(".skrib") === -1){ // not found
                file = file + ".skrib"
            }
            if(projectId == -2){
                projectId = plmData.projectHub().getActiveProject()
            }
            console.log("FileDialog :" , projectId)

            if(projectName == ""){
                projectName = plmData.projectHub().getProjectName(plmData.projectHub().getActiveProject())
            }

            var result = plmData.projectHub().saveProjectAs(projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")){
                // Dialog:
                pathIsReadOnlydialog.open()

            }
            else{
                plmData.projectHub().closeProject(projectId)
                saveOrNotBeforeClosingProjectDialog.close()
            }
        }
        onRejected: {
            plmData.projectHub().closeProject(projectId)
            saveOrNotBeforeClosingProjectDialog.close()
        }
    }


    //------------------------------------------------------------
    //------------Close current project-----------------------------------
    //------------------------------------------------------------

    property string activeProjectName: ""
    Action {
        id: closeCurrentProjectAction
        text: qsTr("&Close \"%1\" project").arg(activeProjectName)
        icon {
            source: "qrc:///icons/backup/document-close.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.New
        onTriggered: {
            console.log("Close Project")
            var activeProjectId = plmData.projectHub().getActiveProject()
            closeProject(activeProjectId)
        }

    }



    Connections{
        target: plmData.projectHub()
        function onActiveProjectChanged(){
            activeProjectName = plmData.projectHub().getProjectName(plmData.projectHub().getActiveProject())
        }
    }
    Connections{
        target: plmData.projectHub()
        function onProjectNameChanged(){
            activeProjectName = plmData.projectHub().getProjectName(plmData.projectHub().getActiveProject())
        }
    }

    //------------------------------------------------------------
    //------------Close logic-----------------------------------
    //------------------------------------------------------------

    Action {
        id: quitAction
        text: qsTr("&Quit")
        icon {
            source: "qrc:///icons/backup/window-close.svg"
            height: 50
            width: 50
        }

        //        shortcut: StandardKey.Quit
        onTriggered: {
            rootWindow.close()

        }
    }

    onClosing: {
        console.log("quiting")


        // determine if all projects are saved


        var projectsNotSavedList = plmData.projectHub().projectsNotSaved()
        var i;
        for (i = 0; i < projectsNotSavedList.length ; i++ ){
            var projectId = projectsNotSavedList[i]

            if(plmData.projectHub().isProjectNotModifiedOnce(projectId)){
                continue
            }
            else {

                saveOrNotBeforeQuitingDialog.projectId = projectId
                saveOrNotBeforeQuitingDialog.projectName = plmData.projectHub().getProjectName(projectId)
                saveOrNotBeforeQuitingDialog.open()
                //saveAsBeforeQuitingFileDialog.currentFile = LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
                close.accepted = false
            }

        }
        if(projectsNotSavedList.length === 0){



            // geometry
            settings.x = rootWindow.x
            settings.y = rootWindow.y
            settings.width = rootWindow.width
            settings.height = rootWindow.height
            settings.visibility = rootWindow.visibility


            close.accepted = true
        }

    }

    Shortcut {
        sequence: StandardKey.Quit
        context: Qt.ApplicationShortcut
        onActivated: quitAction.trigger()
    }


    SimpleDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveOrNotBeforeQuitingDialog
        title: "Warning"
        text: qsTr("The project %1 is not saved. Do you want to save it before quiting ?").arg(projectName)
        standardButtons: Dialog.Save  | Dialog.Discard | Dialog.Cancel

        onRejected: {

        }

        onDiscarded: {
            plmData.projectHub().closeProject(projectId)

            rootWindow.close()
        }

        onAccepted: {


            var result = plmData.projectHub().saveProject(projectId)
            if (result.containsErrorCodeDetail("no_path")){
                var errorProjectId = result.getData("projectId", -2);
                saveAsBeforeQuitingFileDialog.projectId = errorProjectId
                saveAsBeforeQuitingFileDialog.projectName = plmData.projectHub().getProjectName(projectId)
                saveAsBeforeQuitingFileDialog.open()
            }
            else {
                rootWindow.close()
            }

        }
    }

    LabPlatform.FileDialog{
        property int projectId: -2
        property string projectName: ""

        id: saveAsBeforeQuitingFileDialog
        title: qsTr("Save the %1 project as ...").arg(projectName)
        modality: Qt.ApplicationModal
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveAsFileDialog.file.toString()

            if(file.indexOf(".skrib") === -1){ // not found
                file = file + ".skrib"
            }
            if(projectId == -2){
                projectId = plmData.projectHub().getActiveProject()
            }
            console.log("FileDialog :" , projectId)

            if(projectName == ""){
                projectName = plmData.projectHub().getProjectName(plmData.projectHub().getActiveProject())
            }

            var result = plmData.projectHub().saveProjectAs(projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")){
                // Dialog:
                pathIsReadOnlydialog.open()

            }
            else{
                rootWindow.close()
            }
        }
        onRejected: {
            rootWindow.close()
        }
    }




    //------------------------------------------------------------
    //----------------------------------------------
    //------------------------------------------------------------
    property var lastFocusedItem: undefined
    onActiveFocusItemChanged: {
        if(!activeFocusItem){
            return
        }
        var item = activeFocusItem

        if(skrEditMenuSignalHub.isSubscribed(activeFocusItem.objectName)){
            //console.log("activeFocusItem", activeFocusItem.objectName)
            cutAction.enabled = true
            copyAction.enabled = true
            pasteAction.enabled = true
            lastFocusedItem = item
            return
        }

        //        console.log("item", activeFocusItem)
        //        console.log("objectName", activeFocusItem.objectName)
        if(!lastFocusedItem){
            lastFocusedItem = item
        }
        if(skrEditMenuSignalHub.isSubscribed(lastFocusedItem.objectName)){
            //console.log("lastFocusedItem", lastFocusedItem.objectName)
            cutAction.enabled = true
            copyAction.enabled = true
            pasteAction.enabled = true
        }
        else {
            cutAction.enabled = false
            copyAction.enabled = false
            pasteAction.enabled = false

        }
        lastFocusedItem = item


    }

    Action {
        id: cutAction
        text: qsTr("Cut")
        shortcut: StandardKey.Cut
        icon {
            source: "qrc:///icons/backup/edit-cut.svg"
        }

        onTriggered: {
            skrEditMenuSignalHub.cutActionTriggered()
        }
    }
    //    Shortcut{
    //        sequence: StandardKey.Cut
    //        context: Qt.ApplicationShortcut
    //        onActivated: cutAction.trigger()
    //    }

    Action {
        id: copyAction
        text: qsTr("Copy")
        shortcut: StandardKey.Copy
        icon {
            source: "qrc:///icons/backup/edit-copy.svg"
        }

        onTriggered: {
            skrEditMenuSignalHub.copyActionTriggered()
        }
    }
    //    Shortcut{
    //        sequence: StandardKey.Copy
    //        context: Qt.ApplicationShortcut
    //        onActivated: copyAction.trigger()
    //    }

    Action {
        id: pasteAction
        text: qsTr("Paste")
        shortcut: StandardKey.Paste
        icon {
            source: "qrc:///icons/backup/edit-paste.svg"
        }

        onTriggered: {
            skrEditMenuSignalHub.pasteActionTriggered()
        }
    }
    //    Shortcut{
    //        sequence: StandardKey.Paste
    //        context: Qt.ApplicationShortcut
    //        onActivated: pasteAction.trigger()
    //    }



    Action {
        property bool preventTrigger: false
        property bool actionTriggeredVolontarily: false

        id: italicAction
        text: qsTr("Italic")
        icon {
            source: "qrc:///icons/backup/format-text-italic.svg"
            height: 50
            width: 50
        }

        shortcut: StandardKey.Italic
        checkable: true

        onCheckedChanged: {
            console.log("preventTrigger", preventTrigger)
            if(preventTrigger){
                return
            }
            skrEditMenuSignalHub.italicActionTriggered(italicAction.checked)



        }
    }
    //    Shortcut{
    //        sequence: StandardKey.Italic
    //        context: Qt.ApplicationShortcut
    //        onActivated: italicAction.trigger()
    //    }


    Action {
        property bool preventTrigger: false

        id: boldAction
        text: qsTr("Bold")
        icon {
            source: "qrc:///icons/backup/format-text-bold.svg"
            height: 50
            width: 50
        }

        shortcut: StandardKey.Bold
        checkable: true

        onCheckedChanged: {

            if(preventTrigger){
                return
            }
            skrEditMenuSignalHub.boldActionTriggered(boldAction.checked)


        }
    }
    //    Shortcut{
    //        sequence: StandardKey.Bold
    //        context: Qt.ApplicationShortcut
    //        onActivated: boldAction.trigger()
    //    }


    Action {
        property bool preventTrigger: false

        id: strikeAction
        text: qsTr("Strike")
        icon {
            source: "qrc:///icons/backup/format-text-strikethrough.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey
        checkable: true

        onCheckedChanged: {
            if(preventTrigger){
                return
            }
            skrEditMenuSignalHub.strikeActionTriggered(strikeAction.checked)

        }
    }
    //        Shortcut{
    //            sequence:
    //            context: Qt.ApplicationShortcut
    //            onActivated: strikeAction.trigger()
    //        }



    Action {
        property bool preventTrigger: false

        id: underlineAction
        text: qsTr("Underline")
        icon {
            source: "qrc:///icons/backup/format-text-underline.svg"
            height: 50
            width: 50
        }

        shortcut: StandardKey.Underline
        checkable: true

        onCheckedChanged: {
            if(preventTrigger){
                return
            }
            skrEditMenuSignalHub.underlineActionTriggered(underlineAction.checked)

        }
    }
    //    Shortcut{
    //        sequence: StandardKey.Underline
    //        context: Qt.ApplicationShortcut
    //        onActivated: underlineAction.trigger()
    //    }
    //    Keys.onReleased: {
    //        if(event.key === Qt.Key_Alt){
    //            console.log("alt")
    //            Globals.showMenuBarCalled()
    //            event.accepted = true
    //        }
    //    }
    //    Shortcut{
    //        enabled: true
    //        sequence: "Alt+X"
    //        onActivated: {console.log("alt x")}

    //    }
    //    Keys.onShortcutOverride: event.accepted = ((event.modifiers & Qt.AltModifier) && event.key === Qt.Key_F)

    //    Keys.onPressed: {
    //        if ((event.modifiers & Qt.AltModifier) && event.key === Qt.Key_F){
    //            menuBar.visible = true

    //        }
    //        i

    //    }

    //    Keys.onReleased: {
    //        if ((event.modifiers & Qt.AltModifier) && event.key === Qt.Key_F){
    //            menuBar.visible = true

    //        }

    //        //        if(event.key === Qt.Key_Alt){
    //        //            console.log("alt")
    //        //            Globals.showMenuBarCalled()

    //        //            event.accepted = true
    //        //        }
    //    }









} //}

/*##^## Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
 ##^##*/

