import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import ".."
import SkrControls
import eu.skribisto.pluginhub 1.0
import eu.skribisto.skr 1.0

PluginPageForm {

    property alias pluginModel: pluginModel
    ListModel{
        id: pluginModel
    }
    DelegateModel{
        id: visualModel
        model: pluginModel
        delegate: controlComponent

        groups: [
            DelegateModelGroup {
                name: "selected"
                onChanged: {
                    personalizedPluginGroupButton.checked = true
                }
            }
        ]

    }

    Component.onCompleted: {
        var pluginsNameList = skrData.pluginHub().listAllByName(true)

        for(var i in pluginsNameList){
            var pluginName = pluginsNameList[i]
            var enabled = skrData.pluginHub().isThisPluginEnabled(pluginName)
            var displayedName = skrData.pluginHub().getDisplayedName(pluginName)
            var mandatory = skrData.pluginHub().getMandatory(pluginName)
            var pluginGroup = skrData.pluginHub().getPluginGroup(pluginName)
            var pluginGroupText = determinePluginGroupText(pluginGroup)
            var pluginSelectionGroup = skrData.pluginHub().getPluginSelectionGroup(pluginName)
            var use = skrData.pluginHub().getUse(pluginName)

            if(!mandatory ||     SkrSettings.devSettings.devModeEnabled){
                pluginModel.append({"pluginName": pluginName, "displayedName": displayedName, "enabled": enabled, "mandatory": mandatory,
                                       "pluginGroup": pluginGroup, "pluginGroupText": pluginGroupText, "pluginSelectionGroup": pluginSelectionGroup, "use": use })
            }
        }


        pluginGroupComboBox.built = true
    }

    Component.onDestruction: {

        for(var i = 0; i < pluginModel.count ; i++){
            var pluginName = pluginModel.get(i).pluginName

            skrData.pluginHub().setPluginEnabled(pluginName, pluginModel.get(i).enabled)


        }
    }


    function determinePluginGroupText(pluginGroupEnum){
        var pluginGroupText
        switch (pluginGroupEnum) {
        case "PageToolbox":
            pluginGroupText = qsTr("Page Toolboxes")
            break;
        case "ProjectToolbox":
            pluginGroupText = qsTr("Project Toolboxes")
            break;
        case "Page":
            pluginGroupText = qsTr("Pages")
            break;
        case "ProjectPage":
            pluginGroupText = qsTr("Project Pages")
            break;
        case "Importer":
            pluginGroupText = qsTr("Importers")
            break;
        case "Example":
            pluginGroupText = qsTr("Examples")
            break;
        case "Template":
            pluginGroupText = qsTr("Templates")
            break;
        default:
            pluginGroupText = "";
        }

        return pluginGroupText;
    }

    pluginList.model: visualModel

    //----------------------------------------------------------------------------
    pluginList.section.property: "pluginGroupText"
    pluginList.section.criteria: ViewSection.FullString
    pluginList.section.labelPositioning: ViewSection.CurrentLabelAtStart
                                         | ViewSection.InlineLabels
    pluginList.section.delegate: sectionHeading

    // The delegate for each section header
    Component {
        id: sectionHeading
        Rectangle {
            width: pluginList.width
            height: childrenRect.height
            color: SkrTheme.buttonBackground

            required property string section

            SkrLabel {
                anchors.left: parent.left
                anchors.right: parent.right
                activeFocusOnTab: false
                text: parent.section
                font.bold: true
                horizontalAlignment: Qt.AlignHCenter
                color: SkrTheme.buttonForeground
            }
        }
    }
    Component{
        id: controlComponent
        ItemDelegate {
            id: control


            property string pluginName: model.pluginName

            onClicked: {
                pluginList.currentIndex = model.index
                control.forceActiveFocus()
                descriptionLabel.text = model.use
            }

            contentItem:
                RowLayout{

                Rectangle {
                    id: currentItemIndicator
                    color: "lightsteelblue"
                    Layout.fillHeight: true
                    Layout.preferredWidth: 5
                    visible: model.index === pluginList.currentIndex
                }

                SkrLabel{
                    rightPadding: control.indicator.width + control.spacing
                    text: model.mandatory ? qsTr("%1 (mandatory)").arg(model.displayedName) : model.displayedName
                    verticalAlignment: Text.AlignVCenter
                    font.italic: model.mandatory
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignCenter

                }
            }

            indicator: SkrSwitch{
                x: control.width - width - control.rightPadding
                y: parent.height / 2 - height / 2
                checked: model.enabled
                onToggled: {
                    control.DelegateModel.inSelected = true
                }

                onCheckedChanged: {
                    model.enabled = checked
                }

                focusPolicy: Qt.NoFocus
            }



        }
    }

    function selectPluginSelectionGroup(pluginSelectionGroup){
        for(var i = 0; i < pluginModel.count ; i++){
            if(pluginModel.get(i).pluginSelectionGroup === "Mandatory" ||
                    pluginModel.get(i).pluginSelectionGroup === "Common" ||
                    pluginModel.get(i).pluginSelectionGroup === pluginSelectionGroup){
                pluginModel.setProperty(i, "enabled", true)
            }
            else {
                pluginModel.setProperty(i, "enabled", false)
            }

        }
    }

    pluginGroupComboBox.onCurrentValueChanged: {

        if(!pluginGroupComboBox.built){
            return
        }

        var i = 0
        var item
        var group = pluginGroupComboBox.currentValue
        switch (group) {
        case "Writing":
            selectPluginSelectionGroup(group)
            break;
        case "Notes":
            selectPluginSelectionGroup(group)
            break;
        case "Minimum":
            for(i = 0; i < pluginModel.count ; i++){
                item = pluginModel.get(i)
                if(pluginModel.get(i).pluginSelectionGroup === "Mandatory"||
                        pluginModel.get(i).pluginSelectionGroup === "Common"){
                    pluginModel.setProperty(i, "enabled", true)
                }
                else {
                    pluginModel.setProperty(i, "enabled", false)
                }
            }
            break;
        case "Empty":
            for(i = 0; i < pluginModel.count ; i++){
                item = pluginModel.get(i)
                if(pluginModel.get(i).pluginSelectionGroup !== "Mandatory"){
                    pluginModel.setProperty(i, "enabled", false)
                }
            }
            break;
        case "All":
            for(i = 0; i < pluginModel.count ; i++){
                item = pluginModel.get(i)
                if(pluginModel.get(i).pluginSelectionGroup !== "Mandatory"){
                    pluginModel.setProperty(i, "enabled", true)
                }
            }
            break;
        case "Personalized":
            break;
        default:
            break;
        }
    }



    //--------------------------------------------------------

    writingPluginGroupButton.onCheckedChanged: {
        if(writingPluginGroupButton.checked){
            pluginGroupComboBox.currentIndex = pluginGroupComboBox.indexOfValue("Writing")
        }
    }


    //--------------------------------------------------------

    notesPluginGroupButton.onCheckedChanged: {
        if(notesPluginGroupButton.checked){
            pluginGroupComboBox.currentIndex = pluginGroupComboBox.indexOfValue("Notes")
        }
    }
    //--------------------------------------------------------

    personalizedPluginGroupButton.onCheckedChanged: {
        if(personalizedPluginGroupButton.checked){
            pluginGroupComboBox.currentIndex = pluginGroupComboBox.indexOfValue("Personalized")
        }
    }

    //--------------------------------------------------------

    minimumPluginGroupButton.onCheckedChanged: {
        if(minimumPluginGroupButton.checked){
            pluginGroupComboBox.currentIndex = pluginGroupComboBox.indexOfValue("Minimum")
        }
    }
    //--------------------------------------------------------

    emptyPluginGroupButton.onCheckedChanged: {
        if(emptyPluginGroupButton.checked){
            pluginGroupComboBox.currentIndex = pluginGroupComboBox.indexOfValue("Empty")
        }
    }

    //--------------------------------------------------------

    allPluginGroupButton.onCheckedChanged: {
        if(allPluginGroupButton.checked){
            pluginGroupComboBox.currentIndex = pluginGroupComboBox.indexOfValue("All")
        }
    }

}
