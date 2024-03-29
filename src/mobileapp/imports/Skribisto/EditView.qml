import QtQuick
import QtQml
import QtQuick.Controls
import eu.skribisto.skr 1.0
import eu.skribisto.exporter 1.0

EditViewForm {
    id: root

    property int projectId: -2
    property int treeItemId: -2
    property var writingZone

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
    centerTextCursorToolButton.action: centerTextCursorAction
    themesToolButton.action: themesColorAction
    minimapToolButton.action: minimapAction

    swipeView.onCurrentIndexChanged: {
        var i = 0
        for (i = 0; i < swipeView.count; i++) {

            swipeView.itemAt(i).enabled = i === swipeView.currentIndex
        }
    }

    Action {
        id: findAction
        text: qsTr("Find")
        icon {
            source: "icons/3rdparty/backup/edit-find.svg"
        }

        //shortcut: StandardKey.Backspace
        onTriggered: {
            writingZone.showFindPanel()
        }
    }
    findToolButton.action: findAction

    Action {
        id: sizeAction
        text: qsTr("Sizes")
        icon {
            source: "icons/3rdparty/backup/format-font-size-more.svg"
        }

        //shortcut: StandardKey.
        onTriggered: {
            swipeView.setCurrentIndex(1)
            textWidthSlider.forceActiveFocus()
        }
    }
    sizeToolButton.action: sizeAction

    Action {
        id: goBackAction
        text: qsTr("Go back")
        icon {
            source: "icons/3rdparty/backup/go-previous.svg"
        }

        //shortcut: StandardKey.Backspace
        onTriggered: {
            swipeView.setCurrentIndex(0)
        }
    }

    goBackToolButton.action: goBackAction

    // textWidthSlider :
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
        when: fontFamilyLoaded
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    property bool fontFamilyLoaded: false

    function loadFontFamily() {
        var fontFamily = skrSettingsGroup.textFontFamily

        //console.log("fontFamily", fontFamily)
        //console.log("application fontFamily", Qt.application.font.family)
        var index = fontFamilyComboBox.find(fontFamily, Qt.MatchFixedString)
        //console.log("index", index)
        if (index === -1) {
            index = fontFamilyComboBox.find("Liberation Serif",
                                            Qt.MatchFixedString)
        }
        if (index === -1) {
            index = fontFamilyComboBox.find(Qt.application.font.family,
                                            Qt.MatchContains)
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

    //--------------------------------------------------------------
    //----Print-----------------------------------------------------
    //--------------------------------------------------------------
    quickPrintToolButton.visible: skrRootItem.hasPrintSupport()

    Action {
        id: quickPrintAction
        text: qsTr("Quick print")
        icon {
            source: "icons/3rdparty/backup/document-print-direct.svg"
        }

        //shortcut: StandardKey.
        onTriggered: {

            exporter.treeItemIdList = [root.treeItemId]
            exporter.run()
        }
    }
    quickPrintToolButton.action: quickPrintAction

    SKRExporter {
        id: exporter
        projectId: root.projectId
        quick: true
        outputType: SKRExporter.Printer
        includeSynopsis: SkrSettings.quickPrintSettings.includeSynopsis
        numbered: false
        tagsEnabled: SkrSettings.quickPrintSettings.tagsEnabled
        indentWithTitle: 5
        fontFamily: SkrSettings.quickPrintSettings.textFontFamily
        fontPointSize: SkrSettings.quickPrintSettings.textPointSize
        textIndent: SkrSettings.quickPrintSettings.textIndent
        textTopMargin: SkrSettings.quickPrintSettings.textTopMargin
    }

    //--------------------------------------------------------------
    //---------------------------------------------------------
    //--------------------------------------------------------------

    // go back when losing focus
    swipeView.onActiveFocusChanged: {
        //console.log('activeFocus', swipeView.activeFocus)
        if (!swipeView.activeFocus) {
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
