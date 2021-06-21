import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import '../..'

CardViewToolboxForm {

    iconSource: "qrc:///icons/backup/configure.svg"
    showButtonText: qsTr( "Show the card view toolbox")

    property int projectId: -1

    property int minimumHeight: 500


    //-------------------------------------------------------------------------------
    //-----Display------------------------------------------------------------------
    //-------------------------------------------------------------------------------

//    // textWidthSlider :

//    treeItemDisplayModeSlider.value: SkrSettings.cardViewTreeSettings.treeItemDisplayMode

//    Binding {
//        target: SkrSettings.cardViewTreeSettings
//        property: "treeItemDisplayMode"
//        value: treeItemDisplayModeSlider.value
//        delayed: true
//        restoreMode: Binding.RestoreBindingOrValue
//    }


//    // textWidthSlider :


//    treeIndentationSlider.value: SkrSettings.cardViewTreeSettings.treeIndentation

//    Binding {
//        target: SkrSettings.cardViewTreeSettings
//        property: "treeIndentation"
//        value: treeIndentationSlider.value
//        delayed: true
//        restoreMode: Binding.RestoreBindingOrValue
//    }

//    // showOutlineSwitch

//    showOutlineSwitch.checked: SkrSettings.cardViewTreeSettings.outlineBoxVisible

//    Binding {
//        target: SkrSettings.cardViewTreeSettings
//        property: "outlineBoxVisible"
//        value: showOutlineSwitch.checked
//        delayed: true
//        restoreMode: Binding.RestoreBindingOrValue
//    }


//    // showNotePadSwitch

//    showNotePadSwitch.checked: SkrSettings.cardViewTreeSettings.noteBoxVisible

//    Binding {
//        target: SkrSettings.cardViewTreeSettings
//        property: "noteBoxVisible"
//        value: showNotePadSwitch.checked
//        delayed: true
//        restoreMode: Binding.RestoreBindingOrValue
//    }

//    // showTagPadSwitch

//    showTagPadSwitch.checked: SkrSettings.cardViewTreeSettings.tagBoxVisible

//    Binding {
//        target: SkrSettings.cardViewTreeSettings
//        property: "tagBoxVisible"
//        value: showTagPadSwitch.checked
//        delayed: true
//        restoreMode: Binding.RestoreBindingOrValue
//    }

//    // showCharacterCountSwitch

//    showCharacterCountSwitch.checked: SkrSettings.cardViewTreeSettings.characterCountBoxVisible

//    Binding {
//        target: SkrSettings.cardViewTreeSettings
//        property: "characterCountBoxVisible"
//        value: showCharacterCountSwitch.checked
//        delayed: true
//        restoreMode: Binding.RestoreBindingOrValue
//    }

//    // showWordCountSwitch

//    showWordCountSwitch.checked: SkrSettings.cardViewTreeSettings.wordCountBoxVisible

//    Binding {
//        target: SkrSettings.cardViewTreeSettings
//        property: "wordCountBoxVisible"
//        value: showWordCountSwitch.checked
//        delayed: true
//        restoreMode: Binding.RestoreBindingOrValue
//    }


    //focus
    onActiveFocusChanged: {
        if (activeFocus) {

            displayGroupBox.forceActiveFocus()
        }
    }
}
