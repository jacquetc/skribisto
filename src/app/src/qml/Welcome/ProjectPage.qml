import QtQuick 2.4
import ".."

ProjectPageForm {

    //compact mode :
    gridLayout.columns: Globals.compactSize ? 1 : 3

    
    
    createEmpyProjectAtStartSwitch.checked: SkrSettings.welcomeSettings.createEmptyProjectAtStart
    Binding {
        target: SkrSettings.welcomeSettings
        property: "createEmptyProjectAtStart"
        value: createEmpyProjectAtStartSwitch.checked
    }
    
    
    
    //new_project.onClicked:
    onActiveFocusChanged: {
        if (activeFocus) {

            //sssss.forceActiveFocus()
        }
    }
}
