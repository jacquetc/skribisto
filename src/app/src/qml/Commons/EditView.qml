import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import '..'

EditViewForm {
    id: root
    property int minimumHeight: 500



    // must be set :
    property var skrSettingsGroup

    //option:
    property bool textWidthSliderVisible: true

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
        id: themesColorAction
        text: qsTr("Themes")
        icon {
            name: "color-picker-white"
            height: 50
            width: 50
        }

        //shortcut: StandardKey.
        onTriggered: {
            Globals.openThemePageCalled()
        }

    }
    themesToolButton.action: themesColorAction



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

    // textWidthSlider :

    textWidthLabel.visible: !Globals.compactSize && textWidthSliderVisible
    textWidthSlider.visible: !Globals.compactSize && textWidthSliderVisible

    textWidthSlider.value: skrSettingsGroup.textWidth

    Binding {
        target: skrSettingsGroup
        property: "textWidth"
        value: textWidthSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // textPointSizeSlider :

    textPointSizeSlider.value: skrSettingsGroup.textPointSize


    Binding {
        target: skrSettingsGroup
        property: "textPointSize"
        value: textPointSizeSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Font family combo :
    fontFamilyComboBox.model: skrFonts.fontFamilies()

    Binding {
        target: skrSettingsGroup
        property: "textFontFamily"
        value: fontFamilyComboBox.currentText
        when:  fontFamilyLoaded
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    property bool fontFamilyLoaded: false

    function loadFontFamily(){
        var fontFamily = skrSettingsGroup.textFontFamily
        //console.log("fontFamily", fontFamily)
        //console.log("application fontFamily", Qt.application.font.family)

        var index = fontFamilyComboBox.find(fontFamily, Qt.MatchFixedString)
        //console.log("index", index)
        if(index === -1){
            index = fontFamilyComboBox.find(Qt.application.font.family, Qt.MatchFixedString)
        }
        //console.log("index", index)

        fontFamilyComboBox.currentIndex = index
        fontFamilyLoaded = true
    }

    // Indent :
     textIndentSlider.value: skrSettingsGroup.textIndent

    Binding {
        target: skrSettingsGroup
        property: "textIndent"
        value: textIndentSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Margins :
     textTopMarginSlider.value: skrSettingsGroup.textTopMargin

    Binding {
        target: skrSettingsGroup
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
