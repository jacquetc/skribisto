import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import '../..'

OverviewToolboxForm {

    iconSource: "qrc:///icons/backup/configure.svg"
    showButtonText: qsTr( "Show the overview toolbox")

    property int projectId: -1

    property int minimumHeight: 500


    //-------------------------------------------------------------------------------
    //-----Display------------------------------------------------------------------
    //-------------------------------------------------------------------------------

    // textWidthSlider :

    treeItemDisplayModeSlider.value: SkrSettings.overviewTreeSettings.treeItemDisplayMode

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "treeItemDisplayMode"
        value: treeItemDisplayModeSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    // textWidthSlider :


    treeIndentationSlider.value: SkrSettings.overviewTreeSettings.treeIndentation

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "treeIndentation"
        value: treeIndentationSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // showOutlineSwitch

    showOutlineSwitch.checked: SkrSettings.overviewTreeSettings.outlineBoxVisible

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "outlineBoxVisible"
        value: showOutlineSwitch.checked
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    // showNotePadSwitch

    showNotePadSwitch.checked: SkrSettings.overviewTreeSettings.noteBoxVisible

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "noteBoxVisible"
        value: showNotePadSwitch.checked
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // showTagPadSwitch

    showTagPadSwitch.checked: SkrSettings.overviewTreeSettings.tagBoxVisible

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "tagBoxVisible"
        value: showTagPadSwitch.checked
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // showCharacterCountSwitch

    showCharacterCountSwitch.checked: SkrSettings.overviewTreeSettings.characterCountBoxVisible

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "characterCountBoxVisible"
        value: showCharacterCountSwitch.checked
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // showWordCountSwitch

    showWordCountSwitch.checked: SkrSettings.overviewTreeSettings.wordCountBoxVisible

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "wordCountBoxVisible"
        value: showWordCountSwitch.checked
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    //focus
    onActiveFocusChanged: {
        if (activeFocus) {

            displayGroupBox.forceActiveFocus()
        }
    }
}