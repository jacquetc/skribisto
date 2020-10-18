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

    property int currentProjectId : Globals.sheetOverviewCurrentProjectId


    Component.onCompleted: {
        connectToSheetOverviewTree()

    }

    Connections{
        target: plmData.projectHub()
        function onProjectClosed(projectId){

        }
    }
    //--------------------------------------------------------
    //---Writing Zone (little notes in tree)-----------------------------------------
    //--------------------------------------------------------

    //-------------------------------------------------------------
    //-------Sheet Overview------------------------------------------
    //-------------------------------------------------------------



    //--------------------------------------------------------------------------


    SKRSearchSheetListProxyModel {
        id: sheetOverviewProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        projectIdFilter: currentProjectId

        onParentIdFilterChanged: {
            if(sheetOverviewProxyModel.parentIdFilter === -2){
                topFilteringBanner.visible = false
                return
            }

            topFilteringBanner.visible = true

            var title = plmData.sheetHub().getTitle(currentProjectId, sheetOverviewProxyModel.parentIdFilter)
            topFilteringBannerLabel.text = qsTr("The focus is currently on %1").arg(title)

        }
    }

    unsetFilteringParentToolButton.icon.name: "window-close"
    unsetFilteringParentToolButton.onClicked: {
        sheetOverviewProxyModel.parentIdFilter = -2

    }

    sheetOverviewTree.proxyModel: sheetOverviewProxyModel
    sheetOverviewTree.model: sheetOverviewProxyModel
    //--------------------------------------------------------------------------


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
    property int leftDrawerFixedWidth: 300
    SKRDrawer {
        id: leftDrawer
        enabled: base.enabled
        parent: base
        widthInDockMode: leftDrawerFixedWidth
        widthInDrawerMode: 400
        height: base.height
        interactive: Globals.compactSize
        dockModeEnabled: !Globals.compactSize
        settingsCategory: "writeOverviewLeftDrawer"
        edge: Qt.LeftEdge



        LeftDock {
            id: leftDock
            anchors.fill: parent


        }


    }


    property alias rightDock: rightDock
    property int rightDrawerFixedWidth: 300
    SKRDrawer {
        id: rightDrawer
        parent: base
        enabled: base.enabled
        widthInDockMode: rightDrawerFixedWidth
        widthInDrawerMode: 400
        height: base.height
        interactive: Globals.compactSize
        dockModeEnabled: !Globals.compactSize
        settingsCategory: "writeOverviewRightDrawer"
        edge: Qt.RightEdge


        RightDock {
            id: rightDock
            anchors.fill: parent

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
