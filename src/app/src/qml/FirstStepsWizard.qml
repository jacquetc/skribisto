import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15
import eu.skribisto.spellchecker 1.0

import "Items"
import "Commons"
import "WelcomePage"

SkrPopup {

    id: root
    parent: Overlay.overlay
    width: Overlay.overlay.width >= 1000 ? 1000 : Overlay.overlay.width
    height: Overlay.overlay.height >= 1000 ? 1000 : Overlay.overlay.height
    anchors.centerIn: Overlay.overlay

    modal: true
    visible: true

    closePolicy: Popup.CloseOnEscape

    background: Rectangle {

        radius: 10
        color: SkrTheme.pageBackground

    }
    Component.onCompleted: {
        determineAvailableTranslations()
        determineCurrentTranslation()
        populateCheckSpellingComboBox()
        checkSpellingComboBox.currentIndex = checkSpellingComboBox.indexOfValue(SkrSettings.spellCheckingSettings.spellCheckingLangCode)

        for(var i = 0; i < swipeView.count; i++){

            swipeView.itemAt(i).enabled = i === swipeView.currentIndex
        }

        setPage("")
    }

    function setPage(pageName){
        if(pageName === "pluginPage"){
            swipeView.currentIndex = 2
        }
        swipeView.currentItem.forceActiveFocus()
    }

    contentItem: SkrPane {
        anchors.fill: parent
        clip: true
        ColumnLayout {
            anchors.fill: parent

            SkrToolButton {
                id: closeButton
                text: qsTr("Close")
                focusPolicy: Qt.NoFocus
                display: AbstractButton.IconOnly
                Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                icon {
                    source: "qrc:///icons/backup/arrow-down.svg"
                }

                onClicked: root.close()
            }

            SwipeView {
                id: swipeView
                clip: true
                Layout.fillHeight: true
                Layout.fillWidth: true



                onCurrentIndexChanged: {
                    for(var i = 0; i < swipeView.count; i++){

                        swipeView.itemAt(i).enabled = i === swipeView.currentIndex
                    }
                }

                ColumnLayout {
                    id: welcome

                    SkrLabel{
                        Layout.alignment: Qt.AlignHCenter
                        text: "<h1>" + qsTr("First steps with Skribisto") + "</h1>"
                    }

                    Image {
                        id: skribistoImage
                        Layout.alignment: Qt.AlignHCenter
                        source: "qrc:///pics/skribisto.png"
                        Layout.preferredHeight: 200
                        Layout.preferredWidth: 200
                    }

                    SkrLabel{
                        text: qsTr("Welcome in Skribisto")
                        font.bold: true
                    }

                    SkrLabel{
                        text: qsTr("This assistant will help you set up Skribisto to your liking. To begin with, please select the best options for your use.")
                        font.bold: true
                    }

                    RowLayout{
                        SkrLabel{
                            text: qsTr("Select a language:")
                        }
                        SkrComboBox {
                            id: langComboBox
                            wheelEnabled: true
                            model: langModel
                            textRole: "text"
                            valueRole: "langCode"
                            onCurrentValueChanged: {
                                if(langComboBox.activeFocus){
                                    skrRootItem.currentTranslationLanguageCode = langComboBox.currentValue
                                    skrWindowManager.retranslate()
                                    //skrData.errorHub().addOk(qsTr("Please restart Skribisto to apply the change"))

                                    checkSpellingComboBox.currentIndex = checkSpellingComboBox.indexOfValue(langComboBox.currentValue)
                                }
                            }
                        }
                    }

                    RowLayout {
                        id: rowLayout5

                        SkrLabel {
                            id: label
                            text: qsTr("Default dictionary :")
                        }

                        SkrComboBox {
                            id: checkSpellingComboBox
                            wheelEnabled: true
                            model: checkSpellingComboBoxModel
                            textRole: "text"
                            valueRole: "dictCode"
                            onCurrentValueChanged: {
                                if(checkSpellingComboBox.activeFocus){
                                    SkrSettings.spellCheckingSettings.spellCheckingLangCode = langComboBox.currentValue
                                }
                            }
                        }
                    }


                    SkrSwitch {
                        id: accessibilityCheckBox
                        text: qsTr("Help with accessibility")
                        checked: SkrSettings.accessibilitySettings.accessibilityEnabled
                        Binding {
                            target: SkrSettings.accessibilitySettings
                            property: "accessibilityEnabled"
                            value: accessibilityCheckBox.checked
                            restoreMode: Binding.RestoreBindingOrValue
                        }

                        onCheckedChanged: {
                            SkrSettings.accessibilitySettings.showMenuButton = accessibilityCheckBox.checked
                        }

                    }


                    Item {
                        Layout.fillHeight: true
                    }
                }


                ColumnLayout {
                    id: shortcutsPage

                }

                Item{

                    ColumnLayout{
                        anchors.fill: parent
                        PluginPage {
                            id: pluginsPage
                            Layout.fillHeight: true
                            Layout.fillWidth: true
                        }


                        SkrLabel{
                            Layout.alignment: Qt.AlignHCenter
                            text: qsTr("Plugin selection can be changed later in settings.")
                        }
                        SkrButton {
                            id: restartButton
                            Layout.alignment: Qt.AlignHCenter
                            text: qsTr("Please restart to apply changes")
                            onClicked: {
                                for(var i = 0; i < pluginsPage.pluginModel.count; i++){
                                    var item = pluginsPage.pluginModel.get(i)

                                    skrData.pluginHub().setPluginEnabled(item.pluginName, item.enabled)
                                }


                                Qt.exit(-2)
                            }
                        }
                    }
                }

                NewProjectPage {
                    id: newProjectPage

                }



            }

            RowLayout{

                Layout.fillWidth: true

                SkrToolButton {
                    id: previousButton
                    text: qsTr("Previous")
                    display: AbstractButton.IconOnly
                    Layout.alignment: Qt.AlignLeft
                    visible: swipeView.currentIndex !== 0
                    icon {
                        source: "qrc:///icons/backup/go-previous.svg"
                    }

                    onClicked: {
                        swipeView.decrementCurrentIndex()
                    }

                }
                Item{
                    Layout.fillWidth: true

                    PageIndicator {
                        count: swipeView.count
                        currentIndex: swipeView.currentIndex
                        interactive: true
                        anchors.centerIn: parent
                        onCurrentIndexChanged: {
                            swipeView.currentIndex = currentIndex
                        }
                    }

                }


                SkrToolButton {
                    id: nextButton
                    text: qsTr("Next")
                    display: AbstractButton.TextBesideIcon
                    Layout.alignment: Qt.AlignRight
                    visible: swipeView.currentIndex !== swipeView.count - 1
                    icon {
                        source: "qrc:///icons/backup/go-next.svg"
                    }

                    onClicked: {
                        swipeView.incrementCurrentIndex()
                    }

                    KeyNavigation.tab: swipeView.currentItem

                }

                SkrToolButton {
                    id: closeButton2
                    text: qsTr("Close")
                    display: AbstractButton.TextBesideIcon
                    Layout.alignment: Qt.AlignRight
                    visible: swipeView.currentIndex === swipeView.count - 1
                    icon {
                        source: "qrc:///icons/backup/arrow-down.svg"
                    }

                    onClicked: {
                        root.close()
                    }

                }
            }


        }

    }


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




    Connections {
        target: skrRootItem
        function onCurrentTranslationLanguageCodeChanged(langCode){
            langComboBox.currentIndex = langComboBox.indexOfValue(langCode)
        }
    }


    // spell checking combo box :

    SKRSpellChecker {
        id : spellChecker
    }

    ListModel {
        id: checkSpellingComboBoxModel
    }

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
}
