import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material 2.15
import "../.."
import "../../Items"
import "../../Commons"

AppearancePanelForm {



    property var stackView: StackView.view
    goBackButton.onClicked: {
        stackView.pop()
    }

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

    // -------------------------------------------------------
    // ---- minimap scrollbar --------------------------------
    // --------------------------------------------------------



    showMinimapCheckBox.checked: SkrSettings.minimapSettings.visible
    Binding {
        target: SkrSettings.minimapSettings
        property: "visible"
        value: showMinimapCheckBox.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    //dials :

    minimapDividerDial.onMoved: minimapDividerSpinBox.value = minimapDividerDial.value
    minimapDividerSpinBox.onValueModified: minimapDividerDial.value = minimapDividerSpinBox.value


    minimapDividerDial.value: SkrSettings.minimapSettings.divider
    minimapDividerSpinBox.value: SkrSettings.minimapSettings.divider
    Binding {
        delayed: true
        target: SkrSettings.minimapSettings
        property: "divider"
        value: minimapDividerDial.value
        restoreMode: Binding.RestoreBindingOrValue

    }

    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            appearanceGroupBox.forceActiveFocus()
        }
    }


}
