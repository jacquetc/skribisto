import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.platform 1.1 as LabPlatform
import eu.skribisto.themes 1.0
import ".."
import "../Items"



ThemePageForm {
    property string pageType: "theme"
    property string title: "Themes"
    clip: true

    Component.onCompleted: {
        populateThemeListModel()
        populateColorPropertiesListModel()
    }

    //-------------------------------------------------------------
    //-----theme list-----------------------------------------------
    //-------------------------------------------------------------

    addThemeButton.icon.source: "qrc:///icons/backup/edit-copy.svg"
    addThemeButton.onClicked: {
        SkrTheme.duplicate(currentTheme, qsTr("%1 (copy)".arg(currentTheme)))
        populateThemeListModel()
    }

    //--------------------------------------------------------

    removeThemeButton.icon.source: "qrc:///icons/backup/edit-delete.svg"
    removeThemeButton.onClicked: {
        SkrTheme.remove(currentlySelectedTheme)
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


        var i
        for(i = 0 ; i < list.length ; i++) {
            var isEditable = SkrTheme.isThemeEditable(list[i])
            var name = list[i]

            themeListModel.append({"text": name, "isEditable": isEditable})


        }
    }

    //--------------------------------------------------------

    property string currentTheme: SkrTheme.currentThemeName
    property string currentlySelectedTheme: ""


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
                currentlySelectedTheme = model.text
                themeDelegateRoot.forceActiveFocus()
                eventPoint.accepted = true
            }

            onDoubleTapped: {
                //console.log("double tapped")
                themeListView.currentIndex = model.index
                currentlySelectedTheme = model.text
                SkrTheme.currentThemeName = model.text

                eventPoint.accepted = true
            }
        }



        RowLayout {
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
                visible: model.text === currentTheme
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
                text: qsTr("Edit theme")
                flat: true
                display: AbstractButton.IconOnly
                enabled: model.isEditable

                icon.source: "qrc:///icons/backup/edit-rename.svg"

                onClicked: {

                    themeListView.currentIndex = model.index
                    currentlySelectedTheme = model.text
                    SkrTheme.currentThemeName = model.text
                }
            }


        }
    }
    //-------------------------------------------------------------
    //------color properties--------------------------------------------
    //-------------------------------------------------------------

    onCurrentThemeChanged: {
        populateColorPropertiesListModel()
    }

    ListModel {
        id: colorPropertiesModel
    }
    propertiesListView.model: colorPropertiesModel

    function populateColorPropertiesListModel(){
        colorPropertiesModel.clear()

        var propertyHumanNamesList = SkrTheme.getPropertyHumanNames()
        var propertyExactNamesList = SkrTheme.getColorPropertyExactNames()
        var propertyValuesList = SkrTheme.getColorPropertyValues()

        var df_propertyHumanNamesList = []
        var df_propertyExactNamesList = []
        var df_propertyValuesList = []

        //is editable ?
        var isEditable = SkrTheme.isThemeEditable(currentTheme)

        //sort by distractionFree or not, add normal to model

        var i
        for(i = 0 ; i < propertyExactNamesList.length ; i++) {
            var isDistractionFree = propertyExactNamesList[i].slice(0, 15) === "distractionFree"

            if(isDistractionFree){
                df_propertyHumanNamesList.push(propertyHumanNamesList[i])
                df_propertyExactNamesList.push(propertyExactNamesList[i])
                df_propertyValuesList.push(propertyValuesList[i])
            }

            else {
                var isDistractionFreeString = "false"
                colorPropertiesModel.append({"text": propertyHumanNamesList[i], "propertyExactName": propertyExactNamesList[i], "color": propertyValuesList[i], "isDistractionFree": isDistractionFreeString, "isEditable": isEditable})
            }
        }

        //add distraction free to model

        var k
        for(k = 0 ; k < df_propertyExactNamesList.length ; k++) {

            colorPropertiesModel.append({"text": df_propertyHumanNamesList[k], "propertyExactName": df_propertyExactNamesList[k], "color": df_propertyValuesList[k], "isDistractionFree": "true", "isEditable": isEditable})

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



    propertiesListView.section.property: "isDistractionFree"
    propertiesListView.section.criteria: ViewSection.FullString
    propertiesListView.section.labelPositioning: ViewSection.CurrentLabelAtStart |
                                                 ViewSection.InlineLabels
    propertiesListView.section.delegate: propertiesSectionHeading

    // The delegate for each section header
    Component {
        id: propertiesSectionHeading
        Rectangle {
            width: propertiesListView.width
            height: childrenRect.height
            color: SkrTheme.buttonBackground

            required property string section

            SkrLabel {
                text: section === "true" ? qsTr("Distraction free") : qsTr("Normal")
                font.bold: true
                color: SkrTheme.buttonForeground
                //font.pixelSize: 20
            }
        }
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

            SkrTheme.setColorProperty(currentProperty, colorDialog.color)
            populateColorPropertiesListModel()
        }
    }

}
