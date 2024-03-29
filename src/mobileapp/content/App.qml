

/****************************************************************************
**
** Copyright (C) 2021 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of Qt Quick Studio Components.
**
** $QT_BEGIN_LICENSE:GPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 3 or (at your option) any later version
** approved by the KDE Free Qt Foundation. The licenses are as published by
** the Free Software Foundation and appearing in the file LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/
import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtQuick.Layouts
import QtQml
import Qt.labs.settings 1.1
import Qt.labs.platform 1.1 as LabPlatform
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import Skribisto
import SkrControls
import theme

ApplicationWindow {

    id: rootWindow
    objectName: "rootWindow"
    minimumHeight: 500
    minimumWidth: 600
    visible: true

    color: SkrTheme.pageBackground

    property int projectIdToBeLoaded: -1
    property int treeItemIdToBeLoaded: -1
    property string pageTypeToBeLoaded: ""
    property var additionalPropertiesForViewManager: ({})

    property int windowId: -1

    Connections {
        target: skrWindowManager
        function onWindowIdsChanged() {
            windowId = skrWindowManager.getWindowId(this)
        }
    }

    Component.onCompleted: {
        SkrTheme // instanciate singleton

        skrWindowManager.subscribeWindow(rootWindow)
        windowId = skrWindowManager.getWindowId(this)

        rootWindow.visibility = rootWindow.visibility

        if (rootWindow.visibility === Window.FullScreen) {
            fullscreenAction.checked = true
        }

        if (rootWindow.visibility === Window.Hidden) {
            rootWindow.visibility = Window.AutomaticVisibility
        }
        if (windowId === 0) {
            var openingSomethingInArgument = this.openArgument()
        }
        rootWindow.raise()

        toBeLoadedTimer.start()

        // First Steps wizard
        loader_firstStepsWizard.active = SkrSettings.interfaceSettings.firstLaunch
        SkrSettings.interfaceSettings.firstLaunch = false

        if (!loader_firstStepsWizard.active && windowId === 0
                && !openingSomethingInArgument) {
            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showRecentPageCalled()
        }
    }

    Timer {
        id: toBeLoadedTimer
        interval: 100
        onTriggered: {
            if (projectIdToBeLoaded !== -1 && treeItemIdToBeLoaded !== -1) {
                rootPage.viewManager.insertAdditionalPropertyDict(
                            additionalPropertiesForViewManager)
                rootPage.viewManager.loadTreeItemAt(projectIdToBeLoaded,
                                                    treeItemIdToBeLoaded,
                                                    Qt.TopLeftCorner)
            } else if (projectIdToBeLoaded !== -1 && treeItemIdToBeLoaded === -1
                       && pageTypeToBeLoaded !== "") {
                rootPage.viewManager.insertAdditionalPropertyDict(
                            additionalPropertiesForViewManager)
                rootPage.viewManager.loadProjectDependantPageAt(
                            projectIdToBeLoaded, pageTypeToBeLoaded,
                            Qt.TopLeftCorner)
            } else if (projectIdToBeLoaded === -1 && treeItemIdToBeLoaded === -1
                       && pageTypeToBeLoaded !== "") {
                rootPage.viewManager.insertAdditionalPropertyDict(
                            additionalPropertiesForViewManager)
                rootPage.viewManager.loadProjectIndependantPageAt(
                            pageTypeToBeLoaded, Qt.TopLeftCorner)
            }
        }
    }

    Component.onDestruction: {
        skrWindowManager.unSubscribeWindow(rootWindow)
    }

    //------------------------------------------------------------------
    //--------- First Steps Wizard --------------------------------------------------
    //------------------------------------------------------------------
    Component {
        id: component_firstStepsWizard
        FirstStepsWizard {
            id: firstStepsWizard

            onClosed: loader_firstStepsWizard.active = false
        }
    }
    Loader {
        id: loader_firstStepsWizard
        sourceComponent: component_firstStepsWizard
        active: false
    }

    Connections {
        target: rootWindow.protectedSignals
        function onShowFirstStepsWizardCalled(page) {
            loader_firstStepsWizard.active = true
            loader_firstStepsWizard.item.setPage(page)
        }
    }

    Action {
        id: firstStepsAction
        text: qsTr("First steps")
        icon {
            source: ""
        }

        onTriggered: {
            protectedSignals.showFirstStepsWizardCalled("")
        }
    }

    //------------------------------------------------------------------
    //--------- window actions --------------------------------------------------
    //------------------------------------------------------------------
    QtObject {
        id: windowActionPrivate
        property int compactWidthLimit: 1000
    }

    property bool compactMode: false

    onWidthChanged: {
        width < windowActionPrivate.compactWidthLimit ? compactMode = true : compactMode = false
    }
    onCompactModeChanged: console.log("compact = " + compactMode)

    //Menu
    signal openMainMenuCalled
    signal openSubMenuCalled(var menu)

    //Focus
    signal forceFocusOnEscapePressed

    Shortcut {
        sequence: "Esc"
        context: Qt.WindowShortcut
        onActivated: {
            forceFocusOnEscapePressed()
        }
    }

    //drawers:
    signal closeRightDrawerCalled
    signal closeLeftDrawerCalled

    signal setNavigationTreeItemIdCalled(int projectId, int treeItemId)
    signal setNavigationTreeItemParentIdCalled(int projectId, int treeItemParentId)

    property alias protectedSignals: protectedSignals
    QtObject {
        id: protectedSignals

        // Welcome
        signal openWelcomePopupCalled
        signal closeWelcomePopupCalled

        // project:
        signal showNewProjectPageCalled
        signal showRecentPageCalled
        signal showPrintPageCalled
        signal showImportPageCalled
        signal showExportPageCalled
        // examples:
        signal showExamplePageCalled
        // settings:
        signal showSettingsPageCalled
        // help:
        signal showHelpPageCalled
        signal showHelpContentsCalled
        signal showFaqCalled
        signal showAboutCalled
        signal showAboutQtCalled

        signal showFirstStepsWizardCalled(string page)

        signal fullScreenCalled(bool value)

        signal openThemePageCalled
        signal setBreadcrumbCurrentTreeItemCalled(int projectId, int treeItemId)

        Component.onCompleted: {

        }
    }

    //------------------------------------------------------------------
    //---------window title---------
    //------------------------------------------------------------------
    Connections {
        target: skrData.projectHub()
        function onActiveProjectChanged(projectId) {

            rootWindow.title = "Skribisto - %1".arg(
                        skrData.projectHub().getProjectName(projectId))
        }
    }

    Connections {
        target: skrData.projectHub()
        function onProjectNameChanged(projectId, newTitle) {
            var activeProjectId = skrData.projectHub().getActiveProject()
            if (projectId === activeProjectId) {
                rootWindow.title = "Skribisto - %1".arg(
                            skrData.projectHub().getProjectName(projectId))
            }
        }
    }

    //-------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------
    //-------------------------------------------------------------------------------
    property alias viewManager: rootPage.viewManager

    RootPage {
        id: rootPage
        anchors.fill: parent
    }

    //------------------------------------------------------------------
    //---------Fullscreen---------
    //------------------------------------------------------------------
    property bool isDistractionFree: false

    Connections {
        target: protectedSignals
        function onFullScreenCalled(value) {
            console.log("fullscreen")
            if (value) {

                visibility = Window.FullScreen
            } else {
                visibility = Window.AutomaticVisibility
            }
            rootWindow.isDistractionFree = value
            SkrTheme.setDistractionFree(value)
        }
    }
    Action {

        id: fullscreenAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "fullscreen")
        text: skrShortcutManager.description("fullscreen")
        icon {
            source: "icons/3rdparty/backup/view-fullscreen.svg"
        }

        checkable: true
        onCheckedChanged: {
            protectedSignals.fullScreenCalled(fullscreenAction.checked)
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("fullscreen")
        context: Qt.WindowShortcut
        onActivated: fullscreenAction.trigger()
    }

    //------------------------------------------------------------------
    //--------- Read arguments ---------------------------
    //------------------------------------------------------------------
    property url testProjectFileName: "qrc:/testfiles/skribisto_test_project.skrib"
    function openArgument() {

        var arg
        var arguments
        var isTestProject = false
        var oneProjectInArgument = false
        var projectInArgument = ""
        var openingSomethingInArgument = false

        arguments = Qt.application.arguments
        for (arg in arguments) {
            if (arg === 0) {
                continue
            }

            //console.log("argument : " , arguments[arg])
            if (arguments[arg] === "--testProject") {
                //                var result = skrData.projectHub().loadProject(
                //                            testProjectFileName)
                //TODO: temporary until async is done
                Globals.loadingPopupCalled()
                openArgumentTimer.fileName = testProjectFileName
                openArgumentTimer.start()

                //show Write window
                //                writeOverviewWindowAction.trigger()
                isTestProject = true
                openingSomethingInArgument = true
            } else if (arguments[arg].slice(-6) === ".skrib") {

                oneProjectInArgument = true

                //console.log("argument skrib : " , arguments[arg])
                var url = skrQMLTools.getURLFromLocalFile(arguments[arg])
                //console.log("argument skrib url : " , url)
                //TODO: temporary until async is done
                Globals.loadingPopupCalled()
                openArgumentTimer.fileName = url
                openArgumentTimer.start()
                openingSomethingInArgument = true
            } else if (arguments[arg] === "--firstStepsAtPluginPage") {
                openFirstStepWithPluginPageTimer.start()
                openingSomethingInArgument = true
            }
        }
        //        if(!isTestProject & oneProjectInArgument){
        //            var result = skrData.projectHub().loadProject(
        //                        projectInArgument)
        //            //show Write window
        //            //            writeOverviewWindowAction.trigger()
        //        }

        // if create empty project at start is true :
        if (!isTestProject & !oneProjectInArgument & skrData.projectHub(
                    ).getProjectCount(
                    ) === 0 & SkrSettings.behaviorSettings.createEmptyProjectAtStart === true) {
            //TODO: temporary until async is done
            Globals.loadingPopupCalled()
            openArgumentTimer.fileName = ""
            openArgumentTimer.start()
            openingSomethingInArgument = true

            //skrData.projectHub().loadProject("")

            //show Write window
            //            writeOverviewWindowAction.trigger()
        }

        return openingSomethingInArgument
    }

    Timer {
        id: openFirstStepWithPluginPageTimer
        interval: 110
        onTriggered: {
            protectedSignals.showFirstStepsWizardCalled("pluginPage")
        }
    }

    //TODO: temporary until async is done
    Timer {
        id: openArgumentTimer

        property url fileName

        repeat: false
        interval: 100
        onTriggered: {
            var result = skrData.projectHub().loadProject(fileName)
            console.log("project loaded : " + result.success)
            console.log("projectFileName :",
                        testProjectFileName.toString(), "\n")

            //skrData.projectHub().setProjectName(1, "test")
            if (!result.success) {
                return
            }
            var projectId = result.getData("projectId", -1)

            //            rootPage.viewManager.loadTreeItemAt(projectId, 1, Qt.TopLeftCorner)
            //            rootPage.viewManager.loadTreeItemAt(projectId, 8, Qt.BottomLeftCorner)
        }
    }

    //------------------------------------------------------------------
    //---------Show minimap scrollbar---------------------------
    //------------------------------------------------------------------
    Action {

        id: minimapAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "show-minimap-scrollbar")
        text: skrShortcutManager.description("show-minimap-scrollbar")
        icon {
            source: "icons/3rdparty/backup/view-preview.svg"
        }

        //shortcut: StandardKey.FullScreen
        checkable: true
        checked: SkrSettings.minimapSettings.visible
        onCheckedChanged: {
            SkrSettings.minimapSettings.visible = minimapAction.checked
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("show-minimap-scrollbar")
        context: Qt.WindowShortcut
        onActivated: minimapAction.trigger()
    }

    //------------------------------------------------------------------
    //---------Center vertically text cursor---------------------------
    //------------------------------------------------------------------
    Action {

        id: centerTextCursorAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "center-vert-text-cursor")
        text: skrShortcutManager.description("center-vert-text-cursor")
        icon {
            source: "icons/3rdparty/backup/format-align-vertical-center.svg"
        }

        //shortcut: StandardKey.FullScreen
        checkable: true
        checked: SkrSettings.behaviorSettings.centerTextCursor
        onCheckedChanged: {
            SkrSettings.behaviorSettings.centerTextCursor = centerTextCursorAction.checked
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("center-vert-text-cursor")
        context: Qt.WindowShortcut
        onActivated: centerTextCursorAction.trigger()
    }

    //------------------------------------------------------------------
    //--------- Themes---------------------------------------------
    //------------------------------------------------------------------
    Action {
        id: themesColorAction
        text: qsTr("Themes")
        icon {
            source: "icons/3rdparty/backup/color-picker-white.svg"
        }

        onTriggered: {
            protectedSignals.openThemePageCalled()
        }
    }

    Connections {
        target: rootWindow.protectedSignals
        function onOpenThemePageCalled() {
            viewManager.loadProjectIndependantPage("THEME")
        }
    }

    //------------------------------------------------------------------
    //---------Help Content---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showUserManualAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "user-manual")
        text: skrShortcutManager.description("user-manual")
        icon {
            source: "icons/3rdparty/backup/system-help.svg"
            height: 50
            width: 50
        }

        onTriggered: {
            console.log("show user manual")
            Qt.openUrlExternally("https://manual.skribisto.eu/manual")
        }
    }
    Shortcut {
        sequences: skrShortcutManager.shortcuts("user-manual")
        context: Qt.WindowShortcut
        onActivated: showUserManualAction.trigger()
    }

    //------------------------------------------------------------------
    //---------FAQ---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showFaqAction
        text: qsTr("&FAQ")
        icon {
            source: "icons/3rdparty/backup/question.svg"
        }

        onTriggered: {
            console.log("show FAQ")
            Qt.openUrlExternally("https://manual.skribisto.eu/faq")
        }
    }

    //------------------------------------------------------------------
    //---------Website---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showSkribistoWebsiteAction
        text: qsTr("&Skribisto website")
        icon {
            source: "icons/skribisto.svg"
            color: "transparent"
        }

        onTriggered: {
            console.log("show Skribisto website")
            Qt.openUrlExternally("https://www.skribisto.eu")
        }
    }

    //------------------------------------------------------------------
    //---------Discord---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showDiscordAction
        text: qsTr("&Discord")
        icon {
            source: "icons/Discord-Logo-Color.svg"
        }

        onTriggered: {
            console.log("show Discord")
            Qt.openUrlExternally("https://discord.gg/5BSkvQmyVH")
        }
    }

    //------------------------------------------------------------------
    //---------GitHub---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showGitHubAction
        text: qsTr("&GitHub")
        icon {
            source: "icons/Octicons-mark-github.svg"
        }

        onTriggered: {
            console.log("show GitHub")
            Qt.openUrlExternally("https://github.com/jacquetc/skribisto")
        }
    }

    //------------------------------------------------------------------
    //---------Donate---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showDonateAction
        text: qsTr("&Donate")
        icon {
            source: "icons/Love_Heart_symbol.svg"
        }

        onTriggered: {
            console.log("show Donate")
            Qt.openUrlExternally("https://www.skribisto.eu/index.php/donate/")
        }
    }

    //------------------------------------------------------------------
    //--------- About---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showAboutAction
        text: qsTr("&About")
        icon {
            source: "icons/3rdparty/backup/help-about.svg"
            height: 50
            width: 50
        }

        onTriggered: {
            console.log("show about")
            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showHelpPageCalled()
            protectedSignals.showAboutCalled()
        }
    }

    //------------------------------------------------------------------
    //--------- About Qt---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: showAboutQtAction
        text: qsTr("About &Qt")

        //        icon {
        //            source: "icons/3rdparty/backup/system-help.svg"
        //            height: 50
        //            width: 50
        //        }
        onTriggered: {
            console.log("show about Qt")
            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showHelpPageCalled()
            protectedSignals.showAboutQtCalled()
        }
    }

    //------------------------------------------------------------------
    //---------New project---------------------------------------------
    //------------------------------------------------------------------
    Action {

        id: newProjectAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "new-project")
        text: skrShortcutManager.description("new-project")
        icon {
            source: "icons/3rdparty/backup/document-new.svg"
        }

        //shortcut: StandardKey.New
        onTriggered: {
            console.log("New Project")
            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showNewProjectPageCalled()
        }
    }
    Shortcut {
        sequences: skrShortcutManager.shortcuts("new-project")
        context: Qt.WindowShortcut
        onActivated: newProjectAction.trigger()
    }

    //------------------------------------------------------------------
    //--------- Check Spelling ---------
    //------------------------------------------------------------------
    Action {

        id: checkSpellingAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "check-spelling")
        text: skrShortcutManager.description("check-spelling")
        icon {
            source: "icons/3rdparty/backup/tools-check-spelling.svg"
            height: 50
            width: 50
        }
        checkable: true
        checked: SkrSettings.spellCheckingSettings.spellCheckingActivation

        onCheckedChanged: {
            console.log("check spelling", checkSpellingAction.checked)

            SkrSettings.spellCheckingSettings.spellCheckingActivation = checkSpellingAction.checked
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("check-spelling")
        context: Qt.WindowShortcut
        onActivated: checkSpellingAction.trigger()
    }

    //------------------------------------------------------------------
    //---------Open project---------
    //------------------------------------------------------------------
    Action {

        id: openProjectAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "open-project")
        text: skrShortcutManager.description("open-project")
        icon {
            source: "icons/3rdparty/backup/document-open.svg"
        }

        onTriggered: {
            console.log("Open Project")
            openFileDialog.open()
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("open-project")
        context: Qt.WindowShortcut
        onActivated: openProjectAction.trigger()
    }

    LabPlatform.FileDialog {

        id: openFileDialog
        title: qsTr("Open an existing project")
        folder: LabPlatform.StandardPaths.writableLocation(
                    LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.OpenFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = openFileDialog.file

            if (skrData.projectHub().isURLAlreadyLoaded(file)) {

            } else {
                //TODO: temporary until async is done
                Globals.loadingPopupCalled()
                openProjectTimer.fileName = file
                openProjectTimer.start()
            }
        }
        onRejected: {

        }
    }

    //TODO: temporary until async is done
    Timer {
        id: openProjectTimer

        property url fileName

        interval: 100
        onTriggered: {
            var result = skrData.projectHub().loadProject(fileName)
        }
    }

    //------------------------------------------------------------------
    //---------Save---------
    //------------------------------------------------------------------
    Action {

        id: saveAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "save-project")
        text: skrShortcutManager.description("save-project")
        icon {
            source: "icons/3rdparty/backup/document-save.svg"
        }
        enabled: false

        //shortcut: StandardKey.Save
        onTriggered: {
            var projectId = skrData.projectHub().getActiveProject()
            var result = skrData.projectHub().saveProject(projectId)

            if (result.containsErrorCodeDetail("no_path")) {
                saveAsFileDialog.open()
            }
        }
    }
    Shortcut {
        sequences: skrShortcutManager.shortcuts("save-project")
        context: Qt.WindowShortcut
        onActivated: saveAction.trigger()
    }

    Connections {
        target: skrData.projectHub()
        function onActiveProjectChanged(projectId) {
            if (!skrData.projectHub().isProjectNotModifiedOnce(projectId)) {
                saveAction.enabled = true
            } else {
                saveAction.enabled = false
            }
        }
    }

    Connections {
        target: skrData.projectHub()
        function onProjectNotSavedAnymore(projectId) {
            if (projectId === skrData.projectHub().getActiveProject()
                    && !skrData.projectHub().isProjectNotModifiedOnce(
                        projectId)) {
                saveAction.enabled = true
            }
        }
    }

    Connections {
        target: skrData.projectHub()
        function onProjectSaved(projectId) {
            if (projectId === skrData.projectHub().getActiveProject()) {
                saveAction.enabled = false
            }
        }
    }

    //------------------------------------------------------------------
    //---------Save All---------
    //------------------------------------------------------------------
    Action {

        id: saveAllAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "save-all-project")
        text: skrShortcutManager.description("save-all-project")
        icon {
            source: "icons/3rdparty/backup/document-save-all.svg"
        }
        enabled: false

        onTriggered: {
            var projectIdList = skrData.projectHub().getProjectIdList()
            var projectCount = skrData.projectHub().getProjectCount()

            var i
            for (i = 0; i < projectCount; i++) {
                var projectId = projectIdList[i]
                var result = skrData.projectHub().saveProject(projectId)

                if (result.containsErrorCodeDetail("no_path")) {
                    var errorProjectId = result.getData("projectId", -2)
                    saveAsFileDialog.projectId = errorProjectId
                    saveAsFileDialog.open()
                }
            }
        }
    }
    Shortcut {
        sequences: skrShortcutManager.shortcuts("save-all-project")
        context: Qt.ApplicationShortcut
        onActivated: saveAllAction.trigger()
    }

    Connections {
        target: skrData.projectHub()
        function onIsThereAnyLoadedProjectChanged(value) {
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
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "save-as-project")
        text: skrShortcutManager.description("save-as-project")
        icon {
            source: "icons/3rdparty/backup/document-save-as.svg"
        }
        enabled: false

        //shortcut: StandardKey.SaveAs
        onTriggered: {
            var projectId = skrData.projectHub().getActiveProject()
            saveACopyFileDialog.projectId = projectId
            saveACopyFileDialog.projectName = skrData.projectHub(
                        ).getProjectName(projectId)
            saveAsFileDialog.open()

            if (!skrData.projectHub().getPath(projectId)
                    && skrQMLTools.isURLSchemeQRC(skrData.projectHub().getPath(
                                                      projectId))) {
                saveAsFileDialog.folder = LabPlatform.StandardPaths.writableLocation(
                            LabPlatform.StandardPaths.DocumentsLocation)
            } else {
                saveAsFileDialog.currentFile = skrQMLTools.translateURLToLocalFile(
                            skrData.projectHub().getPath(projectId))
            }
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("save-as-project")
        context: Qt.WindowShortcut
        onActivated: saveAsAction.trigger()
    }

    LabPlatform.FileDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveAsFileDialog
        title: qsTr("Save the \"%1\" project as …").arg(projectName)
        folder: LabPlatform.StandardPaths.writableLocation(
                    LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveAsFileDialog.file.toString()

            if (file.indexOf(".skrib") === -1) {
                // not found
                file = file + ".skrib"
            }
            if (projectId == -2) {
                projectId = skrData.projectHub().getActiveProject()
            }

            if (projectName == "") {
                projectName = skrData.projectHub().getProjectName(
                            skrData.projectHub().getActiveProject())
            }

            var result = skrData.projectHub().saveProjectAs(
                        projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")) {
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
            source: "icons/3rdparty/backup/document-save-as-template.svg"
        }

        //shortcut: StandardKey.SaveAs
        onTriggered: {
            var projectId = skrData.projectHub().getActiveProject()
            saveACopyFileDialog.projectId = projectId
            saveACopyFileDialog.projectName = skrData.projectHub(
                        ).getProjectName(projectId)
            saveACopyFileDialog.open()
        }
    }

    LabPlatform.FileDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveACopyFileDialog
        title: qsTr("Save a copy of the \"%1\" project as …").arg(projectName)
        folder: LabPlatform.StandardPaths.writableLocation(
                    LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveACopyFileDialog.file.toString()

            if (file.indexOf(".skrib") === -1) {
                // not found
                file = file + ".skrib"
            }
            if (projectId == -2) {
                projectId = skrData.projectHub().getActiveProject()
            }
            console.log("FileDialog :", projectId)

            if (projectName == "") {
                projectName = skrData.projectHub().getProjectName(
                            skrData.projectHub().getActiveProject())
            }

            var result = skrData.projectHub().saveAProjectCopy(
                        projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")) {
                // Dialog:
                saveACopyFileDialog.open()
            }
        }
        onRejected: {

        }
    }
    SimpleDialog {
        id: pathIsReadOnlySaveACopydialog
        title: qsTr("Error")
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

    //-----------------------------------------------------------------------------
    //--------- Splash screen  ----------------------------------------------------
    //-----------------------------------------------------------------------------
    Item {
        id: splash
        parent: Overlay.overlay
        property int timeoutInterval: 100
        signal timeout
        x: (Overlay.overlay.width - splashImage.width) / 2
        y: (Overlay.overlay.height - splashImage.height) / 2
        width: splashImage.width
        height: splashImage.height

        Image {
            id: splashImage
            source: "icons/skribisto.svg"

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
            interval: splash.timeoutInterval
            running: true
            repeat: false
            onTriggered: {
                splash.visible = false
                splash.timeout()
            }
        }
        Component.onCompleted: splash.visible = true
    }

    //-----------------------------------------------------------------------------
    //--------- Loading Modal Popup  --------------------------------------------
    //-----------------------------------------------------------------------------
    SkrPopup {
        id: loadingPopup
        parent: Overlay.overlay
        property int timeoutInterval: 3000
        signal timeout
        anchors.centerIn: Overlay.overlay
        height: 200

        modal: true

        closePolicy: Popup.NoAutoClose

        background: Rectangle {

            radius: 10
            color: SkrTheme.pageBackground
        }

        contentItem: ColumnLayout {
            width: childrenRect.width

            SkrLabel {
                id: loadingPopupLabel

                Layout.alignment: Qt.AlignHCenter

                text: "<h1>" + qsTr("Loading a project") + "</h1>"
                focus: true
            }

            //            AnimatedSprite {
            //                id: loadingPopupAnimation
            //                Layout.alignment: Qt.AlignHCenter
            //                width: 200
            //                height: 200
            //                frameHeight: 22
            //                frameWidth: 22
            //                frameRate: 10
            //                frameCount: 8

            //                source: "icons/3rdparty/backup/process-working.svg"

            //            }
            SkrBusyIndicator {
                Layout.alignment: Qt.AlignHCenter
                running: loadingPopup.visible
            }
        }

        Timer {
            id: loadingPopupTimeoutTimer
            interval: loadingPopup.timeoutInterval
            repeat: false
            onTriggered: {
                loadingPopup.visible = false
                loadingPopup.timeout()
            }
        }
    }

    //TODO: temporary until async is done
    Connections {
        target: Globals
        function onLoadingPopupCalled() {
            loadingPopup.open()
            loadingPopupTimeoutTimer.start()
        }
    }

    Connections {
        target: skrData.projectHub()
        function onProjectToBeLoaded() {
            loadingPopup.open()
            loadingPopupTimeoutTimer.start()
        }
    }
    Connections {
        target: skrData.projectHub()
        function onProjectLoaded(projectId) {
            closeLoadingPopupTimer.start()
        }
    }

    // delay close of loading popup to let the time to switch to the sheet tab
    Timer {
        id: closeLoadingPopupTimer
        interval: 1000
        repeat: false
        onTriggered: {
            loadingPopup.close()
            loadingPopupTimeoutTimer.stop()
        }
    }

    //------------------------------------------------------------
    //------------Back up-----------------------------------
    //------------------------------------------------------------
    Action {

        id: backUpAction
        text: qsTr("Back up")
        icon {
            source: "icons/3rdparty/backup/tools-media-optical-burn-image.svg"
        }

        //shortcut: StandardKey.SaveAs
        onTriggered: {

            var backupPaths = SkrSettings.backupSettings.paths
            var backupPathList = backupPaths.split(";")

            //no backup path set
            if (backupPaths === "") {
                skrData.errorHub().addWarning(
                            qsTr(
                                "Back up failed: The backup is not configured"))

                return
            }

            var projectIdList = skrData.projectHub().getProjectIdList()
            var projectCount = skrData.projectHub().getProjectCount()

            // all projects :
            var i
            for (i = 0; i < projectCount; i++) {
                var projectId = projectIdList[i]

                //no project path
                if (!skrData.projectHub().getPath(projectId)) {
                    skrData.errorHub().addWarning(
                                qsTr("Back up failed:  the project must be saved at least once"))

                    break
                }

                // in all backup paths :
                var j
                for (j = 0; j < backupPathList.length; j++) {
                    var path = skrQMLTools.getURLFromLocalFile(
                                backupPathList[j])

                    if (!path) {
                        skrData.errorHub().addWarning(
                                    qsTr("Back up failed: The backup path %1 can't be used").arg(
                                        backupPathList[j]))
                        continue
                    }

                    var result = skrData.projectHub().backupAProject(projectId,
                                                                     "skrib",
                                                                     path)

                    if (result.containsErrorCodeDetail("path_is_readonly")) {
                        skrData.errorHub().addWarning(
                                    qsTr("Back up failed: The backup path %1 is read only").arg(
                                        path))
                    }
                    if (result.isSuccess()) {
                        skrData.errorHub().addOk(qsTr("Back up successful"))
                    }
                }
            }
        }
    }

    //------------------------------------------------------------
    //------------show settings-----------------------------------
    //------------------------------------------------------------
    Action {
        id: showSettingsAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "settings")
        text: skrShortcutManager.description("settings")
        icon {
            source: "icons/3rdparty/backup/configure.svg"
        }

        onTriggered: {
            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showSettingsPageCalled()
        }
    }

    Shortcut {
        enabled: skrRootItem.hasPrintSupport()
        sequences: skrShortcutManager.shortcuts("settings")
        context: Qt.WindowShortcut
        onActivated: showSettingsAction.trigger()
    }
    //------------------------------------------------------------
    //------------Print project-----------------------------------
    //------------------------------------------------------------
    Action {
        id: printAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "print")
        text: skrShortcutManager.description("print")
        icon {
            source: "icons/3rdparty/backup/document-print.svg"
        }
        enabled: skrRootItem.hasPrintSupport()

        onTriggered: {
            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showPrintPageCalled()
        }
    }

    Shortcut {
        enabled: skrRootItem.hasPrintSupport()
        sequence: skrShortcutManager.shortcuts("print")
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
            source: "icons/3rdparty/backup/document-import.svg"
        }

        //shortcut: StandardKey
        onTriggered: {
            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showImportPageCalled()
        }
    }
    //------------------------------------------------------------
    //------------Export project-----------------------------------
    //------------------------------------------------------------
    Action {
        id: exportAction
        text: qsTr("&Export")
        icon {
            source: "icons/3rdparty/backup/document-export.svg"
        }

        //shortcut: StandardKey.New
        onTriggered: {

            protectedSignals.openWelcomePopupCalled()
            protectedSignals.showExportPageCalled()
        }
    }

    //------------------------------------------------------------
    //------------Close project-----------------------------------
    //------------------------------------------------------------
    Connections {
        target: Globals
        function onCloseProjectCalled(projectId) {
            closeProject(projectId)
        }
    }

    function closeProject(projectId) {

        var savedBool = skrData.projectHub().isProjectSaved(projectId)
        if (savedBool || skrData.projectHub().isProjectNotModifiedOnce(
                    projectId)) {
            skrData.projectHub().closeProject(projectId)
        } else {
            saveOrNotBeforeClosingProjectDialog.projectId = projectId
            saveOrNotBeforeClosingProjectDialog.projectName = skrData.projectHub(
                        ).getProjectName(projectId)
            saveOrNotBeforeClosingProjectDialog.open()
        }
    }

    SimpleDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveOrNotBeforeClosingProjectDialog
        title: qsTr("Warning")
        text: qsTr("The project %1 is not saved. Do you want to save it before quitting ?").arg(
                  projectName)
        standardButtons: Dialog.Save | Dialog.Discard | Dialog.Cancel

        onRejected: {
            saveOrNotBeforeClosingProjectDialog.close()
        }

        onDiscarded: {
            skrData.projectHub().closeProject(projectId)
            saveOrNotBeforeClosingProjectDialog.close()
        }

        onAccepted: {

            var result = skrData.projectHub().saveProject(projectId)
            if (result.containsErrorCodeDetail("no_path")) {
                var errorProjectId = result.getData("projectId", -2)
                saveAsBeforeClosingProjectFileDialog.projectId = errorProjectId
                saveAsBeforeClosingProjectFileDialog.projectName = skrData.projectHub(
                            ).getProjectName(projectId)
                saveAsBeforeClosingProjectFileDialog.open()
                saveAsBeforeClosingProjectFileDialog.currentFile
                        = LabPlatform.StandardPaths.writableLocation(
                            LabPlatform.StandardPaths.DocumentsLocation)[0]
            } else {
                skrData.projectHub().closeProject(projectId)
            }
            saveOrNotBeforeClosingProjectDialog.close()
        }
    }

    LabPlatform.FileDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveAsBeforeClosingProjectFileDialog
        title: qsTr("Save the %1 project as …").arg(projectName)
        folder: LabPlatform.StandardPaths.writableLocation(
                    LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveAsFileDialog.file.toString()

            if (file.indexOf(".skrib") === -1) {
                // not found
                file = file + ".skrib"
            }
            if (projectId == -2) {
                projectId = skrData.projectHub().getActiveProject()
            }
            console.log("FileDialog :", projectId)

            if (projectName == "") {
                projectName = skrData.projectHub().getProjectName(
                            skrData.projectHub().getActiveProject())
            }

            var result = skrData.projectHub().saveProjectAs(
                        projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")) {
                // Dialog:
                pathIsReadOnlydialog.open()
            } else {
                skrData.projectHub().closeProject(projectId)
                saveOrNotBeforeClosingProjectDialog.close()
            }
        }
        onRejected: {
            skrData.projectHub().closeProject(projectId)
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
        enabled: skrData.projectHub().isThereAnyLoadedProject
        icon {
            source: "icons/3rdparty/backup/document-close.svg"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.New
        onTriggered: {
            console.log("Close Project")
            var activeProjectId = skrData.projectHub().getActiveProject()
            closeProject(activeProjectId)
        }
    }

    Connections {
        target: skrData.projectHub()
        function onActiveProjectChanged() {
            activeProjectName = skrData.projectHub().getProjectName(
                        skrData.projectHub().getActiveProject())
        }
    }
    Connections {
        target: skrData.projectHub()
        function onProjectNameChanged(projectId, newTitle) {
            var activeProjectId = skrData.projectHub().getActiveProject()
            if (projectId === activeProjectId) {
                activeProjectName = skrData.projectHub().getProjectName(
                            skrData.projectHub().getActiveProject())
            }
        }
    }

    //------------------------------------------------------------
    //------------Quit logic-----------------------------------
    //------------------------------------------------------------
    Connections {
        target: Globals
        function onQuitCalled() {
            rootWindow.close()
        }
    }

    Action {
        id: quitAction
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "quit")
        text: skrShortcutManager.description("quit")
        icon {
            source: "icons/3rdparty/backup/window-close.svg"
            color: "transparent"
        }

        //        shortcut: StandardKey.Quit
        property bool quitConfirmed: false
        onTriggered: {
            console.log("quitting")

            // determine if all projects are saved
            var projectsNotSavedList = skrData.projectHub().projectsNotSaved()
            var i
            for (i = 0; i < projectsNotSavedList.length; i++) {
                var projectId = projectsNotSavedList[i]

                if (skrData.projectHub().isProjectNotModifiedOnce(projectId)) {
                    continue
                } else {

                    saveOrNotBeforeQuittingDialog.projectId = projectId
                    saveOrNotBeforeQuittingDialog.projectName = skrData.projectHub(
                                ).getProjectName(projectId)
                    saveOrNotBeforeQuittingDialog.open()
                    saveOrNotBeforeQuittingDialog.forceActiveFocus()
                    //saveAsBeforeQuittingFileDialog.currentFile = LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
                }
            }
            if (projectsNotSavedList.length === 0) {

                skrUserSettings.setSetting(
                            "window", "numberOfWindows",
                            skrWindowManager.getNumberOfWindows())
                quitConfirmed = true
                skrData.projectHub().closeAllProjects()
                Globals.quitCalled()
            }
        }
    }

    onClosing: function (close) {

        var group = "window_" + windowId
        // geometry
        skrUserSettings.setSetting(group, "x", rootWindow.x)
        skrUserSettings.setSetting(group, "y", rootWindow.y)
        skrUserSettings.setSetting(group, "width", rootWindow.width)
        skrUserSettings.setSetting(group, "height", rootWindow.height)
        skrUserSettings.setSetting(group, "visibility", rootWindow.visibility)
        rootPage.viewManager.saveViewsToSettings()

        if (skrWindowManager.getNumberOfWindows() === 1
                && quitAction.quitConfirmed === false) {
            quitAction.trigger()

            close.accepted = false
            return
        }

        skrWindowManager.deleteWindow(this)
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("quit")
        context: Qt.ApplicationShortcut
        onActivated: quitAction.trigger()
    }

    SimpleDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveOrNotBeforeQuittingDialog
        title: qsTr("Warning")
        text: qsTr("The project %1 is not saved. Do you want to save it before quitting ?").arg(
                  projectName)
        standardButtons: Dialog.Save | Dialog.Discard | Dialog.Cancel

        onRejected: {

        }

        onDiscarded: {
            skrData.projectHub().closeProject(projectId)
            quitAction.trigger()
        }

        onAccepted: {

            var result = skrData.projectHub().saveProject(projectId)
            if (result.containsErrorCodeDetail("no_path")) {
                var errorProjectId = result.getData("projectId", -2)
                saveAsBeforeQuittingFileDialog.projectId = errorProjectId
                saveAsBeforeQuittingFileDialog.projectName = skrData.projectHub(
                            ).getProjectName(projectId)
                saveAsBeforeQuittingFileDialog.open()
            } else {
                quitAction.trigger()
            }
        }
    }

    LabPlatform.FileDialog {
        property int projectId: -2
        property string projectName: ""

        id: saveAsBeforeQuittingFileDialog
        title: qsTr("Save the %1 project as …").arg(projectName)
        folder: LabPlatform.StandardPaths.writableLocation(
                    LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["Skribisto file (*.skrib)"]
        onAccepted: {

            var file = saveAsFileDialog.file.toString()

            if (file.indexOf(".skrib") === -1) {
                // not found
                file = file + ".skrib"
            }
            if (projectId == -2) {
                projectId = skrData.projectHub().getActiveProject()
            }
            console.log("FileDialog :", projectId)

            if (projectName == "") {
                projectName = skrData.projectHub().getProjectName(
                            skrData.projectHub().getActiveProject())
            }

            var result = skrData.projectHub().saveProjectAs(
                        projectId, "skrib", Qt.resolvedUrl(file))

            if (result.containsErrorCodeDetail("path_is_readonly")) {
                // Dialog:
                pathIsReadOnlydialog.open()
            } else {
                quitAction.trigger()
            }
        }
        onRejected: {
            quitAction.trigger()
        }
    }

    //------------------------------------------------------------
    //----------------------------------------------
    //------------------------------------------------------------
    property var lastFocusedItem: undefined
    onActiveFocusItemChanged: {
        if (!activeFocusItem) {
            return
        }
        var item = activeFocusItem

        if (skrEditMenuSignalHub.isSubscribed(activeFocusItem.objectName)) {
            //console.log("activeFocusItem", activeFocusItem.objectName)
            cutTextAction.enabled = true
            copyTextAction.enabled = true
            pasteTextAction.enabled = true
            lastFocusedItem = item
            return
        }

        //        console.log("item", activeFocusItem)
        //        console.log("objectName", activeFocusItem.objectName)
        if (!lastFocusedItem) {
            lastFocusedItem = item
        }
        if (skrEditMenuSignalHub.isSubscribed(lastFocusedItem.objectName)) {
            //console.log("lastFocusedItem", lastFocusedItem.objectName)
            cutTextAction.enabled = true
            copyTextAction.enabled = true
            pasteTextAction.enabled = true
        } else {
            cutTextAction.enabled = false
            copyTextAction.enabled = false
            pasteTextAction.enabled = false
        }
        lastFocusedItem = item
    }

    //-------------------------------------------------------------------
    Action {
        id: cutTextAction
        text: skrShortcutManager.description("cut")
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "cut")
        icon {
            source: "icons/3rdparty/backup/edit-cut.svg"
        }

        onTriggered: {
            skrEditMenuSignalHub.cutActionTriggered()
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("cut")
        context: Qt.WindowShortcut
        onActivated: cutTextAction.trigger()
    }

    //-------------------------------------------------------------------
    Action {
        id: copyTextAction
        text: skrShortcutManager.description("copy")
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "copy")
        icon {
            source: "icons/3rdparty/backup/edit-copy.svg"
        }

        onTriggered: {
            skrEditMenuSignalHub.copyActionTriggered()
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("copy")
        context: Qt.WindowShortcut
        onActivated: copyTextAction.trigger()
    }

    //-------------------------------------------------------------------
    Action {
        id: pasteTextAction
        text: skrShortcutManager.description("paste")
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "paste")
        icon {
            source: "icons/3rdparty/backup/edit-paste.svg"
        }

        onTriggered: {
            skrEditMenuSignalHub.pasteActionTriggered()
        }
    }

    Shortcut {
        sequences: skrShortcutManager.shortcuts("paste")
        context: Qt.WindowShortcut
        onActivated: pastetextAction.trigger()
    }

    //-------------------------------------------------------------------
    Action {
        property bool preventTrigger: false
        property bool actionTriggeredVolontarily: false

        id: italicAction
        text: skrShortcutManager.description("italic")
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "italic")
        icon {
            source: "icons/3rdparty/backup/format-text-italic.svg"
        }

        checkable: true

        onCheckedChanged: {
            if (preventTrigger) {
                return
            }
            skrEditMenuSignalHub.italicActionTriggered(italicAction.checked)
        }
    }
    Shortcut {
        sequence: skrShortcutManager.shortcuts("italic")
        context: Qt.WindowShortcut
        onActivated: italicAction.trigger()
    }

    //-------------------------------------------------------------------
    Action {
        property bool preventTrigger: false

        id: boldAction
        text: skrShortcutManager.description("bold")
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "bold")
        icon {
            source: "icons/3rdparty/backup/format-text-bold.svg"
        }

        checkable: true

        onCheckedChanged: {

            if (preventTrigger) {
                return
            }
            skrEditMenuSignalHub.boldActionTriggered(boldAction.checked)
        }
    }

    Shortcut {
        sequence: skrShortcutManager.shortcuts("bold")
        context: Qt.WindowShortcut
        onActivated: boldAction.trigger()
    }

    //-------------------------------------------------------------------
    Action {
        property bool preventTrigger: false

        id: strikeAction
        text: skrShortcutManager.description("strike")
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "strike")
        icon {
            source: "icons/3rdparty/backup/format-text-strikethrough.svg"
        }

        checkable: true

        onCheckedChanged: {
            if (preventTrigger) {
                return
            }
            skrEditMenuSignalHub.strikeActionTriggered(strikeAction.checked)
        }
    }

    Shortcut {
        sequence: skrShortcutManager.shortcuts("strike")
        context: Qt.WindowShortcut
        onActivated: strikeAction.trigger()
    }

    //-------------------------------------------------------------------
    Action {
        property bool preventTrigger: false

        id: underlineAction
        text: skrShortcutManager.description("underline")
        property string shortcutText: skrShortcutManager.nativeShortcutsToString(
                                          "underline")
        icon {
            source: "icons/3rdparty/backup/format-text-underline.svg"
        }

        checkable: true

        onCheckedChanged: {
            if (preventTrigger) {
                return
            }
            skrEditMenuSignalHub.underlineActionTriggered(
                        underlineAction.checked)
        }
    }
    Shortcut {
        sequence: skrShortcutManager.shortcuts("underline")
        context: Qt.WindowShortcut
        onActivated: underlineAction.trigger()
    }
}
