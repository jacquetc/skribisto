import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import eu.skribisto.propertyhub 1.0
import "../Commons"
import "../Items"
import ".."

PropertyPadForm {
    id: root

    property int projectId: -2
    property int treeItemId: -2

    onProjectIdChanged: {
        if(projectId === -2 || treeItemId === -2 ){
            return
        }

        determinePrintableToolButton()
        determineModifiableToolButton()
        determineFavoriteToolButton()
    }
    onTreeItemIdChanged: {
        if(projectId === -2 || treeItemId === -2 ){
            return
        }

        determinePrintableToolButton()
        determineModifiableToolButton()
        determineFavoriteToolButton()
    }

    Connections{
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value){
            if(projectId === root.projectId && treeItemId === root.treeItemId){
                if(name === "printable"){
                    determinePrintableToolButton()
                }
                if(name === "modifiable"){
                    determineModifiableToolButton()
                }
                if(name === "favorite"){
                    determineFavoriteToolButton()
                }
            }
        }
    }



    QtObject{
        id: switchPrivate
        property bool updateBlocked: false
    }

    // printableToolButton :

    function determinePrintableToolButton(){
        switchPrivate.updateBlocked = true

        printableToolButton.checked = skrData.treePropertyHub().getProperty(projectId, treeItemId, "printable", "true") === "true"

        switchPrivate.updateBlocked = false
    }

    printableToolButton.onCheckedChanged: {
        if(switchPrivate.updateBlocked){
            return
        }
        var checked = printableToolButton.checked ? "true" : "false"
        skrData.treePropertyHub().setProperty(projectId, treeItemId, "printable", checked)

        var childrenList = skrData.treeHub().getAllChildren(projectId, treeItemId)

        childrenList.forEach(function(childId, index, array) {
            skrData.treePropertyHub().setProperty(projectId, childId, "printable", checked)
        })
    }
    printableToolButton.icon.source: "qrc:///icons/backup/document-print.svg"



    // modifiableToolButton :

    function determineModifiableToolButton(){
        switchPrivate.updateBlocked = true

        modifiableToolButton.checked = skrData.treePropertyHub().getProperty(projectId, treeItemId, "modifiable", "true") === "true"

        switchPrivate.updateBlocked = false
    }

    modifiableToolButton.onCheckedChanged: {
        if(switchPrivate.updateBlocked){
            return
        }
        var checked = modifiableToolButton.checked ? "true" : "false"
        skrData.treePropertyHub().setProperty(projectId, treeItemId, "modifiable", checked)

        var childrenList = skrData.treeHub().getAllChildren(projectId, treeItemId)

        childrenList.forEach(function(childId, index, array) {
            skrData.treePropertyHub().setProperty(projectId, childId, "modifiable", checked)
        })

    }
    modifiableToolButton.icon.source: "qrc:///icons/backup/edit-rename.svg"



    // favoriteToolButton :


    Action{
        id: favoriteAction
        checkable: true
        icon.source: "qrc:///icons/backup/favorite.svg"
        onCheckedChanged: {
            if(switchPrivate.updateBlocked){
                return
            }
            var checked = favoriteAction.checked ? "true" : "false"
            skrData.treePropertyHub().setProperty(projectId, treeItemId, "favorite", checked)

        }
    }
    favoriteToolButton.action: favoriteAction

    function determineFavoriteToolButton(){
        switchPrivate.updateBlocked = true

        favoriteAction.checked = skrData.treePropertyHub().getProperty(projectId, treeItemId, "favorite", "false") === "true"

        switchPrivate.updateBlocked = false
    }




}
