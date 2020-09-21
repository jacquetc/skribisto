import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.notehub 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

NotePageForm {

    id: root


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


    //---------------------------------------------------------


    property int paperId: -2
    property int projectId: -2



    //---------------------------------------------------------

    Component.onCompleted: {


        openDocument(projectId, paperId)

        //title = getTitle()
        plmData.noteHub().titleChanged.connect(changeTitle)
    }

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
        onTriggered: closeRightDrawer()
    }
    Connections {
        target: boldAction
        onTriggered: closeRightDrawer()
    }
    Connections {
        target: strikeAction
        onTriggered: closeRightDrawer()
    }
    Connections {
        target: underlineAction
        onTriggered: closeRightDrawer()
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
        }


        console.log("opening note :", _projectId, _paperId)
        writingZone.text = plmData.noteHub().getContent(_projectId, _paperId)
        title = plmData.noteHub().getTitle(_projectId, _paperId)

        // apply format
        writingZone.documentHandler.indentEverywhere = SkrSettings.writeSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.writeSettings.textTopMargin


        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    _projectId, "notePositionHash", _paperId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    _projectId, "noteYHash", _paperId, 0)
        console.log("newCursorPosition", position)

        // set positions :
        writingZone.setCursorPosition(position)
        writingZone.flickable.contentY = visibleAreaY
        paperId = _paperId
        projectId = _projectId

        writingZone.forceActiveFocus()
        //save :
        skrUserSettings.setProjectSetting(projectId, "noteCurrentPaperId", paperId)

        // start the timer for automatic position saving
        if(!saveCurrentPaperCursorPositionAndYTimer.running){
            saveCurrentPaperCursorPositionAndYTimer.start()
        }

        leftDock.setCurrentPaperId(projectId, paperId)
        leftDock.setOpenedPaperId(projectId, paperId)


    }

    function saveCurrentPaperCursorPositionAndY(){

        if(paperId != -2 || projectId != -2){
            //save cursor position of current document :

            var previousCursorPosition = writingZone.cursorPosition
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
        when: !Globals.compactSize && middleBase.width - 200 < writingZone.maximumTextAreaWidth
        value: middleBase.width - 200

    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width - 200 >= writingZone.maximumTextAreaWidth
        value: writingZone.maximumTextAreaWidth

    }




    //------------------------------------------------------------------------
    //-----minimap------------------------------------------------------------
    //------------------------------------------------------------------------

    property bool minimapVisibility: false
    minimap.visible: minimapVisibility

    Binding on minimap.text {
        when: minimapVisibility
        value: writingZone.textArea.text
        delayed: true
    }

    //Scrolling minimap
    Binding on writingZone.internalScrollBar.position {
        when: minimapVisibility
        value: minimap.position
        delayed: true
    }
    Binding on  minimap.position {
        when: minimapVisibility
        value: writingZone.internalScrollBar.position
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


    leftDockShowButton.onClicked: leftDrawer.visible ? leftDrawer.visible = false : leftDrawer.visible = true

    leftDockShowButton.icon {
        name: leftDrawer.visible ? "go-previous" : "go-next"
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
    compactHeaderPane.visible: Globals.compactSize

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
        rootSwipeView.interactive = true
    }

    leftDockResizeButton.onPressXChanged: {

        if(leftDockResizeButtonFirstPressX === 0){
            leftDockResizeButtonFirstPressX = root.mapFromItem(leftDockResizeButton, leftDockResizeButton.pressX, 0).x
        }

        var pressX = root.mapFromItem(leftDockResizeButton, leftDockResizeButton.pressX, 0).x
        var displacement = leftDockResizeButtonFirstPressX - pressX
        leftDock.fixedWidth = leftDock.fixedWidth - displacement
        leftDockResizeButtonFirstPressX = pressX

        if(leftDock.fixedWidth < 300){
            leftDock.fixedWidth = 300
        }
        if(leftDock.fixedWidth > 600){
            leftDock.fixedWidth = 600
        }



    }

    leftDockResizeButton.onPressed: {

        rootSwipeView.interactive = false

    }

    leftDockResizeButton.onCanceled: {

        rootSwipeView.interactive = true
        leftDockResizeButtonFirstPressX = 0


    }

    //-------------------------------------------------------------
    //-------Right Dock------------------------------------------
    //-------------------------------------------------------------


    rightDockMenuGroup.visible: !Globals.compactSize && rightDockMenuButton.checked
    rightDockMenuButton.visible: !Globals.compactSize

    rightDockShowButton.onClicked: rightDrawer.visible ? rightDrawer.visible = false : rightDrawer.visible = true

    rightDockShowButton.icon {
        name: rightDock.visible ? "go-next" : "go-previous"
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
        rootSwipeView.interactive = true
    }

    rightDockResizeButton.onPressXChanged: {
        if(rightDockResizeButtonFirstPressX === 0){
            rightDockResizeButtonFirstPressX = root.mapFromItem(rightDockResizeButton, rightDockResizeButton.pressX, 0).x
        }

        var pressX = root.mapFromItem(rightDockResizeButton, rightDockResizeButton.pressX, 0).x
        var displacement = rightDockResizeButtonFirstPressX - pressX
        rightDock.fixedWidth = rightDock.fixedWidth + displacement
        rightDockResizeButtonFirstPressX = pressX

        if(rightDock.fixedWidth < 200){
            rightDock.fixedWidth = 200
        }
        if(rightDock.fixedWidth > 350){
            rightDock.fixedWidth = 350
        }


    }

    rightDockResizeButton.onPressed: {

            rootSwipeView.interactive = false

    }

    rightDockResizeButton.onCanceled: {

            rootSwipeView.interactive = true
            rightDockResizeButtonFirstPressX = 0

}






    //---------------------------------------------------------
    //---------------------------------------------------------


    property alias leftDock: leftDock
    Drawer {
        id: leftDrawer
        parent: base
        enabled: base.enabled

        width: Globals.compactSize ? 400 : leftDock.fixedWidth
        height: base.height
        modal: false
        interactive: Globals.compactSize
        position: Globals.compactSize ? 0 : (leftDrawer.visible ? 1 : 0)
        visible: !Globals.compactSize
        edge: Qt.LeftEdge


        LeftDock {
            id: leftDock
            anchors.fill: parent


        }
    }


    property alias rightDock: rightDock
//    rightDock.projectId: projectId
//    rightDock.paperId: paperId

    Drawer {
        id: rightDrawer
        parent: base
        enabled: base.enabled
        width:  Globals.compactSize ? 400 : rightDock.fixedWidth
        height: base.height
        modal: false
        interactive: Globals.compactSize
        position: Globals.compactSize ? 0 : (rightDrawer.visible ? 1 : 0)
        visible: !Globals.compactSize
        edge: Qt.RightEdge


        RightDock {
            id: rightDock
            anchors.fill: parent
            projectId: root.projectId
            paperId: root.paperId


        }



    }






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
        interval: 100
        onTriggered: saveContent()
    }

    function saveContent(){
        console.log("saving note")
        var error = plmData.noteHub().setContent(projectId, paperId, writingZone.text)
        if (!error.success){
            console.log("saving note failed", projectId, paperId)
        }
        else {
            console.log("saving note success", projectId, paperId)

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



    // fullscreen :

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


/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

