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

    property int currentPaperId: -2

    SkrUserSettings {
        id: skrUserSettings
    }

//    Component.onCompleted: {
//        textAreaFixedWidth = SkrSettings.writeSettings.textWidth
//    }
    function openDocument(projectId, paperId) {
        if (currentPaperId != -2) {

            //save cursor position of current document :
            var previousCursorPosition = writingZone.cursorPosition
            console.log("previousCursorPosition", previousCursorPosition)
            var previousY = writingZone.flickable.contentY
            console.log("previousContentY", previousY)
            skrUserSettings.insertInProjectSettingHash(
                        projectId, "writeSheetPositionHash", currentPaperId,
                        previousCursorPosition)
            skrUserSettings.insertInProjectSettingHash(projectId,
                                                       "writeSheetYHash",
                                                       currentPaperId,
                                                       previousY)
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
        writingZone.cursorPosition = position
        writingZone.flickable.contentY = visibleAreaY
        currentPaperId = paperId

        writingZone.forceActiveFocus()
        //save :
        skrUserSettings.setProjectSetting(projectId, "writeCurrentPaperId", currentPaperId)

        // select the paper in the navigator :


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

    Connections {
        target: Globals

        // @disable-check M16
        onCompactSizeChanged: {
            if (Globals.compactSize === true) {

            } else {
                leftDrawer.close()
                rightDrawer.close()
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
        background: Rectangle {
            Rectangle {
                x: parent.width - 1
                width: 0
                height: parent.height
                color: "transparent"
            }
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
            writingZone.text = ""
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
