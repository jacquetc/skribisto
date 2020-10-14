import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import '..'

EditViewForm {
    property int minimumHeight: 500

    swipeView.currentIndex: 0


    italicToolButton.action: italicAction
    boldToolButton.action: boldAction
    strikeToolButton.action: strikeAction
    underlineToolButton.action: underlineAction

    // fullscreen :
    fullScreenToolButton.action: fullscreenAction


    checkSpellingToolButton.action: checkSpellingAction


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


    goBackToolButton.action: goBackAction
    goBack2ToolButton.action: goBackAction

    // textWidthSlider :

    textWidthLabel.visible: !Globals.compactSize
    textWidthSlider.visible: !Globals.compactSize

    textWidthSlider.value: SkrSettings.writeSettings.textWidth

    Binding {
        target: SkrSettings.writeSettings
        property: "textWidth"
        value: textWidthSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // textPointSizeSlider :

    textPointSizeSlider.value: SkrSettings.writeSettings.textPointSize


    Binding {
        target: SkrSettings.writeSettings
        property: "textPointSize"
        value: textPointSizeSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Font family combo :
    fontFamilyComboBox.model: skrFonts.fontFamilies()

    Binding {
        target: SkrSettings.writeSettings
        property: "textFontFamily"
        value: fontFamilyComboBox.currentText
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // needed because the SkrSettings won't work for FontFamily
    Settings{
        id: settings
            category: "write"
    }

    function loadFontFamily(){
        var fontFamily = settings.value("textFontFamily", Qt.application.font.family)
        //console.log("fontFamily", fontFamily)
        //console.log("application fontFamily", Qt.application.font.family)

        var index = fontFamilyComboBox.find(fontFamily, Qt.MatchFixedString)
        //console.log("index", index)
        if(index === -1){
            index = fontFamilyComboBox.find(Qt.application.font.family, Qt.MatchFixedString)
        }
        //console.log("index", index)

        fontFamilyComboBox.currentIndex = index
    }

    // Indent :
     textIndentSlider.value: SkrSettings.writeSettings.textIndent

    Binding {
        target: SkrSettings.writeSettings
        property: "textIndent"
        value: textIndentSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Margins :
     textTopMarginSlider.value: SkrSettings.writeSettings.textTopMargin

    Binding {
        target: SkrSettings.writeSettings
        property: "textTopMargin"
        value: textTopMarginSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
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


    //focus
    onActiveFocusChanged: {
        if (activeFocus) {
            //swipeView.currentIndex = 0
            italicToolButton.forceActiveFocus()
        }
    }
}
