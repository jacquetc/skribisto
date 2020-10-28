import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.platform 1.1 as LabPlatform
import QtQml 2.15
import eu.skribisto.plmerror 1.0
import "../Items"
import "../Commons"
import ".."

PlumeImporterForm {

    property string plumeFileName: ""
    property string targetFileName: ""

    goBackToolButton.icon.name: "go-previous"
    signal goBackButtonClicked()
    goBackToolButton.onClicked: goBackButtonClicked()

    selectPlumeProjectFileToolButton.onClicked:   {
        fileDialog.open()
        fileDialog.folder = LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)


    }

    LabPlatform.FileDialog{
        id: fileDialog
        nameFilters: ["Plume Creator project file (*.plume)"]
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)


        onAccepted: {
            plumeFileName = fileDialog.file
            plumeProjectFileTextField.text = skrQMLTools.translateURLToLocalFile(plumeFileName)

            plumeProjectDetailPathLabel.text = plumeProjectFileTextField.text.replace(".plume", ".skrib")
            targetFileName = skrQMLTools.getURLFromLocalFile(plumeProjectDetailPathLabel.text)

        }
        onRejected: {

        }
    }

    importPlumeProjectButton.enabled: plumeProjectFileTextField.length !== 0
    importPlumeProjectButton.onClicked: {

        if(plumeProjectFileTextField.text.length !== 0){

        plmData.projectHub().importPlumeCreatorProject(plumeFileName, targetFileName)
        }
    }

}
