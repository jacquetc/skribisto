import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import eu.skribisto.implementationantidote 1.0

import "../../Items"
import "../../Commons"
import "../.."

AntidotePageToolboxForm {
    id: root

    iconSource: "qrc:///icons/backup/story-editor.svg"
    showButtonText: qsTr( "Show Druide Antidote\u2122 toolbox")

    //required property var writingZone
    property var page: rootWindow.viewManager.focusedPage

    property var writingZone
    Component.onCompleted: {
        writingZone = page.writingZone
    }


    ImplementationAntidote{
        id: antidote
        textDocument: writingZone.textArea.textDocument
        selectionStart: writingZone.textArea.selectionStart
        selectionEnd: writingZone.textArea.selectionEnd
        title: "test"
        textIndent: SkrSettings.textSettings.textIndent
        textTopMargin: SkrSettings.textSettings.textTopMargin

        onActivateDocument: writingZone.textArea.forceActiveFocus()

        onForceFocusOnTextAreaCalled: writingZone.textArea.forceActiveFocus()
        onSelect: function(begin, end){
            writingZone.textArea.select(begin, end)
        }

    }



    correctorButton.onClicked: {
        antidote.launchCorrector()
    }

    dictionaryButton.onClicked: {
        antidote.launchDictionaries()

    }


    guideButton.onClicked: {
        antidote.launchGuides()

    }
    //    forbidErasingSwitch.onCheckedChanged: {
    //        if(forbidErasingSwitch.checked){
    //            writingZone.textArea.cursorPosition = writingZone.textArea.length
    //            writingZone.textArea.deselect()
    //            writingZone.textArea.addAdditionalKeyFunction(forbidBackspaceFunction)

    //        }
    //        else {
    //            writingZone.textArea.removeAdditionalKeyFunction(forbidBackspaceFunction)
    //        }
    //    }


    //    Connections {
    //        id: con
    //        target: Qt.isQtObject(writingZone.textArea) ? writingZone.textArea : undefined
    //        enabled: forbidErasingSwitch.checked
    //        ignoreUnknownSignals: true
    //        function onCursorPositionChanged(){
    //             writingZone.textArea.cursorPosition = writingZone.textArea.length
    //        }

    //    }






}
