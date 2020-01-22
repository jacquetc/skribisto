import QtQuick 2.12
import QtQml 2.12
import QtQuick.Controls 2.12
import eu.skribisto.sheethub 1.0
import eu.skribisto.skrusersettings 1.0
import ".."

WritePageForm {
    id: root
    property int textAreaFixedWidth: SkrSettings.writeSettings.textWidth


    //writingZone.textAreaWidth: 800 //

    writingZone.stretch: Globals.compactSize
    writingZone.minimapVisibility: true
    minimap.visible: false

    property int currentPaperId: -2
    property int currentProjectId: -2


    Component.onCompleted: {
        if(!Globals.compactSize){
            leftDrawer.close()
            rightDrawer.close()
            leftDrawer.interactive = false
            rightDrawer.interactive = false
        }
    }


    SkrUserSettings {
        id: skrUserSettings
    }

    function openDocument(projectId, paperId) {
        if (currentPaperId != -2) {

            //save cursor position of current document :
            saveCurrentPaperCursorPositionAndY()
        }

        console.log("opening sheet :", projectId, paperId)
        writingZone.text = plmData.sheetHub().getContent(projectId, paperId)

        //get cursor position
        var position = skrUserSettings.getFromProjectSettingHash(
                    projectId, "writeSheetPositionHash", paperId, 0)
        //get Y
        var visibleAreaY = skrUserSettings.getFromProjectSettingHash(
                    projectId, "writeSheetYHash", paperId, 0)
        console.log("newCursorPosition", position)

        // set positions :
        writingZone.cursorPosition = position
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
        when: !Globals.compactSize && middleBase.width < textAreaFixedWidth
        value: middleBase.width

    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width >= textAreaFixedWidth
        value: textAreaFixedWidth

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
        source: "qrc:/pics/go-next.svg"
        color: "transparent"
        height: 50
        width: 50
    }

    //-------------------------------------------------------------
    //-------Right Dock------------------------------------------
    //-------------------------------------------------------------
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
        name: "go-next"
        source: "qrc:/pics/go-next.svg"
        color: "transparent"
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
            anchors.fill: parent
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
            var paperId = skrUserSettings.getProjectSetting(projectId, "writeCurrentPaperId", topPaperId)
            console.log("paperId ::", paperId)
            console.log("projectId ::", projectId)
            projectIdForProjectLoading = projectId
            paperIdForProjectLoading = paperId

            projectLoadingTimer.start()



        }
    }
    Timer{
        id: projectLoadingTimer
        repeat: false
        interval: 100
        onTriggered: Globals.openSheetCalled(projectIdForProjectLoading, paperIdForProjectLoading)
    }

    // projectClosed :
    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectClosed: function (projectId) {
            // save
            saveCurrentPaperCursorPositionAndY()

            writingZone.text = ""

            //if there is another project, switch to its last paper
            if(plmData.projectHub().getProjectCount() > 0){

                var otherProjectId = plmData.projectHub().getProjectIdList()[0]
                var defaultPaperId = proxyModel.getLastOfHistory(otherProjectId)
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


    // focus :
    onActiveFocusChanged: {
        if (activeFocus) {
            writingZone.forceActiveFocus()
        }
    }
}
