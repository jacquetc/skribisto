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


    //focus
    onActiveFocusChanged: {
        if (activeFocus) {
            //swipeView.currentIndex = 0
            treeItemDisplayModeSlider.forceActiveFocus()
        }
    }
}
