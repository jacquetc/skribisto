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

    // textWidthSlider :

    textWidthSlider.value: SkrSettings.writeSettings.textWidth

    Binding {
        target: SkrSettings.writeSettings
        property: "textWidth"
        value: textWidthSlider.value
        delayed: true
    }

    // textPointSizeSlider :
    textPointSizeSlider.value: SkrSettings.writeSettings.textPointSize


    Binding {
        target: SkrSettings.writeSettings
        property: "textPointSize"
        value: textPointSizeSlider.value
        delayed: true
    }

    // Font family combo :
    fontFamilyComboBox.model: Qt.fontFamilies()

    Binding {
        target: SkrSettings.writeSettings
        property: "textFontFamily"
        value: fontFamilyComboBox.currentText
        delayed: true
    }
    Settings{
        id: settings
            category: "write"
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
     textIndentSlider.value: SkrSettings.writeSettings.textIndent

    Binding {
        target: SkrSettings.writeSettings
        property: "textIndent"
        value: textIndentSlider.value
        delayed: true
    }

    // Indent :
     textTopMarginSlider.value: SkrSettings.writeSettings.textTopMargin

    Binding {
        target: SkrSettings.writeSettings
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
