import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import Qt.labs.settings 1.1
import eu.skribisto.skrusersettings 1.0
import eu.skribisto.plmerror 1.0
import eu.skribisto.projecthub 1.0

import "Write"
import "Welcome"
import "Note"
import "Gallery"
import "Projects"

RootPageForm {
    id: rootPage

    //    Drawer{
    //        id: drawer
    //        width: 70
    //        height: window.height
    //        modal: false
    //        interactive: true
    //        position: 0

    //        Loader{
    //            anchors.fill: parent
    //            sourceComponent: flow_comp

    //        }

    //    }


    Shortcut {
        sequence: "Esc"
        onActivated: {

            Globals.forceFocusOnEscapePressed()
        }
    }


    SkrUserSettings {
        id: skrUserSettings
    }

    ActionGroup {
        id: windowGroup
        exclusive: true

        Action {
            id: welcomeWindowAction
            text: qsTr("Welcome")
            icon {
                source: "qrc:/pics/skribisto.svg"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F5"
            checkable: true
            onTriggered: {

                welcomePage.forceActiveFocus()
            }
        }

        Action {
            id: writeWindowAction
            text: qsTr("Write")
            icon {
                name: "story-editor"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F6"
            checkable: true
            onTriggered: {

                writeOverviewPage.forceActiveFocus()
            }
        }

        Action {
            id: noteWindowAction
            text: qsTr("Note")
            icon {
                name: "story-editor"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F7"
            checkable: true
            onTriggered: {

                noteOverviewPage.forceActiveFocus()
            }
        }
        Action {
            id: galleryWindowAction
            text: qsTr("Gallery")
            icon {
                name: "view-preview"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F8"
            checkable: true
            onTriggered: {
                //                rootStack.

                galleryPage.forceActiveFocus()
            }
        }
        Action {
            id: projectWindowAction
            text: qsTr("Project")
            icon {
                name: "configure"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F9"
            checkable: true
            onTriggered: {

                projectsPage.forceActiveFocus()
            }
        }
    }

    Connections {
        target: Globals
        function onShowWelcomePage() {
            rootSwipeView.currentIndex = 0
        }
    }

    //------------------------------------------------
    // notification :
    Action{
        id: notificationButtonAction
        icon{
            name: "dialog-messages"
            width: 50
            height: 50
        }

        onTriggered: {
            //show notification list
            popup.open()

        }
    }

    notificationButton.action: notificationButtonAction

    Popup {
        id: popup
        x: notificationButton.x - 100
        y: notificationButton.y + 400
        width: 100 + notificationButton.width
        height: 400
        modal: false
        focus: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
    }


    //---------------------------------------------------------




    //---------------------------------------------------------

    saveButton.action: saveAction

    //---------------------------------------------------------

    // fullscreen :

    property bool fullScreen: false
    Connections {
        target: Globals
        function onFullScreenCalled(value) {
            if(value){
                fullScreen = true

                tabBarRevealer.visible = true
                rootTabBar.visible = false
            }
            else{
                fullScreen = false
                tabBarRevealer.visible = false
                rootTabBar.visible = true

            }

        }
    }
    //        HoverHandler{
    //            target: rootTabBar
    //            enabled: fullScreen
    //            onHoveredChanged: {
    //                console.log("hovered", hovered)
    //                if(fullScreen && !hovered){
    //                rootTabBar.visible = false
    //                }
    //            }
    //        }

    HoverHandler{
        target: tabBarRevealer
        enabled: fullScreen
        onHoveredChanged: {
            if(hovered){
                rootTabBar.visible = true
            }
        }
    }
    TapHandler{
        target: tabBarRevealer
        enabled: fullScreen
        onTapped: {
            if(pressed){
                rootTabBar.visible = true
            }
        }
    }
    //---------------------------------------------------------
    // projectLoaded :
    property int projectIdForProjectLoading: 0
    property int paperIdForProjectLoading: 0

    Connections {
        target: plmData.projectHub()
        function onProjectLoaded(projectId) {

            // get last sheet id from settings or get top sheet id
            var topPaperId = plmData.sheetHub().getTopPaperId(projectId)
            console.log("topPaperId ::", topPaperId)

            // when no result (e.g. empty table) :
            if(topPaperId === -2){
                return;
            }

            var paperId = skrUserSettings.getProjectSetting(projectId, "writeCurrentPaperId", topPaperId)
            //            console.log("paperId ::", paperId)
            //            console.log("projectId ::", projectId)
            projectIdForProjectLoading = projectId
            paperIdForProjectLoading = paperId

            // verify if the paperId really exists
            var isPresent = false
            var idList = plmData.sheetHub().getAllIds(projectId)
            var count = idList.length

            for(var i = 0; i < count ; i++ ){

                if(paperId === idList[i]){
                    isPresent = true

                }
            }
            if(!isPresent & count > 0){

                paperIdForProjectLoading = idList[0]            }
            else if(!isPresent & count === 0){

                paperIdForProjectLoading = -2

            }
            projectLoadingTimer.start()



        }
    }
    Timer{
        id: projectLoadingTimer
        repeat: false
        interval: 100
        onTriggered: Globals.openSheetInNewTabCalled(projectIdForProjectLoading, paperIdForProjectLoading)
    }
    //---------------------------------------------------------
    //---------Tab bar ------------------------------------------
    //---------------------------------------------------------


    Shortcut{
        id: tabBarShortcut
        sequence: "F4"
        onActivated: {
            rootTabBar.currentItem.forceActiveFocus()
        }
    }


    Shortcut{
        id: tabBarNextTabShortcut
        sequence: StandardKey.NextChild
        onActivated: {
            console.log("tabBarNextTabShortcut")
            rootTabBar.incrementCurrentIndex()
        }
    }



    Shortcut{
        id: tabBarPreviousTabShortcut
        sequence: StandardKey.PreviousChild
        onActivated: {
            //console.log("tabBarPreviousTabShortcut")
            rootTabBar.decrementCurrentIndex()
        }
    }


    //rootSwipeView.currentIndex: rootTabBar.currentIndex

    rootTabBar.currentIndex: rootSwipeView.currentIndex

    Binding {
        //when: rootTabBar.currentIndex !== rootSwipeView.currentIndex
        when: rootSwipeView.currentIndexChanged
        target: rootSwipeView
        property: "currentIndex"
        value: rootTabBar.currentIndex
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    rootSwipeView.onCurrentItemChanged: {
        var i;
        for(i = 0; i < rootSwipeView.count; i++ ){

            var item = rootSwipeView.itemAt(i)
            if(item === rootSwipeView.currentItem){
                item.enabled = true
            }
            else{
                item.enabled = false
            }
        }
    }


    function addTab(incubator, insertionIndex, pageType, projectId, paperId) {
        var title = incubator.object.title
        //        console.debug("debug title : ", title)
        var page  = incubator.object


        rootSwipeView.insertItem(insertionIndex, incubator);

        var component = Qt.createComponent("Tab.qml");
        var tabIncubator = component.incubateObject(rootTabBar, {text: title, pageType: pageType,projectId: projectId, paperId: paperId, height: rootTabBar.height });
        console.debug("debug : ", component.errorString())
        if (tabIncubator.status !== Component.Ready) {
            tabIncubator.onStatusChanged = function(status) {
                if (status === Component.Ready) {

                    rootTabBar.insertItem(insertionIndex, tabIncubator)
                    page.onTitleChangedString.connect(tabIncubator.object.setTitle)
                    tabIncubator.object.onCloseCalled.connect(closeTab)

                }
            }
        } else {


            rootTabBar.insertItem(insertionIndex, tabIncubator)
            page.onTitleChangedString.connect(tabIncubator.object.setTitle)
            tabIncubator.object.onCloseCalled.connect(closeTab)

        }




    }

    //---------------------------------------------------------

    function closeTab(index) {
        rootSwipeView.itemAt(index).runActionsBedoreDestruction()
        rootSwipeView.removeItem(rootSwipeView.itemAt(index))
        rootTabBar.removeItem(rootTabBar.itemAt(index))

    }


    //---------------------------------------------------------

    function closeTabsByProject(projectId) {

        var i;
        for(i = 0; i < rootTabBar.count; i++){

            if(rootTabBar.itemAt(i).projectId === projectId){

                rootSwipeView.itemAt(i).runActionsBedoreDestruction()
                rootSwipeView.removeItem(rootSwipeView.itemAt(i))
                rootTabBar.removeItem(rootTabBar.itemAt(i))
            }
        }

    }


    // close all tabs :
    Connections {
        target:plmData.projectHub()
        function onProjectToBeClosed(_projectId) {
            closeTabsByProject(_projectId)
            closeProjectSubWindows(_projectId)
        }
    }

    //---------------------------------------------------------

    welcomeTab.action: welcomeWindowAction
    writeOverviewTab.action: writeWindowAction
    noteOverviewTab.action: noteWindowAction
    galleryTab.action: galleryWindowAction
    projectTab.action: projectWindowAction

    //---------------------------------------------------------
    //------------Open Sheet-----------------------------
    //---------------------------------------------------------

    property int insertionIndex: 0
    // openDocument :
    Connections {
        target: Globals
        function onOpenSheetCalled(openedProjectId, openedPaperId, projectId, paperId) {

            if(paperId === -1){
                return
            }

            var pageType = "write"

            // verify if project/sheetId not already opened
            var tabId = pageType + "_" +  projectId + "_" + paperId
            var i;
            for (i = 0; i < rootTabBar.count; i++) {
                if (rootTabBar.itemAt(i).tabId === tabId){

                    rootTabBar.setCurrentIndex(i)

                    break
                }
            }


            // determine from which page the call comes from


            var senderTabId = pageType + "_" +  openedProjectId + "_" + openedPaperId

            var j;
            for (j = 0; j < rootTabBar.count; j++) {

                console.log(rootTabBar.itemAt(j).tabId)
                if (rootTabBar.itemAt(j).tabId === senderTabId){

                    rootSwipeView.itemAt(j).openDocument(projectId, paperId)
                    rootTabBar.itemAt(j).setTitle(plmData.sheetHub().getTitle(projectId, paperId))
                    rootTabBar.itemAt(j).projectId = projectId
                    rootTabBar.itemAt(j).paperId = paperId

                    break
                }
            }


        }
    }

    //---------------------------------------------------------
    Connections {
        target: Globals
        function onOpenSheetInNewTabCalled(projectId, paperId) {

            if(paperId === -1){
                return
            }

            var pageType = "write"
            // verify if project/sheetId not already opened
            var tabId = pageType + "_" +  projectId + "_" + paperId
            var i;
            for (i = 0; i < rootTabBar.count; i++) {
                if (rootTabBar.itemAt(i).tabId === tabId){

                    rootTabBar.setCurrentIndex(i)
                    return
                }
            }

            //

            insertionIndex = rootSwipeView.count
            // create WritePage tab
            var component = Qt.createComponent("Write/WritePage.qml");
            var incubator = component.incubateObject(rootSwipeView, {pageType: pageType, projectId: projectId, paperId: paperId });
            if (incubator.status !== Component.Ready) {
                incubator.onStatusChanged = function(status) {
                    if (status === Component.Ready) {

                        addTab(incubator, insertionIndex, pageType, projectId, paperId)
                        //                        console.debug("paprer 1 : ",paperId)
                        //                        console.debug("count 1 : ",rootSwipeView.count)
                        openTabTimer.start()
                    }
                }
            } else {

                addTab(incubator, insertionIndex,  pageType, projectId, paperId)
                openTabTimer.start()

            }

            //            console.debug("paper : ",paperId)
            //            console.debug("count : ",rootSwipeView.count)

        }
    }
    Timer{
        id: openTabTimer
        repeat: false
        interval: 10
        onTriggered: rootTabBar.setCurrentIndex(insertionIndex)
    }

    //---------------------------------------------------------
    //------------Open Note-----------------------------
    //---------------------------------------------------------


    // openDocument :
    Connections {
        target: Globals
        function onOpenNoteCalled(openedProjectId, openedPaperId, projectId, paperId) {

            if(paperId === -1){
                return
            }

            var pageType = "note"

            // verify if project/noteId not already opened
            var tabId = pageType + "_" +  projectId + "_" + paperId
            var i;
            for (i = 0; i < rootTabBar.count; i++) {
                if (rootTabBar.itemAt(i).tabId === tabId){
                    rootTabBar.setCurrentIndex(i)
                    break
                }
            }


            // determine from which page the call comes from


            var senderTabId = pageType + "_" +  openedProjectId + "_" + openedPaperId
            var j;
            for (j = 0; j < rootTabBar.count; j++) {
                if (rootTabBar.itemAt(j).tabId === senderTabId){

                    rootSwipeView.itemAt(j).openDocument(projectId, paperId)
                    rootTabBar.itemAt(j).setTitle(plmData.noteHub().getTitle(projectId, paperId))
                    rootTabBar.itemAt(j).projectId = projectId
                    rootTabBar.itemAt(j).paperId = paperId

                    break
                }
            }


        }
    }

    //---------------------------------------------------------
    Connections {
        target: Globals
        function onOpenNoteInNewTabCalled(projectId, paperId) {

            if(paperId === -1){
                return
            }


            var pageType = "note"
            // verify if project/noteId not already opened
            var tabId = pageType + "_" +  projectId + "_" + paperId
            var i;
            for (i = 0; i < rootTabBar.count; i++) {
                if (rootTabBar.itemAt(i).tabId === tabId){

                    rootTabBar.setCurrentIndex(i)
                    return
                }
            }

            //

            insertionIndex = rootSwipeView.count
            // create NotePage tab
            var component = Qt.createComponent("Note/NotePage.qml");
            var incubator = component.incubateObject(rootSwipeView, {pageType: pageType,projectId: projectId, paperId: paperId });
            if (incubator.status !== Component.Ready) {
                incubator.onStatusChanged = function(status) {
                    if (status === Component.Ready) {

                        addTab(incubator, insertionIndex, pageType, projectId, paperId)
                        //                        console.debug("paper 1 : ",paperId)
                        //                        console.debug("count 1 : ",rootSwipeView.count)
                        openTabTimer.start()
                    }
                }
            } else {

                addTab(incubator, insertionIndex,  pageType, projectId, paperId)
                openTabTimer.start()

            }

            //            console.debug("paper : ",paperId)
            //            console.debug("count : ",rootSwipeView.count)

        }

    }


    //---------------------------------------------------------

    QtObject {
        id: privateObject
        property var subscribedSubWindows: []
    }



    function getSubWindow(paperType, projectId, paperId){

        var i
        for(i = 0 ; i < privateObject.subscribedSubWindows.length ; i++){
            var window = privateObject.subscribedSubWindows[i]

            if(window.windowId === paperType + "_" +  projectId + "_" + paperId  ){
                return window
            }
        }
    }

    //---------------------------------------------------------

    function closeSubWindow(){

    }

//--------------------------------------------------------------------------

    function closeProjectSubWindows(projectId){
        var newListToClose = []

        var i
        for(i = 0 ; i < privateObject.subscribedSubWindows.length ; i++){
            var window = privateObject.subscribedSubWindows[i]
            if(window.projectId === projectId){
                newListToClose.push(window)
            }
        }

        var j
        for(j = 0 ; j < newListToClose.length ; j++){
            newListToClose[j].close()
        }

    }

//--------------------------------------------------------------------------

    function suscribeSubWindow(subWindow){

        privateObject.subscribedSubWindows.push(subWindow)
    }

    //--------------------------------------------------------------------------

    function unsuscribeSubWindow(subWindow){
        var newList = []
        console.log("subscribedSubWindows before", privateObject.subscribedSubWindows)

        var i
        for(i = 0 ; i < privateObject.subscribedSubWindows.length ; i++){
            var window = privateObject.subscribedSubWindows[i]

            if(window.windowId !== subWindow.windowId){
                newList.push(window)
            }
        }

        privateObject.subscribedSubWindows = newList
    }

//--------------------------------------------------------------------------

    function addNoteSubWindow(projectId, paperId){

        var incubator = noteWindowComponent.incubateObject(rootPage, {projectId: projectId, paperId: paperId})
        if (incubator.status !== Component.Ready) {
            incubator.onStatusChanged = function(status) {
                if (status === Component.Ready) {
                    incubator.object.show()
                    suscribeSubWindow(incubator.object)
                }
            }
        } else {
            incubator.object.show()
            suscribeSubWindow(incubator.object)
        }
    }


    Connections {
        target: Globals
        function onOpenNoteInNewWindowCalled(projectId, paperId) {

            addNoteSubWindow(projectId, paperId)

        }
    }

    Component {
        id: noteWindowComponent
        ApplicationWindow{
            id: subWindow
            x: 0
            y:0
            width: 800
            height: 800

            readonly property string windowId: {return pageType + "_" +  projectId + "_" + paperId }

            property int projectId: -2
            property int paperId: -2
            property string pageType: page.pageType

            NotePage{
                id: page
                anchors.fill: parent
                projectId: subWindow.projectId
                paperId: subWindow.paperId

            }

            onClosing: {
                unsuscribeSubWindow(subWindow)
            }
        }
    }

    //------------------------------------------------------

    function addSheetSubWindow(projectId, paperId){

        var incubator = writeWindowComponent.incubateObject(rootPage, {projectId: projectId, paperId: paperId})
        if (incubator.status !== Component.Ready) {
            incubator.onStatusChanged = function(status) {
                if (status === Component.Ready) {
                    incubator.object.show()
                    suscribeSubWindow(incubator.object)
                }
            }
        } else {
            incubator.object.show()
            suscribeSubWindow(incubator.object)
        }
    }

    Connections {
        target: Globals
        function onOpenSheetInNewWindowCalled(projectId, paperId) {

            addSheetSubWindow(projectId, paperId)

        }
    }

    Component {
        id: writeWindowComponent
        ApplicationWindow{
            id: subWindow
            x: 0
            y:0
            width: 800
            height: 800
            readonly property string windowId: {return pageType + "_" +  projectId + "_" + paperId }

            property int projectId: -2
            property int paperId: -2
            property string pageType: page.pageType

            WritePage{
                id: page
                anchors.fill: parent
                projectId: subWindow.projectId
                paperId: subWindow.paperId

            }

            onClosing: {
                unsuscribeSubWindow(subWindow)
            }
        }
    }


    //---------------------------------------------------------

    //---------------------------------------------------------

    //---------------------------------------------------------



    //---------------------------------------------------------
    Component.onCompleted: {

        this.openArgument()
    }
    Component.onDestruction: {

    }

    property url testProjectFileName: "qrc:/testfiles/skribisto_test_project.sqlite"
    function openArgument(){


        var arg
        var arguments
        var isTestProject = false
        var oneProjectInArgument = false
        var projectInArgument = ""

        arguments = Qt.application.arguments
        for (arg in arguments) {
            console.log("argument : " , arguments[arg])
            if(arg === 0 ){
                continue
            }

            if (arguments[arg] === "--testProject") {
                var error = plmData.projectHub().loadProject(
                            testProjectFileName)
                console.log("project loaded : " + error.success)
                console.log("projectFileName :", testProjectFileName.toString(), "\n")

                //show Write window
                //                writeWindowAction.trigger()
                isTestProject = true

            }
            else {
                if (arguments[arg][-6] === ".skrib"){
                    oneProjectInArgument = true
                    projectInArgument = plmData.projectHub().loadProject(
                                arguments[arg])

                }
            }
        }
        if(!isTestProject & oneProjectInArgument){
            var error = plmData.projectHub().loadProject(
                        projectInArgument)
            //show Write window
            //            writeWindowAction.trigger()
        }


        if (!isTestProject & !oneProjectInArgument & plmData.projectHub().getProjectCount() === 0 & SkrSettings.welcomeSettings.createEmptyProjectAtStart === true) {
            plmData.projectHub().loadProject("")

            //show Write window
            //            writeWindowAction.trigger()

        }
    }
}
