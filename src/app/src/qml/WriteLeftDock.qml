import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1

WriteLeftDockForm {

    property bool folded: state === "leftDockFolded" ? true : false
    onFoldedChanged:  folded ? fold(): unfold()

    function fold(){
        state = "leftDockFolded"

    }
    function unfold(){
        state = "leftDockUnfolded"

    }


    //
    splitView.handle:      Rectangle {
                                implicitWidth: 4
                                implicitHeight: 4
    }

    Action{

        id: fullscreenAction
        text: qsTr("Fullscreen")
        icon {
            name: "welcome-icon"
            source: "qrc:/pics/skribisto.svg"
            color: "transparent"
            height: 50
            width: 50

        }

        shortcut: "F5"
        checkable: true
        checked: true

        onTriggered: root_stack.currentIndex = 0
    }


    transitions: [
        Transition {

            PropertyAnimation { properties: "implicitWidth"; easing.type: Easing.InOutQuad;duration: 500  }


        }
    ]

    Settings {
        id: settings
        property string writeLeftDockSplitView: "0"
        property bool writeTreeViewFrameFolded: writeTreeViewFrame.folded ? true : false
        property bool writeToolsFrameFolded: writeToolsFrame.folded ? true : false


    }



    Component.onCompleted:{

        writeTreeViewFrame.folded = settings.writeTreeViewFrameFolded
        writeToolsFrame.folded = settings.writeToolsFrameFolded
        splitView.restoreState(settings.writeLeftDockSplitView)

    }
    Component.onDestruction:{
        settings.writeLeftDockSplitView = splitView.saveState()
    }

}
