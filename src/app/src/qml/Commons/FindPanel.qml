import QtQuick 2.15
import eu.skribisto.documenthandler 1.0

FindPanelForm {
    id: root

    required property var documentHandler
    required property var highlighter
    required property var textArea
    property string stringToFind: ""


    onStringToFindChanged: {
        findTextField.text = stringToFind
    }

    visible: false

    showFindReplaceToolButton.onClicked: replaceRowLayout.visible = !replaceRowLayout.visible

    findTextField.onTextChanged: {
        highlighter.setTextToHighlight(findTextField.text)
    }

    QtObject{
        id: priv
        property real fromPosition: 0
        property int findFlags: 0
        property bool selectedByFind: false
    }

    Connections{
        target: textArea
        function onCursorPositionChanged(){
            priv.selectedByFind = false
        }

    }

    closeToolButton.onClicked: visible = false

    findNextToolButton.onClicked: {
        findNext()
    }

    function findNext(){
        priv.fromPosition = textArea.cursorPosition
        var nextPosition = documentHandler.findNextPosition(findTextField.text, priv.fromPosition, priv.findFlags)

        if(nextPosition === -1){
            priv.fromPosition = 0
            nextPosition = documentHandler.findNextPosition(findTextField.text, priv.fromPosition, priv.findFlags)

            if(nextPosition === -1){
                return nextPosition
            }

        }

        var endPosition = nextPosition + findTextField.text.length
        console.log(nextPosition, endPosition)

        textArea.cursorPosition = nextPosition
        textArea.moveCursorSelection(endPosition, TextEdit.SelectCharacters)
        priv.selectedByFind = true

        priv.fromPosition = endPosition

        return nextPosition

    }


    findPreviousToolButton.onClicked: {

        priv.fromPosition = textArea.selectionStart
        var previousPosition = documentHandler.findPreviousPosition(findTextField.text, priv.fromPosition, priv.findFlags)

        if(previousPosition === -1){
            priv.fromPosition = textArea.length
            var e = textArea.length
            previousPosition = documentHandler.findPreviousPosition(findTextField.text, priv.fromPosition, priv.findFlags)

            if(previousPosition === -1){
                return previousPosition
            }

        }

        var endPosition = previousPosition + findTextField.text.length
        //console.log(previousPosition, endPosition)

        textArea.cursorPosition = previousPosition
        textArea.moveCursorSelection(endPosition, TextEdit.SelectCharacters)
        priv.selectedByFind = true

        priv.fromPosition = previousPosition
        return previousPosition
    }

    replaceToolButton.onClicked: {
        replace()
    }

    function replace(){
        if(priv.selectedByFind){

            var replaceText = replaceTextField.text

            documentHandler.replaceWordAt(findTextField.text, replaceTextField.text, textArea.selectionStart)
            textArea.cursorPosition = textArea.selectionStart - replaceText.length
            textArea.moveCursorSelection(textArea.cursorPosition + replaceText.length, TextEdit.SelectCharacters)

        }

    }

    replaceAllToolButton.onClicked: {
        documentHandler.replaceAllWords(findTextField.text, replaceTextField.text, priv.findFlags)
    }

    findAndReplaceWithToolButton.onClicked: {
        if(findNext() !== -1){
            replace()
        }
    }


    onActiveFocusChanged: {
        if(activeFocus){
            findTextField.forceActiveFocus()
        }
    }
}

