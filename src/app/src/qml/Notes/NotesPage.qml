import QtQuick 2.12
import QtQml 2.12
import QtQuick.Controls 2.12
import eu.skribisto.sheethub 1.0
import ".."

NotesPageForm {
    property string pageType: "notes"


    property int textAreaFixedWidth: 400

    writingZone.textAreaWidth: 400
    writingZone.stretch: Globals.compactSize
    writingZone.minimapVisibility: true

    Component.onCompleted: {
        if(!Globals.compactSize){
            leftDrawer.close()
            rightDrawer.close()
            leftDrawer.interactive = false
            rightDrawer.interactive = false
        }
    }

    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width < textAreaFixedWidth
        value: middleBase.width
    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width >= textAreaFixedWidth
        value: textAreaFixedWidth
        //only on 5.14 restoreMode: Binding.RestoreBindingOrValue
    }
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
        NotesLeftDock {
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
    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectLoaded: function (projectId) {
            writingZone.text = plmData.sheetHub().getContent(1, 1)
        }
    }

    // projectClosed :
    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectClosed: function (projectId) {
            writingZone.text = ""
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

