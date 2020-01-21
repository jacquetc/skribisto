import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1
import eu.skribisto.notelistproxymodel 1.0

NotesLeftDockForm {

    property bool folded: settings.dockFolded
    onFoldedChanged: folded ? fold() : unfold()

    function fold() {
        state = "leftDockFolded"
        settings.dockFolded = true
        folded = true
    }
    function unfold() {
        state = "leftDockUnfolded"
        settings.dockFolded = false
        folded = false
    }
    //    splitView.handle:      Rectangle {
    //                                implicitWidth: 4
    //                                implicitHeight: 4
    //    }

    // tree management :
    PLMNoteListProxyModel {

        id: proxyModel
    }

    notesTreeView.model: proxyModel
    notesTreeView.proxyModel: proxyModel

    Action {

        id: fullscreenAction
        text: qsTr("Fullscreen")
        icon {
            name: "welcome-icon"
            source: "qrc:/pics/skribisto.svg"
            color: "transparent"
            height: 50
            width: 50
        }

        shortcut: "F11"
        checkable: true
        checked: true

        onTriggered: root_stack.currentIndex = 0
    }

    transitions: [
        Transition {

            PropertyAnimation {
                properties: "implicitWidth"
                easing.type: Easing.InOutQuad
                duration: 500
            }
        }
    ]

    Settings {
        id: settings
        category: "notesLeftDock"
        //property string dockSplitView: "0"
        property bool dockFolded: false
        property bool treeViewFrameFolded: treeViewFrame.folded ? true : false
        property bool toolsFrameFolded: toolsFrame.folded ? true : false
    }

    //        PropertyAnimation {
    //            target: writeTreeViewFrame
    //            property: "SplitView.preferredHeight"
    //            duration: 500
    //            easing.type: Easing.InOutQuad

    //        }
    Component.onCompleted: {
        folded ? fold() : unfold()

        treeViewFrame.folded = settings.treeViewFrameFolded
        toolsFrame.folded = settings.toolsFrameFolded
        //        splitView.restoreState(settings.dockSplitView)
    }
    Component.onDestruction: {

        settings.dockFolded = folded
    }
}
