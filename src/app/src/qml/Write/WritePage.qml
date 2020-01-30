import QtQuick 2.12
import QtQml 2.12
import QtQuick.Controls 2.12
import eu.skribisto.sheethub 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

WritePageForm {
    id: root
    //property int textAreaFixedWidth: SkrSettings.writeSettings.textWidth
    property var lastFocused: writingZone





    //--------------------------------------------------------
    //---Writing Zone-----------------------------------------
    //--------------------------------------------------------

    writingZone.maximumTextAreaWidth: SkrSettings.writeSettings.textWidth
    writingZone.textPointSize: SkrSettings.writeSettings.textPointSize
    writingZone.textFontFamily: SkrSettings.writeSettings.textFontFamily
    writingZone.textIndent: SkrSettings.writeSettings.textIndent
    writingZone.textTopMargin: SkrSettings.writeSettings.textTopMargin

    writingZone.stretch: Globals.compactSize
    writingZone.minimapVisibility: false
    writingZone.name: "write-0" //useful ?
    minimap.visible: false

    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectCountChanged: function (count){
            if(count === 0){
                writingZone.enabled = false
            }
            else {
                writingZone.enabled = true
            }

        }
    }


    //---------------------------------------------------------


    property int currentPaperId: -2
    property int currentProjectId: -2



    //---------------------------------------------------------

    Component.onCompleted: {
        if(!Globals.compactSize){
            leftDrawer.close()
            rightDrawer.close()
            leftDrawer.interactive = false
            rightDrawer.interactive = false
        }
    }

    //--------------------------------------------------------
    //---Left Scroll Area-----------------------------------------
    //--------------------------------------------------------


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

    function openDocument(projectId, paperId) {
        if (currentProjectId === projectId & currentPaperId === paperId){
            writingZone.forceActiveFocus()
            return
        }

        // invalid paper id, empty project ?
        if(paperId === -2){
            return;
        }

        if (currentPaperId != -2) {
            //save content :
            contentSaveTimer.stop()
            saveContent()
            //save cursor position of current document :
            saveCurrentPaperCursorPositionAndY()
        }

        console.log("opening sheet :", projectId, paperId)
        writingZone.text = plmData.sheetHub().getContent(projectId, paperId)

        // apply format
        writingZone.documentHandler.indentEverywhere = SkrSettings.writeSettings.textIndent
        writingZone.documentHandler.topMarginEverywhere = SkrSettings.writeSettings.textTopMargin


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
        currentPaperId = paperId
        currentProjectId = projectId

        writingZone.forceActiveFocus()
        //save :
        skrUserSettings.setProjectSetting(projectId, "writeCurrentPaperId", currentPaperId)

        // start the timer for automatic position saving
        if(!saveCurrentPaperCursorPositionAndYTimer.running){
            saveCurrentPaperCursorPositionAndYTimer.start()
        }



    }

    function saveCurrentPaperCursorPositionAndY(){

        if(currentPaperId != -2 || currentProjectId != -2){
            //save cursor position of current document :

            var previousCursorPosition = writingZone.cursorPosition
            console.log("previousCursorPosition", previousCursorPosition)
            var previousY = writingZone.flickable.contentY
            console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        currentProjectId, "writeSheetPositionHash", currentPaperId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(currentProjectId,
                                                       "writeSheetYHash",
                                                       currentPaperId,
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



    // minimap:
    Binding on minimap.text {
        value: writingZone.textArea.text
        delayed: true
    }

    //Scrolling minimap
    writingZone.internalScrollBar.position: minimap.position
    minimap.position: writingZone.internalScrollBar.position

    // ScrollBar invisible while minimap is on
    writingZone.scrollBarVerticalPolicy: writingZone.minimapVisibility ? ScrollBar.AlwaysOff : ScrollBar.AsNeeded

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



    //---------------------------------------------------------

    Connections {
        target: Globals

        // @disable-check M16
        onCompactSizeChanged: {
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

            editView.italicToolButton.action: italicAction
            editView.boldToolButton.action: boldAction
            editView.strikeToolButton.action: strikeAction
            editView.underlineToolButton.action: underlineAction
            editView.fullScreenToolButton.action: fullscreenAction
        }
    }

    // projectLoaded :
    property int projectIdForProjectLoading: 0
    property int paperIdForProjectLoading: 0

    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectLoaded: function (projectId) {

            var topPaperId = plmData.sheetHub().getTopPaperId(projectId)
            console.log("topPaperId ::", topPaperId)
            var paperId = skrUserSettings.getProjectSetting(projectId, "writeCurrentPaperId", topPaperId)
            console.log("paperId ::", paperId)
            console.log("projectId ::", projectId)
            projectIdForProjectLoading = projectId
            paperIdForProjectLoading = paperId

            var isPresent = false
            var idList = plmData.sheetHub().getAllIds(projectId)
            var count = idList.length
            console.log("idList", idList)
            console.log("count", count)
            console.log("a", paperId)
            for(var i = 0; i < count ; i++ ){
                console.log("b", paperId)
                if(paperId === idList[i]){
                    isPresent = true
                    console.log("c", paperId)
                }
            }
            if(!isPresent & count > 0){
                console.log("d", paperId)
                paperIdForProjectLoading = idList[0]            }
            else if(!isPresent & count === 0){
                console.log("e", paperId)
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




// save content once after writing:
    writingZone.textArea.onTextChanged: {
        if(contentSaveTimer.running){
            contentSaveTimer.stop()
        }
        contentSaveTimer.start()
    }
    Timer{
        id: contentSaveTimer
        repeat: false
        interval: 1000
        onTriggered: saveContent()
    }

    function saveContent(){
        console.log("saving sheet")
        plmData.sheetHub().setContent(currentProjectId, currentPaperId, writingZone.text)
    }




    // project to be closed :
    Connections{
        target: plmData.projectHub()
        // @disable-check M16
        onProjectToBeClosed: function (projectId) {
            // save
            saveCurrentPaperCursorPositionAndY()

            writingZone.text = ""
        }
    }

    // projectClosed :
    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectClosed: function (projectId) {

            //if there is another project, switch to its last paper
            if(plmData.projectHub().getProjectCount() > 0){

                var otherProjectId = plmData.projectHub().getProjectIdList()[0]
                var defaultPaperId = leftDock.treeView.proxyModel.getLastOfHistory(otherProjectId)
                if (defaultPaperId === -2) {// no history
                    defaultPaperId = plmData.sheetHub().getTopPaperId(otherProjectId)
                }
                var otherPaperId = skrUserSettings.getProjectSetting(otherProjectId, "writeCurrentPaperId", defaultPaperId)
                Globals.openSheetCalled(otherProjectId, otherPaperId)
            }
            else { // no project

                if(saveCurrentPaperCursorPositionAndYTimer.running){
                    saveCurrentPaperCursorPositionAndYTimer.stop()
                }

            }
        }
    }

    // openDocument :
    Connections {
        target: Globals
        // @disable-check M16
        onOpenSheetCalled: function (projectId, paperId) {
            openDocument(projectId, paperId)
        }
    }


    property bool fullscreen_left_dock_folded: false
    property bool fullscreen_right_dock_folded: false
    // fullscreen :
    Connections {
        target: Globals
        // @disable-check M16
        onFullScreenCalled: function (value) {
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
