import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import '..'

SheetOverviewToolForm {
    property int minimumHeight: 200


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

    showOutlineSwitch.checked: SkrSettings.overviewTreeSettings.synopsisBoxVisible

    Binding {
        target: SkrSettings.overviewTreeSettings
        property: "synopsisBoxVisible"
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

    //focus
    onActiveFocusChanged: {
        if (activeFocus) {
            //swipeView.currentIndex = 0
            treeItemDisplayModeSlider.forceActiveFocus()
        }
    }
}
