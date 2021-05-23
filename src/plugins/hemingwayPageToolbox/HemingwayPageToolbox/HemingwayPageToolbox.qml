import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15

import "../../Items"
import "../../Commons"
import "../.."

HemingwayPageToolboxForm {
    id: root

    iconSource: "qrc:///icons/backup/story-editor.svg"
    showButtonText: qsTr( "Show Hemingway toolbox")

    //required property var writingZone
    property var page: root.ApplicationWindow.window.viewManager.focusedPage
    property var writingZone: page.writingZone

    forbidErasingSwitch.onCheckedChanged: {
        if(forbidErasingSwitch.checked){
            writingZone.textArea.cursorPosition = writingZone.textArea.length
            writingZone.textArea.deselect()
            writingZone.textArea.addAdditionalKeyFunction(forbidBackspaceFunction)

        }
        else {
            writingZone.textArea.removeAdditionalKeyFunction(forbidBackspaceFunction)
        }
    }


    Connections {
        target: writingZone.textArea
        enabled: forbidErasingSwitch.checked
        function onCursorPositionChanged(){
             writingZone.textArea.cursorPosition = writingZone.textArea.length
        }

    }

    function forbidBackspaceFunction(event){
        if(event.key === Qt.Key_Backspace){

            event.accepted = true
            return true
        }

        return false
    }





}
