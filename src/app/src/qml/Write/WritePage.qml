import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.sheethub 1.0
import eu.skribisto.skrusersettings 1.0
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
        enabled: writingZone.visible
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
        if(!Globals.compactSize){
            leftDrawer.close()
            leftDrawer.interactive = false
            rightDrawer.close()
            rightDrawer.interactive = false

        }


        openDocument(projectId, paperId)

        //title = getTitle()
        plmData.sheetHub().titleChanged.connect(changeTitle)
    }

    function changeTitle(_projectId, _paperId, newTitle) {
        if (_projectId === projectId && _paperId === paperId ){
            title = newTitle
            onTitleChangedString(newTitle)
        }
    }

    //---------------------------------------------------------

    function runActionsBedoreDestruction() {

        console.log("runActionsBedoreDestruction")
        saveCurrentPaperCursorPositionAndY()
        contentSaveTimer.stop()
        saveContent()
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
    Binding on rightBasePreferredWidth {
        value:  {
            var value = 0
            if (Globals.compactSize === true){
                value = -1;
            }
            else {

                value = 400 + offset
                if (value < 0) {
                    value = 0
                }
                //                console.debug("right writingZone.wantedCenteredWritingZoneLeftPos :: ", writingZone.wantedCenteredWritingZoneLeftPos)
                //                console.debug("right offset :: ", offset)
                //                console.debug("right value :: ", value)

            }
            rightBasePreferredWidth = value
        }
    }
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

    // fullscreen :
    rightDock.editView.fullScreenToolButton.action: fullscreenAction

    Action {

        id: italicAction
        text: qsTr("Italic")
        icon {
            name: "format-text-italic"
            height: 50
            width: 50
        }

        shortcut: StandardKey.Italic
        checkable: true

        onCheckedChanged: {
            writingZone.documentHandler.italic = italicAction.checked
            if(!lastFocused.activeFocus){
                writingZone.forceActiveFocus()
                if(Globals.compactSize){
                    rightDrawer.close()
                }
            }
        }
    }
    rightDock.editView.italicToolButton.action: italicAction

    Action {

        id: boldAction
        text: qsTr("Bold")
        icon {
            name: "format-text-bold"
            height: 50
            width: 50
        }

        shortcut: StandardKey.Bold
        checkable: true

        onCheckedChanged: {
            writingZone.documentHandler.bold = boldAction.checked
            if(!lastFocused.activeFocus){
                writingZone.forceActiveFocus()
                if(Globals.compactSize){
                    rightDrawer.close()
                }
            }
        }
    }
    rightDock.editView.boldToolButton.action: boldAction

    Action {

        id: strikeAction
        text: qsTr("Strike")
        icon {
            name: "format-text-strikethrough"
            height: 50
            width: 50
        }

        //shortcut: StandardKey
        checkable: true

        onCheckedChanged: {
            writingZone.documentHandler.strikeout = strikeAction.checked
            if(!lastFocused.activeFocus){
                writingZone.forceActiveFocus()
                if(Globals.compactSize){
                    rightDrawer.close()
                }
            }
        }
    }
    rightDock.editView.strikeToolButton.action: strikeAction


    Action {

        id: underlineAction
        text: qsTr("Underline")
        icon {
            name: "format-text-underline"
            height: 50
            width: 50
        }

        shortcut: StandardKey.Underline
        checkable: true

        onCheckedChanged: {
            writingZone.documentHandler.underline = underlineAction.checked
            if(!lastFocused.activeFocus){
                writingZone.forceActiveFocus()
                if(Globals.compactSize){
                    rightDrawer.close()
                }
            }
        }
    }
    rightDock.editView.underlineToolButton.action: underlineAction
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

        console.log("opening sheet :", _projectId, _paperId)
        writingZone.text = plmData.sheetHub().getContent(_projectId, _paperId)
        title = plmData.sheetHub().getTitle(_projectId, _paperId)

        // apply format
        writingZone.documentHandler.indentEverywhere = SkrSettings.writeSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.writeSettings.textTopMargin


        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    _projectId, "writeSheetPositionHash", _paperId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    _projectId, "writeSheetYHash", _paperId, 0)
        console.log("newCursorPosition", position)

        // set positions :
        writingZone.setCursorPosition(position)
        writingZone.flickable.contentY = visibleAreaY
        paperId = _paperId
        projectId = _projectId

        writingZone.forceActiveFocus()
        //save :
        skrUserSettings.setProjectSetting(projectId, "writeCurrentPaperId", paperId)

        // start the timer for automatic position saving
        if(!saveCurrentPaperCursorPositionAndYTimer.running){
            saveCurrentPaperCursorPositionAndYTimer.start()
        }

        leftDock.setCurrentPaperId(projectId, paperId)
        leftDock.setOpenedPaperId(projectId, paperId)


        compactLeftDock.setCurrentPaperId(projectId, paperId)
        compactLeftDock.setOpenedPaperId(projectId, paperId)

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
        when: !Globals.compactSize && middleBase.width < writingZone.maximumTextAreaWidth
        value: middleBase.width

    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width >= writingZone.maximumTextAreaWidth
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
    leftDock.enabled: !Globals.compactSize

    leftDock.onFoldedChanged: {
        if (leftDock.folded) {
            leftDockMenuGroup.visible = false
            leftDockMenuButton.checked = false
            leftDockMenuButton.visible = false
        } else {
            leftDockMenuButton.visible = true
        }
    }

    leftDockShowButton.onClicked: leftDock.folded ? leftDock.unfold(
                                                        ) : leftDock.fold()
    leftDockShowButton.icon {
        name: leftDock.folded ? "go-next" : "go-previous"
        height: 50
        width: 50
    }

    leftDockMenuButton.onCheckedChanged: leftDockMenuButton.checked ? leftDockMenuGroup.visible = true : leftDockMenuGroup.visible = false
    leftDockMenuButton.checked: false
    leftDockMenuButton.icon {
        name: "overflow-menu"
        height: 50
        width: 50
    }

    //leftDockResizeButton.onVisibleChanged: leftDock.folded = false
    //leftDockResizeButton.onClicked:
    leftDockMenuGroup.visible: false
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

    //-------------------------------------------------------------
    //-------Right Dock------------------------------------------
    //-------------------------------------------------------------



    rightDock.enabled: !Globals.compactSize
    rightDock.onFoldedChanged: {
        if (rightDock.folded) {
            rightDockMenuGroup.visible = false
            rightDockMenuButton.checked = false
            rightDockMenuButton.visible = false
        } else {
            rightDockMenuButton.visible = true
        }
    }

    rightDockShowButton.onClicked: rightDock.folded ? rightDock.unfold(
                                                          ) : rightDock.fold()
    rightDockShowButton.icon {
        name: rightDock.folded ? "go-previous" : "go-next"
        height: 50
        width: 50
    }

    rightDockMenuButton.onCheckedChanged: rightDockMenuButton.checked ? rightDockMenuGroup.visible = true : rightDockMenuGroup.visible = false
    rightDockMenuButton.checked: false
    rightDockMenuButton.icon {
        name: "overflow-menu"
        height: 50
        width: 50
    }

    //rightDockResizeButton.onVisibleChanged: rightDock.folded = false
    //rightDockResizeButton.onClicked:
    rightDockMenuGroup.visible: false
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





    rightDock.projectId: projectId
    rightDock.paperId: paperId


    //---------------------------------------------------------
    //---------------------------------------------------------


    Connections {
        target: Globals
        function onCompactSizeChanged() {

            if (Globals.compactSize === true) {
                leftDrawer.interactive = true
                rightDrawer.interactive = true

            } else {
                leftDrawer.close()
                rightDrawer.close()
                leftDrawer.interactive = false
                rightDrawer.interactive = false
            }
        }
    }

    Drawer {
        id: leftDrawer
        enabled: Globals.compactSize
        width: if (base.width * 0.6 > 400) {
                   return 400
               } else {
                   return base.width * 0.6
               }
        height: base.height
        modal: Globals.compactSize ? true : false
        edge: Qt.LeftEdge

        //        interactive: Globals.compactSize ? true : false
        //        visible:true
        //        position: Globals.compactSize ? 0 : 1
        WriteLeftDock {
            id: compactLeftDock
            anchors.fill: parent

            settings.category: settings.category + "-drawer"

        }
    }

    Drawer {
        id: rightDrawer
        enabled: Globals.compactSize
        width: if (base.width * 0.6 > 400) {
                   return 400
               } else {
                   return base.width * 0.6
               }
        height: base.height
        modal: Globals.compactSize ? true : false
        edge: Qt.RightEdge

        //        interactive: Globals.compactSize ? true : false
        //        visible:true
        //        position: Globals.compactSize ? 0 : 1

        WriteRightDock {
            id: compactRightDock
            anchors.fill: parent

            settings.category: settings.category + "-drawer"

            projectId: projectId
            paperId: paperId

            editView.italicToolButton.action: italicAction
            editView.boldToolButton.action: boldAction
            editView.strikeToolButton.action: strikeAction
            editView.underlineToolButton.action: underlineAction
            editView.fullScreenToolButton.action: fullscreenAction
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
        console.log("saving sheet")
        var error = plmData.sheetHub().setContent(projectId, paperId, writingZone.text)
        if (!error.success){
            console.log("saving sheet failed", projectId, paperId)
        }
        else {
            console.log("saving sheet success", projectId, paperId)

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


    property bool fullscreen_left_dock_folded: false
    property bool fullscreen_right_dock_folded: false
    // fullscreen :
    Connections {
        target: Globals
        function onFullScreenCalled(value) {
            if(value){
                //save previous conf
                fullscreen_left_dock_folded = leftDock.folded
                fullscreen_right_dock_folded = rightDock.folded
                leftDock.fold()
                rightDock.fold()
            }
            else{
                leftDock.folded = fullscreen_left_dock_folded
                rightDock.folded = fullscreen_right_dock_folded
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
