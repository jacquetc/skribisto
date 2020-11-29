import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.notehub 1.0
import eu.skribisto.usersettings 1.0
import Qt.labs.settings 1.1
import "../Commons"
import ".."

NotePageForm {

    id: root
    clip: true


    property string title: {return getTitle()}

    function getTitle(){
        return plmData.noteHub().getTitle(projectId, paperId)

    }

    signal onTitleChangedString(string newTitle)

    //property int textAreaFixedWidth: SkrSettings.writeSettings.textWidth
    property var lastFocused: writingZone

    property string pageType: "note"




    //--------------------------------------------------------
    //---Writing Zone-----------------------------------------
    //--------------------------------------------------------

    writingZone.maximumTextAreaWidth: SkrSettings.noteSettings.textWidth
    writingZone.textPointSize: SkrSettings.noteSettings.textPointSize
    writingZone.textFontFamily: SkrSettings.noteSettings.textFontFamily
    writingZone.textIndent: SkrSettings.noteSettings.textIndent
    writingZone.textTopMargin: SkrSettings.noteSettings.textTopMargin

    writingZone.stretch: Globals.compactMode


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
        plmData.noteHub().titleChanged.connect(changeTitle)

    }

    //---------------------------------------------------------

    function changeTitle(_projectId, _paperId, newTitle) {
        if (_projectId === projectId && _paperId === paperId ){
            title = newTitle
            onTitleChangedString(newTitle)
        }

    }

    //---------------------------------------------------------

    function clearNoteWritingZone(){
        if(paperId !== -2 && projectId !== -2){
            contentSaveTimer.stop()
            saveContent()
            saveCurrentPaperCursorPositionAndYTimer.stop()
            saveCurrentPaperCursorPositionAndY()
            skrTextBridge.unsubscribeTextDocument(pageType, projectId, paperId, writingZone.textArea.objectName, writingZone.textArea.textDocument)

            root.projectId = -2
            root.paperId = -2
        }

        writingZone.clear()
    }

    //---------------------------------------------------------

    function runActionsBeforeDestruction() {
        clearNoteWritingZone()
    }

    Component.onDestruction: {
        runActionsBeforeDestruction()
    }


    //---------------------------------------------------------
    // modifiable :

    property bool isModifiable: true

    Connections{
        target: plmData.notePropertyHub()
        function onPropertyChanged(projectId, propertyId, paperId, name, value){
            if(projectId === root.projectId && paperId === root.paperId){

                if(name === "modifiable"){
                    determineModifiable()
                }
            }
        }
    }
    function determineModifiable(){

        root.isModifiable = plmData.notePropertyHub().getProperty(projectId, paperId, "modifiable", "true") === "true"

        saveCurrentPaperCursorPositionAndY()
         writingZone.textArea.readOnly = !root.isModifiable
        restoreCurrentPaperCursorPositionAndY()
    }


    //--------------------------------------------------------
    //---Left Scroll Area-----------------------------------------
    //--------------------------------------------------------
    property int leftOffset: leftDrawer.width * leftDrawer.position


    Binding on leftBasePreferredWidth {
        value:  {
            var value = 0
            if (Globals.compactMode === true){
                value = -1;
            }
            else {

                value = writingZone.wantedCenteredWritingZoneLeftPos - leftOffset
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
    //            if (Globals.compactMode === true){
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
    //        when: SkrSettings.rootSettings.onLeftDockWidthChanged || Globals.onCompactModeChanged || writingZone.onWidthChanged
    //            value:  {
    //                if (Globals.compactMode === true){
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
        if(Globals.compactMode){
            rightDrawer.close()
        }
    }


    //---------------------------------------------------------

    QtObject {
        id: documentPrivate
        property bool contentSaveTimerAllowedToStart: true
        property bool saveCurrentPaperCursorPositionAndYTimerAllowedToStart: true
    }

    //---------------------------------------------------------



    SKRUserSettings {
        id: skrUserSettings
    }

    function openDocument(_projectId, _paperId) {
        // save current
        if(projectId !== _projectId && paperId !== _paperId ){ //meaning it hasn't just used the constructor
            clearNoteWritingZone()
        }

        documentPrivate.contentSaveTimerAllowedToStart = false
        documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = false

        paperId = _paperId
        projectId = _projectId
        writingZone.paperId = _paperId
        writingZone.projectId = _projectId

        //console.log("opening note :", _projectId, _paperId)
        writingZone.text = plmData.noteHub().getContent(_projectId, _paperId)
        title = plmData.noteHub().getTitle(_projectId, _paperId)

        skrTextBridge.subscribeTextDocument(pageType, projectId, paperId, writingZone.textArea.objectName, writingZone.textArea.textDocument)

        // apply format
        writingZone.documentHandler.indentEverywhere = SkrSettings.noteSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.noteSettings.textTopMargin

        restoreCurrentPaperCursorPositionAndY()

        writingZone.forceActiveFocus()
        //save :
        skrUserSettings.setProjectSetting(projectId, "noteCurrentPaperId", paperId)

        // start the timer for automatic position saving
        documentPrivate.saveCurrentPaperCursorPositionAndYTimerAllowedToStart = true
        if(!saveCurrentPaperCursorPositionAndYTimer.running){
            saveCurrentPaperCursorPositionAndYTimer.start()
        }

        documentPrivate.contentSaveTimerAllowedToStart = true

        leftDock.setCurrentPaperId(projectId, paperId)
        leftDock.setOpenedPaperId(projectId, paperId)

        determineModifiable()

    }

    function restoreCurrentPaperCursorPositionAndY(){

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    projectId, "notePositionHash", paperId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId, "noteYHash", paperId, 0)
        //console.log("newCursorPosition", position)

        // set positions :
        writingZone.setCursorPosition(position)
        writingZone.flickable.contentY = visibleAreaY

    }

    function saveCurrentPaperCursorPositionAndY(){

        if(paperId !== -2 || projectId !== -2){
            //save cursor position of current document :

            var previousCursorPosition = writingZone.textArea.cursorPosition
            //console.log("previousCursorPosition", previousCursorPosition)
            var previousY = writingZone.flickable.contentY
            //console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId, "notePositionHash", paperId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId,
                                                       "noteYHash",
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
        when: !Globals.compactMode && middleBase.width - 200 < writingZone.maximumTextAreaWidth
        value: middleBase.width - 200
        restoreMode: Binding.RestoreBindingOrValue

    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactMode && middleBase.width - 200 >= writingZone.maximumTextAreaWidth
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


    leftDockMenuGroup.visible: !Globals.compactMode && leftDockMenuButton.checked
    leftDockMenuButton.visible: !Globals.compactMode


    leftDockShowButton.onClicked: leftDrawer.isVisible ? leftDrawer.isVisible = false : leftDrawer.isVisible = true

    leftDockShowButton.icon {
        source: leftDrawer.isVisible ? "qrc:///icons/backup/go-previous.svg" : "qrc:///icons/backup/go-next.svg"
        height: 50
        width: 50
    }

    leftDockMenuButton.icon {
        source: "qrc:///icons/backup/overflow-menu.svg"
        height: 50
        width: 50
    }

    leftDockResizeButton.icon {
        source: "qrc:///icons/backup/resizecol.svg"
        height: 50
        width: 50
    }


    // compact mode :
    compactLeftDockShowButton.visible: Globals.compactMode

    compactLeftDockShowButton.onClicked: leftDrawer.open()
    compactLeftDockShowButton.icon {
        source: "qrc:///icons/backup/go-next.svg"
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


    rightDockMenuGroup.visible: !Globals.compactMode && rightDockMenuButton.checked
    rightDockMenuButton.visible: !Globals.compactMode

    rightDockShowButton.onClicked: rightDrawer.isVisible ? rightDrawer.isVisible = false : rightDrawer.isVisible = true

    rightDockShowButton.icon {
        source: rightDrawer.isVisible ? "qrc:///icons/backup/go-next.svg" : "qrc:///icons/backup/go-previous.svg"
        height: 50
        width: 50
    }

    rightDockMenuButton.icon {
        source: "qrc:///icons/backup/overflow-menu.svg"
        height: 50
        width: 50
    }

    rightDockResizeButton.icon {
        source: "qrc:///icons/backup/resizecol.svg"
        height: 50
        width: 50
    }

    // compact mode :
    compactRightDockShowButton.visible: Globals.compactMode

    compactRightDockShowButton.onClicked: rightDrawer.open()
    compactRightDockShowButton.icon {
        source: "qrc:///icons/backup/go-previous.svg"
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
        interactive: Globals.compactMode
        dockModeEnabled: !Globals.compactMode
        settingsCategory: "noteLeftDrawer"
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
        enabled: base.enabled
        parent: base
        widthInDockMode: rightDrawerFixedWidth
        widthInDrawerMode: 400
        height: base.height
        interactive: Globals.compactMode
        dockModeEnabled: !Globals.compactMode
        settingsCategory: "noteRightDrawer"
        edge: Qt.RightEdge


        RightDock {
            id: rightDock
            anchors.fill: parent

            projectId: root.projectId
            paperId: root.paperId

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
        if(documentPrivate.contentSaveTimerAllowedToStart){
            contentSaveTimer.start()
        }

    }
    Timer{
        id: contentSaveTimer
        repeat: false
        interval: 200
        onTriggered: saveContent()
    }

    function saveContent(){
        //console.log("saving note")
        var result = plmData.noteHub().setContent(projectId, paperId, writingZone.text)
        if (!result.success){
            console.log("saving note failed", projectId, paperId)
        }
        else {
            //console.log("saving note success", projectId, paperId)

        }
    }




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
    //                    defaultPaperId = plmData.noteHub().getTopPaperId(otherProjectId)
    //                }
    //                var otherPaperId = skrUserSettings.getProjectSetting(otherProjectId, "writeCurrentPaperId", defaultPaperId)
    //                Globals.openNoteCalled(otherProjectId, otherPaperId)
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

        var menuCount = mainMenu.count


        // move Help menu
        menuCount = mainMenu.count
        var m;
        for(m = 0 ; m < menuCount ; m++){
            var menu = mainMenu.menuAt(m)
            if(menu.objectName === "helpMenu"){
                mainMenu.moveItem(m, mainMenu.count -1)
            }
        }

        // move bottomMenuItem
        var bottomMenuItem
        var p;
        for(p = 0 ; p < menuCount ; p++){
            var p_menu = mainMenu.itemAt(p)
            if(p_menu.objectName === "bottomMenuItem"){
                mainMenu.moveItem(p, mainMenu.count - 1)
            }
        }

    }

    function removeMenus(){
        if(mainMenu === null){
            return
        }

        var menuCount = mainMenu.count

        menuCount = mainMenu.count
        var i;
        for(i = menuCount - 1 ; i >= 0  ; i--){
            var menu1 = mainMenu.menuAt(i)

            if(!menu1){
                continue
            }

            if(menu1.objectName === "navigationDockMenu-" + privateMenuObject.dockUniqueId
                    || menu1.objectName === "toolDockMenu-" + privateMenuObject.dockUniqueId){

                mainMenu.removeMenu(menu1)

            }
        }



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



    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            writingZone.forceActiveFocus()
        }
    }
}


/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

