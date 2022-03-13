import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import ".."

OutlinePadForm {
    id: root
    property int projectId: -2
    property int treeItemId: -2
    property int milestone: -2

    //---------------------------------------------------------
    Component.onCompleted: {

        temp.start()
    }

    Timer{
        id :temp
        interval: 20
        onTriggered: {
            //console.log("outlineWritingZone.writingZone.textArea.length", outlineWritingZone.writingZone.textArea.length)
            if (outlineWritingZone.writingZone.textArea.length === 0) {
                addOutlineToolButton.visible = true
                openOutlineToolButton.visible = false
            }
            else {
                outlineWritingZone.visible = true
                addOutlineToolButton.visible = false
                outlineWritingZone.Layout.preferredHeight = 400
            }
        }
    }

    //---------------------------------------------------------
    Action {
        id: openOutlineAction
        text: qsTr("Open outline")
        icon.source: "qrc:///icons/backup/quickopen-file.svg"
        onTriggered: {
            saveContent()
            saveCurrentCursorPositionAndY()
            rootWindow.viewManager.insertAdditionalProperty("isSecondary", true)
            rootWindow.viewManager.loadTreeItemAtAnotherView(projectId,
                                                             treeItemId)
        }
    }
    openOutlineToolButton.action: openOutlineAction

    //---------------------------------------------------------
    Action {
        id: addOutlineAction
        text: qsTr("Add outline")
        icon.source: "qrc:///icons/backup/list-add.svg"
        onTriggered: {
            addOutlineToolButton.visible = false
            openOutlineToolButton.visible = true
            outlineWritingZone.visible = true
            outlineWritingZone.forceActiveFocus()
            outlineWritingZone.Layout.preferredHeight = 400
        }
    }
    addOutlineToolButton.action: addOutlineAction

    //---------------------------------------------------------

    outlineWritingZone.projectId: projectId
    outlineWritingZone.treeItemId: treeItemId
    outlineWritingZone.milestone: milestone


    outlineWritingZone.writingZone.highlighter.spellCheckHighlightColor: SkrTheme.spellCheckHighlight
    outlineWritingZone.writingZone.highlighter.findHighlightColor: SkrTheme.findHighlight
    outlineWritingZone.writingZone.highlighter.otherHighlightColor_1: SkrTheme.otherHighlight_1
    outlineWritingZone.writingZone.highlighter.otherHighlightColor_2: SkrTheme.otherHighlight_2
    outlineWritingZone.writingZone.highlighter.otherHighlightColor_3: SkrTheme.otherHighlight_3


}
