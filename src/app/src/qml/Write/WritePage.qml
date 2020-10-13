import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.sheethub 1.0
import eu.skribisto.skrusersettings 1.0
import Qt.labs.settings 1.1
import "../Commons"
import ".."

WritePageForm {
    id: root

    property string title: {return getTitle()}

    function getTitle(){
        return plmData.sheetHub().getTitle(projectId, paperId)

    }

    signal onTitleChangedString(string newTitle)

    //property int textAreaFixedWidth: SkrSettings.writeSettings.textWidth
    property var lastFocused: writingZone

    property string pageType: "write"




    //--------------------------------------------------------
    //---Accessibility-----------------------------------------
    //--------------------------------------------------------



    //    Keys.onReleased: {
    //        if(event.key === Qt.Key_Alt){
    //            console.log("alt")
    //            Globals.showMenuBarCalled()

    //            event.accepted = true
    //        }
    //    }

    //--------------------------------------------------------
    //---Writing Zone-----------------------------------------
    //--------------------------------------------------------

    writingZone.maximumTextAreaWidth: SkrSettings.writeSettings.textWidth
    writingZone.textPointSize: SkrSettings.writeSettings.textPointSize
    writingZone.textFontFamily: SkrSettings.writeSettings.textFontFamily
    writingZone.textIndent: SkrSettings.writeSettings.textIndent
    writingZone.textTopMargin: SkrSettings.writeSettings.textTopMargin

    writingZone.stretch: Globals.compactSize
    writingZone.name: "write-0" //useful ?

    Connections {
        target: plmData.projectHub()
        function onProjectCountChanged(count){
            if(count === 0){
                writingZone.enabled = false
            }
            else {
                writingZone.enabled = true
            }

        }
    }
    // focus
    Connections {
        enabled: writingZone.enabled
        target: Globals
        function onForceFocusOnEscapePressed(){
            writingZone.forceActiveFocus()
        }
    }

    //---------------------------------------------------------


    property int paperId: -2
    property int projectId: -2



    //---------------------------------------------------------

    Component.onCompleted: {


        openDocument(projectId, paperId)

        //title = getTitle()
        plmData.sheetHub().titleChanged.connect(changeTitle)

    }

    //---------------------------------------------------------

    function changeTitle(_projectId, _paperId, newTitle) {
        if (_projectId === projectId && _paperId === paperId ){
            title = newTitle
            onTitleChangedString(newTitle)
        }
    }

    //---------------------------------------------------------

    function runActionsBedoreDestruction() {

        saveCurrentPaperCursorPositionAndY()
        contentSaveTimer.stop()
        saveContent()
    }

    Component.onDestruction: {
        runActionsBedoreDestruction()
    }
    //--------------------------------------------------------
    //---Left Scroll Area-----------------------------------------
    //--------------------------------------------------------
    property int offset: leftDock.width

    //    Connections {
    //        target: Globals
    //        function onWidthChanged() {applyOffset()}

    //    }
    //    Connections {
    //        target: Globals
    //        function onCompactSizeChanged() {applyOffset()}

    //    }
    //    Connections {
    //        target: SkrSettings.rootSettings
    //        function onLeftDockWidthChanged() {applyOffset()}

    //    }
    //    Connections {
    //        target: writingZone
    //        function onWidthChanged() {applyOffset()}

    //    }

    Binding on leftBasePreferredWidth {
        value:  {
            var value = 0
            if (Globals.compactSize === true){
                value = -1;
            }
            else {

                value = writingZone.wantedCenteredWritingZoneLeftPos - offset
                if (value < 0) {
                    value = 0
                }
                //                console.debug("writingZone.wantedCenteredWritingZoneLeftPos :: ", writingZone.wantedCenteredWritingZoneLeftPos)
                //                console.debug("offset :: ", offset)
                //                console.debug("value :: ", value)

            }
            return value
        }
        restoreMode: Binding.RestoreBindingOrValue
    }
    //    Binding on rightBasePreferredWidth {
    //        value:  {
    //            var value = 0
    //            if (Globals.compactSize === true){
    //                value = -1;
    //            }
    //            else {

    //                value = 400 + offset
    //                if (value < 0) {
    //                    value = 0
    //                }
    //                //                console.debug("right writingZone.wantedCenteredWritingZoneLeftPos :: ", writingZone.wantedCenteredWritingZoneLeftPos)
    //                //                console.debug("right offset :: ", offset)
    //                //                console.debug("right value :: ", value)

    //            }
    //            rightBasePreferredWidth = value
    //        }
    //    }
    //    Binding on leftBaseMaximumWidth {
    //        when: SkrSettings.rootSettings.onLeftDockWidthChanged || Globals.onCompactSizeChanged || writingZone.onWidthChanged
    //            value:  {
    //                if (Globals.compactSize === true){
    //                    return -1;
    //                }
    //                else {
    //                    var value = writingZone.wantedTextAreaLeftPos - SkrSettings.rootSettings.leftDockWidth

    //                    if (value < 0) {
    //                        value = 0
    //                    }

    //                    return value;

    //                }
    //    }
    //    }


    leftPaneScrollTouchArea.onUpdated: {
        var deltaY = touchPoints[0].y - touchPoints[0].previousY
        //        console.log("deltaY :", deltaY)

        if (writingZone.flickable.atYBeginning && deltaY > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && deltaY < 0) {
            writingZone.flickable.returnToBounds()
            return
        }

        writingZone.flickable.flick(0, deltaY * 50)

    }

    leftPaneScrollMouseArea.onWheel: {

        var deltaY = wheel.angleDelta.y *10

        writingZone.flickable.flick(0, deltaY)

        if (writingZone.flickable.atYBeginning && wheel.angleDelta.y > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && wheel.angleDelta.y < 0) {
            writingZone.flickable.returnToBounds()
            return
        }
    }

    //--------------------------------------------------------
    //---Right Scroll Area-----------------------------------------
    //--------------------------------------------------------




    rightPaneScrollTouchArea.onUpdated: {
        var deltaY = touchPoints[0].y - touchPoints[0].previousY
        //        console.log("deltaY :", deltaY)

        if (writingZone.flickable.atYBeginning && deltaY > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && deltaY < 0) {
            writingZone.flickable.returnToBounds()
            return
        }

        writingZone.flickable.flick(0, deltaY * 50)

    }

    rightPaneScrollMouseArea.onWheel: {

        var deltaY = wheel.angleDelta.y *10

        writingZone.flickable.flick(0, deltaY)

        if (writingZone.flickable.atYBeginning && wheel.angleDelta.y > 0) {
            writingZone.flickable.returnToBounds()
            return
        }
        if (writingZone.flickable.atYEnd && wheel.angleDelta.y < 0) {
            writingZone.flickable.returnToBounds()
            return
        }
    }

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



    //---------------------------------------------------------


    SkrUserSettings {
        id: skrUserSettings
    }

    function openDocument(_projectId, _paperId) {
        // save current
        if(projectId !== _projectId && paperId !== _paperId ){ //meaning it hasn't just used the constructor
            saveContent()
            saveCurrentPaperCursorPositionAndY()
            skrTextBridge.unsubscribeTextDocument(pageType, projectId, paperId, writingZone.textArea.objectName, writingZone.textArea.textDocument)
        }


        paperId = _paperId
        projectId = _projectId
        writingZone.paperId = _paperId
        writingZone.projectId = _projectId


        console.log("opening sheet :", _projectId, _paperId)
        writingZone.text = plmData.sheetHub().getContent(_projectId, _paperId)
        title = plmData.sheetHub().getTitle(_projectId, _paperId)

        skrTextBridge.subscribeTextDocument(pageType, projectId, paperId, writingZone.textArea.objectName, writingZone.textArea.textDocument)

        // apply format
        writingZone.documentHandler.indentEverywhere = SkrSettings.writeSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.writeSettings.textTopMargin

        restoreCurrentPaperCursorPositionAndY()

        writingZone.forceActiveFocus()
        //save :
        skrUserSettings.setProjectSetting(projectId, "writeCurrentPaperId", paperId)

        // start the timer for automatic position saving
        if(!saveCurrentPaperCursorPositionAndYTimer.running){
            saveCurrentPaperCursorPositionAndYTimer.start()
        }

        leftDock.setCurrentPaperId(projectId, paperId)
        leftDock.setOpenedPaperId(projectId, paperId)

    }

    function restoreCurrentPaperCursorPositionAndY(){

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    projectId, "writeSheetPositionHash", paperId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId, "writeSheetYHash", paperId, 0)
        console.log("newCursorPosition", position)

        // set positions :
        writingZone.setCursorPosition(position)
        writingZone.flickable.contentY = visibleAreaY

    }


    function saveCurrentPaperCursorPositionAndY(){

        if(paperId != -2 || projectId != -2){
            //save cursor position of current document :

            var previousCursorPosition = writingZone.cursorPosition
            //console.log("previousCursorPosition", previousCursorPosition)
            var previousY = writingZone.flickable.contentY
            //console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId, "writeSheetPositionHash", paperId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId,
                                                       "writeSheetYHash",
                                                       paperId,
                                                       previousY)
        }
    }

    Timer{
        id: saveCurrentPaperCursorPositionAndYTimer
        repeat: true
        interval: 10000
        onTriggered: saveCurrentPaperCursorPositionAndY()
    }

    //needed to adapt width to a shrinking window
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width - 200 < writingZone.maximumTextAreaWidth
        value: middleBase.width - 200
        restoreMode: Binding.RestoreBindingOrValue

    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width - 200 >= writingZone.maximumTextAreaWidth
        value: writingZone.maximumTextAreaWidth
        restoreMode: Binding.RestoreBindingOrValue

    }


    //------------------------------------------------------------------------
    //-----minimap------------------------------------------------------------
    //------------------------------------------------------------------------

    property bool minimapVisibility: false
    minimap.visible: minimapVisibility

    Binding on minimap.text {
        when: minimapVisibility
        value: writingZone.textArea.text
        restoreMode: Binding.RestoreBindingOrValue
        delayed: true
    }

    //Scrolling minimap
    Binding on writingZone.internalScrollBar.position {
        when: minimapVisibility
        value: minimap.position
        restoreMode: Binding.RestoreBindingOrValue
        delayed: true
    }
    Binding on  minimap.position {
        when: minimapVisibility
        value: writingZone.internalScrollBar.position
        restoreMode: Binding.RestoreBindingOrValue
        delayed: true
    }

    // ScrollBar invisible while minimap is on
    writingZone.scrollBarVerticalPolicy: minimapVisibility ? ScrollBar.AlwaysOff : ScrollBar.AsNeeded

    //minimap width depends on writingZone width
    //minimapFlickable.width: 300

    //    {
    //        //console.log("writingZone.flickable.width :" + writingZone.flickable.width)
    //        console.log("ratio :" + minimapFlickable.height / writingZone.flickable.contentHeight)
    //        return writingZone.flickable.width * (1 - minimapRatio())
    //    }

    //minimap.height: minimapRatio() >= 1 ? minimapFlickable.height : writingZone.flickable.contentHeight * minimapRatio()


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
        leftDockFixedWidth = leftDockFixedWidth - displacement
        leftDockResizeButtonFirstPressX = pressX

        if(leftDockFixedWidth < 300){
            leftDockFixedWidth = 300
        }
        if(leftDockFixedWidth > 600){
            leftDockFixedWidth = 600
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
        rightDockFixedWidth = rightDockFixedWidth + displacement
        rightDockResizeButtonFirstPressX = pressX

        if(rightDockFixedWidth < 200){
            rightDockFixedWidth = 200
        }
        if(rightDockFixedWidth > 350){
            rightDockFixedWidth = 350
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
    property int leftDockFixedWidth: 400

    Dock {
        id: leftDrawer
        enabled: base.enabled
        parent: base

        width: Globals.compactSize ? 400 : leftDockFixedWidth
        height: base.height
        interactive: Globals.compactSize
        //position: Globals.compactSize ? 0 : 1 (leftDrawer.isVisible ? 1 : 0)
        //isVisible: !Globals.compactSize
        edge: Qt.LeftEdge

        Connections {
            target: Globals
            function onCompactSizeChanged(){
                leftDrawer.isVisible = !Globals.compactSize
            }
        }

        WriteLeftDock {
            id: leftDock
            anchors.fill: parent

        }



        Component.onCompleted: {
            leftDockFixedWidth = leftSettings.width
            Globals.resetDockConfCalled.connect(resetConf)
            if(Globals.compactSize){
                leftDrawer.close()
            }
        }


        Settings {
            id: leftSettings
            category: "writeLeftDock"
            property int width: leftDockFixedWidth
        }

        function resetConf(){
            leftDockFixedWidth = 300

        }

    }

    property alias rightDock: rightDock
    property int rightDockFixedWidth: 400
    Dock {
        id: rightDrawer
        enabled: base.enabled
        parent: base
        width:  Globals.compactSize ? 400 : rightDockFixedWidth
        height: base.height
        interactive: Globals.compactSize
        //position: Globals.compactSize ? 0 : (rightDrawer.isVisible ? 1 : 0)
        //isVisible: !Globals.compactSize
        edge: Qt.RightEdge

        Connections {
            target: Globals
            function onCompactSizeChanged(){
                rightDrawer.isVisible = !Globals.compactSize
            }
        }



        WriteRightDock {
            id: rightDock
            anchors.fill: parent

            projectId: root.projectId
            paperId: root.paperId

        }


        Component.onCompleted: {
            rightDockFixedWidth = rightSettings.width
            Globals.resetDockConfCalled.connect(resetConf)

            if(Globals.compactSize){
                rightDrawer.close()
            }
        }


        Settings {
            id: rightSettings
            category: "writeRightDock"
            property int width: rightDockFixedWidth
        }

        function resetConf(){
            rightDockFixedWidth = 300
        }
    }

    //------------------------------------------------------------
    //------------------------------------------------------------
    //------------------------------------------------------------
    // save content once after writing:
    writingZone.textArea.onTextChanged: {

        //avoid first text change, when blank HTML is inserted
        if(writingZone.textArea.length === 0
                && plmData.projectHub().isProjectNotModifiedOnce(projectId)){
            return
        }

        if(contentSaveTimer.running){
            contentSaveTimer.stop()
        }
        contentSaveTimer.start()
    }
    Timer{
        id: contentSaveTimer
        repeat: false
        interval: 200
        onTriggered: saveContent()
    }

    function saveContent(){
        console.log("saving sheet")
        var error = plmData.sheetHub().setContent(projectId, paperId, writingZone.text)
        if (!error.success){
            console.log("saving sheet failed", projectId, paperId)
        }
        else {
            console.log("saving sheet success", projectId, paperId)

        }
    }

//    writingZone.onActiveFocusChanged: {
//        writingZone.text = plmData.sheetHub().getContent(projectId, paperId)
//        //        restoreCurrentPaperCursorPositionAndY()
//    }


    //    // project to be closed :
    //    Connections{
    //        target: plmData.projectHub()
    //        function onProjectToBeClosed(projectId) {

    //            if (projectId === this.projectId){
    //                // save
    //                saveContent()
    //                saveCurrentPaperCursorPositionAndY()

    //            }
    //        }
    //    }

    //    // projectClosed :
    //    Connections {
    //        target: plmData.projectHub()
    //        // @disable-check M16
    //        onProjectClosed: function (projectId) {

    //            //if there is another project, switch to its last paper
    //            if(plmData.projectHub().getProjectCount() > 0){

    //                var otherProjectId = plmData.projectHub().getProjectIdList()[0]
    //                var defaultPaperId = leftDock.treeView.proxyModel.getLastOfHistory(otherProjectId)
    //                if (defaultPaperId === -2) {// no history
    //                    defaultPaperId = plmData.sheetHub().getTopPaperId(otherProjectId)
    //                }
    //                var otherPaperId = skrUserSettings.getProjectSetting(otherProjectId, "writeCurrentPaperId", defaultPaperId)
    //                Globals.openSheetCalled(otherProjectId, otherPaperId)
    //            }
    //            else { // no project

    //                if(saveCurrentPaperCursorPositionAndYTimer.running){
    //                    saveCurrentPaperCursorPositionAndYTimer.stop()
    //                }

    //            }
    //        }
    //    }



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
                fullscreen_left_drawer_visible = leftDrawer.visible
                fullscreen_right_drawer_visible = rightDrawer.visible
                leftDrawer.visible = false
                rightDrawer.visible = false
            }
            else{
                leftDrawer.visible = fullscreen_left_drawer_visible
                rightDrawer.visible = fullscreen_right_drawer_visible
            }

        }
    }


    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            writingZone.forceActiveFocus()
        }
    }
}
