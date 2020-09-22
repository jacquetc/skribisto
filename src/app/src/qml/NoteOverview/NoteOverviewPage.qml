import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.settings 1.1
import "../Commons"
import ".."


NoteOverviewPageForm {
    id: root
    property string pageType: "noteOverview"

    Component.onCompleted: {


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

        rootSwipeView.interactive = true
        leftDockResizeButtonFirstPressX = 0

    }
    //---------------------------------------------------------



    property alias leftDock: leftDock
    property int leftDockFixedWidth: 400
    Dock {
        id: leftDrawer
        parent: base
        enabled: base.enabled

        width: Globals.compactSize ? 400 : leftDockFixedWidth
        height: base.height
        interactive: Globals.compactSize
        position: Globals.compactSize ? 0 : (leftDrawer.isVisible ? 1 : 0)
        isVisible: !Globals.compactSize
        edge: Qt.LeftEdge


        LeftDock {
            id: leftDock
            anchors.fill: parent


        }



        Component.onCompleted: {
            leftDockFixedWidth = leftSettings.width
        }


        Settings {
            id: leftSettings
            category: "noteOverviewLeftDock"
            property int width: leftDockFixedWidth
        }
    }


    // fullscreen :


    property bool fullscreen_left_drawer_visible: false

    Connections {
        target: Globals
        function onFullScreenCalled(value) {
            if(value){
                //save previous conf
                fullscreen_left_drawer_visible = leftDrawer.visible

                leftDrawer.visible = false

            }
            else{
                leftDrawer.visible = fullscreen_left_drawer_visible

            }

        }
    }


}
