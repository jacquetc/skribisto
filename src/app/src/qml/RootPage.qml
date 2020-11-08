import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQml 2.15
import QtQuick.Controls.Material 2.15
import Qt.labs.settings 1.1
import eu.skribisto.usersettings 1.0
import eu.skribisto.plmerror 1.0
import eu.skribisto.projecthub 1.0

import "Items"
import "Write"
import "Welcome"
import "Note"
import "Gallery"
import "Projects"

RootPageForm {
    id: rootPage


    Shortcut {
        sequence: "Esc"
        onActivated: {

            Globals.forceFocusOnEscapePressed()
        }
    }


    SKRUserSettings {
        id: skrUserSettings
    }


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
        onTriggered: {

            rootSwipeView.currentIndex = 0
            welcomePage.forceActiveFocus()
        }
    }

    Connections {
        target: Globals
        function onShowWelcomePageCalled() {
          welcomeWindowAction.trigger()
        }
    }

    Action {
        id: writeOverviewWindowAction
        text: qsTr("Write")
        icon {
            name: "view-media-playlist"
            color: SkrTheme.buttonIcon
            height: 100
            width: 100
        }

        shortcut: "F6"
        onTriggered: {
            rootSwipeView.currentIndex = 1
            writeOverviewPage.forceActiveFocus()
        }
    }

    Connections {
        target: Globals
        function onShowWriteOverviewPageCalled() {
          writeOverviewWindowAction.trigger()
        }
    }

    Action {
        id: noteOverviewWindowAction
        text: qsTr("Note")
        icon {
            name: "story-editor"
            color: SkrTheme.buttonIcon
            height: 100
            width: 100
        }

        shortcut: "F7"
        onTriggered: {
            rootSwipeView.currentIndex = 2
            noteOverviewPage.forceActiveFocus()
        }
    }

    Connections {
        target: Globals
        function onShowNoteOverviewPageCalled() {
          noteOverviewWindowAction.trigger()
        }
    }

    Action {
        id: galleryWindowAction
        text: qsTr("Gallery")
        icon {
            name: "view-preview"
            color: SkrTheme.buttonIcon
            height: 100
            width: 100
        }

        shortcut: "F8"
        onTriggered: {
            //                rootStack.
            rootSwipeView.currentIndex = 3
            galleryPage.forceActiveFocus()
        }
    }

    Connections {
        target: Globals
        function onShowGalleryPageCalled() {
          galleryWindowAction.trigger()
        }
    }

    Action {
        id: projectWindowAction
        text: qsTr("Project")
        icon {
            name: "configure"
            color: SkrTheme.buttonIcon
            height: 100
            width: 100
        }

        shortcut: "F9"
        onTriggered: {
            rootSwipeView.currentIndex = 4
            projectsMainPage.forceActiveFocus()
        }
    }
    Connections {
        target: Globals
        function onShowProjectPageCalled() {
          projectWindowAction.trigger()
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

    SkrPopup {
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

    rootSwipeView.interactive:  SkrSettings.accessibilitySettings.allowSwipeBetweenTabs


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

                rootTabBar.visible = false
            }
            else{
                fullScreen = false
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

    // compact :

    Connections {
        target: Globals
        function onCompactModeChanged() {
            rootTabBar.visible = !Globals.compactMode


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
        sequence: "Ctrl+Tab"
        onActivated: {
            rootTabBar.incrementCurrentIndex()
            rootTabBar.currentItem.forceActiveFocus()

        }
    }



    //---------------------------------------------------------

    Shortcut{
        id: tabBarPreviousTabShortcut
        sequence: "Ctrl+Shift+Tab"
        onActivated: {
            rootTabBar.decrementCurrentIndex()
            rootTabBar.currentItem.forceActiveFocus()
        }
    }

    //---------------------------------------------------------


    //rootSwipeView.currentIndex: rootTabBar.currentIndex

    rootTabBar.currentIndex: rootSwipeView.currentIndex

    //---------------------------------------------------------

    Binding {
        //when: rootTabBar.currentIndex !== rootSwipeView.currentIndex
        when: rootSwipeView.currentIndexChanged
        target: rootSwipeView
        property: "currentIndex"
        value: rootTabBar.currentIndex
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    //---------------------------------------------------------

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

    //---------------------------------------------------------


    function addTab(incubator, insertionIndex, pageType, projectId, paperId, iconName) {
        var title = incubator.object.title
        //        console.debug("debug title : ", title)
        var page  = incubator.object


        rootSwipeView.insertItem(insertionIndex, incubator);

        var component = Qt.createComponent("Items/SkrTabButton.qml");
        var tabIncubator = component.incubateObject(rootTabBar, {text: title, pageType: pageType,projectId: projectId, paperId: paperId, iconName: iconName});
        console.debug("debug : ", component.errorString())
        if (tabIncubator.status !== Component.Ready) {
            tabIncubator.onStatusChanged = function(status) {
                if (status === Component.Ready) {

                    rootTabBar.insertItem(insertionIndex, tabIncubator)
                    if(page.onTitleChangedString){
                        page.onTitleChangedString.connect(tabIncubator.object.setTitle)
                    }
                    tabIncubator.object.onCloseCalled.connect(closeTab)

                }
            }
        } else {


            rootTabBar.insertItem(insertionIndex, tabIncubator)
            page.onTitleChangedString.connect(tabIncubator.object.setTitle)
            tabIncubator.object.onCloseCalled.connect(closeTab)

        }

        //add in drop-down menu :
        dropDownTabMenuModel.insert(insertionIndex ,{"title": title, "closable": true, "tabId": pageType + "_" +  projectId + "_" + paperId })


    }

    //---------------------------------------------------------

    function closeTab(index) {
        if(rootSwipeView.itemAt(index).runActionsBedoreDestruction){
            rootSwipeView.itemAt(index).runActionsBedoreDestruction()
        }
        rootSwipeView.removeItem(rootSwipeView.itemAt(index))
        rootTabBar.removeItem(rootTabBar.itemAt(index))



        //remove from drop-down menu :
        dropDownTabMenuModel.remove(index)



    }


    //---------------------------------------------------------

    function closeTabsByProject(projectId) {

        var i;
        for(i = 0; i < rootTabBar.count; i++){

            if(rootTabBar.itemAt(i).projectId === projectId){

                closeTab(i)
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

    // drop-down menu for tabs in compact size

    function addMainMenuToDropDownTabMenuModel(){
        dropDownTabMenuModel.append({"title": welcomeWindowAction.text, "closable": false, "tabId": welcomePage.pageType + "_" +  -2 + "_" + -2 })
        dropDownTabMenuModel.append({"title": writeOverviewWindowAction.text, "closable": false, "tabId": writeOverviewPage.pageType + "_" +  -2 + "_" + -2 })
        dropDownTabMenuModel.append({"title": noteOverviewWindowAction.text, "closable": false, "tabId": noteOverviewTab.pageType + "_" +  -2 + "_" + -2 })
        //NOTE: waiting to be implemented
        //dropDownTabMenuModel.append({"title": galleryWindowAction.text, "closable": false, "tabId": galleryTab.pageType + "_" +  -2 + "_" + -2 })
        dropDownTabMenuModel.append({"title": projectWindowAction.text, "closable": false, "tabId": projectTab.pageType + "_" +  -2 + "_" + -2 })

    }

    showTabListButton.onCheckedChanged: {
        showTabListButton.checked ? dropDownTabMenuPopup.open() : dropDownTabMenuPopup.close()
    }

    showTabListButton.checked: dropDownTabMenuPopup.visible

    ListModel {
        id: dropDownTabMenuModel
    }

    QtObject {
        id: dropDownMenuPrivate
        property string tabToOpen: ""
    }

    SkrPopup {
        id: dropDownTabMenuPopup
        x: showTabListButton.x
        y: showTabListButton.y + showTabListButton.height
        width: 200
        height: 200
        modal: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        padding: 0

        ColumnLayout {
            anchors.fill: parent


            ScrollView {
                id: dropDownTabMenuScrollView
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                ListView {
                    id: dropDownTabMenuList
                    anchors.fill: parent
                    clip: true
                    smooth: true
                    focus: true
                    boundsBehavior: Flickable.StopAtBounds


                    model: dropDownTabMenuModel
                    interactive: true
                    spacing: 1
                    delegate: Component {
                        id: itemDelegate

                        Item {
                            id: delegateRoot
                            height: 30
                            focus: true


                            anchors {
                                left: Qt.isQtObject(parent) ? parent.left : undefined
                                right: Qt.isQtObject(parent) ? parent.right : undefined
                                leftMargin: 5
                                rightMargin: 5
                            }

                            TapHandler {
                                id: tapHandler
                                onSingleTapped: {
                                    dropDownTabMenuList.currentIndex = model.index
                                    delegateRoot.forceActiveFocus()
                                    eventPoint.accepted = true
                                }
                                onDoubleTapped: {
                                    // open tab

                                    var noteId = model.tabId

                                    dropDownMenuPrivate.tabToOpen = tabId
                                    displayTabAfterClosingPopupTimer.start()
                                    dropDownTabMenuPopup.close()

                                }
                            }
                            Keys.priority: Keys.AfterItem

                            Keys.onPressed: {
                                if (event.key === Qt.Key_Return || event.key === Qt.Key_Space){
                                    console.log("Return key pressed title")


                                    // open tab

                                    var noteId = model.tabId

                                    dropDownMenuPrivate.tabToOpen = tabId
                                    displayTabAfterClosingPopupTimer.start()
                                    dropDownTabMenuPopup.close()


                                }




                            }


                            RowLayout {
                                anchors.fill: parent

                                SkrLabel {
                                    text: model.title

                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft

                                    horizontalAlignment: Qt.AlignLeft
                                    verticalAlignment: Qt.AlignVCenter
                                }

                                SkrToolButton {
                                    id: closeTabFromDropDownMenuButton
                                    text: "x"
                                    visible: model.closable
                                    Layout.fillHeight: true
                                    Layout.alignment: Qt.AlignVCenter | Qt.AlignRight

                                    onClicked: {
                                        closeTab(model.index)
                                    }

                                }
                            }
                        }
                    }

                    highlight:  Component {
                        id: highlight
                        Rectangle {
                            //                            x: 0
                            //                            y: searchResultList.currentItem.y + 1
                            //                            width: searchResultList.width
                            //                            height: searchResultList.currentItem.height - 1
                            //                            color: "transparent"
                            radius: 5
                            border.color:  SkrTheme.accent
                            border.width: 2
                            color: "transparent"
                            visible: dropDownTabMenuList.activeFocus
                            Behavior on y {
                                SpringAnimation {
                                    spring: 5
                                    damping: 0.2
                                }
                            }
                        }
                    }
                }
            }
        }

    }

    Timer {
        id: displayTabAfterClosingPopupTimer
        interval: 0
        onTriggered: {

            var tabToOpen = dropDownMenuPrivate.tabToOpen

            var i
            for(i = 0 ; i < rootTabBar.count ; i++){
                var tab = rootTabBar.itemAt(i)
                if(tab.tabId === tabToOpen){
                    tab.toggle()
                }
            }

        }
    }




    //---------------------------------------------------------

    welcomeTab.action: welcomeWindowAction
    writeOverviewTab.action: writeOverviewWindowAction
    noteOverviewTab.action: noteOverviewWindowAction
    //NOTE: waiting to be implemented
    //galleryTab.action: galleryWindowAction
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
            var incubator = component.incubateObject(rootSwipeView, {pageType: pageType, projectId: projectId, paperId: paperId, enabled: false });
            if (incubator.status !== Component.Ready) {
                incubator.onStatusChanged = function(status) {
                    if (status === Component.Ready) {

                        addTab(incubator, insertionIndex, pageType, projectId, paperId, "document-edit")
                        //                        console.debug("paprer 1 : ",paperId)
                        //                        console.debug("count 1 : ",rootSwipeView.count)
                        openTabTimer.start()
                    }
                }
            } else {

                addTab(incubator, insertionIndex,  pageType, projectId, paperId, "document-edit")
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
            var incubator = component.incubateObject(rootSwipeView, {pageType: pageType,projectId: projectId, paperId: paperId, enabled: false });
            if (incubator.status !== Component.Ready) {
                incubator.onStatusChanged = function(status) {
                    if (status === Component.Ready) {

                        addTab(incubator, insertionIndex, pageType, projectId, paperId, "document-edit-sign")
                        //                        console.debug("paper 1 : ",paperId)
                        //                        console.debug("count 1 : ",rootSwipeView.count)
                        openTabTimer.start()
                    }
                }
            } else {

                addTab(incubator, insertionIndex,  pageType, projectId, paperId, "document-edit-sign")
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
    //------------ Theme -----------------------------
    //---------------------------------------------------------

    Action {
        id: themePageAction
        text: qsTr("Theme")
        icon {
            name: "color-picker-white"
            height: 100
            width: 100
        }

        onTriggered: {

            addThemePage()
        }
    }

    Connections {
        target: Globals
        function onOpenThemePageCalled() {
            themePageAction.trigger()
        }
    }

    function addThemePage(){

        var pageType = "theme"
        // verify if theme page not already opened
        var tabId = pageType + "_" +  -2 + "_" + -2
        var i;
        for (i = 0; i < rootTabBar.count; i++) {
            if (rootTabBar.itemAt(i).tabId === tabId){

                rootTabBar.setCurrentIndex(i)
                return
            }
        }


        insertionIndex = rootSwipeView.count
        // create Theme tab
        var component = Qt.createComponent("Theme/ThemePage.qml");
        var incubator = component.incubateObject(rootSwipeView, {pageType: pageType,projectId: -2, paperId: -2, enabled: false });
        if (incubator.status !== Component.Ready) {
            incubator.onStatusChanged = function(status) {
                if (status === Component.Ready) {

                    addTab(incubator, insertionIndex, pageType, -2, -2, "color-picker-white")
                    openTabTimer.start()
                }
            }
        } else {

            addTab(incubator, insertionIndex,  pageType,  -2, -2, "color-picker-white")
            openTabTimer.start()

        }



    }

    //---------------------------------------------------------

    //---------------------------------------------------------

    //---------------------------------------------------------


    Connections {
        target: Globals
        function onOpenMainMenuCalled(){
            mainMenuButton.checked = false
            mainMenuButton.checked = true
        }
    }
    mainMenuButton.objectName: "mainMenuButton"
    mainMenuButton.icon.name: "application-menu"

    function subscribeMainMenu() {
        skrEditMenuSignalHub.subscribe(mainMenuButton.objectName)
    }

    mainMenuButton.onCheckedChanged: {
        if(mainMenuButton.checked){
            var coordinates = mainMenuButton.mapToItem(rootPage, mainMenuButton.x, mainMenuButton.y)
            mainMenu.y = coordinates.y + mainMenuButton.height
            mainMenu.x = coordinates.x
            mainMenu.open()


        }
        else {
            mainMenu.dismiss()

        }
    }


    Shortcut {
        id: fileMenuShortcut
        sequence: skrQMLTools.mnemonic(fileMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(fileMenu)

        }
    }

    Shortcut {
        id: editMenuShortcut
        sequence: skrQMLTools.mnemonic(editMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(editMenu)

        }
    }

    Shortcut {
        id: viewMenuShortcut
        sequence: skrQMLTools.mnemonic(viewMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(viewMenu)

        }
    }

    Shortcut {
        id: helpMenuShortcut
        sequence: skrQMLTools.mnemonic(helpMenu.title)
        onActivated: {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(helpMenu)

        }
    }

    Connections{
        target: Globals
        function onOpenSubMenuCalled(menu) {
            Globals.openMainMenuCalled()
            mainMenu.openSubMenu(menu)
        }
    }

    SkrMenu {
        id: mainMenu

        onClosed: {
            mainMenuButton.checked = false
        }

        function findMenuIndex(menu){
            var i
            for(i = 0; i< mainMenu.count ; i++){
                if(menu.title === mainMenu.menuAt(i).title){

                    return i
                }
            }
        }

        function openSubMenu(menu){
            mainMenu.currentIndex = mainMenu.findMenuIndex(menu)
            menu.open()
            menu.currentIndex = 0
        }

        Component.onCompleted: {
            skrEditMenuSignalHub.subscribe(mainMenu.objectName)
        }

        SkrMenu {
            id: fileMenu
            title: qsTr("&File")

            SkrMenuItem{
                action: newProjectAction

            }
            SkrMenuItem{
                action: openProjectAction
            }
            MenuSeparator { }
            SkrMenuItem{
                action: printAction
            }
            SkrMenuItem{
                action: importAction
            }
            SkrMenuItem{
                action: exportAction
            }

            MenuSeparator { }
            SkrMenuItem{
                action: saveAction
            }
            SkrMenuItem{
                action: saveAsAction
            }
            SkrMenuItem{
                action: saveACopyAction
            }
            SkrMenuItem{
                action: saveAllAction
            }

            MenuSeparator { }
            SkrMenuItem{
                action: closeCurrentProjectAction
            }
            SkrMenuItem{
                action: quitAction
            }
        }
        SkrMenu {
            id: editMenu
            objectName: "editMenu"
            title: qsTr("&Edit")



            SkrMenuItem{
                id: cutItem
                objectName: "cutItem"
                action: cutAction
            }
            SkrMenuItem{
                id: copyItem
                objectName: "copyItem"
                action: copyAction
            }
            SkrMenuItem{
                id: pasteItem
                objectName: "pasteItem"
                action: pasteAction
            }

            Component.onCompleted:{
                skrEditMenuSignalHub.subscribe(editMenu.objectName)
                skrEditMenuSignalHub.subscribe(cutItem.objectName)
                skrEditMenuSignalHub.subscribe(copyItem.objectName)
                skrEditMenuSignalHub.subscribe(pasteItem.objectName)
            }

        }
        SkrMenu {
            id: viewMenu
            objectName: "viewMenu"
            title: qsTr("&View")

            SkrMenuItem{
                action: welcomeWindowAction
            }

            SkrMenuItem{
                action: writeOverviewWindowAction
            }

            SkrMenuItem{
                action: noteOverviewWindowAction
            }

            //            SkrMenuItem{
            //                action: galleryWindowAction
            //            }

            SkrMenuItem{
                action: projectWindowAction
            }

            MenuSeparator{}

            SkrMenuItem {
                action: themePageAction
            }

            MenuSeparator{}

            SkrMenuItem {
                action: fullscreenAction
            }


        }
        SkrMenu {
            id: helpMenu
            objectName: "helpMenu"
            title: qsTr("&Help")



            Action { text: qsTr("&About") }
        }

    }




    //---------------------------------------------------------
    //------ status bar --------------------------------------------
    //---------------------------------------------------------

    statusLeftLabel.text: qsTr("Skribisto %1 created by Cyril Jacquet").arg(skrRootItem.skribistoVersion())
    statusLeftLabel.visible: !Globals.fullScreen

    //---------------------------------------------------------
    //------ showTabListButton--------------------------------------------
    //---------------------------------------------------------

    showTabListButton.icon{
        name: "arrow-down"
        height: 50
        width: 50
    }



    //---------------------------------------------------------
    Component.onCompleted: {

        this.openArgument()
        this.subscribeMainMenu()

        this.addMainMenuToDropDownTabMenuModel()
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
            if(arg === 0 ){
                continue
            }
            console.log("argument : " , arguments[arg])

            if (arguments[arg] === "--testProject") {
                var error = plmData.projectHub().loadProject(
                            testProjectFileName)
                console.log("project loaded : " + error.success)
                console.log("projectFileName :", testProjectFileName.toString(), "\n")

                //show Write window
                //                writeOverviewWindowAction.trigger()
                isTestProject = true

            }
            else {
                if (arguments[arg].slice(-6) === ".skrib"){


                    oneProjectInArgument = true

                    console.log("argument skrib : " , arguments[arg])
                    var url = skrQMLTools.getURLFromLocalFile(arguments[arg])
                    console.log("argument skrib url : " , url)

                    projectInArgument = plmData.projectHub().loadProject(url)

                }
            }
        }
        //        if(!isTestProject & oneProjectInArgument){
        //            var error = plmData.projectHub().loadProject(
        //                        projectInArgument)
        //            //show Write window
        //            //            writeOverviewWindowAction.trigger()
        //        }


        if (!isTestProject & !oneProjectInArgument & plmData.projectHub().getProjectCount() === 0 & SkrSettings.behaviorSettings.createEmptyProjectAtStart === true) {
            plmData.projectHub().loadProject("")

            //show Write window
            //            writeOverviewWindowAction.trigger()

        }
    }
}
