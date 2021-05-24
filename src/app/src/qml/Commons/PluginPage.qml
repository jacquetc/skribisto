import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."
import "../Items"
import eu.skribisto.pluginhub 1.0

PluginPageForm {

    property alias pluginModel: pluginModel
    ListModel{
        id: pluginModel
    }

    Component.onCompleted: {
        var pluginsNameList = skrData.pluginHub().listAllByName()

        for(var i in pluginsNameList){
            var pluginName = pluginsNameList[i]
            var enabled = skrData.pluginHub().isThisPluginEnabled(pluginName)
            var displayedName = skrData.pluginHub().getDisplayedName(pluginName)
            var mandatory = skrData.pluginHub().getMandatory(pluginName)

            if(!mandatory ||     SkrSettings.devSettings.devModeEnabled){
                pluginModel.append({"pluginName": pluginName, "displayedName": displayedName, "enabled": enabled, "mandatory": mandatory})
            }
        }
    }

    pluginList.model: pluginModel

    pluginList.delegate: SwitchDelegate {
        id: control


        text: model.displayedName
        checked: model.enabled

        property string pluginName: model.pluginName


        contentItem: SkrLabel{
            rightPadding: control.indicator.width + control.spacing
            text: model.mandatory ? qsTr("%1 (mandatory)").arg(control.text) : control.text
            verticalAlignment: Text.AlignVCenter
            font.italic: model.mandatory

        }

        indicator: SkrSwitch{
            x: control.width - width - control.rightPadding
            y: parent.height / 2 - height / 2
            checked: control.checked



            onCheckedChanged: {
                model.enabled = checked
            }
        }



    }



    //--------------------------------------------------------

    classicThemeButton.onClicked: {
        pluginThemeComboBox.currentIndex = 0

    }

    //--------------------------------------------------------

    writingThemeButton.onClicked: {
        pluginThemeComboBox.currentIndex = 1
    }
    //--------------------------------------------------------

    personalizedThemeButton.onClicked: {
        pluginThemeComboBox.currentIndex = 2
    }


}
