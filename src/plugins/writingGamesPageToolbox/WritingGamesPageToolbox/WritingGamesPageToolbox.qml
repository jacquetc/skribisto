import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15

import "../../Items"
import "../../Commons"
import "../.."

WritingGamesPageToolboxForm {
    id: root

    iconSource: "qrc:///icons/backup/roll.svg"
    showButtonText: qsTr( "Show Writing Games toolbox")

    //required property var writingZone
    property var page: rootWindow.viewManager.focusedPage

    property var writingZone
    Component.onCompleted: {
        writingZone = page.writingZone
    }


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
        id: con
        target: Qt.isQtObject(writingZone.textArea) ? writingZone.textArea : undefined
        enabled: forbidErasingSwitch.checked
        ignoreUnknownSignals: true
        function onCursorPositionChanged(){
             writingZone.textArea.cursorPosition = writingZone.textArea.length
        }

    }

    function forbidBackspaceFunction(event){
        if(event.key === Qt.Key_Backspace){

            event.accepted = true
            return true
        }        
        if((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Z){

            event.accepted = true
            return true
        }
        return false
    }





}
