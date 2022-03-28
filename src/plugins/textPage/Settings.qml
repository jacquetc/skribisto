import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material
import "../../../.."
import "../../../../Items"
import "../../../../Commons"

SettingsForm {
    property var skrSettingsGroup: SkrSettings.textSettings



    centerTextCursorSwitch.checked: SkrSettings.behaviorSettings.centerTextCursor
    Binding {
        target: SkrSettings.behaviorSettings
        property: "centerTextCursor"
        value: centerTextCursorSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
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


    // -------------------------------------------------------
    // ---- text layout --------------------------------
    // --------------------------------------------------------

    textWidthLabel.visible: !rootWindow.compactMode && textWidthSliderVisible
    textWidthSlider.visible: !rootWindow.compactMode && textWidthSliderVisible

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
            index = fontFamilyComboBox.find("Liberation Serif", Qt.MatchFixedString)
        }
        if(index === -1){
            index = fontFamilyComboBox.find(Qt.application.font.family, Qt.MatchContains)
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


    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            textGroupBox.forceActiveFocus()
        }
    }

}
