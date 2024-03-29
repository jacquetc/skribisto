import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material

import SkrControls
import Skribisto

AppearancePanelForm {

    Component.onCompleted: {
        determineAvailableTranslations()
        determineCurrentTranslation()
    }

    // --------------------------------------------
    // ---- appearance --------------------------------
    // --------------------------------------------



    // interface languages :

    function determineAvailableTranslations(){
        langModel.clear()

        var translationsMap = skrRootItem.findAvailableTranslationsMap()

        for(var translation in translationsMap){
            langModel.append({"text": translationsMap[translation] + " (" + translation + ")", "langCode": translation})
        }
    }
    function determineCurrentTranslation(){
        langComboBox.currentIndex = langComboBox.indexOfValue(skrRootItem.getLanguageFromSettings())
    }

    ListModel {
        id: langModel
    }


    langComboBox.model: langModel
    langComboBox.textRole: "text"
    langComboBox.valueRole: "langCode"
    langComboBox.onCurrentValueChanged: {
        if(langComboBox.activeFocus){
            skrRootItem.currentTranslationLanguageCode = langComboBox.currentValue
            skrWindowManager.retranslate()

        }
    }


    Connections {
        target: skrRootItem
        function onCurrentTranslationLanguageCodeChanged(langCode){
            langComboBox.currentIndex = langComboBox.indexOfValue(langCode)
        }
    }

    openThemePageButton.onClicked: {
        rootWindow.protectedSignals.openThemePageCalled()
        closeCalled()
    }


    //--------------------------------------------------
   // Zoom

    zoomSpinBox.value: SkrSettings.interfaceSettings.zoom * 100
    Binding {
        delayed: true
        target: SkrSettings.interfaceSettings
        property: "zoom"
        value: zoomSpinBox.value / 100
        restoreMode: Binding.RestoreBindingOrValue

    }

    //--------------------------------------------------

    // --------------------------------------------
    // ---- word count --------------------------------
    // --------------------------------------------

    wordCountSwitch.checked: SkrSettings.interfaceSettings.wordCountVisible
    Binding {
        target: SkrSettings.interfaceSettings
        property: "wordCountVisible"
        value: wordCountSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    wordCountSwitch.onCheckedChanged: {
        if(wordCountSwitch.checked){
            SkrSettings.interfaceSettings.charCountVisible = false
        }
    }

    charCountSwitch.checked: SkrSettings.interfaceSettings.charCountVisible
    Binding {
        target: SkrSettings.interfaceSettings
        property: "charCountVisible"
        value: charCountSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    charCountSwitch.onCheckedChanged: {
        if(charCountSwitch.checked){
            SkrSettings.interfaceSettings.wordCountVisible = false
        }
    }

    //--------------------------------------------------


    onActiveFocusChanged: {
        if (activeFocus) {
            appearanceGroupBox.forceActiveFocus()
        }
    }


}
