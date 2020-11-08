import QtQuick 2.15
import Qt.labs.platform 1.1 as LabPlatform
import QtQml 2.15
import eu.skribisto.plmerror 1.0
import "../Items"
import ".."

NewProjectForm {
    id: root


    property string fileName: fileName
    property url folderNameURL

    goBackToolButton.icon.name: "go-previous"
    signal goBackButtonClicked()
    goBackToolButton.onClicked:  goBackButtonClicked()


    selectProjectPathToolButton.onClicked: {
        folderDialog.open()
        folderDialog.currentFolder = LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)


    }


    LabPlatform.FolderDialog{
        id: folderDialog
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)

        onAccepted: {
            folderNameURL = folderDialog.currentFolder
            projectPathTextField.text = skrQMLTools.translateURLToLocalFile(folderNameURL)
        }
        onRejected: {

        }



    }

    property bool projectFileTextFiledEdited: false
    // title :
    projectTitleTextField.onTextChanged: {
        if(!projectFileTextFiledEdited){
            var name = projectTitleTextField.text

            name = name.replace(/[\"\/\%\(\)|.'?!$#\n\r]/g, "");

            projectFileTextField.text = name
        }

    }



    //file :
    projectFileTextField.validator: RegExpValidator { regExp: /^[^ ][\w\s]{1,60}$/ }
    projectFileTextField.onTextChanged: {
        var file = projectPathTextField.text + "/" + projectFileTextField.text + ".skrib"


        fileName = skrQMLTools.setURLScheme(Qt.resolvedUrl(file), "file")
    }
    projectFileTextField.onTextEdited: {
        projectFileTextFiledEdited = true
    }

    // path :
    projectPathTextField.text: {

        var path = skrQMLTools.translateURLToLocalFile(LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation))
        //
        //        path = path.replace(/^(file:\/{2})/,"")

        return path


    }
    projectPathTextField.onTextChanged: {

        folderNameURL = skrQMLTools.setURLScheme(Qt.resolvedUrl(projectPathTextField.text), "file")

        var result = Qt.resolvedUrl(folderNameURL + "/" + projectFileTextField.text + ".skrib")
        //result.protocol = "file"
        fileName = result
    }


    onFileNameChanged: {
        console.log("onFileNameChanged",fileName.toString() )
        projectDetailPathLabel.text = skrQMLTools.translateURLToLocalFile(fileName)
    }



    // create :

    createNewProjectButton.onClicked: {
        //TODO: test fileName


        plmData.projectHub().createNewEmptyProject(fileName)

        var projetId = plmData.projectHub().getLastLoaded()
        console.log("new project : getLastLoaded : ", projetId)
        plmData.projectHub().setProjectName(projetId, projectTitleTextField.text)

        var firstSheetId = -2
        for(var i = 1; i <= partSpinBox.value ; ++i){
            var error = plmData.sheetHub().addChildPaper(projetId, -1)
            console.log("new project : add sheet : ", error.isSuccess())
            var sheetId = plmData.sheetHub().getLastAddedId()
            plmData.sheetHub().setTitle(projetId, sheetId, qsTr("Part ") + i)

            if(sheetId === 1){
                firstSheetId = sheetId
            }

        }

        swipeView.currentIndex = 0
        //root_stack.currentIndex = 1
        Globals.openSheetInNewTabCalled(projetId, firstSheetId)

        //reset :
        projectTitleTextField.text = ""
        projectFileTextField.text = ""
        projectFileTextFiledEdited = false
        projectPathTextField.text = skrQMLTools.translateURLToLocalFile(LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation))
    }



    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            newProjectButton.forceActiveFocus()
        }
    }


}
