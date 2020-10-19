import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import '..'

SheetOverviewToolForm {
    property int minimumHeight: 500

    //-------------------------------------------------------------------------------
    //-----Project-------------------------------------------------------------------
    //-------------------------------------------------------------------------------
    property int currentProjectId: -2

    ListModel {
        id: projectComboBoxModel
    }

    projectComboBox.model: projectComboBoxModel
    projectComboBox.textRole: "projectName"
    projectComboBox.valueRole: "projectId"


    function populateProjectComboBox(){
        projectComboBoxModel.clear()

        var projectIdList = plmData.projectHub().getProjectIdList()

        if(projectIdList.length === 0){
            return
        }

        // populate model
        var i;
        for(i = 0 ; i < projectIdList.length ; i++){
            projectComboBoxModel.append({"projectName": plmData.projectHub().getProjectName(projectIdList[i]), "projectId": projectIdList[i]})
        }

        // set value
        if(currentProjectId == -2){

            var value = plmData.projectHub().getActiveProject()
            projectComboBox.currentIndex = projectComboBox.indexOfValue(value)
            currentProjectId = value
            Globals.sheetOverviewCurrentProjectId = currentProjectId
        }
        else {
            var index = projectComboBox.indexOfValue(currentProjectId)

            if(index === -1){ // not found (maybe closed)
                value = plmData.projectHub().getActiveProject()
                projectComboBox.currentIndex = projectComboBox.indexOfValue(value)
                currentProjectId = value
                Globals.sheetOverviewCurrentProjectId = currentProjectId
            }
            else {
                projectComboBox.currentIndex = index
                Globals.sheetOverviewCurrentProjectId = currentProjectId
            }

        }

    }


    projectComboBox.onCurrentValueChanged: {
        if(projectComboBox.activeFocus){
            currentProjectId = projectComboBox.currentValue
            Globals.sheetOverviewCurrentProjectId = currentProjectId

        }
    }

    Connections{
        target: Globals
        enabled: !projectComboBox.activeFocus
        function onSheetOverviewCurrentProjectIdChanged() {
            var value = Globals.sheetOverviewCurrentProjectId
            projectComboBox.currentIndex = projectComboBox.indexOfValue(value)
        }
    }


    Connections{
        target: plmData.projectHub()
        function onProjectLoaded(projectId) {
            populateProjectComboBox()
        }
    }


    Connections{
        target: plmData.projectHub()
        function onProjectNameChanged(projectId, projectName) {

            var index = projectComboBox.indexOfValue(projectId)
            projectComboBoxModel.setProperty(index, "projectName", projectName)
        }
    }

    //-------------------------------------------------------------------------------
    //-----Display------------------------------------------------------------------
    //-------------------------------------------------------------------------------

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
            projectGroupBox.forceActiveFocus()
        }
    }
}
