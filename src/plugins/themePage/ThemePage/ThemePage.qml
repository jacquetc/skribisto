import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.platform 1.1 as LabPlatform
import eu.skribisto.themes 1.0
import QtQuick.Window 2.15

import "../../Items"
import "../../Commons"
import "../.."



ThemePageForm {
    id: root
    property string pageType: "THEME"
    property string title: qsTr("Themes")
    clip: true

    Component.onCompleted: {
        populateThemeListModel()
        populateColorPropertiesListModel()
    }
    Component.onDestruction: {
           if(rootWindow.visibility !== Window.FullScreen){
               SkrTheme.setDistractionFree(false)
           }
    }

    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------

    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
        skrWindowManager.addWindowForProjectIndependantPageType(root.pageType)
    }

    viewButtons.onSplitCalled: function(position){
        viewManager.loadProjectIndependantPageAt(root.pageType, position)
    }
    titleLabel.text: title


    //-------------------------------------------------------------
    //-----theme list-----------------------------------------------
    //-------------------------------------------------------------

    addThemeButton.icon.source: "qrc:///icons/backup/edit-copy.svg"
    addThemeButton.onClicked: {
        SkrTheme.duplicate(selectedTheme, qsTr("%1 (copy)".arg(selectedTheme)))
        populateThemeListModel()
    }

    //--------------------------------------------------------


    ListModel {
        id: themeListModel
    }

    themeListView.model: themeListModel

    function populateThemeListModel(){
        themeListModel.clear()
        var list = SkrTheme.getThemeList()
        list.sort()

        var i
        for(i = 0 ; i < list.length ; i++) {
            var isEditable = SkrTheme.isThemeEditable(list[i])
            var name = list[i]

            themeListModel.append({"text": name, "isEditable": isEditable})


        }
    }

    //--------------------------------------------------------

    readonly property string currentLightThemeName: SkrTheme.currentLightThemeName
    readonly property string currentDarkThemeName: SkrTheme.currentDarkThemeName
    readonly property string currentDistractionFreeThemeName: SkrTheme.currentDistractionFreeThemeName
    property string selectedTheme: ""


    themeListView.delegate: SkrListItemPane {
        id: themeDelegateRoot

        height: 60
        anchors{
            left: Qt.isQtObject(parent) ? parent.left : undefined
            right: Qt.isQtObject(parent) ? parent.right : undefined
        }


        TapHandler {
            id: themeTapHandler

            onSingleTapped: {
                themeListView.currentIndex = model.index
                selectedTheme = model.text
                themeDelegateRoot.forceActiveFocus()
                eventPoint.accepted = true
            }

            onDoubleTapped: {
                //console.log("double tapped")
                themeListView.currentIndex = model.index
                selectedTheme = model.text
                SkrTheme.currentThemeName = model.text
                eventPoint.accepted = true
            }
        }



        contentItem: RowLayout {
            anchors.fill: parent


            Rectangle {
                id: themeCurrentItemIndicator
                color: "lightsteelblue"
                Layout.fillHeight: true
                Layout.preferredWidth: 5
                visible: themeListView.currentIndex === model.index && themeListView.activeFocus
            }
            Rectangle {
                id: themeOpenedItemIndicator
                color:  SkrTheme.accent
                Layout.fillHeight: true
                Layout.preferredWidth: 5
                visible: model.text === selectedTheme
            }


            SkrLabel {
                id: themeNameLabel
                text: model.text
                Layout.preferredHeight: 30
            }

            Item {
                id: stretcher2
                Layout.fillWidth: true
            }


            SkrToolButton {


                Layout.preferredHeight: 40
                Layout.preferredWidth: 40

                id: editThemeToolButton
                text: qsTr("Rename")
                flat: true
                display: AbstractButton.IconOnly
                enabled: model.isEditable

                icon.source: "qrc:///icons/backup/edit-rename.svg"

                onClicked: {
                    renameDialog.name = model.text
                    renameDialog.open()
                }
            }


            Item {
                id: stretcher3
                Layout.minimumWidth: 50
            }

            SkrRoundButton{
                id: lightButton
                display: AbstractButton.IconOnly
                text: SkrTheme.currentLightThemeName === model.text ? qsTr("Light") :  qsTr("Set as light theme")
                icon{
                    source:  SkrTheme.currentLightThemeName === model.text ? "qrc:///icons/backup/color-picker-white.svg" : ""
                    color: "transparent"
                }
                onClicked:{
                    SkrTheme.currentLightThemeName = model.text

                }
            }

            SkrRoundButton{
                id: darkButton
                display: AbstractButton.IconOnly
                text: SkrTheme.currentDarkThemeName === model.text ? qsTr("Dark") : qsTr("Set as dark theme")
                icon {
                    source: SkrTheme.currentDarkThemeName === model.text ? "qrc:///icons/backup/color-picker-black.svg" : ""
                    color: "transparent"
                }


                onClicked:{
                    SkrTheme.currentDarkThemeName = model.text
                }
            }

            SkrRoundButton{
                id: distractionFreeButton
                display: AbstractButton.IconOnly
                text: SkrTheme.currentDistractionFreeThemeName === model.text ? qsTr("Distraction free") : qsTr("Set as distraction free theme")
                icon.source: SkrTheme.currentDistractionFreeThemeName === model.text ? "qrc:///icons/backup/view-fullscreen.svg" : ""

                onClicked:{
                    SkrTheme.currentDistractionFreeThemeName = model.text

                }

            }

        }
    }

    //-------------------------------------------------------------
    //----- rename theme --------------------------------------------
    //-------------------------------------------------------------


    SimpleDialog {
        id: renameDialog
        property string name: ""
        title: qsTr("Rename a theme")
        contentItem: SkrTextField {
            id: renameTextField
            text: renameDialog.name

            onAccepted: {
                renameDialog.accept()
            }

        }

        standardButtons: Dialog.Ok  | Dialog.Cancel

        onRejected: {
            renameDialog.name = ""

        }

        onDiscarded: {


            renameDialog.name = ""

        }

        onAccepted: {

            SkrTheme.rename(renameDialog.name, renameTextField.text)
            populateThemeListModel()
            renameDialog.name = ""
        }

        onActiveFocusChanged: {
            if(activeFocus){
                contentItem.forceActiveFocus()
            }

        }

        onOpened: {
            contentItem.forceActiveFocus()
            renameTextField.selectAll()
        }

    }

    //-------------------------------------------------------------
    //----- remove theme --------------------------------------------
    //-------------------------------------------------------------


    //--------------------------------------------------------

    removeThemeButton.icon.source: "qrc:///icons/backup/edit-delete.svg"
    removeThemeButton.onClicked: {
        removeDialog.name = selectedTheme
        removeDialog.open()
        populateThemeListModel()
    }


    SimpleDialog {
        id: removeDialog
        property string name: ""
        title: qsTr("Remove a theme")
        text: qsTr("Do you really want to remove the theme \"%1\"?").arg(removeDialog.name)


        standardButtons: Dialog.Ok  | Dialog.Cancel

        onRejected: {
            removeDialog.name = ""

        }

        onDiscarded: {


            removeDialog.name = ""

        }

        onAccepted: {
            SkrTheme.remove(removeDialog.name)
            populateThemeListModel()
            removeDialog.name = ""
        }

        onActiveFocusChanged: {
            if(activeFocus){
                contentItem.forceActiveFocus()
            }

        }

        onOpened: {
            contentItem.forceActiveFocus()
            removeDialog.selectAll()
        }

    }


    //-------------------------------------------------------------
    //------color properties--------------------------------------------
    //-------------------------------------------------------------

    onSelectedThemeChanged: {
        SkrTheme.selectedThemeName = selectedTheme
        populateColorPropertiesListModel()
        removeThemeButton.enabled = SkrTheme.isThemeEditable(selectedTheme)
    }

    ListModel {
        id: colorPropertiesModel
    }
    propertiesListView.model: colorPropertiesModel

    function populateColorPropertiesListModel(){
        colorPropertiesModel.clear()

        var propertyHumanNamesMap = SkrTheme.getPropertyHumanNames()
        var propertyValuesMap = SkrTheme.selectedColorsMap

        var list = []
        for(var name in propertyValuesMap) {
            list.push(name)
        }
        list.sort()



        //is editable ?
        var isEditable = SkrTheme.isThemeEditable(selectedTheme)


        for(var i in list) {
            var exactName = list[i]
            if(SkrTheme.changedColorsMap[exactName]){
                colorPropertiesModel.append({"text": propertyHumanNamesMap[exactName], "propertyExactName": exactName, "color": SkrTheme.changedColorsMap[exactName], "isEditable": isEditable})
            }
            else{
                colorPropertiesModel.append({"text": propertyHumanNamesMap[exactName], "propertyExactName": exactName, "color": propertyValuesMap[exactName], "isEditable": isEditable})

            }


        }






    }


    property string currentProperty: ""


    propertiesListView.delegate: SkrPane {
        id: delegateRoot

        height: 60
        anchors{
            left: Qt.isQtObject(parent) ? parent.left : undefined
            right: Qt.isQtObject(parent) ? parent.right : undefined
        }


        TapHandler {
            id: tapHandler

            onSingleTapped: {
                propertiesListView.currentIndex = model.index
                delegateRoot.forceActiveFocus()
                eventPoint.accepted = true
            }

            onDoubleTapped: {
                //console.log("double tapped")
                propertiesListView.currentIndex = model.index
                currentProperty = model.propertyExactName
                colorCodeTextField.text = model.color

                colorCodeTextField.enabled = model.isEditable

                if(model.isEditable){

                    colorDialog.currentColor = model.color
                    colorDialog.open()
                }

                eventPoint.accepted = true
            }
        }



        RowLayout {
            anchors.fill: parent


            Rectangle {
                id: currentItemIndicator
                color: "lightsteelblue"
                Layout.fillHeight: true
                Layout.preferredWidth: 5
                visible: propertiesListView.currentIndex === model.index
            }
            Rectangle {
                id: openedItemIndicator
                color: SkrTheme.accent
                Layout.fillHeight: true
                Layout.preferredWidth: 5
                visible: model.propertyExactName === currentProperty
            }


            SkrLabel {
                id: colorPropertyLabel
                text: model.text
                Layout.preferredHeight: 30
            }

            Item {
                id: stretcher
                Layout.fillWidth: true
            }

            Rectangle {
                id: chosenColorRectangle
                Layout.preferredHeight: 40
                Layout.preferredWidth: 40

                color: model.color
                border.color: "black"
                border.width: 1

            }

            SkrToolButton {


                Layout.preferredHeight: 40
                Layout.preferredWidth: 40

                id: editColorToolButton
                text: qsTr("Edit color")
                flat: true
                display: AbstractButton.IconOnly
                enabled: model.isEditable

                icon.source: "qrc:///icons/backup/edit-rename.svg"

                onClicked: {
                    propertiesListView.currentIndex = model.index
                    currentProperty = model.propertyExactName
                    colorCodeTextField.text = model.color

                    colorDialog.currentColor = model.color
                    colorDialog.open()
                }
            }


        }
    }




    //-------------------------------------------------------------------------
    //-------Examples ----------------------------------------------------------
    //-------------------------------------------------------------------------

    forceLightButton.onClicked:{
        SkrTheme.forceCurrentColorMode(SKRThemes.Light)
    }

    forceDarkButton.onClicked:{
        SkrTheme.forceCurrentColorMode(SKRThemes.Dark)
    }

    forceDistractionFreeButton.onClicked:{
        SkrTheme.forceCurrentColorMode(SKRThemes.DistractionFree)
    }

    primaryTextAreaSample.textArea.text: qsTr("Primary text %1").arg("erroor")




    //-------------------------------------------------------------------------
    //-------------------------------------------------------------------------
    //-------------------------------------------------------------------------

    resetThemeButton.onClicked: {
        SkrTheme.resetColorProperties()
        populateColorPropertiesListModel()
    }

    saveThemeButton.onClicked: {
        SkrTheme.saveColorProperties()

        // shake to apply highlight color
        primaryTextAreaSample.textArea.width += 1
        primaryTextAreaSample.textArea.width -= 1
    }


    //-------------------------------------------------------------------------
    //-------color picker----------------------------------------------------
    //-------------------------------------------------------------------------

    colorCodeTextField.onEditingFinished: {
        SkrTheme.setColorProperty(currentProperty, colorCodeTextField.text)
        populateColorPropertiesListModel()
    }


    LabPlatform.ColorDialog {
        id: colorDialog


        onAccepted: {
            var colorString = skrQMLTools.colorString(colorDialog.color)
            SkrTheme.setColorProperty(currentProperty, colorString)
            populateColorPropertiesListModel()
        }
    }

}
