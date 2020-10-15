import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.settings 1.1
import eu.skribisto.searchsheetlistproxymodel 1.0
import "../Commons"
import ".."


WriteOverviewPageForm {
    id: root
    property string pageType: "writeOverview"

    property int currentProjectId : -2


    Component.onCompleted: {
        connectToSheetOverviewTree()

    }

    Connections{
        target: plmData.projectHub()
        function onProjectLoaded(projectId){
            //test :
            currentProjectId = projectId
        }
    }

    Connections{
        target: plmData.projectHub()
        function onProjectClosed(projectId){

        }
    }
    //--------------------------------------------------------
    //---Writing Zone (little notes in tree)-----------------------------------------
    //--------------------------------------------------------

//    writingZone.maximumTextAreaWidth: SkrSettings.noteSettings.textWidth
//    writingZone.textPointSize: SkrSettings.noteSettings.textPointSize
//    writingZone.textFontFamily: SkrSettings.noteSettings.textFontFamily
//    writingZone.textIndent: SkrSettings.noteSettings.textIndent
//    writingZone.textTopMargin: SkrSettings.noteSettings.textTopMargin

    //-------------------------------------------------------------
    //-------Sheet Overview------------------------------------------
    //-------------------------------------------------------------


    //--------------------------------------------------------------------------


    SKRSearchSheetListProxyModel {
        id: restoreSheetProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
    }

    sheetOverviewTree.proxyModel: restoreSheetProxyModel
    sheetOverviewTree.model: restoreSheetProxyModel
    //--------------------------------------------------------------------------

    onCurrentProjectIdChanged: {
        restoreSheetProxyModel.projectIdFilter = 1
    }


    //--------------------------------------------------------------------------
    Connections {
        target: sheetOverviewTree
        function onOpenDocument(openedProjectId, openedPaperId, _projectId, _paperId) {
            Globals.openSheetInNewTabCalled(_projectId, _paperId)
        }
    }

    function connectToSheetOverviewTree(){

        sheetOverviewTree.openDocumentInNewTab.connect(Globals.openSheetInNewTabCalled)
        sheetOverviewTree.openDocumentInNewWindow.connect(Globals.openSheetInNewWindowCalled)

    }



    sheetOverviewTree.displayMode: SkrSettings.overviewTreeSettings.treeItemDisplayMode
    sheetOverviewTree.treeIndentMultiplier: SkrSettings.overviewTreeSettings.treeIndentation

    //---------------------------------------------------------
    //------Actions----------------------------------------
    //---------------------------------------------------------




    Connections {
        target: italicAction
        function onTriggered() {closeRightDrawer()}
    }
    Connections {
        target: boldAction
        function onTriggered() {closeRightDrawer()}
    }
    Connections {
        target: strikeAction
        function onTriggered() {closeRightDrawer()}
    }
    Connections {
        target: underlineAction
        function onTriggered() {closeRightDrawer()}
    }

    function closeRightDrawer(){
        if(Globals.compactSize){
            rightDrawer.close()
        }
    }

    //-------------------------------------------------------------
    //-------Left Dock------------------------------------------
    //-------------------------------------------------------------




    leftDockMenuGroup.visible: !Globals.compactSize && leftDockMenuButton.checked
    leftDockMenuButton.visible: !Globals.compactSize


    leftDockShowButton.onClicked: leftDrawer.isVisible ? leftDrawer.isVisible = false : leftDrawer.isVisible = true

    leftDockShowButton.icon {
        name: leftDrawer.isVisible ? "go-previous" : "go-next"
        height: 50
        width: 50
    }

    leftDockMenuButton.icon {
        name: "overflow-menu"
        height: 50
        width: 50
    }

    leftDockResizeButton.icon {
        name: "resizecol"
        height: 50
        width: 50
    }


    // compact mode :
    compactLeftDockShowButton.visible: Globals.compactSize

    compactLeftDockShowButton.onClicked: leftDrawer.open()
    compactLeftDockShowButton.icon {
        name: "go-next"
        height: 50
        width: 50
    }

    // resizing with leftDockResizeButton:

    property int leftDockResizeButtonFirstPressX: 0
    leftDockResizeButton.onReleased: {
        leftDockResizeButtonFirstPressX = 0
        rootSwipeView.interactive = SkrSettings.accessibilitySettings.allowSwipeBetweenTabs
    }

    leftDockResizeButton.onPressXChanged: {

        if(leftDockResizeButtonFirstPressX === 0){
            leftDockResizeButtonFirstPressX = root.mapFromItem(leftDockResizeButton, leftDockResizeButton.pressX, 0).x
        }

        var pressX = root.mapFromItem(leftDockResizeButton, leftDockResizeButton.pressX, 0).x
        var displacement = leftDockResizeButtonFirstPressX - pressX
        leftDrawerFixedWidth = leftDrawerFixedWidth - displacement
        leftDockResizeButtonFirstPressX = pressX

        if(leftDrawerFixedWidth < 300){
            leftDrawerFixedWidth = 300
        }
        if(leftDrawerFixedWidth > 600){
            leftDrawerFixedWidth = 600
        }


        leftSettings.width = leftDrawerFixedWidth

    }

    leftDockResizeButton.onPressed: {

        rootSwipeView.interactive = false

    }

    leftDockResizeButton.onCanceled: {

        rootSwipeView.interactive = SkrSettings.accessibilitySettings.allowSwipeBetweenTabs
        leftDockResizeButtonFirstPressX = 0

    }



    //-------------------------------------------------------------
    //-------Right Dock------------------------------------------
    //-------------------------------------------------------------


    rightDockMenuGroup.visible: !Globals.compactSize && rightDockMenuButton.checked
    rightDockMenuButton.visible: !Globals.compactSize

    rightDockShowButton.onClicked: rightDrawer.isVisible ? rightDrawer.isVisible = false : rightDrawer.isVisible = true

    rightDockShowButton.icon {
        name: rightDrawer.isVisible ? "go-next" : "go-previous"
        height: 50
        width: 50
    }

    rightDockMenuButton.icon {
        name: "overflow-menu"
        height: 50
        width: 50
    }

    rightDockResizeButton.icon {
        name: "resizecol"
        height: 50
        width: 50
    }

    // compact mode :
    compactRightDockShowButton.visible: Globals.compactSize

    compactRightDockShowButton.onClicked: rightDrawer.open()
    compactRightDockShowButton.icon {
        name: "go-previous"
        height: 50
        width: 50
    }

    // resizing with rightDockResizeButton:

    property int rightDockResizeButtonFirstPressX: 0
    rightDockResizeButton.onReleased: {
        rightDockResizeButtonFirstPressX = 0
        rootSwipeView.interactive = SkrSettings.accessibilitySettings.allowSwipeBetweenTabs
    }

    rightDockResizeButton.onPressXChanged: {
        if(rightDockResizeButtonFirstPressX === 0){
            rightDockResizeButtonFirstPressX = root.mapFromItem(rightDockResizeButton, rightDockResizeButton.pressX, 0).x
        }

        var pressX = root.mapFromItem(rightDockResizeButton, rightDockResizeButton.pressX, 0).x
        var displacement = rightDockResizeButtonFirstPressX - pressX
        rightDrawerFixedWidth = rightDrawerFixedWidth + displacement
        rightDockResizeButtonFirstPressX = pressX

        if(rightDrawerFixedWidth < 200){
            rightDrawerFixedWidth = 200
        }
        if(rightDrawerFixedWidth > 350){
            rightDrawerFixedWidth = 350
        }

        rightSettings.width = rightDrawerFixedWidth

    }

    rightDockResizeButton.onPressed: {

        rootSwipeView.interactive = false

    }

    rightDockResizeButton.onCanceled: {

        rootSwipeView.interactive = SkrSettings.accessibilitySettings.allowSwipeBetweenTabs
        rightDockResizeButtonFirstPressX = 0

    }


  //---------------------------------------------------------
    //---------------------------------------------------------


    property alias leftDock: leftDock
    property int leftDrawerFixedWidth: 400
    SKRDrawer {
        id: leftDrawer
        parent: base
        enabled: base.enabled
        width: Globals.compactSize ? 400 : leftDrawerFixedWidth
        height: base.height
        interactive: Globals.compactSize
        edge: Qt.LeftEdge


        Connections {
            target: Globals
            function onCompactSizeChanged(){
                if(Globals.compactSize){
                    leftDrawer.close()
                }
                else {
                    leftDrawer.isVisible = leftSettings.isVisible
                }
            }
        }

        LeftDock {
            id: leftDock
            anchors.fill: parent


        }

        onIsVisibleChanged: if(!Globals.compactSize) leftSettings.isVisible = leftDrawer.isVisible


        Component.onCompleted: {
            leftDrawerFixedWidth = leftSettings.dockWidth
            Globals.resetDockConfCalled.connect(resetConf)
            if(Globals.compactSize){
                leftDrawer.close()
            }
            else {
                leftDrawer.isVisible = leftSettings.isVisible
            }
        }


        Settings {
            id: leftSettings
            category: "writeOverviewLeftDrawer"
            property int dockWidth: 300
            property bool isVisible: true
        }

        function resetConf(){
            leftSettings.dockWidth = 300
            leftDrawerFixedWidth = 300
            leftSettings.isVisible = true
            leftDrawer.isVisible = leftSettings.isVisible
        }

    }


    property alias rightDock: rightDock
    property int rightDrawerFixedWidth: 400
    SKRDrawer {
        id: rightDrawer
        parent: base
        enabled: base.enabled
        width:  Globals.compactSize ? 400 : rightDrawerFixedWidth
        height: base.height
        interactive: Globals.compactSize
        edge: Qt.RightEdge

        Connections {
            target: Globals
            function onCompactSizeChanged(){
                if(Globals.compactSize){
                    rightDrawer.close()
                }
                else {
                    rightDrawer.isVisible = rightSettings.isVisible

                }
            }
        }

        RightDock {
            id: rightDock
            anchors.fill: parent

        }

        onIsVisibleChanged:if(!Globals.compactSize) rightSettings.isVisible = rightDrawer.isVisible

        Component.onCompleted: {
            rightDrawerFixedWidth = rightSettings.dockWidth
            Globals.resetDockConfCalled.connect(resetConf)

            if(Globals.compactSize){
                rightDrawer.close()
            }
            else {
                rightDrawer.isVisible = rightSettings.isVisible
            }
        }


        Settings {
            id: rightSettings
            category: "writeOverviewRightDrawer"
            property int dockWidth: 300
            property bool isVisible: true
        }

        function resetConf(){
            rightSettings.dockWidth = 300
            rightDrawerFixedWidth = 300
            rightSettings.isVisible = true
            rightDrawer.isVisible = rightSettings.isVisible
        }
    }


    //------------------------------------------------------------
    //------------------------------------------------------------
    //------------------------------------------------------------



    //------------------------------------------------------------
    //--------menus--------------------------------------------
    //------------------------------------------------------------

    QtObject{
        id: privateMenuObject
        property string dockUniqueId: ""
    }

    function addMenus(){
        //create dockUniqueId:
        privateMenuObject.dockUniqueId = Qt.formatDateTime(new Date(), "yyyyMMddhhmmsszzz")

        var helpMenu

        var menuCount = mainMenu.count

        // take Help menu
        menuCount = mainMenu.count
        var m;
        for(m = 0 ; m < menuCount ; m++){
            var menu = mainMenu.menuAt(m)
            if(menu.objectName === "helpMenu"){
                helpMenu = mainMenu.takeMenu(m)
            }
        }

        // add new menus
        var k
        for(k = 0 ; k < leftDock.menuComponents.length ; k++){

            var newMenu = leftDock.menuComponents[k].createObject(mainMenu)
            newMenu.objectName = newMenu.objectName + "-" + privateMenuObject.dockUniqueId
            mainMenu.addMenu(newMenu)

        }
        var l
        for(l = 0 ; l < rightDock.menuComponents.length ; l++){
            var newMenu2 = rightDock.menuComponents[l].createObject(mainMenu)
            newMenu2.objectName = newMenu2.objectName + "-" + privateMenuObject.dockUniqueId
            mainMenu.addMenu(newMenu2)
        }

        // shortcuts:


        //reinsert Help menu
        mainMenu.addMenu(helpMenu)

    }

    function removeMenus(){
        if(mainMenu === null){
            return
        }


        var helpMenu

        var menuCount = mainMenu.count

        // take Help menu
        menuCount = mainMenu.count
        var m;
        for(m = 0 ; m < menuCount ; m++){
            var menu = mainMenu.menuAt(m)
            if(menu.objectName === "helpMenu"){
                helpMenu = mainMenu.takeMenu(m)
            }
        }

        // remove additional menus
        menuCount = mainMenu.count
        var i;
        for(i = menuCount - 1 ; i >= 0  ; i--){
            var menu1 = mainMenu.menuAt(i)

            if(menu1.objectName === "navigationDockMenu-" + privateMenuObject.dockUniqueId
                    || menu1.objectName === "toolDockMenu-" + privateMenuObject.dockUniqueId){

                mainMenu.removeMenu(menu1)

            }
        }


        //reinsert Help menu
        mainMenu.addMenu(helpMenu)

        privateMenuObject.dockUniqueId = ""

    }

    onEnabledChanged: {

        if(root.enabled){
            addMenus()
        }
        else{
            removeMenus()
        }
    }


    //------------------------------------------------------------
    // fullscreen :
    //------------------------------------------------------------
    property bool fullscreen_left_drawer_visible: false
    property bool fullscreen_right_drawer_visible: false

    Connections {
        target: Globals
        function onFullScreenCalled(value) {
            if(value){
                //save previous conf
                fullscreen_left_drawer_visible = leftDrawer.isVisible
                fullscreen_right_drawer_visible = rightDrawer.isVisible
                leftDrawer.isVisible = false
                rightDrawer.isVisible = false

            }
            else{
                leftDrawer.isVisible = fullscreen_left_drawer_visible
                rightDrawer.isVisible = fullscreen_right_drawer_visible
            }

        }
    }


}
