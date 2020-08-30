import QtQuick 2.12
import QtQml 2.12
import QtQuick.Controls 2.12
import Qt.labs.settings 1.1
import '..'

EditViewForm {

    swipeView.currentIndex: 0


    Action{
        id: sizeAction
        text: qsTr("Sizes")
        icon {
            name: "format-font-size-more"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.
        onTriggered: {
            swipeView.setCurrentIndex(1)
            textWidthSlider.forceActiveFocus()
        }

    }
    sizeToolButton.action: sizeAction

    Action{
        id: fullScreenColorAction
        text: qsTr("Full Screen Colors")
        icon {
            name: "color-picker-white"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.
        onTriggered: {
            swipeView.setCurrentIndex(2)
            backroundColorTextField.forceActiveFocus()
        }

    }
    fullScreenColorToolButton.action: fullScreenColorAction



    Action{
        id: goBackAction
        text: qsTr("Go back")
        icon {
            name: "go-previous"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.Backspace
        onTriggered: {
            swipeView.setCurrentIndex(0)
        }

    }
    //    Shortcut{
    //        id: goBackShortcut
    //        sequences: ["Backspace"]
    //        onActivated: {
    //            console.log("goBackShortcut")
    //            goBackAction.trigger()}
    //    }

    goBackToolButton.action: goBackAction
    goBack2ToolButton.action: goBackAction

    // textWidthSlider :

    textWidthSlider.visible: !Globals.compactSize

    textWidthSlider.value: SkrSettings.noteSettings.textWidth

    Binding {
        target: SkrSettings.noteSettings
        property: "textWidth"
        value: textWidthSlider.value
        delayed: true
    }

    // textPointSizeSlider :

    textPointSizeSlider.value: SkrSettings.noteSettings.textPointSize


    Binding {
        target: SkrSettings.noteSettings
        property: "textPointSize"
        value: textPointSizeSlider.value
        delayed: true
    }

    // Font family combo :
    //fontFamilyComboBox.model: skrFonts.getModel()

    Binding {
        target: SkrSettings.noteSettings
        property: "textFontFamily"
        value: fontFamilyComboBox.currentText
        delayed: true
    }

    // needed because the SkrSettings won't work for FontFamily
    Settings{
        id: settings
            category: "note"
    }

    function loadFontFamily(){
        var fontFamily = settings.value("textFontFamily", Qt.application.font.family)
        console.log("fontFamily", fontFamily)
        console.log("fontFamily", Qt.application.font.family)

        var index = fontFamilyComboBox.find(fontFamily)
        console.log("index", index)
        if(index === -1){
            index = fontFamilyComboBox.find(Qt.application.font.family)
        }
        console.log("index", index)

        fontFamilyComboBox.currentIndex = index
    }

    // Indent :
     textIndentSlider.value: SkrSettings.noteSettings.textIndent

    Binding {
        target: SkrSettings.noteSettings
        property: "textIndent"
        value: textIndentSlider.value
        delayed: true
    }

    // Margins :
     textTopMarginSlider.value: SkrSettings.noteSettings.textTopMargin

    Binding {
        target: SkrSettings.noteSettings
        property: "textTopMargin"
        value: textTopMarginSlider.value
        delayed: true
    }


    Component.onCompleted: {
        loadFontFamily()
    }



    // go back when losing focus

    swipeView.onActiveFocusChanged: {
        console.log('activeFocus', swipeView.activeFocus)
        if(!swipeView.activeFocus){
            goBackAction.trigger()
        }
    }

}
