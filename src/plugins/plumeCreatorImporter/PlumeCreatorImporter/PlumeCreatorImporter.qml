import QtQuick
import QtQml
import QtQuick.Controls
import Qt.labs.platform 1.1 as LabPlatform
import eu.skribisto.plumecreatorimporter 1.0

import "../../Items"
import "../../Commons"
import "../.."

PlumeCreatorImporterForm {
    id: root


    property string plumeFileName: ""
    property string targetFileName: ""
    signal closeCalled()

    SKRPlumeCreatorImporter{
        id: plumeCreatorImporter
    }

    goBackToolButton.icon.source: "qrc:///icons/backup/go-previous.svg"
    signal goBackButtonClicked()
    goBackToolButton.onClicked: goBackButtonClicked()


    selectPlumeProjectFileToolButton.onClicked: {
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

            plumeCreatorImporter.importPlumeCreatorProject(plumeFileName, targetFileName)
            closeCalled()
        }
    }





    onActiveFocusChanged: {
        if (activeFocus) {
            selectPlumeProjectFileToolButton.forceActiveFocus()
        }
    }

}
