import QtQuick 2.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1
import eu.skribisto.skrusersettings 1.0

import "Write"
import "Welcome"
import "Notes"
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

                //                welcomePage.forceActiveFocus()
            }
        }
        Action {
            id: galleryWindowAction
            text: qsTr("Gallery")
            icon {
                name: "view-preview"
                height: 100
                width: 100
            }

            shortcut: "F6"
            checkable: true
            onTriggered: {
                //                rootStack.

                //                galleryPage.forceActiveFocus()
            }
        }
        Action {
            id: projectsWindowAction
            text: qsTr("Project")
            icon {
                name: "configure"
                height: 100
                width: 100
            }

            shortcut: "F7"
            checkable: true
            onTriggered: {

                //                projectsPage.forceActiveFocus()
            }
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
        }
    }

    notificationButton.action: notificationButtonAction



    //-------------------------------------------------------------
    //-------Left Dock------------------------------------------
    //-------------------------------------------------------------
    rootLeftDock.enabled: !Globals.compactSize

    rootLeftDock.onFoldedChanged: {
        if (rootLeftDock.folded) {
            leftDockMenuGroup.visible = false
            leftDockMenuButton.checked = false
            leftDockMenuButton.visible = false
        } else {
            leftDockMenuButton.visible = true
        }
    }

    leftDockShowButton.onClicked: rootLeftDock.folded ? rootLeftDock.unfold(
                                                            ) : rootLeftDock.fold()
    leftDockShowButton.icon {
        name: rootLeftDock.folded ? "go-next" : "go-previous"
        height: 30
        width: 30
    }

    leftDockMenuButton.onCheckedChanged: leftDockMenuButton.checked ? leftDockMenuGroup.visible = true : leftDockMenuGroup.visible = false
    leftDockMenuButton.checked: false
    leftDockMenuButton.icon {
        name: "overflow-menu"
        height: 30
        width: 30
    }

    //leftDockResizeButton.onVisibleChanged: leftDock.folded = false
    //leftDockResizeButton.onClicked:
    leftDockMenuGroup.visible: false
    leftDockResizeButton.icon {
        name: "resizecol"
        height: 30
        width: 30
    }

    // compact mode :
    //    compactHeaderPane.visible: Globals.compactSize

    //    compactLeftDockShowButton.onClicked: leftDrawer.open()
    //    compactLeftDockShowButton.icon {
    //        name: "go-next"
    //        height: 50
    //        width: 50
    //    }

    // resizing with leftDockResizeButton:

    property int leftDockResizeButtonFirstPressX: 0
    leftDockResizeButton.onReleased: {
        leftDockResizeButtonFirstPressX = 0
    }

    leftDockResizeButton.onPressXChanged: {
        if(leftDockResizeButtonFirstPressX === 0){
            leftDockResizeButtonFirstPressX = rootPage.mapFromItem(leftDockResizeButton, leftDockResizeButton.pressX, 0).x
        }

        var pressX = rootPage.mapFromItem(leftDockResizeButton, leftDockResizeButton.pressX, 0).x
        var displacement = leftDockResizeButtonFirstPressX - pressX
        rootLeftDock.fixedWidth = rootLeftDock.fixedWidth - displacement
        leftDockResizeButtonFirstPressX = pressX

        if(rootLeftDock.fixedWidth < 300){
            rootLeftDock.fixedWidth = 300
        }
        if(rootLeftDock.fixedWidth > 600){
            rootLeftDock.fixedWidth = 600
        }

        SkrSettings.rootSettings.leftDockWidth = rootLeftDock.fixedWidth
    }


    //---------------------------------------------------------

    Connections {
        target: Globals

        // @disable-check M16
        onCompactSizeChanged: {
            if (Globals.compactSize === true) {
                leftDrawer.interactive = true

            } else {
                leftDrawer.close()
                leftDrawer.interactive = false
            }
        }
    }

    Drawer {
        id: leftDrawer
        enabled: Globals.compactSize
        width: if (rootPageBase.width * 0.6 > 400) {
                   return 400
               } else {
                   return rootPageBase.width * 0.6
               }
        height: rootPageBase.height
        modal: Globals.compactSize ? true : false
        edge: Qt.LeftEdge

        //        interactive: Globals.compactSize ? true : false
        //        visible:true
        //        position: Globals.compactSize ? 0 : 1
        RootLeftDock {
            id: compactLeftDock
            anchors.fill: parent
        }
    }
    //---------------------------------------------------------

    // fullscreen :

    property bool fullscreen_left_dock_folded: false
    Connections {
        target: Globals
        // @disable-check M16
        onFullScreenCalled: function (value) {
            if(value){
                //save previous conf
                fullscreen_left_dock_folded = rootLeftDock.folded
                rootLeftDock.fold()
            }
            else{
                rootLeftDock.folded = fullscreen_left_dock_folded
            }

        }
    }

    //---------------------------------------------------------
    // projectLoaded :
    property int projectIdForProjectLoading: 0
    property int paperIdForProjectLoading: 0

    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectLoaded: function (projectId) {

            // get last sheet id from e=settings or get top sheet id

            var topPaperId = plmData.sheetHub().getTopPaperId(projectId)
            console.log("topPaperId ::", topPaperId)
            var paperId = skrUserSettings.getProjectSetting(projectId, "writeCurrentPaperId", topPaperId)
//            console.log("paperId ::", paperId)
//            console.log("projectId ::", projectId)
            projectIdForProjectLoading = projectId
            paperIdForProjectLoading = paperId

            var isPresent = false
            var idList = plmData.sheetHub().getAllIds(projectId)
            var count = idList.length
//            console.log("idList", idList)
//            console.log("count", count)
//            console.log("a", paperId)
            for(var i = 0; i < count ; i++ ){
//                console.log("b", paperId)
                if(paperId === idList[i]){
                    isPresent = true
//                    console.log("c", paperId)
                }
            }
            if(!isPresent & count > 0){
//                console.log("d", paperId)
                paperIdForProjectLoading = idList[0]            }
            else if(!isPresent & count === 0){
//                console.log("e", paperId)
                paperIdForProjectLoading = -2

            }
            projectLoadingTimer.start()



        }
    }
    Timer{
        id: projectLoadingTimer
        repeat: false
        interval: 100
        onTriggered: Globals.openSheetCalled(projectIdForProjectLoading, paperIdForProjectLoading)
    }
    //---------------------------------------------------------

    //rootSwipeView.currentIndex: rootTabBar.currentIndex

    rootTabBar.currentIndex: rootSwipeView.currentIndex

    Binding {
        when: rootTabBar.currentIndex !== rootSwipeView.currentIndex
        target: rootSwipeView
        property: "currentIndex"
        value: rootTabBar.currentIndex
    }

    function addTab(incubator, insertionIndex, projectId, paperId) {
        var title = incubator.object.title
//        console.debug("debug title : ", title)
        var page  = incubator.object


        rootSwipeView.insertItem(insertionIndex, incubator);

        var component = Qt.createComponent("Tab.qml");
        var tabIncubator = component.incubateObject(rootTabBar, {text: title, projectId: projectId, paperId: paperId, height: rootTabBar.height });
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


    // openDocument :
    Connections {
        target: Globals
        // @disable-check M16
        onOpenSheetCalled: function (projectId, paperId) {
            // verify if project/sheetId not already opened

            // create WritePage tab
            var component = Qt.createComponent("Write/WritePage.qml");
            var incubator = component.incubateObject(rootSwipeView, {projectId: projectId, paperId: paperId });
            if (incubator.status !== Component.Ready) {
                incubator.onStatusChanged = function(status) {
                    if (status === Component.Ready) {

                        var insertionIndex = rootSwipeView.currentIndex + 1
                        addTab(incubator, insertionIndex, projectId, paperId)
                        console.debug("paprer 1 : ",paperId)
                        console.debug("count 1 : ",rootSwipeView.count)
                    }
                }
            } else {

                var insertionIndex = rootSwipeView.currentIndex + 1
                addTab(incubator, projectId, paperId)

            }

            console.debug("paprer : ",paperId)
            console.debug("count : ",rootSwipeView.count)
        }
    }
    //---------------------------------------------------------

    Component.onCompleted: {
        if(!Globals.compactSize){
            leftDrawer.close()
            leftDrawer.interactive = false
        }

        this.openArgument()
    }
    Component.onDestruction: {

    }

    property string testProjectFileName: ":/testfiles/skribisto_test_project.sqlite"
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
                console.log("projectFileName :", testProjectFileName, "\n")

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
