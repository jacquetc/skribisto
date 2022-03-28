import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform 1.1 as LabPlatform
import Qt.labs.settings 1.1
import eu.skribisto.projecthub 1.0
import eu.skribisto.spellchecker 1.0
import QtQuick.Controls.Material
import ".."
import "../Items"
import "../Commons"

SettingsPageForm {
    signal closeCalled()


    Component.onCompleted: {
        populateSettingsPanelButtonModel()
        //        determineAndAddSettingsPanelPlugins()
        //buildButtons()

        //-----------------




        populateCheckSpellingComboBox()
        checkSpellingComboBox.currentIndex = checkSpellingComboBox.indexOfValue(SkrSettings.spellCheckingSettings.spellCheckingLangCode)

        loadFontFamily()

    }

    //------------------------------------------------------------------------
    //-----setting panels------------------------------------------------------------
    //-----------------------------------------------------------------------
    //    property list<Component> panels
    function populateSettingsPanelButtonModel(){
        var list = skrPluginGetter.findSettingsPanelNames()

        for(var i in list){
            var name = list[i]
            var iconSource = skrPluginGetter.findSettingsPanelIconSource(name)
            var buttonText = skrPluginGetter.findSettingsPanelButtonText(name)
            var qmlUrl = skrPluginGetter.findSettingsPanelUrl(name)

            toolButtonModel.append({ "name": name, "qmlUrl": qmlUrl, "iconSource": iconSource, "buttonText": buttonText})
        }
    }


    //    function determineAndAddSettingsPanelPlugins() {

    //        var settingsPanelList = skrRootItem.findSettingsPanels()

    //        for (var i in settingsPanelList) {
    //            var url = settingsPanelList[i]
    //            var pluginComponent = Qt.createComponent(
    //                        url, Component.PreferSynchronous, root)
    //            panels.push(pluginComponent)
    //        }
    //    }

    //-----------------------------------------------------------------------

    //    function buildButtons(){
    //        for (var i in panels) {
    //            var iconSource = toolboxLoader.item.iconSource
    //            var showButtonText = toolboxLoader.item.showButtonText
    //        toolButtonModel
    //        }
    //    }

    ListModel {
        id: toolButtonModel

        ListElement{
            name: "accessibility"
            buttonText :qsTr("Accessibility")
            iconSource: "qrc:///icons/backup/accessibility.svg"
            qmlUrl: "qrc:///eu.skribisto.skribisto/qml/WelcomePage/SettingsPanels/AccessibilityPanel.qml"

        }
        ListElement{
            name: "appearance"
            buttonText :qsTr("Appearance")
            iconSource: "qrc:///icons/backup/folder-pictures.svg"
            qmlUrl: "qrc:///eu.skribisto.skribisto/qml/WelcomePage/SettingsPanels/AppearancePanel.qml"

        }

        ListElement{
            name: "save"
            buttonText :qsTr("Save")
            iconSource: "qrc:///icons/backup/document-save.svg"
            qmlUrl: "qrc:///eu.skribisto.skribisto/qml/WelcomePage/SettingsPanels/SavePanel.qml"

        }


        ListElement{
            name: "backup"
            buttonText :qsTr("Backup")
            iconSource: "qrc:///icons/backup/tools-media-optical-burn-image.svg"
            qmlUrl: "qrc:///eu.skribisto.skribisto/qml/WelcomePage/SettingsPanels/BackupPanel.qml"

        }
        ListElement{
            name: "ePaper"
            buttonText :qsTr("E-Paper")
            iconSource: "qrc:///icons/backup/smartphone.svg"
            qmlUrl: "qrc:///eu.skribisto.skribisto/qml/WelcomePage/SettingsPanels/EPaperPanel.qml"

        }

    }
    toolButtonRepeater.delegate: toolButtonComponent
    toolButtonRepeater.model: toolButtonModel


    Component {
        id: toolButtonComponent
        SkrToolButton {
            id: toolButton
            width: 200 * SkrSettings.interfaceSettings.zoom
            height: 200 * SkrSettings.interfaceSettings.zoom
            icon{
                source: model.iconSource
                width: 100 * SkrSettings.interfaceSettings.zoom
                height: 100 * SkrSettings.interfaceSettings.zoom
            }
            text: model.buttonText
            display: AbstractButton.TextUnderIcon

            onClicked: {
                stackView.push(model.qmlUrl)
            }
        }
    }



    //-----------------------------------------------------------------------



    //-----------------------------------------------------------------------



    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            toolButtonRepeater.itemAt(0).forceActiveFocus()
        }
    }



    //    // scrollview :
    //    Component.onCompleted: scroll.contentItem = contener
    //	function scrollChange(){
    //                if ((width > 600) && (height > 600)){
    //                        if(scroll.contentItem != itemNull) scroll.contentItem = itemNull
    //                        contener.width = width
    //                        contener.height = height
    //                } else if ((width < 600) && (height < 600)){
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.width = 600
    //                        contener.height = 600
    //                } else if ((width < 600)){
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.width = 600
    //                        contener.height = height
    //                } else {
    //                        if(scroll.contentItem != contener) scroll.contentItem = contener
    //                        contener.height = 600
    //                        contener.width = width
    //                }
    //        }

    //	onWidthChanged: scrollChange()
    //	onHeightChanged: scrollChange()





    // --------------------------------------------
    // ---- Behavior --------------------------------
    // --------------------------------------------


    createEmpyProjectAtStartSwitch.checked: SkrSettings.behaviorSettings.createEmptyProjectAtStart
    Binding {
        target: SkrSettings.behaviorSettings
        property: "createEmptyProjectAtStart"
        value: createEmpyProjectAtStartSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }




    // plugins:


    pluginPageButton.onClicked: {
        loader_pluginPagePopup.active = true
    }

    Component {
        id: component_pluginPagePopup
        SkrPopup {
            property alias closeButton: inner_closeButton


            id: pluginPagePopup
            parent: Overlay.overlay
            width: Overlay.overlay.width >= 1000 * SkrSettings.interfaceSettings.zoom ? 1000 * SkrSettings.interfaceSettings.zoom : Overlay.overlay.width
            height: Overlay.overlay.height >= 1000 * SkrSettings.interfaceSettings.zoom ? 1000 * SkrSettings.interfaceSettings.zoom : Overlay.overlay.height
            anchors.centerIn: Overlay.overlay

            modal: true
            visible: true

            closePolicy: Popup.CloseOnEscape
            onClosed: loader_pluginPagePopup.active = false

            onOpened: {
                pluginPage.forceActiveFocus()
            }

            background: Rectangle {

                radius: 10
                color: SkrTheme.pageBackground

            }
            contentItem: SkrPane {
                anchors.fill: parent
                clip: true
                ColumnLayout {
                    anchors.fill: parent

                    SkrToolButton {
                        id: inner_closeButton
                        text: qsTr("Close")
                        focusPolicy: Qt.NoFocus
                        display: AbstractButton.IconOnly
                        Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                        icon {
                            source: "qrc:///icons/backup/arrow-down.svg"
                        }

                        onClicked: pluginPagePopup.close()
                    }

                    PluginPage {
                        id: pluginPage
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                    }

                    SkrLabel{
                        Layout.alignment: Qt.AlignHCenter
                        text: qsTr("Please restart to apply changes.")
                    }
                }
            }
        }
    }
    Loader {
        id: loader_pluginPagePopup
        sourceComponent: component_pluginPagePopup
        active: false
    }


    firstStepsButton.action: firstStepsAction




    // --------------------------------------------
    // ---- Quick print --------------------------------
    // --------------------------------------------



    // textPointSizeSlider :

    textPointSizeSlider.value: SkrSettings.quickPrintSettings.textPointSize


    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textPointSize"
        value: textPointSizeSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Font family combo :
    fontFamilyComboBox.model: skrFonts.fontFamilies()

    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textFontFamily"
        value: fontFamilyComboBox.currentText
        when:  fontFamilyLoaded
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    property bool fontFamilyLoaded: false

    function loadFontFamily(){
        var fontFamily = SkrSettings.quickPrintSettings.textFontFamily
        //console.log("fontFamily", fontFamily)
        //console.log("application fontFamily", Qt.application.font.family)

        var index = fontFamilyComboBox.find(fontFamily, Qt.MatchFixedString)
        //console.log("index", index)
        if(index === -1){
            index = fontFamilyComboBox.find("Liberation Serif", Qt.MatchFixedString)
        }
        if(index === -1){
            index = fontFamilyComboBox.find(skrRootItem.defaultFontFamily(), Qt.MatchContains)
        }
        //console.log("index", index)

        fontFamilyComboBox.currentIndex = index
        fontFamilyLoaded = true
    }

    // Indent :
    textIndentSlider.value: SkrSettings.quickPrintSettings.textIndent

    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textIndent"
        value: textIndentSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Margins :
    textTopMarginSlider.value: SkrSettings.quickPrintSettings.textTopMargin

    Binding {
        target: SkrSettings.quickPrintSettings
        property: "textTopMargin"
        value: textTopMarginSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }



    // --------------------------------------------
    // ---- spell checking --------------------------------
    // --------------------------------------------

    checkSpellingCheckBox.action: checkSpellingAction


    // combo box :

    SKRSpellChecker {
        id : spellChecker
    }

    ListModel {
        id: checkSpellingComboBoxModel
    }

    checkSpellingComboBox.model: checkSpellingComboBoxModel
    checkSpellingComboBox.textRole: "text"
    checkSpellingComboBox.valueRole: "dictCode"

    function populateCheckSpellingComboBox(){

        var dictList = spellChecker.dictList()

        var i;
        for(i = 0 ; i < dictList.length ; i++){
            checkSpellingComboBoxModel.append({"text": dictList[i], "dictCode": dictList[i]})
        }

    }


    Connections {
        target: SkrSettings.spellCheckingSettings
        function onSpellCheckingLangCodeChanged(){
            var value = SkrSettings.spellCheckingSettings.spellCheckingLangCode
            checkSpellingComboBox.currentIndex = checkSpellingComboBox.indexOfValue(value)
        }
    }
    checkSpellingComboBox.onCurrentValueChanged: {
        if(checkSpellingComboBox.activeFocus){
            SkrSettings.spellCheckingSettings.spellCheckingLangCode = checkSpellingComboBox.currentValue
        }
    }

    //------------------------------------------------
    // install dict

    Component {
        id: component_newDictWizard
        NewDictWizard {
            id: newDictWizard

            onClosed: loader_newDictWizard.active = false
        }
    }
    Loader {
        id: loader_newDictWizard
        active: false
        sourceComponent: component_newDictWizard
    }

    installDictButton.onClicked: {
        loader_newDictWizard.active = true
    }

    Connections {
        target: Globals
        function onNewDictInstalled(dictName){
            populateCheckSpellingComboBox()

        }
    }

}
